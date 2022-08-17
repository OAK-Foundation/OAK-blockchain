
//! Autogenerated weights for `pallet_automation_time`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-05-26, STEPS: `1`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 128

// Executed Command:
// ./target/release/neumann-collator
// benchmark pallet
// --chain
// dev
// --execution
// wasm
// --wasm-execution
// compiled
// --pallet
// pallet_automation_time
// --extrinsic
// "*"
// --repeat
// 64
// --steps
// 100
// --output
// ./pallets/automation-time/src/weights.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_automation_time.
pub trait WeightInfo {
	fn emit_event() -> Weight;
	fn run_native_transfer_task() -> Weight;
	fn reset_asset(v: u32, ) -> Weight;
	fn update_asset_reset() -> Weight;
	fn delete_asset_tasks() -> Weight;
	
	fn delete_asset_extrinsic() -> Weight;
	fn asset_price_update_extrinsic() -> Weight;
	fn add_asset_extrinsic() -> Weight;
	fn schedule_transfer_task_extrinsic() -> Weight;
}

/// Weight functions for `pallet_automation_time`.
pub struct AutomationWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for AutomationWeight<T> {
	fn emit_event() -> Weight {
		(20_000_000 as Weight)
	}
	fn run_native_transfer_task() -> Weight {
		(230_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	fn reset_asset(v: u32, ) -> Weight {
		(200_000_000 as Weight)
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
			.saturating_add((20_000_000 as Weight).saturating_mul(v as Weight))
			.saturating_add(T::DbWeight::get().reads((4 as Weight).saturating_mul(v as Weight)))
			.saturating_add(T::DbWeight::get().writes((5 as Weight).saturating_mul(v as Weight)))
	}
	fn update_asset_reset() -> Weight{
		(200_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn delete_asset_tasks() -> Weight{
		(200_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	fn delete_asset_extrinsic() -> Weight{
		(220_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(7 as Weight))
	}
	fn asset_price_update_extrinsic() -> Weight{
		(220_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(6 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
			.saturating_add(T::DbWeight::get().reads(21 as Weight))
			.saturating_add(T::DbWeight::get().writes(21 as Weight))
	}
	fn add_asset_extrinsic() -> Weight{
		(220_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(5 as Weight))
	}
	fn schedule_transfer_task_extrinsic() -> Weight{
		(200_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(6 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
}
