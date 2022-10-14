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
//! DATE: 2022-09-15, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
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
//:schedule_notify_task_empty 58_311_000
//:schedule_notify_task_full 74_430_000
//:schedule_xcmp_task_full 111_741_000
//:schedule_native_transfer_task_empty 58_346_000
//:schedule_native_transfer_task_full 81_795_000
//:schedule_auto_compound_delegated_stake_task_full 89_414_000
//:cancel_scheduled_task_full 807_870_000
//:force_cancel_scheduled_task 28_072_000
//:force_cancel_scheduled_task_full 809_933_000
//:run_notify_task 8_851_000
//:run_native_transfer_task 33_995_000
//:run_xcmp_task 102_000_000
//:run_auto_compound_delegated_stake_task 105_359_000
//:run_dynamic_dispatch_action 17_152_000
//:run_dynamic_dispatch_action_fail_decode 8_984_000
//:run_missed_tasks_many_found 191_000
//:run_missed_tasks_many_missing 189_000
//:run_tasks_many_found 247_000
//:run_tasks_many_missing 184_000
//:update_task_queue_overhead 1_974_000
//:append_to_missed_tasks 1_919_000
//:update_scheduled_task_queue 45_209_000
//:shift_missed_tasks 29_532_000
//:migration_v4_2_slots 32_078_000
//:migration_v4_1_slots 17_955_000
//:migration_v4_0_slots 4_583_000

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
	fn schedule_dynamic_dispatch_task(v: u32, ) -> Weight;
	fn schedule_dynamic_dispatch_task_full(v: u32, ) -> Weight;
	fn cancel_scheduled_task_full() -> Weight;
	fn force_cancel_scheduled_task() -> Weight;
	fn force_cancel_scheduled_task_full() -> Weight;
	fn run_notify_task() -> Weight;
	fn run_native_transfer_task() -> Weight;
	fn run_xcmp_task() -> Weight;
	fn run_auto_compound_delegated_stake_task() -> Weight;
	fn run_dynamic_dispatch_action() -> Weight;
	fn run_dynamic_dispatch_action_fail_decode() -> Weight;
	fn run_missed_tasks_many_found(v: u32, ) -> Weight;
	fn run_missed_tasks_many_missing(v: u32, ) -> Weight;
	fn run_tasks_many_found(v: u32, ) -> Weight;
	fn run_tasks_many_missing(v: u32, ) -> Weight;
	fn update_task_queue_overhead() -> Weight;
	fn append_to_missed_tasks(v: u32, ) -> Weight;
	fn update_scheduled_task_queue() -> Weight;
	fn shift_missed_tasks() -> Weight;
	fn migration_add_schedule_to_task(v: u32, ) -> Weight;
}

