use core::marker::PhantomData;

use crate::{
	weights::WeightInfo, AccountOf, ActionOf, AssetPayment, BalanceOf, Config, Schedule, TaskId,
	TaskOf, XcmFlow,
};
use codec::{Decode, Encode};
use cumulus_primitives_core::ParaId;
use frame_support::{traits::OnRuntimeUpgrade, weights::Weight, Twox64Concat};
use scale_info::TypeInfo;
use sp_std::vec::Vec;
use xcm::{latest::prelude::*, VersionedMultiLocation};

#[derive(Debug, Encode, Decode, TypeInfo)]
#[scale_info(skip_type_params(MaxExecutionTimes))]
pub struct OldTask<T: Config> {
	pub owner_id: T::AccountId,
	pub provided_id: Vec<u8>,
	pub schedule: Schedule,
	pub action: OldAction<T>,
}

impl<T: Config> From<OldTask<T>> for TaskOf<T> {
	fn from(task: OldTask<T>) -> Self {
		TaskOf::<T> {
			owner_id: task.owner_id,
			provided_id: task.provided_id,
			schedule: task.schedule,
			action: task.action.into(),
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
		xcm_asset_location: VersionedMultiLocation,
		encoded_call: Vec<u8>,
		encoded_call_weight: Weight,
		schedule_as: Option<T::AccountId>,
	},
	AutoCompoundDelegatedStake {
		delegator: T::AccountId,
		collator: T::AccountId,
		account_minimum: BalanceOf<T>,
	},
	DynamicDispatch {
		encoded_call: Vec<u8>,
	},
}

impl<T: Config> From<OldAction<T>> for ActionOf<T> {
	fn from(action: OldAction<T>) -> Self {
		match action {
			OldAction::AutoCompoundDelegatedStake { delegator, collator, account_minimum } =>
				Self::AutoCompoundDelegatedStake { delegator, collator, account_minimum },
			OldAction::Notify { message } => Self::Notify { message },
			OldAction::NativeTransfer { sender, recipient, amount } =>
				Self::NativeTransfer { sender, recipient, amount },
			OldAction::XCMP {
				para_id,
				currency_id,
				encoded_call,
				encoded_call_weight,
				schedule_as,
				..
			} => Self::XCMP {
				destination: MultiLocation::new(1, X1(Parachain(para_id.into()))),
				schedule_fee: currency_id,
				execution_fee: AssetPayment {
					asset_location: MultiLocation::new(0, Here).into(),
					amount: 3000000000,
				},
				encoded_call,
				encoded_call_weight: encoded_call_weight.clone(),
				overall_weight: encoded_call_weight
					.saturating_add(Weight::from_ref_time(1_000_000_000).saturating_mul(6)),
				schedule_as,
				flow: XcmFlow::Normal,
			},
			OldAction::DynamicDispatch { encoded_call } => Self::DynamicDispatch { encoded_call },
		}
	}
}

#[frame_support::storage_alias]
pub type AccountTasks<T: Config> = StorageDoubleMap<
	AutomationTime,
	Twox64Concat,
	AccountOf<T>,
	Twox64Concat,
	TaskId<T>,
	OldTask<T>,
>;

pub struct UpdateXcmpTask<T>(PhantomData<T>);
impl<T: Config> OnRuntimeUpgrade for UpdateXcmpTask<T> {
	fn on_runtime_upgrade() -> Weight {
		log::info!(target: "automation-time", "UpdateXcmpTask migration");

		let mut migrated_tasks = 0u32;
		AccountTasks::<T>::iter().for_each(|(account_id, task_id, task)| {
			let migrated_task: TaskOf<T> = task.into();
			crate::AccountTasks::<T>::insert(account_id, task_id, migrated_task);

			migrated_tasks += 1;
		});

		log::info!(
			target: "automation-time",
			"migration: UpdateXcmpTask succesful! Migrated {} tasks.",
			migrated_tasks
		);

		<T as Config>::WeightInfo::migration_upgrade_xcmp_task(migrated_tasks)
	}

	#[cfg(feature = "try-runtime")]
	fn pre_upgrade() -> Result<Vec<u8>, &'static str> {
		let prev_count = crate::AccountTasks::<T>::iter().count() as u32;
		Ok(prev_count.encode())
	}

	#[cfg(feature = "try-runtime")]
	fn post_upgrade(prev_count: Vec<u8>) -> Result<(), &'static str> {
		let prev_count: u32 = Decode::decode(&mut prev_count.as_slice())
			.expect("the state parameter should be something that was generated by pre_upgrade");
		let post_count = crate::AccountTasks::<T>::iter().count() as u32;
		assert!(post_count == prev_count);

		Ok(())
	}
}

#[cfg(test)]
mod test {
	use super::{OldAction, OldTask, ParaId, UpdateXcmpTask};
	use crate::{mock::*, ActionOf, AssetPayment, Pallet, Schedule, TaskOf, XcmFlow};
	use frame_support::{traits::OnRuntimeUpgrade, weights::Weight};
	use sp_runtime::AccountId32;
	use xcm::latest::prelude::*;

	#[test]
	fn on_runtime_upgrade() {
		new_test_ext(0).execute_with(|| {
			let para_id: ParaId = 1000.into();
			let account_id = AccountId32::new(ALICE);
			let schedule_as = Some(AccountId32::new(BOB));
			let task_id = Pallet::<Test>::generate_task_id(account_id.clone(), vec![1]);
			let encoded_call_weight = Weight::from_ref_time(10);

			let task = OldTask::<Test> {
				owner_id: account_id.clone(),
				provided_id: vec![1],
				schedule: Schedule::Fixed {
					execution_times: vec![0, 1].try_into().unwrap(),
					executions_left: 2,
				},
				action: OldAction::<Test>::XCMP {
					para_id,
					currency_id: 0u32.into(),
					xcm_asset_location: MultiLocation::new(1, X1(Parachain(para_id.into()))).into(),
					encoded_call: vec![0u8],
					encoded_call_weight: encoded_call_weight.clone(),
					schedule_as: schedule_as.clone(),
				},
			};

			super::AccountTasks::<Test>::insert(account_id.clone(), task_id, task);

			UpdateXcmpTask::<Test>::on_runtime_upgrade();

			assert_eq!(crate::AccountTasks::<Test>::iter().count(), 1);
			assert_eq!(
				crate::AccountTasks::<Test>::get(account_id.clone(), task_id).unwrap(),
				TaskOf::<Test> {
					owner_id: account_id.clone(),
					provided_id: vec![1],
					schedule: Schedule::Fixed {
						execution_times: vec![0, 1].try_into().unwrap(),
						executions_left: 2
					},
					action: ActionOf::<Test>::XCMP {
						destination: MultiLocation::new(1, X1(Parachain(para_id.into()))),
						schedule_fee: 0u32.into(),
						execution_fee: AssetPayment {
							asset_location: MultiLocation::new(0, Here).into(),
							amount: 3000000000
						},
						encoded_call: vec![0u8],
						encoded_call_weight: encoded_call_weight.clone(),
						overall_weight: encoded_call_weight
							.saturating_add(Weight::from_ref_time(1_000_000_000).saturating_mul(6)),
						schedule_as,
						flow: XcmFlow::Normal,
					},
				}
			);
		})
	}
}
