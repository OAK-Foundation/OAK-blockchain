use core::marker::PhantomData;

use crate::{
	AccountOf, ActionOf, BalanceOf, Config, Schedule, Seconds, TaskId, TaskOf, UnixTime, Vec,
};
use codec::{Decode, Encode};
use cumulus_primitives_core::ParaId;
use frame_support::{
	traits::{Get, OnRuntimeUpgrade},
	weights::Weight,
	BoundedVec, Twox64Concat,
};
use scale_info::TypeInfo;

#[derive(Debug, Encode, Decode, TypeInfo)]
#[scale_info(skip_type_params(MaxExecutionTimes))]
pub struct OldTask<T: Config> {
	pub owner_id: T::AccountId,
	pub provided_id: Vec<u8>,
	pub execution_times: BoundedVec<UnixTime, T::MaxExecutionTimes>,
	pub executions_left: u32,
	pub action: OldAction<T>,
}

impl<T: Config> From<OldTask<T>> for TaskOf<T> {
	fn from(task: OldTask<T>) -> Self {
		let schedule = match task.action {
			OldAction::AutoCompoundDelegatedStake { frequency, .. } => Schedule::Recurring {
				next_execution_time: *task.execution_times.last().expect("Atleast one execution"),
				frequency,
			},
			_ => Schedule::Fixed {
				execution_times: task.execution_times,
				executions_left: task.executions_left,
			},
		};
		TaskOf::<T> {
			owner_id: task.owner_id,
			provided_id: task.provided_id,
			action: task.action.into(),
			schedule,
		}
	}
}

/// The enum that stores all action specific data.
#[derive(Clone, Debug, Eq, PartialEq, Encode, Decode, TypeInfo)]
pub enum OldAction<T: Config> {
	Notify {
		message: Vec<u8>,
	},
	NativeTransfer {
		sender: T::AccountId,
		recipient: T::AccountId,
		amount: BalanceOf<T>,
	},
	XCMP {
		para_id: ParaId,
		currency_id: T::CurrencyId,
		encoded_call: Vec<u8>,
		encoded_call_weight: Weight,
	},
	AutoCompoundDelegatedStake {
		delegator: T::AccountId,
		collator: T::AccountId,
		account_minimum: BalanceOf<T>,
		frequency: Seconds,
	},
	DynamicDispatch {
		encoded_call: Vec<u8>,
	},
}

impl<T: Config> From<OldAction<T>> for ActionOf<T> {
	fn from(action: OldAction<T>) -> Self {
		match action {
			OldAction::AutoCompoundDelegatedStake {
				delegator, collator, account_minimum, ..
			} => Self::AutoCompoundDelegatedStake { delegator, collator, account_minimum },
			OldAction::Notify { message } => Self::Notify { message },
			OldAction::NativeTransfer { sender, recipient, amount } =>
				Self::NativeTransfer { sender, recipient, amount },
			OldAction::XCMP { para_id, currency_id, encoded_call, encoded_call_weight } =>
				Self::XCMP { para_id, currency_id, encoded_call, encoded_call_weight },
			OldAction::DynamicDispatch { encoded_call } => Self::DynamicDispatch { encoded_call },
		}
	}
}

#[frame_support::storage_alias]
type AccountTasks<T: Config> = StorageDoubleMap<
	AutomationTime,
	Twox64Concat,
	AccountOf<T>,
	Twox64Concat,
	TaskId<T>,
	OldTask<T>,
>;

pub struct MigrateToV5<T>(PhantomData<T>);
impl<T: Config> OnRuntimeUpgrade for MigrateToV5<T> {
	#[cfg(feature = "try-runtime")]
	fn pre_upgrade() -> Result<(), &'static str> {
		use frame_support::traits::OnRuntimeUpgradeHelpersExt;

		let task_count = AccountTasks::<T>::iter().count();
		Self::set_temp_storage::<u32>(task_count as u32, "pre_migration_task_count");

		log::info!(
			target: "automation-time",
			"migration: AutomationTime storage version v5 PRE migration checks succesful!"
		);

		Ok(())
	}

	fn on_runtime_upgrade() -> Weight {
		log::info!(target: "automation-time", "Migrating automation-time v5");

		let mut migrated_tasks = 0u64;
		AccountTasks::<T>::iter().for_each(|(account_id, task_id, task)| {
			let migrated_task: TaskOf<T> = task.into();
			crate::AccountTasks::<T>::insert(account_id, task_id, migrated_task);

			migrated_tasks += 1;
		});

		log::info!(
			target: "automation-time",
			"migration: AutomationTime storage version v5 migration succesful! Migrated {} tasks.",
			migrated_tasks
		);

		// 1 read + write to transform each task
		let weight = T::DbWeight::get().reads_writes(migrated_tasks, migrated_tasks);

		// Adding a buffer for the rest of the code
		weight + (migrated_tasks * 10_000_000)
	}

	#[cfg(feature = "try-runtime")]
	fn post_upgrade() -> Result<(), &'static str> {
		use frame_support::traits::OnRuntimeUpgradeHelpersExt;

		let post_task_count = crate::AccountTasks::<T>::iter().count() as u32;
		let pre_task_count = Self::get_temp_storage::<u32>("pre_migration_task_count").unwrap();
		assert_eq!(post_task_count, pre_task_count);

		log::info!(
			target: "automation-time",
			"migration: AutomationTime storage version v5 POST migration checks succesful! Migrated {} tasks.",
			post_task_count
		);

		Ok(())
	}
}
