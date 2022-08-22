
//! Autogenerated weights for `pallet_automation_time`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-08-22, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
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

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_automation_time`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_automation_time::WeightInfo for WeightInfo<T> {
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: System Account (r:2 w:2)
	// Storage: AutomationTime AccountTasks (r:1 w:1)
	// Storage: AutomationTime ScheduledTasksV2 (r:1 w:1)
	fn schedule_notify_task_empty() -> Weight {
		(57_851_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: System Account (r:2 w:2)
	// Storage: AutomationTime AccountTasks (r:1 w:1)
	// Storage: AutomationTime ScheduledTasksV2 (r:1 w:1)
	/// The range of component `v` is `[1, 24]`.
	fn schedule_notify_task_full(v: u32, ) -> Weight {
		(74_771_000 as Weight)
			// Standard Error: 26_000
			.saturating_add((34_322_000 as Weight).saturating_mul(v as Weight))
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
		(112_104_000 as Weight)
			// Standard Error: 23_000
			.saturating_add((33_919_000 as Weight).saturating_mul(v as Weight))
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
		(57_930_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: System Account (r:2 w:2)
	// Storage: AutomationTime AccountTasks (r:1 w:1)
	// Storage: AutomationTime ScheduledTasksV2 (r:1 w:1)
	/// The range of component `v` is `[1, 24]`.
	fn schedule_native_transfer_task_full(v: u32, ) -> Weight {
		(77_734_000 as Weight)
			// Standard Error: 115_000
			.saturating_add((34_292_000 as Weight).saturating_mul(v as Weight))
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
		(111_280_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	// Storage: AutomationTime AccountTasks (r:1 w:1)
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: AutomationTime LastTimeSlot (r:1 w:0)
	// Storage: AutomationTime ScheduledTasksV2 (r:24 w:24)
	fn cancel_scheduled_task_full() -> Weight {
		(828_022_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(27 as Weight))
			.saturating_add(T::DbWeight::get().writes(25 as Weight))
	}
	// Storage: AutomationTime AccountTasks (r:1 w:1)
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: AutomationTime LastTimeSlot (r:1 w:0)
	// Storage: AutomationTime ScheduledTasksV2 (r:1 w:1)
	fn force_cancel_scheduled_task() -> Weight {
		(27_600_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: AutomationTime AccountTasks (r:1 w:1)
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: AutomationTime LastTimeSlot (r:1 w:0)
	// Storage: AutomationTime ScheduledTasksV2 (r:24 w:24)
	fn force_cancel_scheduled_task_full() -> Weight {
		(827_398_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(27 as Weight))
			.saturating_add(T::DbWeight::get().writes(25 as Weight))
	}
	fn run_notify_task() -> Weight {
		(8_704_000 as Weight)
	}
	// Storage: System Account (r:2 w:2)
	fn run_native_transfer_task() -> Weight {
		(34_564_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: XcmpHandler XcmChainCurrencyData (r:1 w:0)
	// Storage: ParachainInfo ParachainId (r:1 w:0)
	// Storage: UnknownTokens ConcreteFungibleBalances (r:1 w:0)
	// Storage: System Account (r:2 w:2)
	// Storage: ParachainSystem RelevantMessagingState (r:1 w:0)
	fn run_xcmp_task() -> Weight {
		(79_534_000 as Weight)
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
		(103_858_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(11 as Weight))
			.saturating_add(T::DbWeight::get().writes(9 as Weight))
	}
	// Storage: AutomationTime AccountTasks (r:1 w:1)
	/// The range of component `v` is `[0, 1]`.
	fn run_missed_tasks_many_found(v: u32, ) -> Weight {
		(160_000 as Weight)
			// Standard Error: 23_000
			.saturating_add((14_658_000 as Weight).saturating_mul(v as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(v as Weight)))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(v as Weight)))
	}
	// Storage: AutomationTime AccountTasks (r:1 w:0)
	/// The range of component `v` is `[0, 1]`.
	fn run_missed_tasks_many_missing(v: u32, ) -> Weight {
		(156_000 as Weight)
			// Standard Error: 19_000
			.saturating_add((11_200_000 as Weight).saturating_mul(v as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(v as Weight)))
	}
	// Storage: AutomationTime AccountTasks (r:1 w:1)
	// Storage: System Account (r:2 w:2)
	/// The range of component `v` is `[0, 1]`.
	fn run_tasks_many_found(v: u32, ) -> Weight {
		(185_000 as Weight)
			// Standard Error: 52_000
			.saturating_add((39_759_000 as Weight).saturating_mul(v as Weight))
			.saturating_add(T::DbWeight::get().reads((3 as Weight).saturating_mul(v as Weight)))
			.saturating_add(T::DbWeight::get().writes((3 as Weight).saturating_mul(v as Weight)))
	}
	// Storage: AutomationTime AccountTasks (r:1 w:0)
	/// The range of component `v` is `[0, 1]`.
	fn run_tasks_many_missing(v: u32, ) -> Weight {
		(156_000 as Weight)
			// Standard Error: 25_000
			.saturating_add((11_193_000 as Weight).saturating_mul(v as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(v as Weight)))
	}
	// Storage: Timestamp Now (r:1 w:0)
	fn update_task_queue_overhead() -> Weight {
		(1_898_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
	}
	// Storage: AutomationTime MissedQueueV2 (r:1 w:1)
	// Storage: AutomationTime ScheduledTasksV2 (r:1 w:1)
	/// The range of component `v` is `[0, 2]`.
	fn append_to_missed_tasks(v: u32, ) -> Weight {
		(1_890_000 as Weight)
			// Standard Error: 227_000
			.saturating_add((2_476_000 as Weight).saturating_mul(v as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(v as Weight)))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(v as Weight)))
	}
	// Storage: AutomationTime TaskQueueV2 (r:1 w:1)
	// Storage: AutomationTime MissedQueueV2 (r:1 w:1)
	// Storage: AutomationTime ScheduledTasksV2 (r:1 w:1)
	fn update_scheduled_task_queue() -> Weight {
		(47_916_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: AutomationTime ScheduledTasksV2 (r:1 w:1)
	fn shift_missed_tasks() -> Weight {
		(28_672_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: AutomationTime Tasks (r:7 w:6)
	// Storage: AutomationTime ScheduledTasks (r:3 w:2)
	// Storage: AutomationTime TaskQueue (r:1 w:1)
	// Storage: AutomationTime MissedQueue (r:1 w:1)
	// Storage: AutomationTime AccountTasks (r:0 w:6)
	// Storage: AutomationTime ScheduledTasksV2 (r:0 w:2)
	// Storage: AutomationTime MissedQueueV2 (r:0 w:1)
	// Storage: AutomationTime TaskQueueV2 (r:0 w:1)
	fn migration_v3() -> Weight {
		(79_722_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(12 as Weight))
			.saturating_add(T::DbWeight::get().writes(20 as Weight))
	}
}
