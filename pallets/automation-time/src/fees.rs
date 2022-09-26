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

/// ! Traits and default implementation for paying execution fees.
use crate::{AccountOf, Action, ActionOf, BalanceOf, Config, Error, Pallet};

use frame_support::traits::{Currency, ExistenceRequirement, OnUnbalanced, WithdrawReasons};
use pallet_xcmp_handler::XcmpTransactor;
use sp_runtime::{
	traits::{CheckedSub, Saturating, Zero},
	DispatchError, DispatchResult, SaturatedConversion,
	TokenError::BelowMinimum,
};
use sp_std::marker::PhantomData;

type NegativeImbalanceOf<T> = <<T as Config>::Currency as Currency<
	<T as frame_system::Config>::AccountId,
>>::NegativeImbalance;

/// Handle execution fee payments in the context of automation actions
pub trait HandleFees<T: Config>
where
	Self: Sized,
{
	/// Build a FeeHandler instance to validate and pay fees for a specific user action
	fn build(
		owner: &T::AccountId,
		action: &ActionOf<T>,
		executions: u32,
	) -> Result<Self, DispatchError>;
	/// Execute the fee handler and withdraw fees
	fn pay_fees(self) -> DispatchResult;
}

pub struct FeeHandler<T: Config, OU> {
	owner: T::AccountId,
	execution_fee: BalanceOf<T>,
	xcmp_fee: Option<BalanceOf<T>>,
	_phantom_data: PhantomData<OU>,
}

impl<T, OU> HandleFees<T> for FeeHandler<T, OU>
where
	T: Config,
	OU: OnUnbalanced<NegativeImbalanceOf<T>>,
{
	/// Builds a validated instance of the struct
	fn build(
		owner: &AccountOf<T>,
		action: &ActionOf<T>,
		executions: u32,
	) -> Result<Self, DispatchError> {
		let execution_fee = Pallet::<T>::calculate_execution_fee(action, executions)?;

		let xcmp_fee = match *action {
			Action::XCMP { para_id, currency_id, encoded_call_weight, .. } => Some(
				T::XcmpTransactor::get_xcm_fee(
					u32::from(para_id),
					currency_id,
					encoded_call_weight.clone(),
				)?
				.saturating_mul(executions.into())
				.saturated_into(),
			),
			_ => None,
		};

		// Note: will need to account for fees in non-native tokens once we start accepting them
		Self::can_pay_fee(owner, execution_fee.saturating_add(xcmp_fee.unwrap_or(0u32.into())))
			.map_err(|_| Error::<T>::InsufficientBalance)?;

		Ok(Self {
			owner: owner.clone(),
			execution_fee,
			xcmp_fee,
			_phantom_data: Default::default(),
		})
	}

	/// Executes the fee handler. Should never fail when running on a validated instance.
	fn pay_fees(self) -> DispatchResult {
		// This should never error if can_pay_fee passed.
		Self::withdraw_fee(&self.owner, self.execution_fee)
			.map_err(|_| Error::<T>::LiquidityRestrictions)?;

		if let Some(xcmp_fee) = self.xcmp_fee {
			T::XcmpTransactor::pay_xcm_fee(self.owner, xcmp_fee.saturated_into())?;
		}

		Ok(())
	}
}

impl<T, OU> FeeHandler<T, OU>
where
	T: Config,
	OU: OnUnbalanced<NegativeImbalanceOf<T>>,
{
	// Ensure the fee can be paid.
	fn can_pay_fee(who: &T::AccountId, fee: BalanceOf<T>) -> Result<(), DispatchError> {
		if fee.is_zero() {
			return Ok(())
		}

		let free_balance = T::Currency::free_balance(who);
		let new_amount =
			free_balance.checked_sub(&fee).ok_or(DispatchError::Token(BelowMinimum))?;
		T::Currency::ensure_can_withdraw(who, fee, WithdrawReasons::FEE, new_amount)?;

		Ok(())
	}

	/// Withdraw the fee.
	fn withdraw_fee(who: &T::AccountId, fee: BalanceOf<T>) -> Result<(), DispatchError> {
		if fee.is_zero() {
			return Ok(())
		}

		let withdraw_reason = WithdrawReasons::FEE;

		match T::Currency::withdraw(who, fee, withdraw_reason, ExistenceRequirement::KeepAlive) {
			Ok(imbalance) => {
				OU::on_unbalanceds(Some(imbalance).into_iter());
				Ok(())
			},
			Err(_) => Err(DispatchError::Token(BelowMinimum)),
		}
	}
}
