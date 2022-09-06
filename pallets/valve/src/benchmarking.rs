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

#![cfg(feature = "runtime-benchmarks")]

use super::*;
use frame_benchmarking::{account, benchmarks};
use frame_system::RawOrigin;

fn assert_last_event<T: Config>(generic_event: <T as Config>::Event) {
	frame_system::Pallet::<T>::assert_last_event(generic_event.into());
}

benchmarks! {
	close_valve {
	}: _(RawOrigin::Root)
	verify {
		assert_last_event::<T>(Event::ValveClosed.into())
	}

	open_valve {
		ValveClosed::<T>::put(true);
	}: _(RawOrigin::Root)
	verify {
		assert_last_event::<T>(Event::ValveOpen.into())
	}

	close_pallet_gate_new {
		let pallet_name = b"System".to_vec();
	}: close_pallet_gate(RawOrigin::Root, pallet_name.clone())
	verify {
		assert_last_event::<T>(Event::PalletGateClosed{ pallet_name_bytes: pallet_name }.into())
	}

	close_pallet_gate_existing {
		let pallet_name = b"System".to_vec();
		ClosedPallets::<T>::insert(pallet_name.clone(), ());
	}: close_pallet_gate(RawOrigin::Root, pallet_name.to_vec())

	open_pallet_gate {
		let pallet_name = b"System".to_vec();
		ClosedPallets::<T>::insert(pallet_name.clone(), ());
	}: _(RawOrigin::Root, pallet_name.clone())
	verify {
		assert_last_event::<T>(Event::PalletGateOpen{ pallet_name_bytes: pallet_name }.into())
	}

	open_pallet_gates {
		ClosedPallets::<T>::insert(b"System".to_vec(), ());
		ClosedPallets::<T>::insert(b"Balances".to_vec(), ());
		ClosedPallets::<T>::insert(b"Bounties".to_vec(), ());
		ClosedPallets::<T>::insert(b"CollatorSelection".to_vec(), ());
		ClosedPallets::<T>::insert(b"Treasury".to_vec(), ());
		ClosedPalletCount::<T>::put(5);
	}: _(RawOrigin::Root)
	verify {
		assert_last_event::<T>(Event::PalletGatesClosed{ count: 0 }.into())
	}

	stop_scheduled_tasks {

	}: _(RawOrigin::Root)
	verify {
		assert_last_event::<T>(Event::ScheduledTasksStopped.into())
	}

	start_scheduled_tasks {
		T::AutomationTime::shutdown();
	}: _(RawOrigin::Root)
	verify {
		assert_last_event::<T>(Event::ScheduledTasksResumed.into())
	}
}
