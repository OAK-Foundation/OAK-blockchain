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
//! DATE: 2023-01-30, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
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
//:schedule_notify_task_empty 73_004_000
//:schedule_notify_task_full 101_463_000
//:schedule_xcmp_task_full 132_975_000
//:schedule_native_transfer_task_empty 73_241_000
//:schedule_native_transfer_task_full 102_459_000
//:schedule_auto_compound_delegated_stake_task_full 107_584_000
//:schedule_dynamic_dispatch_task 70_690_000
//:schedule_dynamic_dispatch_task_full 98_508_000
//:cancel_scheduled_task_full 643_569_000
//:force_cancel_scheduled_task 31_740_000
//:force_cancel_scheduled_task_full 642_271_000
//:run_notify_task 9_662_000
//:run_native_transfer_task 35_565_000
//:run_xcmp_task 88_857_000
//:run_auto_compound_delegated_stake_task 68_804_000
//:run_dynamic_dispatch_action 19_046_000
//:run_dynamic_dispatch_action_fail_decode 10_032_000
//:run_missed_tasks_many_found 205_000
//:run_missed_tasks_many_missing 193_000
//:run_tasks_many_found 272_000
//:run_tasks_many_missing 185_000
//:update_task_queue_overhead 3_282_000
//:append_to_missed_tasks 4_180_000
//:update_scheduled_task_queue 42_579_000
//:shift_missed_tasks 28_300_000
//:migration_add_schedule_to_task 14_127_000

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
	// Storage: AssetRegistry Metadata (r:1 w:0)
	// Storage: AssetRegistry LocationToAssetId (r:1 w:0)
	fn schedule_notify_task_empty() -> Weight {
		(73_004_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(7 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: System Account (r:2 w:2)
	// Storage: AutomationTime AccountTasks (r:1 w:1)
	// Storage: AutomationTime ScheduledTasksV3 (r:1 w:1)
	// Storage: AssetRegistry Metadata (r:1 w:0)
	// Storage: AssetRegistry LocationToAssetId (r:1 w:0)
	/// The range of component `v` is `[1, 24]`.
	fn schedule_notify_task_full(v: u32, ) -> Weight {
		(101_463_000 as Weight)
			// Standard Error: 20_000
			.saturating_add((26_555_000 as Weight).saturating_mul(v as Weight))
			.saturating_add(T::DbWeight::get().reads(6 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(v as Weight)))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(v as Weight)))
	}
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: XcmpHandler XcmChainCurrencyData (r:1 w:0)
	// Storage: System Account (r:3 w:3)
	// Storage: AutomationTime AccountTasks (r:1 w:1)
	// Storage: AutomationTime ScheduledTasksV3 (r:1 w:1)
	// Storage: AssetRegistry Metadata (r:1 w:0)
	// Storage: AssetRegistry LocationToAssetId (r:1 w:0)
	// Storage: ParachainInfo ParachainId (r:1 w:0)
	/// The range of component `v` is `[1, 24]`.
	fn schedule_xcmp_task_full(v: u32, ) -> Weight {
		(132_975_000 as Weight)
			// Standard Error: 15_000
			.saturating_add((26_160_000 as Weight).saturating_mul(v as Weight))
			.saturating_add(T::DbWeight::get().reads(9 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(v as Weight)))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(v as Weight)))
	}
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: System Account (r:2 w:2)
	// Storage: AutomationTime AccountTasks (r:1 w:1)
	// Storage: AutomationTime ScheduledTasksV3 (r:1 w:1)
	// Storage: AssetRegistry Metadata (r:1 w:0)
	// Storage: AssetRegistry LocationToAssetId (r:1 w:0)
	fn schedule_native_transfer_task_empty() -> Weight {
		(73_241_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(7 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: System Account (r:2 w:2)
	// Storage: AutomationTime AccountTasks (r:1 w:1)
	// Storage: AutomationTime ScheduledTasksV3 (r:1 w:1)
	// Storage: AssetRegistry Metadata (r:1 w:0)
	// Storage: AssetRegistry LocationToAssetId (r:1 w:0)
	/// The range of component `v` is `[1, 24]`.
	fn schedule_native_transfer_task_full(v: u32, ) -> Weight {
		(102_459_000 as Weight)
			// Standard Error: 13_000
			.saturating_add((26_356_000 as Weight).saturating_mul(v as Weight))
			.saturating_add(T::DbWeight::get().reads(6 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(v as Weight)))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(v as Weight)))
	}
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: System Account (r:2 w:2)
	// Storage: AutomationTime AccountTasks (r:1 w:1)
	// Storage: AutomationTime ScheduledTasksV3 (r:1 w:1)
	// Storage: AssetRegistry Metadata (r:1 w:0)
	// Storage: AssetRegistry LocationToAssetId (r:1 w:0)
	fn schedule_auto_compound_delegated_stake_task_full() -> Weight {
		(107_584_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(7 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: System Account (r:2 w:2)
	// Storage: AutomationTime AccountTasks (r:1 w:1)
	// Storage: AutomationTime ScheduledTasksV3 (r:1 w:1)
	// Storage: AssetRegistry Metadata (r:1 w:0)
	// Storage: AssetRegistry LocationToAssetId (r:1 w:0)
	/// The range of component `v` is `[1, 24]`.
	fn schedule_dynamic_dispatch_task(v: u32, ) -> Weight {
		(70_690_000 as Weight)
			// Standard Error: 6_000
			.saturating_add((3_771_000 as Weight).saturating_mul(v as Weight))
			.saturating_add(T::DbWeight::get().reads(6 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(v as Weight)))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(v as Weight)))
	}
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: System Account (r:2 w:2)
	// Storage: AutomationTime AccountTasks (r:1 w:1)
	// Storage: AutomationTime ScheduledTasksV3 (r:1 w:1)
	// Storage: AssetRegistry Metadata (r:1 w:0)
	// Storage: AssetRegistry LocationToAssetId (r:1 w:0)
	/// The range of component `v` is `[1, 24]`.
	fn schedule_dynamic_dispatch_task_full(v: u32, ) -> Weight {
		(98_508_000 as Weight)
			// Standard Error: 15_000
			.saturating_add((26_723_000 as Weight).saturating_mul(v as Weight))
			.saturating_add(T::DbWeight::get().reads(6 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(v as Weight)))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(v as Weight)))
	}
	// Storage: AutomationTime AccountTasks (r:1 w:1)
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: AutomationTime LastTimeSlot (r:1 w:0)
	// Storage: AutomationTime ScheduledTasksV3 (r:24 w:24)
	fn cancel_scheduled_task_full() -> Weight {
		(643_569_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(27 as Weight))
			.saturating_add(T::DbWeight::get().writes(25 as Weight))
	}
	// Storage: AutomationTime AccountTasks (r:1 w:1)
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: AutomationTime LastTimeSlot (r:1 w:0)
	// Storage: AutomationTime ScheduledTasksV3 (r:1 w:1)
	fn force_cancel_scheduled_task() -> Weight {
		(31_740_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: AutomationTime AccountTasks (r:1 w:1)
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: AutomationTime LastTimeSlot (r:1 w:0)
	// Storage: AutomationTime ScheduledTasksV3 (r:24 w:24)
	fn force_cancel_scheduled_task_full() -> Weight {
		(642_271_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(27 as Weight))
			.saturating_add(T::DbWeight::get().writes(25 as Weight))
	}
	fn run_notify_task() -> Weight {
		(9_662_000 as Weight)
	}
	// Storage: System Account (r:2 w:2)
	fn run_native_transfer_task() -> Weight {
		(35_565_000 as Weight)
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
		(88_857_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(8 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: System Account (r:1 w:1)
	// Storage: ParachainStaking DelegatorState (r:1 w:1)
	// Storage: ParachainStaking DelegationScheduledRequests (r:1 w:0)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: ParachainStaking CandidateInfo (r:1 w:1)
	// Storage: ParachainStaking TopDelegations (r:1 w:1)
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	// Storage: ParachainStaking Total (r:1 w:1)
	fn run_auto_compound_delegated_stake_task() -> Weight {
		(68_804_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(8 as Weight))
			.saturating_add(T::DbWeight::get().writes(7 as Weight))
	}
	// Storage: Valve ValveClosed (r:1 w:0)
	// Storage: Valve ClosedPallets (r:1 w:0)
	fn run_dynamic_dispatch_action() -> Weight {
		(19_046_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
	}
	fn run_dynamic_dispatch_action_fail_decode() -> Weight {
		(10_032_000 as Weight)
	}
	// Storage: AutomationTime AccountTasks (r:1 w:1)
	/// The range of component `v` is `[0, 1]`.
	fn run_missed_tasks_many_found(v: u32, ) -> Weight {
		(205_000 as Weight)
			// Standard Error: 19_000
			.saturating_add((19_219_000 as Weight).saturating_mul(v as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(v as Weight)))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(v as Weight)))
	}
	// Storage: AutomationTime AccountTasks (r:1 w:0)
	/// The range of component `v` is `[0, 1]`.
	fn run_missed_tasks_many_missing(v: u32, ) -> Weight {
		(193_000 as Weight)
			// Standard Error: 6_000
			.saturating_add((13_267_000 as Weight).saturating_mul(v as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(v as Weight)))
	}
	// Storage: AutomationTime AccountTasks (r:1 w:1)
	// Storage: System Account (r:2 w:2)
	/// The range of component `v` is `[0, 1]`.
	fn run_tasks_many_found(v: u32, ) -> Weight {
		(272_000 as Weight)
			// Standard Error: 9_000
			.saturating_add((43_026_000 as Weight).saturating_mul(v as Weight))
			.saturating_add(T::DbWeight::get().reads((3 as Weight).saturating_mul(v as Weight)))
			.saturating_add(T::DbWeight::get().writes((3 as Weight).saturating_mul(v as Weight)))
	}
	// Storage: AutomationTime AccountTasks (r:1 w:0)
	/// The range of component `v` is `[0, 1]`.
	fn run_tasks_many_missing(v: u32, ) -> Weight {
		(185_000 as Weight)
			// Standard Error: 5_000
			.saturating_add((13_198_000 as Weight).saturating_mul(v as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(v as Weight)))
	}
	// Storage: Timestamp Now (r:1 w:0)
	fn update_task_queue_overhead() -> Weight {
		(3_282_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
	}
	// Storage: AutomationTime MissedQueueV2 (r:1 w:1)
	// Storage: AutomationTime ScheduledTasksV3 (r:1 w:1)
	/// The range of component `v` is `[0, 2]`.
	fn append_to_missed_tasks(v: u32, ) -> Weight {
		(4_180_000 as Weight)
			// Standard Error: 41_000
			.saturating_add((1_234_000 as Weight).saturating_mul(v as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: AutomationTime TaskQueueV2 (r:1 w:1)
	// Storage: AutomationTime MissedQueueV2 (r:1 w:1)
	// Storage: AutomationTime ScheduledTasksV3 (r:1 w:1)
	fn update_scheduled_task_queue() -> Weight {
		(42_579_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: AutomationTime ScheduledTasksV3 (r:1 w:1)
	fn shift_missed_tasks() -> Weight {
		(28_300_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: AutomationTime AccountTasks (r:2 w:1)
	/// The range of component `v` is `[1, 100]`.
	fn migration_add_schedule_to_task(v: u32, ) -> Weight {
		(14_127_000 as Weight)
			// Standard Error: 9_000
			.saturating_add((8_020_000 as Weight).saturating_mul(v as Weight))
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
	// Storage: AssetRegistry Metadata (r:1 w:0)
	// Storage: AssetRegistry LocationToAssetId (r:1 w:0)
	fn schedule_notify_task_empty() -> Weight {
		(73_004_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(7 as Weight))
			.saturating_add(RocksDbWeight::get().writes(4 as Weight))
	}
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: System Account (r:2 w:2)
	// Storage: AutomationTime AccountTasks (r:1 w:1)
	// Storage: AutomationTime ScheduledTasksV3 (r:1 w:1)
	// Storage: AssetRegistry Metadata (r:1 w:0)
	// Storage: AssetRegistry LocationToAssetId (r:1 w:0)
	/// The range of component `v` is `[1, 24]`.
	fn schedule_notify_task_full(v: u32, ) -> Weight {
		(101_463_000 as Weight)
			// Standard Error: 20_000
			.saturating_add((26_555_000 as Weight).saturating_mul(v as Weight))
			.saturating_add(RocksDbWeight::get().reads(6 as Weight))
			.saturating_add(RocksDbWeight::get().reads((1 as Weight).saturating_mul(v as Weight)))
			.saturating_add(RocksDbWeight::get().writes(3 as Weight))
			.saturating_add(RocksDbWeight::get().writes((1 as Weight).saturating_mul(v as Weight)))
	}
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: XcmpHandler XcmChainCurrencyData (r:1 w:0)
	// Storage: System Account (r:3 w:3)
	// Storage: AutomationTime AccountTasks (r:1 w:1)
	// Storage: AutomationTime ScheduledTasksV3 (r:1 w:1)
	// Storage: AssetRegistry Metadata (r:1 w:0)
	// Storage: AssetRegistry LocationToAssetId (r:1 w:0)
	// Storage: ParachainInfo ParachainId (r:1 w:0)
	/// The range of component `v` is `[1, 24]`.
	fn schedule_xcmp_task_full(v: u32, ) -> Weight {
		(132_975_000 as Weight)
			// Standard Error: 15_000
			.saturating_add((26_160_000 as Weight).saturating_mul(v as Weight))
			.saturating_add(RocksDbWeight::get().reads(9 as Weight))
			.saturating_add(RocksDbWeight::get().reads((1 as Weight).saturating_mul(v as Weight)))
			.saturating_add(RocksDbWeight::get().writes(4 as Weight))
			.saturating_add(RocksDbWeight::get().writes((1 as Weight).saturating_mul(v as Weight)))
	}
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: System Account (r:2 w:2)
	// Storage: AutomationTime AccountTasks (r:1 w:1)
	// Storage: AutomationTime ScheduledTasksV3 (r:1 w:1)
	// Storage: AssetRegistry Metadata (r:1 w:0)
	// Storage: AssetRegistry LocationToAssetId (r:1 w:0)
	fn schedule_native_transfer_task_empty() -> Weight {
		(73_241_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(7 as Weight))
			.saturating_add(RocksDbWeight::get().writes(4 as Weight))
	}
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: System Account (r:2 w:2)
	// Storage: AutomationTime AccountTasks (r:1 w:1)
	// Storage: AutomationTime ScheduledTasksV3 (r:1 w:1)
	// Storage: AssetRegistry Metadata (r:1 w:0)
	// Storage: AssetRegistry LocationToAssetId (r:1 w:0)
	/// The range of component `v` is `[1, 24]`.
	fn schedule_native_transfer_task_full(v: u32, ) -> Weight {
		(102_459_000 as Weight)
			// Standard Error: 13_000
			.saturating_add((26_356_000 as Weight).saturating_mul(v as Weight))
			.saturating_add(RocksDbWeight::get().reads(6 as Weight))
			.saturating_add(RocksDbWeight::get().reads((1 as Weight).saturating_mul(v as Weight)))
			.saturating_add(RocksDbWeight::get().writes(3 as Weight))
			.saturating_add(RocksDbWeight::get().writes((1 as Weight).saturating_mul(v as Weight)))
	}
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: System Account (r:2 w:2)
	// Storage: AutomationTime AccountTasks (r:1 w:1)
	// Storage: AutomationTime ScheduledTasksV3 (r:1 w:1)
	// Storage: AssetRegistry Metadata (r:1 w:0)
	// Storage: AssetRegistry LocationToAssetId (r:1 w:0)
	fn schedule_auto_compound_delegated_stake_task_full() -> Weight {
		(107_584_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(7 as Weight))
			.saturating_add(RocksDbWeight::get().writes(4 as Weight))
	}
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: System Account (r:2 w:2)
	// Storage: AutomationTime AccountTasks (r:1 w:1)
	// Storage: AutomationTime ScheduledTasksV3 (r:1 w:1)
	// Storage: AssetRegistry Metadata (r:1 w:0)
	// Storage: AssetRegistry LocationToAssetId (r:1 w:0)
	/// The range of component `v` is `[1, 24]`.
	fn schedule_dynamic_dispatch_task(v: u32, ) -> Weight {
		(70_690_000 as Weight)
			// Standard Error: 6_000
			.saturating_add((3_771_000 as Weight).saturating_mul(v as Weight))
			.saturating_add(RocksDbWeight::get().reads(6 as Weight))
			.saturating_add(RocksDbWeight::get().reads((1 as Weight).saturating_mul(v as Weight)))
			.saturating_add(RocksDbWeight::get().writes(3 as Weight))
			.saturating_add(RocksDbWeight::get().writes((1 as Weight).saturating_mul(v as Weight)))
	}
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: System Account (r:2 w:2)
	// Storage: AutomationTime AccountTasks (r:1 w:1)
	// Storage: AutomationTime ScheduledTasksV3 (r:1 w:1)
	// Storage: AssetRegistry Metadata (r:1 w:0)
	// Storage: AssetRegistry LocationToAssetId (r:1 w:0)
	/// The range of component `v` is `[1, 24]`.
	fn schedule_dynamic_dispatch_task_full(v: u32, ) -> Weight {
		(98_508_000 as Weight)
			// Standard Error: 15_000
			.saturating_add((26_723_000 as Weight).saturating_mul(v as Weight))
			.saturating_add(RocksDbWeight::get().reads(6 as Weight))
			.saturating_add(RocksDbWeight::get().reads((1 as Weight).saturating_mul(v as Weight)))
			.saturating_add(RocksDbWeight::get().writes(3 as Weight))
			.saturating_add(RocksDbWeight::get().writes((1 as Weight).saturating_mul(v as Weight)))
	}
	// Storage: AutomationTime AccountTasks (r:1 w:1)
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: AutomationTime LastTimeSlot (r:1 w:0)
	// Storage: AutomationTime ScheduledTasksV3 (r:24 w:24)
	fn cancel_scheduled_task_full() -> Weight {
		(643_569_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(27 as Weight))
			.saturating_add(RocksDbWeight::get().writes(25 as Weight))
	}
	// Storage: AutomationTime AccountTasks (r:1 w:1)
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: AutomationTime LastTimeSlot (r:1 w:0)
	// Storage: AutomationTime ScheduledTasksV3 (r:1 w:1)
	fn force_cancel_scheduled_task() -> Weight {
		(31_740_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(4 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
	// Storage: AutomationTime AccountTasks (r:1 w:1)
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: AutomationTime LastTimeSlot (r:1 w:0)
	// Storage: AutomationTime ScheduledTasksV3 (r:24 w:24)
	fn force_cancel_scheduled_task_full() -> Weight {
		(642_271_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(27 as Weight))
			.saturating_add(RocksDbWeight::get().writes(25 as Weight))
	}
	fn run_notify_task() -> Weight {
		(9_662_000 as Weight)
	}
	// Storage: System Account (r:2 w:2)
	fn run_native_transfer_task() -> Weight {
		(35_565_000 as Weight)
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
		(88_857_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(8 as Weight))
			.saturating_add(RocksDbWeight::get().writes(3 as Weight))
	}
	// Storage: System Account (r:1 w:1)
	// Storage: ParachainStaking DelegatorState (r:1 w:1)
	// Storage: ParachainStaking DelegationScheduledRequests (r:1 w:0)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: ParachainStaking CandidateInfo (r:1 w:1)
	// Storage: ParachainStaking TopDelegations (r:1 w:1)
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	// Storage: ParachainStaking Total (r:1 w:1)
	fn run_auto_compound_delegated_stake_task() -> Weight {
		(68_804_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(8 as Weight))
			.saturating_add(RocksDbWeight::get().writes(7 as Weight))
	}
	// Storage: Valve ValveClosed (r:1 w:0)
	// Storage: Valve ClosedPallets (r:1 w:0)
	fn run_dynamic_dispatch_action() -> Weight {
		(19_046_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
	}
	fn run_dynamic_dispatch_action_fail_decode() -> Weight {
		(10_032_000 as Weight)
	}
	// Storage: AutomationTime AccountTasks (r:1 w:1)
	/// The range of component `v` is `[0, 1]`.
	fn run_missed_tasks_many_found(v: u32, ) -> Weight {
		(205_000 as Weight)
			// Standard Error: 19_000
			.saturating_add((19_219_000 as Weight).saturating_mul(v as Weight))
			.saturating_add(RocksDbWeight::get().reads((1 as Weight).saturating_mul(v as Weight)))
			.saturating_add(RocksDbWeight::get().writes((1 as Weight).saturating_mul(v as Weight)))
	}
	// Storage: AutomationTime AccountTasks (r:1 w:0)
	/// The range of component `v` is `[0, 1]`.
	fn run_missed_tasks_many_missing(v: u32, ) -> Weight {
		(193_000 as Weight)
			// Standard Error: 6_000
			.saturating_add((13_267_000 as Weight).saturating_mul(v as Weight))
			.saturating_add(RocksDbWeight::get().reads((1 as Weight).saturating_mul(v as Weight)))
	}
	// Storage: AutomationTime AccountTasks (r:1 w:1)
	// Storage: System Account (r:2 w:2)
	/// The range of component `v` is `[0, 1]`.
	fn run_tasks_many_found(v: u32, ) -> Weight {
		(272_000 as Weight)
			// Standard Error: 9_000
			.saturating_add((43_026_000 as Weight).saturating_mul(v as Weight))
			.saturating_add(RocksDbWeight::get().reads((3 as Weight).saturating_mul(v as Weight)))
			.saturating_add(RocksDbWeight::get().writes((3 as Weight).saturating_mul(v as Weight)))
	}
	// Storage: AutomationTime AccountTasks (r:1 w:0)
	/// The range of component `v` is `[0, 1]`.
	fn run_tasks_many_missing(v: u32, ) -> Weight {
		(185_000 as Weight)
			// Standard Error: 5_000
			.saturating_add((13_198_000 as Weight).saturating_mul(v as Weight))
			.saturating_add(RocksDbWeight::get().reads((1 as Weight).saturating_mul(v as Weight)))
	}
	// Storage: Timestamp Now (r:1 w:0)
	fn update_task_queue_overhead() -> Weight {
		(3_282_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
	}
	// Storage: AutomationTime MissedQueueV2 (r:1 w:1)
	// Storage: AutomationTime ScheduledTasksV3 (r:1 w:1)
	/// The range of component `v` is `[0, 2]`.
	fn append_to_missed_tasks(v: u32, ) -> Weight {
		(4_180_000 as Weight)
			// Standard Error: 41_000
			.saturating_add((1_234_000 as Weight).saturating_mul(v as Weight))
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: AutomationTime TaskQueueV2 (r:1 w:1)
	// Storage: AutomationTime MissedQueueV2 (r:1 w:1)
	// Storage: AutomationTime ScheduledTasksV3 (r:1 w:1)
	fn update_scheduled_task_queue() -> Weight {
		(42_579_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(3 as Weight))
			.saturating_add(RocksDbWeight::get().writes(3 as Weight))
	}
	// Storage: AutomationTime ScheduledTasksV3 (r:1 w:1)
	fn shift_missed_tasks() -> Weight {
		(28_300_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: AutomationTime AccountTasks (r:2 w:1)
	/// The range of component `v` is `[1, 100]`.
	fn migration_add_schedule_to_task(v: u32, ) -> Weight {
		(14_127_000 as Weight)
			// Standard Error: 9_000
			.saturating_add((8_020_000 as Weight).saturating_mul(v as Weight))
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().reads((1 as Weight).saturating_mul(v as Weight)))
			.saturating_add(RocksDbWeight::get().writes((1 as Weight).saturating_mul(v as Weight)))
	}
}