/// Weights for pallet_automation_time using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: System Account (r:2 w:2)
	// Storage: AutomationTime AccountTasks (r:1 w:1)
	// Storage: AutomationTime ScheduledTasksV3 (r:1 w:1)
	fn schedule_notify_task_empty() -> Weight {
		(58_311_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: System Account (r:2 w:2)
	// Storage: AutomationTime AccountTasks (r:1 w:1)
	// Storage: AutomationTime ScheduledTasksV3 (r:1 w:1)
	/// The range of component `v` is `[1, 24]`.
	fn schedule_notify_task_full(v: u32, ) -> Weight {
		(74_430_000 as Weight)
			// Standard Error: 40_000
			.saturating_add((34_085_000 as Weight).saturating_mul(v as Weight))
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(v as Weight)))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(v as Weight)))
	}
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: XcmpHandler XcmChainCurrencyData (r:1 w:0)
	// Storage: System Account (r:3 w:3)
	// Storage: AutomationTime AccountTasks (r:1 w:1)
	// Storage: AutomationTime ScheduledTasksV3 (r:1 w:1)
	// Storage: ParachainInfo ParachainId (r:1 w:0)
	/// The range of component `v` is `[1, 24]`.
	fn schedule_xcmp_task_full(v: u32, ) -> Weight {
		(111_741_000 as Weight)
			// Standard Error: 32_000
			.saturating_add((32_935_000 as Weight).saturating_mul(v as Weight))
			.saturating_add(T::DbWeight::get().reads(7 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(v as Weight)))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(v as Weight)))
	}
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: System Account (r:2 w:2)
	// Storage: AutomationTime AccountTasks (r:1 w:1)
	// Storage: AutomationTime ScheduledTasksV3 (r:1 w:1)
	fn schedule_native_transfer_task_empty() -> Weight {
		(58_346_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: System Account (r:2 w:2)
	// Storage: AutomationTime AccountTasks (r:1 w:1)
	// Storage: AutomationTime ScheduledTasksV3 (r:1 w:1)
	/// The range of component `v` is `[1, 24]`.
	fn schedule_native_transfer_task_full(v: u32, ) -> Weight {
		(81_795_000 as Weight)
			// Standard Error: 45_000
			.saturating_add((33_404_000 as Weight).saturating_mul(v as Weight))
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(v as Weight)))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(v as Weight)))
	}
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: System Account (r:2 w:2)
	// Storage: AutomationTime AccountTasks (r:1 w:1)
	// Storage: AutomationTime ScheduledTasksV3 (r:1 w:1)
	fn schedule_auto_compound_delegated_stake_task_full() -> Weight {
		(89_414_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: System Account (r:2 w:2)
	// Storage: AutomationTime AccountTasks (r:1 w:1)
	// Storage: AutomationTime ScheduledTasksV3 (r:1 w:1)
	/// The range of component `v` is `[1, 24]`.
	fn schedule_dynamic_dispatch_task(v: u32, ) -> Weight {
		(45_793_000 as Weight)
			// Standard Error: 15_000
			.saturating_add((3_890_000 as Weight).saturating_mul(v as Weight))
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(v as Weight)))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(v as Weight)))
	}
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: System Account (r:2 w:2)
	// Storage: AutomationTime AccountTasks (r:1 w:1)
	// Storage: AutomationTime ScheduledTasksV3 (r:1 w:1)
	/// The range of component `v` is `[1, 24]`.
	fn schedule_dynamic_dispatch_task_full(v: u32, ) -> Weight {
		(52_072_000 as Weight)
			// Standard Error: 130_000
			.saturating_add((33_164_000 as Weight).saturating_mul(v as Weight))
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(v as Weight)))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(v as Weight)))
	}
	// Storage: AutomationTime AccountTasks (r:1 w:1)
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: AutomationTime LastTimeSlot (r:1 w:0)
	// Storage: AutomationTime ScheduledTasksV3 (r:24 w:24)
	fn cancel_scheduled_task_full() -> Weight {
		(807_870_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(27 as Weight))
			.saturating_add(T::DbWeight::get().writes(25 as Weight))
	}
	// Storage: AutomationTime AccountTasks (r:1 w:1)
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: AutomationTime LastTimeSlot (r:1 w:0)
	// Storage: AutomationTime ScheduledTasksV3 (r:1 w:1)
	fn force_cancel_scheduled_task() -> Weight {
		(28_072_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: AutomationTime AccountTasks (r:1 w:1)
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: AutomationTime LastTimeSlot (r:1 w:0)
	// Storage: AutomationTime ScheduledTasksV3 (r:24 w:24)
	fn force_cancel_scheduled_task_full() -> Weight {
		(809_933_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(27 as Weight))
			.saturating_add(T::DbWeight::get().writes(25 as Weight))
	}
	fn run_notify_task() -> Weight {
		(8_851_000 as Weight)
	}
	// Storage: System Account (r:2 w:2)
	fn run_native_transfer_task() -> Weight {
		(33_995_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: XcmpHandler XcmChainCurrencyData (r:1 w:0)
	// Storage: ParachainInfo ParachainId (r:1 w:0)
	// Storage: UnknownTokens ConcreteFungibleBalances (r:1 w:0)
	// Storage: AssetRegistry LocationToAssetId (r:1 w:0)
	// Storage: System Account (r:2 w:2)
	// Storage: ParachainSystem HostConfiguration (r:1 w:0)
	// Storage: ParachainSystem PendingUpwardMessages (r:1 w:1)
	fn run_xcmp_task() -> Weight {
		(102_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(8 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
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
	// Storage: AutomationTime ScheduledTasksV3 (r:1 w:1)
	fn run_auto_compound_delegated_stake_task() -> Weight {
		(105_359_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(11 as Weight))
			.saturating_add(T::DbWeight::get().writes(9 as Weight))
	}
	// Storage: Valve ValveClosed (r:1 w:0)
	// Storage: Valve ClosedPallets (r:1 w:0)
	fn run_dynamic_dispatch_action() -> Weight {
		(17_152_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
	}
	fn run_dynamic_dispatch_action_fail_decode() -> Weight {
		(8_984_000 as Weight)
	}
	// Storage: AutomationTime AccountTasks (r:1 w:1)
	/// The range of component `v` is `[0, 1]`.
	fn run_missed_tasks_many_found(v: u32, ) -> Weight {
		(191_000 as Weight)
			// Standard Error: 64_000
			.saturating_add((14_895_000 as Weight).saturating_mul(v as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(v as Weight)))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(v as Weight)))
	}
	// Storage: AutomationTime AccountTasks (r:1 w:0)
	/// The range of component `v` is `[0, 1]`.
	fn run_missed_tasks_many_missing(v: u32, ) -> Weight {
		(189_000 as Weight)
			// Standard Error: 29_000
			.saturating_add((11_304_000 as Weight).saturating_mul(v as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(v as Weight)))
	}
	// Storage: AutomationTime AccountTasks (r:1 w:1)
	// Storage: System Account (r:2 w:2)
	/// The range of component `v` is `[0, 1]`.
	fn run_tasks_many_found(v: u32, ) -> Weight {
		(247_000 as Weight)
			// Standard Error: 30_000
			.saturating_add((39_253_000 as Weight).saturating_mul(v as Weight))
			.saturating_add(T::DbWeight::get().reads((3 as Weight).saturating_mul(v as Weight)))
			.saturating_add(T::DbWeight::get().writes((3 as Weight).saturating_mul(v as Weight)))
	}
	// Storage: AutomationTime AccountTasks (r:1 w:0)
	/// The range of component `v` is `[0, 1]`.
	fn run_tasks_many_missing(v: u32, ) -> Weight {
		(184_000 as Weight)
			// Standard Error: 32_000
			.saturating_add((11_425_000 as Weight).saturating_mul(v as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(v as Weight)))
	}
	// Storage: Timestamp Now (r:1 w:0)
	fn update_task_queue_overhead() -> Weight {
		(1_974_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
	}
	// Storage: AutomationTime MissedQueueV2 (r:1 w:1)
	// Storage: AutomationTime ScheduledTasksV3 (r:1 w:1)
	/// The range of component `v` is `[0, 2]`.
	fn append_to_missed_tasks(v: u32, ) -> Weight {
		(1_919_000 as Weight)
			// Standard Error: 238_000
			.saturating_add((2_500_000 as Weight).saturating_mul(v as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(v as Weight)))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(v as Weight)))
	}
	// Storage: AutomationTime TaskQueueV2 (r:1 w:1)
	// Storage: AutomationTime MissedQueueV2 (r:1 w:1)
	// Storage: AutomationTime ScheduledTasksV3 (r:1 w:1)
	fn update_scheduled_task_queue() -> Weight {
		(45_209_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: AutomationTime ScheduledTasksV3 (r:1 w:1)
	fn shift_missed_tasks() -> Weight {
		(29_532_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: AutomationTime AccountTasks (r:2 w:1)
	/// The range of component `v` is `[1, 100]`.
	fn migration_add_schedule_to_task(v: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 10_000
			.saturating_add((8_680_000 as Weight).saturating_mul(v as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(v as Weight)))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(v as Weight)))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: System Account (r:2 w:2)
	// Storage: AutomationTime AccountTasks (r:1 w:1)
	// Storage: AutomationTime ScheduledTasksV3 (r:1 w:1)
	fn schedule_notify_task_empty() -> Weight {
		(58_311_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(5 as Weight))
			.saturating_add(RocksDbWeight::get().writes(4 as Weight))
	}
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: System Account (r:2 w:2)
	// Storage: AutomationTime AccountTasks (r:1 w:1)
	// Storage: AutomationTime ScheduledTasksV3 (r:1 w:1)
	/// The range of component `v` is `[1, 24]`.
	fn schedule_notify_task_full(v: u32, ) -> Weight {
		(74_430_000 as Weight)
			// Standard Error: 40_000
			.saturating_add((34_085_000 as Weight).saturating_mul(v as Weight))
			.saturating_add(RocksDbWeight::get().reads(4 as Weight))
			.saturating_add(RocksDbWeight::get().reads((1 as Weight).saturating_mul(v as Weight)))
			.saturating_add(RocksDbWeight::get().writes(3 as Weight))
			.saturating_add(RocksDbWeight::get().writes((1 as Weight).saturating_mul(v as Weight)))
	}
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: XcmpHandler XcmChainCurrencyData (r:1 w:0)
	// Storage: System Account (r:3 w:3)
	// Storage: AutomationTime AccountTasks (r:1 w:1)
	// Storage: AutomationTime ScheduledTasksV3 (r:1 w:1)
	// Storage: ParachainInfo ParachainId (r:1 w:0)
	/// The range of component `v` is `[1, 24]`.
	fn schedule_xcmp_task_full(v: u32, ) -> Weight {
		(111_741_000 as Weight)
			// Standard Error: 32_000
			.saturating_add((32_935_000 as Weight).saturating_mul(v as Weight))
			.saturating_add(RocksDbWeight::get().reads(7 as Weight))
			.saturating_add(RocksDbWeight::get().reads((1 as Weight).saturating_mul(v as Weight)))
			.saturating_add(RocksDbWeight::get().writes(4 as Weight))
			.saturating_add(RocksDbWeight::get().writes((1 as Weight).saturating_mul(v as Weight)))
	}
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: System Account (r:2 w:2)
	// Storage: AutomationTime AccountTasks (r:1 w:1)
	// Storage: AutomationTime ScheduledTasksV3 (r:1 w:1)
	fn schedule_native_transfer_task_empty() -> Weight {
		(58_346_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(5 as Weight))
			.saturating_add(RocksDbWeight::get().writes(4 as Weight))
	}
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: System Account (r:2 w:2)
	// Storage: AutomationTime AccountTasks (r:1 w:1)
	// Storage: AutomationTime ScheduledTasksV3 (r:1 w:1)
	/// The range of component `v` is `[1, 24]`.
	fn schedule_native_transfer_task_full(v: u32, ) -> Weight {
		(81_795_000 as Weight)
			// Standard Error: 45_000
			.saturating_add((33_404_000 as Weight).saturating_mul(v as Weight))
			.saturating_add(RocksDbWeight::get().reads(4 as Weight))
			.saturating_add(RocksDbWeight::get().reads((1 as Weight).saturating_mul(v as Weight)))
			.saturating_add(RocksDbWeight::get().writes(3 as Weight))
			.saturating_add(RocksDbWeight::get().writes((1 as Weight).saturating_mul(v as Weight)))
	}
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: System Account (r:2 w:2)
	// Storage: AutomationTime AccountTasks (r:1 w:1)
	// Storage: AutomationTime ScheduledTasksV3 (r:1 w:1)
	fn schedule_auto_compound_delegated_stake_task_full() -> Weight {
		(89_414_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(5 as Weight))
			.saturating_add(RocksDbWeight::get().writes(4 as Weight))
	}
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: System Account (r:2 w:2)
	// Storage: AutomationTime AccountTasks (r:1 w:1)
	// Storage: AutomationTime ScheduledTasksV3 (r:1 w:1)
	/// The range of component `v` is `[1, 24]`.
	fn schedule_dynamic_dispatch_task(v: u32, ) -> Weight {
		(45_793_000 as Weight)
			// Standard Error: 15_000
			.saturating_add((3_890_000 as Weight).saturating_mul(v as Weight))
			.saturating_add(RocksDbWeight::get().reads(4 as Weight))
			.saturating_add(RocksDbWeight::get().reads((1 as Weight).saturating_mul(v as Weight)))
			.saturating_add(RocksDbWeight::get().writes(3 as Weight))
			.saturating_add(RocksDbWeight::get().writes((1 as Weight).saturating_mul(v as Weight)))
	}
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: System Account (r:2 w:2)
	// Storage: AutomationTime AccountTasks (r:1 w:1)
	// Storage: AutomationTime ScheduledTasksV3 (r:1 w:1)
	/// The range of component `v` is `[1, 24]`.
	fn schedule_dynamic_dispatch_task_full(v: u32, ) -> Weight {
		(52_072_000 as Weight)
			// Standard Error: 130_000
			.saturating_add((33_164_000 as Weight).saturating_mul(v as Weight))
			.saturating_add(RocksDbWeight::get().reads(4 as Weight))
			.saturating_add(RocksDbWeight::get().reads((1 as Weight).saturating_mul(v as Weight)))
			.saturating_add(RocksDbWeight::get().writes(3 as Weight))
			.saturating_add(RocksDbWeight::get().writes((1 as Weight).saturating_mul(v as Weight)))
	}
	// Storage: AutomationTime AccountTasks (r:1 w:1)
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: AutomationTime LastTimeSlot (r:1 w:0)
	// Storage: AutomationTime ScheduledTasksV3 (r:24 w:24)
	fn cancel_scheduled_task_full() -> Weight {
		(807_870_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(27 as Weight))
			.saturating_add(RocksDbWeight::get().writes(25 as Weight))
	}
	// Storage: AutomationTime AccountTasks (r:1 w:1)
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: AutomationTime LastTimeSlot (r:1 w:0)
	// Storage: AutomationTime ScheduledTasksV3 (r:1 w:1)
	fn force_cancel_scheduled_task() -> Weight {
		(28_072_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(4 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
	// Storage: AutomationTime AccountTasks (r:1 w:1)
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: AutomationTime LastTimeSlot (r:1 w:0)
	// Storage: AutomationTime ScheduledTasksV3 (r:24 w:24)
	fn force_cancel_scheduled_task_full() -> Weight {
		(809_933_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(27 as Weight))
			.saturating_add(RocksDbWeight::get().writes(25 as Weight))
	}
	fn run_notify_task() -> Weight {
		(8_851_000 as Weight)
	}
	// Storage: System Account (r:2 w:2)
	fn run_native_transfer_task() -> Weight {
		(33_995_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
	// Storage: XcmpHandler XcmChainCurrencyData (r:1 w:0)
	// Storage: ParachainInfo ParachainId (r:1 w:0)
	// Storage: UnknownTokens ConcreteFungibleBalances (r:1 w:0)
	// Storage: AssetRegistry LocationToAssetId (r:1 w:0)
	// Storage: System Account (r:2 w:2)
	// Storage: ParachainSystem HostConfiguration (r:1 w:0)
	// Storage: ParachainSystem PendingUpwardMessages (r:1 w:1)
	fn run_xcmp_task() -> Weight {
		(102_000_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(8 as Weight))
			.saturating_add(RocksDbWeight::get().writes(3 as Weight))
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
	// Storage: AutomationTime ScheduledTasksV3 (r:1 w:1)
	fn run_auto_compound_delegated_stake_task() -> Weight {
		(105_359_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(11 as Weight))
			.saturating_add(RocksDbWeight::get().writes(9 as Weight))
	}
	// Storage: Valve ValveClosed (r:1 w:0)
	// Storage: Valve ClosedPallets (r:1 w:0)
	fn run_dynamic_dispatch_action() -> Weight {
		(17_152_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
	}
	fn run_dynamic_dispatch_action_fail_decode() -> Weight {
		(8_984_000 as Weight)
	}
	// Storage: AutomationTime AccountTasks (r:1 w:1)
	/// The range of component `v` is `[0, 1]`.
	fn run_missed_tasks_many_found(v: u32, ) -> Weight {
		(191_000 as Weight)
			// Standard Error: 64_000
			.saturating_add((14_895_000 as Weight).saturating_mul(v as Weight))
			.saturating_add(RocksDbWeight::get().reads((1 as Weight).saturating_mul(v as Weight)))
			.saturating_add(RocksDbWeight::get().writes((1 as Weight).saturating_mul(v as Weight)))
	}
	// Storage: AutomationTime AccountTasks (r:1 w:0)
	/// The range of component `v` is `[0, 1]`.
	fn run_missed_tasks_many_missing(v: u32, ) -> Weight {
		(189_000 as Weight)
			// Standard Error: 29_000
			.saturating_add((11_304_000 as Weight).saturating_mul(v as Weight))
			.saturating_add(RocksDbWeight::get().reads((1 as Weight).saturating_mul(v as Weight)))
	}
	// Storage: AutomationTime AccountTasks (r:1 w:1)
	// Storage: System Account (r:2 w:2)
	/// The range of component `v` is `[0, 1]`.
	fn run_tasks_many_found(v: u32, ) -> Weight {
		(247_000 as Weight)
			// Standard Error: 30_000
			.saturating_add((39_253_000 as Weight).saturating_mul(v as Weight))
			.saturating_add(RocksDbWeight::get().reads((3 as Weight).saturating_mul(v as Weight)))
			.saturating_add(RocksDbWeight::get().writes((3 as Weight).saturating_mul(v as Weight)))
	}
	// Storage: AutomationTime AccountTasks (r:1 w:0)
	/// The range of component `v` is `[0, 1]`.
	fn run_tasks_many_missing(v: u32, ) -> Weight {
		(184_000 as Weight)
			// Standard Error: 32_000
			.saturating_add((11_425_000 as Weight).saturating_mul(v as Weight))
			.saturating_add(RocksDbWeight::get().reads((1 as Weight).saturating_mul(v as Weight)))
	}
	// Storage: Timestamp Now (r:1 w:0)
	fn update_task_queue_overhead() -> Weight {
		(1_974_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
	}
	// Storage: AutomationTime MissedQueueV2 (r:1 w:1)
	// Storage: AutomationTime ScheduledTasksV3 (r:1 w:1)
	/// The range of component `v` is `[0, 2]`.
	fn append_to_missed_tasks(v: u32, ) -> Weight {
		(1_919_000 as Weight)
			// Standard Error: 238_000
			.saturating_add((2_500_000 as Weight).saturating_mul(v as Weight))
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().reads((1 as Weight).saturating_mul(v as Weight)))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes((1 as Weight).saturating_mul(v as Weight)))
	}
	// Storage: AutomationTime TaskQueueV2 (r:1 w:1)
	// Storage: AutomationTime MissedQueueV2 (r:1 w:1)
	// Storage: AutomationTime ScheduledTasksV3 (r:1 w:1)
	fn update_scheduled_task_queue() -> Weight {
		(45_209_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(3 as Weight))
			.saturating_add(RocksDbWeight::get().writes(3 as Weight))
	}
	// Storage: AutomationTime ScheduledTasksV3 (r:1 w:1)
	fn shift_missed_tasks() -> Weight {
		(29_532_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: AutomationTime AccountTasks (r:2 w:1)
	/// The range of component `v` is `[1, 100]`.
	fn migration_add_schedule_to_task(v: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 10_000
			.saturating_add((8_680_000 as Weight).saturating_mul(v as Weight))
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().reads((1 as Weight).saturating_mul(v as Weight)))
			.saturating_add(RocksDbWeight::get().writes((1 as Weight).saturating_mul(v as Weight)))
	}
}
