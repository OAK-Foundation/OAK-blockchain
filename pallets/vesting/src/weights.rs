
//! Autogenerated weights for `pallet_vesting`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-05-25, STEPS: `20`, REPEAT: 64, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/oak-collator
// benchmark
// pallet
// --chain
// dev
// --execution
// wasm
// --wasm-execution
// compiled
// --pallet
// pallet_vesting
// --extrinsic
// *
// --repeat
// 20
// --steps
// 50
// --output
// ./pallets/vesting/src/raw-weights.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_vest.
pub trait WeightInfo {
	fn vest(v: u32, ) -> Weight;
}

/// Weights for pallet_vesting using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: Vesting VestingSchedule (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: Vesting TotalUnvestedAllocation (r:1 w:1)
	/// The range of component `v` is `[0, 20]`.
	fn vest(v: u32, ) -> Weight {
		(14_917_000 as Weight)
			// Standard Error: 29_000
			.saturating_add((20_808_000 as Weight).saturating_mul(v as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(v as Weight)))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(v as Weight)))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: Vesting VestingSchedule (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: Vesting TotalUnvestedAllocation (r:1 w:1)
	fn vest(v: u32, ) -> Weight {
		(18_229_000 as Weight)
			// Standard Error: 25_000
			.saturating_add((23_793_000 as Weight).saturating_mul(v as Weight))
			.saturating_add(RocksDbWeight::get().reads(3 as Weight))
			.saturating_add(RocksDbWeight::get().reads((1 as Weight).saturating_mul(v as Weight)))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes((1 as Weight).saturating_mul(v as Weight)))
	}
}
