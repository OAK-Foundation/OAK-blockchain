// This file is part of OAK Blockchain.

// Copyright (C) 2022 OAK Network
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! # Automation time pallet
//!
//! DISCLAIMER: This pallet is still in it's early stages. At this point
//! we only support scheduling two tasks per minute, and sending an on-chain
//! with a custom message.
//!
//! This pallet allows a user to schedule tasks. Tasks can scheduled for any whole minute in the future.
//! In order to run tasks this pallet consumes up to a certain amount of weight during `on_initialize`.
//!
//! The pallet supports the following tasks:
//! * On-chain events with custom text
//!

#![cfg_attr(not(feature = "std"), no_std)]
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

mod benchmarking;
pub mod weights;

mod exchange;
pub use exchange::*;

use core::convert::TryInto;
use frame_support::{pallet_prelude::*, sp_runtime::traits::Hash, BoundedVec};
use frame_system::pallet_prelude::*;
use pallet_timestamp::{self as timestamp};
use scale_info::TypeInfo;
use sp_runtime::{
	traits::{SaturatedConversion, Saturating},
	Perbill,
};
use sp_std::{vec, vec::Vec};

pub use weights::WeightInfo;

#[frame_support::pallet]
pub mod pallet {
	use super::*;

	pub type AccountOf<T> = <T as frame_system::Config>::AccountId;
	pub type BalanceOf<T> = <<T as Config>::NativeTokenExchange as NativeTokenExchange<T>>::Balance;
	type UnixTime = u64;

	/// The enum that stores all action specific data.
	#[derive(Debug, Eq, PartialEq, Encode, Decode, TypeInfo)]
	#[scale_info(skip_type_params(T))]
	pub enum Action<T: Config> {
		Notify { message: Vec<u8> },
		NativeTransfer { sender: AccountOf<T>, recipient: AccountOf<T>, amount: BalanceOf<T> },
	}

	/// The struct that stores all information needed for a task.
	#[derive(Debug, Eq, PartialEq, Encode, Decode, TypeInfo)]
	#[scale_info(skip_type_params(T))]
	pub struct Task<T: Config> {
		owner_id: AccountOf<T>,
		provided_id: Vec<u8>,
		time: UnixTime,
		action: Action<T>,
	}

	impl<T: Config> Task<T> {
		pub fn create_event_task(
			owner_id: AccountOf<T>,
			provided_id: Vec<u8>,
			time: UnixTime,
			message: Vec<u8>,
		) -> Task<T> {
			let action = Action::Notify { message };
			Task::<T> { owner_id, provided_id, time, action }
		}
		pub fn create_native_transfer_task(
			owner_id: AccountOf<T>,
			provided_id: Vec<u8>,
			time: UnixTime,
			recipient_id: AccountOf<T>,
			amount: BalanceOf<T>,
		) -> Task<T> {
			let action = Action::NativeTransfer {
				sender: owner_id.clone(),
				recipient: recipient_id,
				amount,
			};
			Task::<T> { owner_id, provided_id, time, action }
		}
	}

	#[derive(Debug, Encode, Decode, TypeInfo)]
	#[scale_info(skip_type_params(T))]
	pub struct TaskHashInput<T: Config> {
		owner_id: AccountOf<T>,
		provided_id: Vec<u8>,
	}

	impl<T: Config> TaskHashInput<T> {
		pub fn create_hash_input(owner_id: AccountOf<T>, provided_id: Vec<u8>) -> TaskHashInput<T> {
			TaskHashInput::<T> { owner_id, provided_id }
		}
	}

	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_timestamp::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		/// Weight information for the extrinsics in this module.
		type WeightInfo: WeightInfo;

		/// The maximum number of tasks that can be scheduled for a time slot.
		#[pallet::constant]
		type MaxTasksPerSlot: Get<u32>;

		/// The farthest out a task can be scheduled.
		#[pallet::constant]
		type MaxScheduleSeconds: Get<u64>;

		/// The maximum weight per block.
		#[pallet::constant]
		type MaxBlockWeight: Get<Weight>;

		/// The maximum percentage of weight per block used for scheduled tasks.
		#[pallet::constant]
		type MaxWeightPercentage: Get<Perbill>;

