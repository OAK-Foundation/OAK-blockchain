/// ! Traits and default implementation for paying execution fees.
use crate::Config;

use codec::FullCodec;
use sp_runtime::{
    DispatchError,
	traits::{
		AtLeast32BitUnsigned, CheckedSub, MaybeSerializeDeserialize, Zero,
	},
    TokenError::BelowMinimum,

};
use sp_std::{fmt::Debug, marker::PhantomData};

use frame_support::{
	traits::{Currency, ExistenceRequirement, Imbalance, OnUnbalanced, WithdrawReasons},
};

type NegativeImbalanceOf<C, T> =
	<C as Currency<<T as frame_system::Config>::AccountId>>::NegativeImbalance;

/// Handle withdrawing, refunding and depositing of transaction fees.
pub trait NativeTokenExchange<T: Config> {
	/// The underlying integer type in which fees are calculated.
	type Balance: AtLeast32BitUnsigned
		+ FullCodec
		+ Copy
		+ MaybeSerializeDeserialize
		+ Debug
		+ Default
		+ scale_info::TypeInfo;
	type LiquidityInfo: Default;

    /// The minimum balance any single account may have. This is equivalent to the `Balances`
	/// module's `ExistentialDeposit`.
	fn minimum_balance() -> Self::Balance;

    ///Transfer some liquid free balance to another staker.
    /// This is a very high-level function. It will ensure all appropriate fees are paid and no imbalance in the system remains.
    fn transfer(
        source: &T::AccountId,
        dest: &T::AccountId,
        value: Self::Balance,
    ) -> Result<(), DispatchError> ;

    /// Ensure the fee can be paid.
    fn can_pay_fee(
        who: &T::AccountId,
        fee: Self::Balance,
	) -> Result<(), DispatchError>;

	/// Once the task has been scheduled we need to charge for the execution cost.
	fn withdraw_fee(
		who: &T::AccountId,
		fee: Self::Balance,
	) -> Result<(), DispatchError>;
}

/// Implements the transaction payment for a pallet implementing the `Currency`
/// trait (eg. the pallet_balances) using an unbalance handler (implementing
/// `OnUnbalanced`).
///
/// The unbalance handler is given 2 unbalanceds in [`OnUnbalanced::on_unbalanceds`]: fee and
/// then tip.
pub struct CurrencyAdapter<C, OU>(PhantomData<(C, OU)>);

/// Default implementation for a Currency and an OnUnbalanced handler.
///
/// The unbalance handler is given 2 unbalanceds in [`OnUnbalanced::on_unbalanceds`]: fee and
/// then tip.
impl<T, C, OU> NativeTokenExchange<T> for CurrencyAdapter<C, OU>
where
	T: Config,
	C: Currency<<T as frame_system::Config>::AccountId>,
	C::PositiveImbalance: Imbalance<
		<C as Currency<<T as frame_system::Config>::AccountId>>::Balance,
		Opposite = C::NegativeImbalance,
	>,
	C::NegativeImbalance: Imbalance<
		<C as Currency<<T as frame_system::Config>::AccountId>>::Balance,
		Opposite = C::PositiveImbalance,
	>,
	OU: OnUnbalanced<NegativeImbalanceOf<C, T>>,
{
	type LiquidityInfo = Option<NegativeImbalanceOf<C, T>>;
	type Balance = <C as Currency<<T as frame_system::Config>::AccountId>>::Balance;

    /// The minimum balance any single account may have. This is equivalent to the `Balances`
	/// module's `ExistentialDeposit`. 
    fn minimum_balance() -> Self::Balance {
        C::minimum_balance()
    }

    ///Transfer some liquid free balance to another staker.
    /// This is a very high-level function. It will ensure all appropriate fees are paid and no imbalance in the system remains.
    fn transfer(
        source: &T::AccountId,
        dest: &T::AccountId,
        value: Self::Balance,
    ) -> Result<(), DispatchError> {
        C::transfer(source, dest, value, ExistenceRequirement::KeepAlive)?;
        Ok(())
    }

    // Ensure the fee can be paid.
    fn can_pay_fee(
        who: &T::AccountId,
        fee: Self::Balance,
	) -> Result<(), DispatchError> {
        if fee.is_zero() {
			return Ok(())
		}

        let free_balance = C::free_balance(who);
        let new_amount = free_balance.checked_sub(&fee).ok_or(DispatchError::Token(BelowMinimum))?;
        C::ensure_can_withdraw(who, fee, WithdrawReasons::FEE, new_amount)?;

        Ok(())
    }


	/// Withdraw the fee.
	fn withdraw_fee(
		who: &T::AccountId,
		fee: Self::Balance,
	) -> Result<(), DispatchError> {
		if fee.is_zero() {
			return Ok(())
		}

		let withdraw_reason = WithdrawReasons::FEE;

		match C::withdraw(who, fee, withdraw_reason, ExistenceRequirement::KeepAlive) {
			Ok(imbalance) => {
                OU::on_unbalanceds(Some(imbalance).into_iter());
                Ok(())
            },
			Err(_) => Err(DispatchError::Token(BelowMinimum)),
		}
	}
}
