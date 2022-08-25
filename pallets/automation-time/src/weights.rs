// This file is part of Substrate.

// Copyright (C) 2022 Parity Technologies (UK) Ltd.
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

//! Autogenerated weights for pallet_automation_time
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-08-25, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `actions-runner-1`, CPU: `Intel(R) Xeon(R) E-2388G CPU @ 3.20GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("turing-dev"), DB CACHE: 1024

// Executed Command:
// ./oak-collator
// benchmark
// pallet
// --chain
// turing-dev
// --execution
// wasm
// --wasm-execution
// compiled
// --pallet
// pallet_automation_time
// --extrinsic
// *
// --repeat
// 20
// --steps
// 50
// --output
// ./automation_time-raw-weights.rs
// --template
// ./.maintain/frame-weight-template.hbs

// Summary:
//:schedule_notify_task_empty 58_192_000
//:schedule_notify_task_full 76_351_000
//:schedule_xcmp_task_full 110_806_000
//:schedule_native_transfer_task_empty 58_460_000
//:schedule_native_transfer_task_full 78_459_000
//:schedule_auto_compound_delegated_stake_task_full 110_233_000
//:cancel_scheduled_task_full 797_946_000
//:force_cancel_scheduled_task 28_026_000
//:force_cancel_scheduled_task_full 798_084_000
//:run_notify_task 9_109_000
//:run_native_transfer_task 34_836_000
//:run_xcmp_task 79_266_000
//:run_auto_compound_delegated_stake_task 104_155_000
//:run_missed_tasks_many_found 179_000
//:run_missed_tasks_many_missing 176_000
//:run_tasks_many_found 186_000
//:run_tasks_many_missing 164_000
//:update_task_queue_overhead 1_977_000
//:append_to_missed_tasks 1_963_000
//:update_scheduled_task_queue 48_568_000
//:shift_missed_tasks 28_630_000

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_automation_time.
pub trait WeightInfo {
	fn schedule_notify_task_empty() -> Weight;
	fn schedule_notify_task_full(v: u32, ) -> Weight;
	fn schedule_xcmp_task_full(v: u32, ) -> Weight;
	fn schedule_native_transfer_task_empty() -> Weight;
	fn schedule_native_transfer_task_full(v: u32, ) -> Weight;
	fn schedule_auto_compound_delegated_stake_task_full() -> Weight;
	fn cancel_scheduled_task_full() -> Weight;
	fn force_cancel_scheduled_task() -> Weight;
	fn force_cancel_scheduled_task_full() -> Weight;
	fn run_notify_task() -> Weight;
	fn run_native_transfer_task() -> Weight;
	fn run_xcmp_task() -> Weight;
	fn run_auto_compound_delegated_stake_task() -> Weight;
	fn run_missed_tasks_many_found(v: u32, ) -> Weight;
	fn run_missed_tasks_many_missing(v: u32, ) -> Weight;
	fn run_tasks_many_found(v: u32, ) -> Weight;
	fn run_tasks_many_missing(v: u32, ) -> Weight;
	fn update_task_queue_overhead() -> Weight;
	fn append_to_missed_tasks(v: u32, ) -> Weight;
	fn update_scheduled_task_queue() -> Weight;
	fn shift_missed_tasks() -> Weight;
}

/// Weights for pallet_automation_time using the Substrate node and recommended hardware.
pub struct pallet_automation_timeWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for pallet_automation_timeWeight<T> {
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: System Account (r:2 w:2)
	// Storage: AutomationTime AccountTasks (r:1 w:1)
	// Storage: AutomationTime ScheduledTasksV2 (r:1 w:1)
	fn schedule_notify_task_empty() -> Weight {
		(58_192_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: System Account (r:2 w:2)
	// Storage: AutomationTime AccountTasks (r:1 w:1)
	// Storage: AutomationTime ScheduledTasksV2 (r:1 w:1)
	/// The range of component `v` is `[1, 24]`.
	fn schedule_notify_task_full(v: u32, ) -> Weight {
		(76_351_000 as Weight)
			// Standard Error: 90_000
			.saturating_add((33_559_000 as Weight).saturating_mul(v as Weight))
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(v as Weight)))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(v as Weight)))
	}
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: XcmpHandler XcmChainCurrencyData (r:1 w:0)
	// Storage: System Account (r:3 w:3)
	// Storage: AutomationTime AccountTasks (r:1 w:1)
	// Storage: AutomationTime ScheduledTasksV2 (r:1 w:1)
	// Storage: ParachainInfo ParachainId (r:1 w:0)
	/// The range of component `v` is `[1, 24]`.
	fn schedule_xcmp_task_full(v: u32, ) -> Weight {
		(110_806_000 as Weight)
			// Standard Error: 31_000
			.saturating_add((33_418_000 as Weight).saturating_mul(v as Weight))
			.saturating_add(T::DbWeight::get().reads(7 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(v as Weight)))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(v as Weight)))
	}
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: System Account (r:2 w:2)
	// Storage: AutomationTime AccountTasks (r:1 w:1)
	// Storage: AutomationTime ScheduledTasksV2 (r:1 w:1)
	fn schedule_native_transfer_task_empty() -> Weight {
		(58_460_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: System Account (r:2 w:2)
	// Storage: AutomationTime AccountTasks (r:1 w:1)
	// Storage: AutomationTime ScheduledTasksV2 (r:1 w:1)
	/// The range of component `v` is `[1, 24]`.
	fn schedule_native_transfer_task_full(v: u32, ) -> Weight {
		(78_459_000 as Weight)
			// Standard Error: 33_000
			.saturating_add((33_156_000 as Weight).saturating_mul(v as Weight))
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(v as Weight)))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(v as Weight)))
	}
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: System Account (r:2 w:2)
	// Storage: AutomationTime AccountTasks (r:1 w:1)
	// Storage: AutomationTime ScheduledTasksV2 (r:1 w:1)
	fn schedule_auto_compound_delegated_stake_task_full() -> Weight {
		(110_233_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	// Storage: AutomationTime AccountTasks (r:1 w:1)
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: AutomationTime LastTimeSlot (r:1 w:0)
	// Storage: AutomationTime ScheduledTasksV2 (r:24 w:24)
	fn cancel_scheduled_task_full() -> Weight {
		(797_946_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(27 as Weight))
			.saturating_add(T::DbWeight::get().writes(25 as Weight))
	}
	// Storage: AutomationTime AccountTasks (r:1 w:1)
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: AutomationTime LastTimeSlot (r:1 w:0)
	// Storage: AutomationTime ScheduledTasksV2 (r:1 w:1)
	fn force_cancel_scheduled_task() -> Weight {
		(28_026_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: AutomationTime AccountTasks (r:1 w:1)
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: AutomationTime LastTimeSlot (r:1 w:0)
	// Storage: AutomationTime ScheduledTasksV2 (r:24 w:24)
	fn force_cancel_scheduled_task_full() -> Weight {
		(798_084_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(27 as Weight))
			.saturating_add(T::DbWeight::get().writes(25 as Weight))
	}
	fn run_notify_task() -> Weight {
		(9_109_000 as Weight)
	}
	// Storage: System Account (r:2 w:2)
	fn run_native_transfer_task() -> Weight {
		(34_836_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: XcmpHandler XcmChainCurrencyData (r:1 w:0)
	// Storage: ParachainInfo ParachainId (r:1 w:0)
	// Storage: UnknownTokens ConcreteFungibleBalances (r:1 w:0)
	// Storage: System Account (r:2 w:2)
	// Storage: ParachainSystem RelevantMessagingState (r:1 w:0)
	fn run_xcmp_task() -> Weight {
		(79_266_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(6 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: System Account (r:2 w:2)
	// Storage: ParachainStaking DelegatorState (r:1 w:1)
	// Storage: ParachainStaking DelegationScheduledRequests (r:1 w:0)
	// Storage: ParachainStaking DelegatorReserveToLockMigrations (r:1 w:0)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: ParachainStaking CandidateInfo (r:1 w:1)
	// Storage: ParachainStaking TopDelegations (r:1 w:1)
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	// Storage: ParachainStaking Total (r:1 w:1)
	// Storage: AutomationTime ScheduledTasksV2 (r:1 w:1)
	fn run_auto_compound_delegated_stake_task() -> Weight {
		(104_155_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(11 as Weight))
			.saturating_add(T::DbWeight::get().writes(9 as Weight))
	}
	// Storage: AutomationTime AccountTasks (r:1 w:1)
	/// The range of component `v` is `[0, 1]`.
	fn run_missed_tasks_many_found(v: u32, ) -> Weight {
		(179_000 as Weight)
			// Standard Error: 28_000
			.saturating_add((14_782_000 as Weight).saturating_mul(v as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(v as Weight)))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(v as Weight)))
	}
	// Storage: AutomationTime AccountTasks (r:1 w:0)
	/// The range of component `v` is `[0, 1]`.
	fn run_missed_tasks_many_missing(v: u32, ) -> Weight {
		(176_000 as Weight)
			// Standard Error: 32_000
			.saturating_add((11_337_000 as Weight).saturating_mul(v as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(v as Weight)))
	}
	// Storage: AutomationTime AccountTasks (r:1 w:1)
	// Storage: System Account (r:2 w:2)
	/// The range of component `v` is `[0, 1]`.
	fn run_tasks_many_found(v: u32, ) -> Weight {
		(186_000 as Weight)
			// Standard Error: 41_000
			.saturating_add((39_741_000 as Weight).saturating_mul(v as Weight))
			.saturating_add(T::DbWeight::get().reads((3 as Weight).saturating_mul(v as Weight)))
			.saturating_add(T::DbWeight::get().writes((3 as Weight).saturating_mul(v as Weight)))
	}
	// Storage: AutomationTime AccountTasks (r:1 w:0)
	/// The range of component `v` is `[0, 1]`.
	fn run_tasks_many_missing(v: u32, ) -> Weight {
		(164_000 as Weight)
			// Standard Error: 28_000
			.saturating_add((11_403_000 as Weight).saturating_mul(v as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(v as Weight)))
	}
	// Storage: Timestamp Now (r:1 w:0)
	fn update_task_queue_overhead() -> Weight {
		(1_977_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
	}
	// Storage: AutomationTime MissedQueueV2 (r:1 w:1)
	// Storage: AutomationTime ScheduledTasksV2 (r:1 w:1)
	/// The range of component `v` is `[0, 2]`.
	fn append_to_missed_tasks(v: u32, ) -> Weight {
		(1_963_000 as Weight)
			// Standard Error: 236_000
			.saturating_add((2_454_000 as Weight).saturating_mul(v as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(v as Weight)))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(v as Weight)))
	}
	// Storage: AutomationTime TaskQueueV2 (r:1 w:1)
	// Storage: AutomationTime MissedQueueV2 (r:1 w:1)
	// Storage: AutomationTime ScheduledTasksV2 (r:1 w:1)
	fn update_scheduled_task_queue() -> Weight {
		(48_568_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: AutomationTime ScheduledTasksV2 (r:1 w:1)
	fn shift_missed_tasks() -> Weight {
		(28_630_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: System Account (r:2 w:2)
	// Storage: AutomationTime AccountTasks (r:1 w:1)
	// Storage: AutomationTime ScheduledTasksV2 (r:1 w:1)
	fn schedule_notify_task_empty() -> Weight {
		(58_192_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(5 as Weight))
			.saturating_add(RocksDbWeight::get().writes(4 as Weight))
	}
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: System Account (r:2 w:2)
	// Storage: AutomationTime AccountTasks (r:1 w:1)
	// Storage: AutomationTime ScheduledTasksV2 (r:1 w:1)
	/// The range of component `v` is `[1, 24]`.
	fn schedule_notify_task_full(v: u32, ) -> Weight {
		(76_351_000 as Weight)
			// Standard Error: 90_000
			.saturating_add((33_559_000 as Weight).saturating_mul(v as Weight))
			.saturating_add(RocksDbWeight::get().reads(4 as Weight))
			.saturating_add(RocksDbWeight::get().reads((1 as Weight).saturating_mul(v as Weight)))
			.saturating_add(RocksDbWeight::get().writes(3 as Weight))
			.saturating_add(RocksDbWeight::get().writes((1 as Weight).saturating_mul(v as Weight)))
	}
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: XcmpHandler XcmChainCurrencyData (r:1 w:0)
	// Storage: System Account (r:3 w:3)
	// Storage: AutomationTime AccountTasks (r:1 w:1)
	// Storage: AutomationTime ScheduledTasksV2 (r:1 w:1)
	// Storage: ParachainInfo ParachainId (r:1 w:0)
	/// The range of component `v` is `[1, 24]`.
	fn schedule_xcmp_task_full(v: u32, ) -> Weight {
		(110_806_000 as Weight)
			// Standard Error: 31_000
			.saturating_add((33_418_000 as Weight).saturating_mul(v as Weight))
			.saturating_add(RocksDbWeight::get().reads(7 as Weight))
			.saturating_add(RocksDbWeight::get().reads((1 as Weight).saturating_mul(v as Weight)))
			.saturating_add(RocksDbWeight::get().writes(4 as Weight))
			.saturating_add(RocksDbWeight::get().writes((1 as Weight).saturating_mul(v as Weight)))
	}
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: System Account (r:2 w:2)
	// Storage: AutomationTime AccountTasks (r:1 w:1)
	// Storage: AutomationTime ScheduledTasksV2 (r:1 w:1)
	fn schedule_native_transfer_task_empty() -> Weight {
		(58_460_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(5 as Weight))
			.saturating_add(RocksDbWeight::get().writes(4 as Weight))
	}
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: System Account (r:2 w:2)
	// Storage: AutomationTime AccountTasks (r:1 w:1)
	// Storage: AutomationTime ScheduledTasksV2 (r:1 w:1)
	/// The range of component `v` is `[1, 24]`.
	fn schedule_native_transfer_task_full(v: u32, ) -> Weight {
		(78_459_000 as Weight)
			// Standard Error: 33_000
			.saturating_add((33_156_000 as Weight).saturating_mul(v as Weight))
			.saturating_add(RocksDbWeight::get().reads(4 as Weight))
			.saturating_add(RocksDbWeight::get().reads((1 as Weight).saturating_mul(v as Weight)))
			.saturating_add(RocksDbWeight::get().writes(3 as Weight))
			.saturating_add(RocksDbWeight::get().writes((1 as Weight).saturating_mul(v as Weight)))
	}
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: System Account (r:2 w:2)
	// Storage: AutomationTime AccountTasks (r:1 w:1)
	// Storage: AutomationTime ScheduledTasksV2 (r:1 w:1)
	fn schedule_auto_compound_delegated_stake_task_full() -> Weight {
		(110_233_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(5 as Weight))
			.saturating_add(RocksDbWeight::get().writes(4 as Weight))
	}
	// Storage: AutomationTime AccountTasks (r:1 w:1)
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: AutomationTime LastTimeSlot (r:1 w:0)
	// Storage: AutomationTime ScheduledTasksV2 (r:24 w:24)
	fn cancel_scheduled_task_full() -> Weight {
		(797_946_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(27 as Weight))
			.saturating_add(RocksDbWeight::get().writes(25 as Weight))
	}
	// Storage: AutomationTime AccountTasks (r:1 w:1)
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: AutomationTime LastTimeSlot (r:1 w:0)
	// Storage: AutomationTime ScheduledTasksV2 (r:1 w:1)
	fn force_cancel_scheduled_task() -> Weight {
		(28_026_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(4 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
	// Storage: AutomationTime AccountTasks (r:1 w:1)
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: AutomationTime LastTimeSlot (r:1 w:0)
	// Storage: AutomationTime ScheduledTasksV2 (r:24 w:24)
	fn force_cancel_scheduled_task_full() -> Weight {
		(798_084_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(27 as Weight))
			.saturating_add(RocksDbWeight::get().writes(25 as Weight))
	}
	fn run_notify_task() -> Weight {
		(9_109_000 as Weight)
	}
	// Storage: System Account (r:2 w:2)
	fn run_native_transfer_task() -> Weight {
		(34_836_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
	// Storage: XcmpHandler XcmChainCurrencyData (r:1 w:0)
	// Storage: ParachainInfo ParachainId (r:1 w:0)
	// Storage: UnknownTokens ConcreteFungibleBalances (r:1 w:0)
	// Storage: System Account (r:2 w:2)
	// Storage: ParachainSystem RelevantMessagingState (r:1 w:0)
	fn run_xcmp_task() -> Weight {
		(79_266_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(6 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
	// Storage: System Account (r:2 w:2)
	// Storage: ParachainStaking DelegatorState (r:1 w:1)
	// Storage: ParachainStaking DelegationScheduledRequests (r:1 w:0)
	// Storage: ParachainStaking DelegatorReserveToLockMigrations (r:1 w:0)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: ParachainStaking CandidateInfo (r:1 w:1)
	// Storage: ParachainStaking TopDelegations (r:1 w:1)
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	// Storage: ParachainStaking Total (r:1 w:1)
	// Storage: AutomationTime ScheduledTasksV2 (r:1 w:1)
	fn run_auto_compound_delegated_stake_task() -> Weight {
		(104_155_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(11 as Weight))
			.saturating_add(RocksDbWeight::get().writes(9 as Weight))
	}
	// Storage: AutomationTime AccountTasks (r:1 w:1)
	/// The range of component `v` is `[0, 1]`.
	fn run_missed_tasks_many_found(v: u32, ) -> Weight {
		(179_000 as Weight)
			// Standard Error: 28_000
			.saturating_add((14_782_000 as Weight).saturating_mul(v as Weight))
			.saturating_add(RocksDbWeight::get().reads((1 as Weight).saturating_mul(v as Weight)))
			.saturating_add(RocksDbWeight::get().writes((1 as Weight).saturating_mul(v as Weight)))
	}
	// Storage: AutomationTime AccountTasks (r:1 w:0)
	/// The range of component `v` is `[0, 1]`.
	fn run_missed_tasks_many_missing(v: u32, ) -> Weight {
		(176_000 as Weight)
			// Standard Error: 32_000
			.saturating_add((11_337_000 as Weight).saturating_mul(v as Weight))
			.saturating_add(RocksDbWeight::get().reads((1 as Weight).saturating_mul(v as Weight)))
	}
	// Storage: AutomationTime AccountTasks (r:1 w:1)
	// Storage: System Account (r:2 w:2)
	/// The range of component `v` is `[0, 1]`.
	fn run_tasks_many_found(v: u32, ) -> Weight {
		(186_000 as Weight)
			// Standard Error: 41_000
			.saturating_add((39_741_000 as Weight).saturating_mul(v as Weight))
			.saturating_add(RocksDbWeight::get().reads((3 as Weight).saturating_mul(v as Weight)))
			.saturating_add(RocksDbWeight::get().writes((3 as Weight).saturating_mul(v as Weight)))
	}
	// Storage: AutomationTime AccountTasks (r:1 w:0)
	/// The range of component `v` is `[0, 1]`.
	fn run_tasks_many_missing(v: u32, ) -> Weight {
		(164_000 as Weight)
			// Standard Error: 28_000
			.saturating_add((11_403_000 as Weight).saturating_mul(v as Weight))
			.saturating_add(RocksDbWeight::get().reads((1 as Weight).saturating_mul(v as Weight)))
	}
	// Storage: Timestamp Now (r:1 w:0)
	fn update_task_queue_overhead() -> Weight {
		(1_977_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
	}
	// Storage: AutomationTime MissedQueueV2 (r:1 w:1)
	// Storage: AutomationTime ScheduledTasksV2 (r:1 w:1)
	/// The range of component `v` is `[0, 2]`.
	fn append_to_missed_tasks(v: u32, ) -> Weight {
		(1_963_000 as Weight)
			// Standard Error: 236_000
			.saturating_add((2_454_000 as Weight).saturating_mul(v as Weight))
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().reads((1 as Weight).saturating_mul(v as Weight)))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes((1 as Weight).saturating_mul(v as Weight)))
	}
	// Storage: AutomationTime TaskQueueV2 (r:1 w:1)
	// Storage: AutomationTime MissedQueueV2 (r:1 w:1)
	// Storage: AutomationTime ScheduledTasksV2 (r:1 w:1)
	fn update_scheduled_task_queue() -> Weight {
		(48_568_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(3 as Weight))
			.saturating_add(RocksDbWeight::get().writes(3 as Weight))
	}
	// Storage: AutomationTime ScheduledTasksV2 (r:1 w:1)
	fn shift_missed_tasks() -> Weight {
		(28_630_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
}