		/// The maximum percentage of weight per block used for scheduled tasks.
		#[pallet::constant]
		type UpdateQueueRatio: Get<Perbill>;

		/// The time each block takes.
		#[pallet::constant]
		type SecondsPerBlock: Get<u64>;

		#[pallet::constant]
		type ExecutionWeightFee: Get<BalanceOf<Self>>;

		/// Handler for fees and native token transfers.
		type NativeTokenExchange: NativeTokenExchange<Self>;
	}

	#[pallet::pallet]
	#[pallet::without_storage_info]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn get_scheduled_tasks)]
	pub type ScheduledTasks<T: Config> =
		StorageMap<_, Twox64Concat, u64, BoundedVec<T::Hash, T::MaxTasksPerSlot>>;

	#[pallet::storage]
	#[pallet::getter(fn get_task)]
	pub type Tasks<T: Config> = StorageMap<_, Twox64Concat, T::Hash, Task<T>>;

	#[pallet::storage]
	#[pallet::getter(fn get_task_queue)]
	pub type TaskQueue<T: Config> = StorageValue<_, Vec<T::Hash>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn get_missed_queue)]
	pub type MissedQueue<T: Config> = StorageValue<_, Vec<T::Hash>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn get_last_slot)]
	pub type LastTimeSlot<T: Config> = StorageValue<_, (UnixTime, UnixTime)>;

	#[pallet::storage]
	#[pallet::getter(fn is_shutdown)]
	pub type Shutdown<T: Config> = StorageValue<_, bool, ValueQuery>;

	#[pallet::error]
	pub enum Error<T> {
		/// Time must end in a whole minute.
		InvalidTime,
		/// Time must be in the future.
		PastTime,
		/// Time cannot be too far in the future.
		TimeTooFarOut,
		/// The message cannot be empty.
		EmptyMessage,
		/// The provided_id cannot be empty
		EmptyProvidedId,
		/// There can be no duplicate tasks.
		DuplicateTask,
		/// Time slot is full. No more tasks can be scheduled for this time.
		TimeSlotFull,
		/// You are not the owner of the task.
		NotTaskOwner,
		/// The task does not exist.
		TaskDoesNotExist,
		/// Block time not set.
		BlockTimeNotSet,
		/// Amount has to be larger than 0.1 OAK.
		InvalidAmount,
		/// Sender cannot transfer money to self.
		TransferToSelf,
		/// Insufficient balance to pay execution fee.
		InsufficientBalance,
		/// Account liquidity restrictions prevent withdrawal.
		LiquidityRestrictions,
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Schedule task success.
		TaskScheduled {
			who: AccountOf<T>,
			task_id: T::Hash,
		},
		// Cancelled a task.
		TaskCancelled {
			who: AccountOf<T>,
			task_id: T::Hash,
		},
		/// Notify event for the task.
		Notify {
			message: Vec<u8>,
		},
		/// A Task was not found.
		TaskNotFound {
			task_id: T::Hash,
		},
		/// Succcessfully transferred funds
		SuccesfullyTransferredFunds {
			task_id: T::Hash,
		},
		/// Transfer Failed
		TransferFailed {
			task_id: T::Hash,
			error: DispatchError,
		},
		/// The task could not be run at the scheduled time.
		TaskMissed {
			who: T::AccountId,
			task_id: T::Hash,
		},
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		fn on_initialize(_: T::BlockNumber) -> Weight {
			if Self::is_shutdown() == true {
				return T::DbWeight::get().reads(1 as Weight);
			}

			let max_weight: Weight = T::MaxWeightPercentage::get().mul_floor(T::MaxBlockWeight::get());
			Self::trigger_tasks(max_weight)
		}
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Schedule a task to fire an event with a custom message.
		///
		/// Before the task can be scheduled the task must past validation checks.
		/// * The transaction is signed
		/// * The provided_id's length > 0
		/// * The message's length > 0
		/// * The time is valid
		///
		/// # Parameters
		/// * `provided_id`: An id provided by the user. This id must be unique for the user.
		/// * `time`: The unix standard time in seconds for when the task should run.
		/// * `message`: The message you want the event to have.
		///
		/// # Errors
		/// * `InvalidTime`: Time must end in a whole minute.
		/// * `PastTime`: Time must be in the future.
		/// * `EmptyMessage`: The message cannot be empty.
		/// * `DuplicateTask`: There can be no duplicate tasks.
		/// * `TimeSlotFull`: Time slot is full. No more tasks can be scheduled for this time.
		#[pallet::weight(<T as Config>::WeightInfo::schedule_notify_task_full())]
		pub fn schedule_notify_task(
			origin: OriginFor<T>,
			provided_id: Vec<u8>,
			time: UnixTime,
			message: Vec<u8>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			if message.len() == 0 {
				Err(Error::<T>::EmptyMessage)?
			}

			Self::validate_and_schedule_task(Action::Notify { message }, who, provided_id, time)?;
			Ok(().into())
		}

		/// Schedule a task to transfer native token balance from sender to recipient.
		///
		/// Before the task can be scheduled the task must past validation checks.
		/// * The transaction is signed
		/// * The provided_id's length > 0
		/// * The time is valid
		/// * Larger transfer amount than the acceptable minimum
		/// * Transfer to account other than to self
		///
		/// # Parameters
		/// * `provided_id`: An id provided by the user. This id must be unique for the user.
		/// * `time`: The unix standard time in seconds for when the task should run.
		/// * `recipient_id`: Account ID of the recipient.
		/// * `amount`: Amount of balance to transfer.
		///
		/// # Errors
		/// * `InvalidTime`: Time must end in a whole minute.
		/// * `PastTime`: Time must be in the future.
		/// * `DuplicateTask`: There can be no duplicate tasks.
		/// * `TimeSlotFull`: Time slot is full. No more tasks can be scheduled for this time.
		/// * `InvalidAmount`: Amount has to be larger than 0.1 OAK.
		/// * `TransferToSelf`: Sender cannot transfer money to self.
		/// * `TransferFailed`: Transfer failed for unknown reason.
		#[pallet::weight(<T as Config>::WeightInfo::schedule_native_transfer_task_full())]
		pub fn schedule_native_transfer_task(
			origin: OriginFor<T>,
			provided_id: Vec<u8>,
			time: UnixTime,
			recipient_id: T::AccountId,
			#[pallet::compact] amount: BalanceOf<T>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// check for greater than existential deposit
			if amount < T::NativeTokenExchange::minimum_balance() {
				Err(<Error<T>>::InvalidAmount)?
			}
			// check not sent to self
			if who == recipient_id {
				Err(<Error<T>>::TransferToSelf)?
			}
			let action =
				Action::NativeTransfer { sender: who.clone(), recipient: recipient_id, amount };
			Self::validate_and_schedule_task(action, who, provided_id, time)?;
			Ok(().into())
		}

		/// Cancel a task.
		///
		/// Tasks can only can be cancelled by their owners.
		///
		/// # Parameters
		/// * `task_id`: The id of the task.
		///
		/// # Errors
		/// * `NotTaskOwner`: You are not the owner of the task.
		/// * `TaskDoesNotExist`: The task does not exist.
		#[pallet::weight(<T as Config>::WeightInfo::cancel_overflow_task())]
		pub fn cancel_task(origin: OriginFor<T>, task_id: T::Hash) -> DispatchResult {
			let who = ensure_signed(origin)?;

			match Self::get_task(task_id) {
				None => Err(Error::<T>::TaskDoesNotExist)?,
				Some(task) => {
					if who != task.owner_id {
						Err(Error::<T>::NotTaskOwner)?
					}
					Self::remove_task(task_id, task);
				},
			}
			Ok(().into())
		}

		/// Sudo can force cancel a task.
		///
		/// # Parameters
		/// * `task_id`: The id of the task.
		///
		/// # Errors
		/// * `TaskDoesNotExist`: The task does not exist.
		#[pallet::weight(<T as Config>::WeightInfo::force_cancel_overflow_task())]
		pub fn force_cancel_task(origin: OriginFor<T>, task_id: T::Hash) -> DispatchResult {
			ensure_root(origin)?;

			match Self::get_task(task_id) {
				None => Err(Error::<T>::TaskDoesNotExist)?,
				Some(task) => Self::remove_task(task_id, task),
			}

			Ok(().into())
		}
	}

	impl<T: Config> Pallet<T> {
		/// Based on the block time, return the time slot.
		///
		/// In order to do this we:
		/// * Get the most recent timestamp from the block.
		/// * Convert the ms unix timestamp to seconds.
		/// * Bring the timestamp down to the last whole minute.
		pub fn get_current_time_slot() -> Result<UnixTime, Error<T>> {
			let now = <timestamp::Pallet<T>>::get().saturated_into::<UnixTime>();
			if now == 0 {
				Err(Error::<T>::BlockTimeNotSet)?
			}
			let now = now.saturating_div(1000);
			let diff_to_min = now % 60;
			Ok(now.saturating_sub(diff_to_min))
		}

		/// Checks to see if the scheduled time is valid.
		///
		/// In order for a time to be valid it must
		/// - End in a whole minute
		/// - Be in the future
		/// - Not be more than MaxScheduleSeconds out
		fn is_valid_time(scheduled_time: UnixTime) -> Result<(), Error<T>> {
			let remainder = scheduled_time % 60;
			if remainder != 0 {
				Err(<Error<T>>::InvalidTime)?;
			}

			let current_time_slot = Self::get_current_time_slot()?;
			if scheduled_time <= current_time_slot {
				Err(<Error<T>>::PastTime)?;
			}

			if scheduled_time > current_time_slot + T::MaxScheduleSeconds::get() {
				Err(Error::<T>::TimeTooFarOut)?;
			}

			Ok(())
		}

		/// Trigger tasks for the block time.
		///
		/// Complete as many tasks as possible given the maximum weight.
		pub fn trigger_tasks(max_weight: Weight) -> Weight {
			let mut weight_left: Weight = max_weight;

			// There is a chance we use more than our max_weight to update the task queue.
			// This would occur if the system is not producting blocks for a very long time.
			// Regardless of how long it takes we still need to update the task queue.
			let max_update_weight: Weight = T::UpdateQueueRatio::get().mul_floor(weight_left);
			let update_weight = Self::update_task_queue(max_update_weight);

			if update_weight >= weight_left {
				return update_weight
			}

			weight_left = weight_left.saturating_sub(update_weight);

			// need to calculate the weight of running just 1 task below.
			let run_task_weight = <T as Config>::WeightInfo::run_tasks_many_found(1)
				.saturating_add(T::DbWeight::get().reads(1 as Weight))
				.saturating_add(T::DbWeight::get().writes(1 as Weight));
			if weight_left < run_task_weight {
				return weight_left
			}

			// run as many scheduled tasks as we can
			let task_queue = Self::get_task_queue();
			weight_left = weight_left.saturating_sub(T::DbWeight::get().reads(1 as Weight));
			if task_queue.len() > 0 {
				let (tasks_left, new_weight_left) = Self::run_tasks(task_queue, weight_left);
				TaskQueue::<T>::put(tasks_left);
				weight_left = new_weight_left.saturating_sub(T::DbWeight::get().writes(1 as Weight));
			}

			// if there is weight left we need to handled the missed tasks
			let run_missed_task_weight = <T as Config>::WeightInfo::run_missed_tasks_many_found(1)
				.saturating_add(T::DbWeight::get().reads(1 as Weight))
				.saturating_add(T::DbWeight::get().writes(1 as Weight));
			if weight_left >= run_missed_task_weight {
				let missed_queue = Self::get_missed_queue();
				weight_left = weight_left.saturating_sub(T::DbWeight::get().reads(1 as Weight));
				if missed_queue.len() > 0 {
					let (tasks_left, new_weight_left) =
						Self::run_missed_tasks(missed_queue, weight_left);

					MissedQueue::<T>::put(tasks_left);
					weight_left = new_weight_left.saturating_sub(T::DbWeight::get().writes(1 as Weight));
				}
			}

			max_weight.saturating_sub(weight_left)
		}

		/// Update the task queue.
		///
		/// This function checks to see if we are in a new time slot, and if so it updates the task queue and missing queue by doing the following.
		/// 1. Append the current task queue to the missed queue.
		/// 2. Make all tasks from the new slot into the task queue.
		/// 3. If we skipped any time slots (due to an outage) move those tasks to the missed queue.
		/// 4. Remove all relevant time slots from the Scheduled tasks map.
		///
		pub fn update_task_queue(allotted_weight: Weight) -> Weight {
			let mut total_weight = <T as Config>::WeightInfo::update_task_queue_overhead();

			let current_time_slot = match Self::get_current_time_slot() {
				Ok(time_slot) => time_slot,
				Err(_) => return total_weight,
			};

			if let Some((last_time_slot, last_missed_slot)) = Self::get_last_slot() {
				let missed_queue_allotted_weight = allotted_weight
					.saturating_sub(T::DbWeight::get().reads(1 as Weight))
					.saturating_sub(T::DbWeight::get().writes(1 as Weight))
					.saturating_sub(<T as Config>::WeightInfo::update_scheduled_task_queue());
				let (updated_last_missed_slot, missed_queue_update_weight) =
					Self::update_missed_queue(current_time_slot, last_missed_slot, missed_queue_allotted_weight);
				let (updated_last_time_slot, scheduled_queue_update_weight) =
					Self::update_scheduled_task_queue(current_time_slot, last_time_slot);
				LastTimeSlot::<T>::put((updated_last_time_slot, updated_last_missed_slot));
				total_weight = total_weight
					.saturating_add(missed_queue_update_weight)
					.saturating_add(scheduled_queue_update_weight)
					.saturating_add(T::DbWeight::get().reads(1 as Weight));
			} else {
				LastTimeSlot::<T>::put((current_time_slot, current_time_slot));
				total_weight = total_weight
				.saturating_add(T::DbWeight::get().writes(1 as Weight))
				.saturating_add(T::DbWeight::get().reads(1 as Weight));
			}

			total_weight
		}

		pub fn update_scheduled_task_queue(current_time_slot: u64, last_time_slot: u64) -> (Weight, u64) {
			if current_time_slot != last_time_slot {
				// move current time slot to task queue or clear the task queue
				if let Some(task_ids) = Self::get_scheduled_tasks(current_time_slot) {
					TaskQueue::<T>::put(task_ids);
					ScheduledTasks::<T>::remove(current_time_slot);
				} else {
					let empty_queue: Vec<T::Hash> = vec![];
					TaskQueue::<T>::put(empty_queue);
				}
			}
			let weight_used = <T as Config>::WeightInfo::update_scheduled_task_queue();
			(current_time_slot, weight_used)
		}

		pub fn update_missed_queue(current_time_slot: u64, last_missed_slot: u64, allotted_weight: Weight) -> (Weight, u64) {
			let mut last_missed_slot_tracker = last_missed_slot;
			let mut used_weight = 0;
			if current_time_slot != last_missed_slot_tracker {
				// will need to move missed time slots into missed queue
				let (append_weight, missed_slots_moved) =
					Self::append_to_missed_tasks(current_time_slot, last_missed_slot, allotted_weight);

				last_missed_slot_tracker = last_missed_slot.saturating_add(missed_slots_moved.saturating_mul(60));
				used_weight = used_weight
					.saturating_add(append_weight);
			}
			(last_missed_slot_tracker, used_weight)
		}

		pub fn append_to_missed_tasks(
			current_time_slot: UnixTime,
			last_missed_slot: UnixTime,
			mut allotted_weight: Weight,
		) -> (Weight, u64) {
			// will need to move task queue into missed queue
			let mut missed_tasks = Self::get_task_queue();
			let mut diff = current_time_slot.saturating_sub(last_missed_slot).saturating_div(60).saturating_sub(1);
			for i in 0..diff {
				if allotted_weight < <T as Config>::WeightInfo::shift_missed_tasks() {
					diff = i;
					break
				}
				missed_tasks = Self::shift_missed_tasks(missed_tasks, last_missed_slot, i);
				allotted_weight = allotted_weight.saturating_sub(<T as Config>::WeightInfo::shift_missed_tasks());
			}
			// Update the missed queue
			let mut missed_queue = Self::get_missed_queue();
			missed_queue.append(&mut missed_tasks);
			MissedQueue::<T>::put(missed_queue);
			let weight = <T as Config>::WeightInfo::append_to_missed_tasks(diff.saturated_into());
			(weight, diff)
		}

		pub fn shift_missed_tasks(
			mut missed_tasks: Vec<T::Hash>,
			last_missed_slot: UnixTime,
			number_of_missed_slots: u64,
		) -> Vec<T::Hash> {
			let seconds_in_slot = 60;
			let shift = seconds_in_slot.saturating_mul(number_of_missed_slots + 1);
			let new_time_slot = last_missed_slot.saturating_add(shift);
			if let Some(task_ids) = Self::get_scheduled_tasks(new_time_slot) {
				missed_tasks.append(&mut task_ids.into_inner());
				ScheduledTasks::<T>::remove(new_time_slot);
			}
			missed_tasks
		}

		/// Runs as many tasks as the weight allows from the provided vec of task_ids.
		///
		/// Returns a vec with the tasks that were not run and the remaining weight.
		pub fn run_tasks(
			mut task_ids: Vec<T::Hash>,
			mut weight_left: Weight,
		) -> (Vec<T::Hash>, Weight) {
			let mut consumed_task_index: usize = 0;
			for task_id in task_ids.iter() {
				consumed_task_index.saturating_inc();
				let action_weight = match Self::get_task(task_id) {
					None => {
						Self::deposit_event(Event::TaskNotFound { task_id: task_id.clone() });
						<T as Config>::WeightInfo::run_tasks_many_missing(1)
					},
					Some(task) => {
						let task_action_weight = match task.action {
							Action::Notify { message } => Self::run_notify_task(message),
							Action::NativeTransfer { sender, recipient, amount } =>
								Self::run_native_transfer_task(
									sender,
									recipient,
									amount,
									task_id.clone(),
								),
						};
						Tasks::<T>::remove(task_id);
						task_action_weight
							.saturating_add(T::DbWeight::get().writes(1 as Weight))
							.saturating_add(T::DbWeight::get().reads(1 as Weight))
					},
				};

				weight_left = weight_left.saturating_sub(action_weight);

				if weight_left < <T as Config>::WeightInfo::run_tasks_many_found(1) {
					break
				}
			}

			if consumed_task_index == task_ids.len() {
				return (vec![], weight_left)
			} else {
				return (task_ids.split_off(consumed_task_index), weight_left)
			}
		}

		/// Send events for as many missed tasks as the weight allows from the provided vec of task_ids.
		///
		/// Returns a vec with the tasks that were not run and the remaining weight.
		pub fn run_missed_tasks(
			mut task_ids: Vec<T::Hash>,
			mut weight_left: Weight,
		) -> (Vec<T::Hash>, Weight) {
			let mut consumed_task_index: usize = 0;
			for task_id in task_ids.iter() {
				consumed_task_index += 1;

				let action_weight = match Self::get_task(task_id) {
					None => {
						Self::deposit_event(Event::TaskNotFound { task_id: task_id.clone() });
						<T as Config>::WeightInfo::run_missed_tasks_many_missing(1)
					},
					Some(task) => {
						Self::deposit_event(Event::TaskMissed {
							who: task.owner_id.clone(),
							task_id: task_id.clone(),
						});
						Tasks::<T>::remove(task_id);
						<T as Config>::WeightInfo::run_missed_tasks_many_found(1)
					},
				};

				weight_left = weight_left.saturating_sub(action_weight);

				if weight_left < <T as Config>::WeightInfo::run_missed_tasks_many_found(1) {
					break
				}
			}

			if consumed_task_index == task_ids.len() {
				return (vec![], weight_left)
			} else {
				return (task_ids.split_off(consumed_task_index), weight_left)
			}
		}

		/// Fire the notify event with the custom message.
		pub fn run_notify_task(message: Vec<u8>) -> Weight {
			Self::deposit_event(Event::Notify { message });
			<T as Config>::WeightInfo::run_notify_task()
		}

		pub fn run_native_transfer_task(
			sender: T::AccountId,
			recipient: T::AccountId,
			amount: BalanceOf<T>,
			task_id: T::Hash,
		) -> Weight {
			match T::NativeTokenExchange::transfer(&sender, &recipient, amount) {
				Ok(_number) => Self::deposit_event(Event::SuccesfullyTransferredFunds { task_id }),
				Err(e) => Self::deposit_event(Event::TransferFailed { task_id, error: e }),
			};

			<T as Config>::WeightInfo::run_native_transfer_task()
		}

		fn remove_task(task_id: T::Hash, task: Task<T>) {
			let mut found_task: bool = false;
			match Self::get_scheduled_tasks(task.time) {
				None => {
					let mut task_queue = Self::get_task_queue();
					for i in 0..task_queue.len() {
						if task_queue[i] == task_id {
							task_queue.remove(i);
							TaskQueue::<T>::put(task_queue);
							found_task = true;
							break
						}
					}
				},
				Some(mut task_ids) =>
					for i in 0..task_ids.len() {
						if task_ids[i] == task_id {
							if task_ids.len() == 1 {
								<ScheduledTasks<T>>::remove(task.time);
							} else {
								task_ids.remove(i);
								<ScheduledTasks<T>>::insert(task.time, task_ids);
							}
							found_task = true;
							break
						}
					},
			}

			if !found_task {
				Self::deposit_event(Event::TaskNotFound { task_id });
			}

			<Tasks<T>>::remove(task_id);
			Self::deposit_event(Event::TaskCancelled { who: task.owner_id, task_id });
		}

		/// Schedule task and return it's task_id.
		pub fn schedule_task(
			owner_id: AccountOf<T>,
			provided_id: Vec<u8>,
			time: u64,
		) -> Result<T::Hash, Error<T>> {
			let task_id = Self::generate_task_id(owner_id.clone(), provided_id.clone());

			if let Some(_) = Self::get_task(task_id) {
				Err(Error::<T>::DuplicateTask)?
			}

			match Self::get_scheduled_tasks(time) {
				None => {
					let task_ids: BoundedVec<T::Hash, T::MaxTasksPerSlot> =
						vec![task_id].try_into().unwrap();
					<ScheduledTasks<T>>::insert(time, task_ids);
				},
				Some(mut task_ids) => {
					if let Err(_) = task_ids.try_push(task_id) {
						Err(Error::<T>::TimeSlotFull)?
					}
					<ScheduledTasks<T>>::insert(time, task_ids);
				},
			}
			Ok(task_id)
		}

		/// Validate and schedule task.
		/// This will also charge the execution fee.
		pub fn validate_and_schedule_task(
			action: Action<T>,
			who: T::AccountId,
			provided_id: Vec<u8>,
			time: UnixTime,
		) -> Result<(), Error<T>> {
			if provided_id.len() == 0 {
				Err(Error::<T>::EmptyProvidedId)?
			}
			Self::is_valid_time(time)?;

			let fee = Self::calculate_execution_fee(&action);
			T::NativeTokenExchange::can_pay_fee(&who, fee.clone())
				.map_err(|_| Error::InsufficientBalance)?;

			let task_id = Self::schedule_task(who.clone(), provided_id.clone(), time)?;
			let task: Task<T> = Task::<T> { owner_id: who.clone(), provided_id, time, action };
			<Tasks<T>>::insert(task_id, task);

			// This should never error if can_pay_fee passed.
			T::NativeTokenExchange::withdraw_fee(&who, fee.clone())
				.map_err(|_| Error::LiquidityRestrictions)?;

			Self::deposit_event(Event::TaskScheduled { who, task_id });
			Ok(())
		}

		pub fn generate_task_id(owner_id: AccountOf<T>, provided_id: Vec<u8>) -> T::Hash {
			let task_hash_input =
				TaskHashInput::<T> { owner_id: owner_id.clone(), provided_id: provided_id.clone() };
			T::Hashing::hash_of(&task_hash_input)
		}

		fn calculate_execution_fee(action: &Action<T>) -> BalanceOf<T> {
			let raw_weight = match action {
				Action::Notify { message: _ } => 1_000u32,
				Action::NativeTransfer { sender: _, recipient: _, amount: _ } => 2_000u32,
			};
			let weight = <BalanceOf<T>>::from(raw_weight);
			T::ExecutionWeightFee::get().saturating_mul(weight)
		}
	}
}
