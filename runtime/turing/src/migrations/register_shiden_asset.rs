use crate::*;
use frame_support::{pallet_prelude::Weight, traits::OnRuntimeUpgrade};
use orml_asset_registry::AssetMetadata;
use primitives::assets::{ConversionRate, CustomMetadata};

pub type AssetMetadataOf = AssetMetadata<Balance, CustomMetadata>;

pub fn dollar(decimals: u32) -> Balance {
	10_u128.pow(decimals)
}

pub fn cent(decimals: u32) -> Balance {
	dollar(decimals) / 100
}

#[frame_support::storage_alias]
type LastAssetId<T: Config> =
	StorageValue<AssetRegistry, <T as orml_asset_registry::Config>::AssetId>;

pub struct AddShidenAsset;
impl OnRuntimeUpgrade for AddShidenAsset {
	fn on_runtime_upgrade() -> Weight {
		log::info!(
			target: "asset_registry",
			"on_runtime_upgrade: add shiden asset"
		);

		let asset = AssetMetadataOf {
			decimals: 18,
			name: b"Shiden".to_vec(),
			symbol: b"SDN".to_vec(),
			additional: CustomMetadata {
				fee_per_second: Some(416_000_000_000),
				conversion_rate: Some(ConversionRate { native: 1, foreign: 1 }),
			},
			existential_deposit: cent(18),
			location: Some(MultiLocation::new(1, X1(Parachain(2007))).into()),
		};

		let _ = orml_asset_registry::Pallet::<Runtime>::do_register_asset(asset.clone(), None)
			.map_err(|e| {
				log::error!(
						target: "asset_registry",
						"failed to register shiden native token with err: {:?}", e
				)
			});

		log::info!(
			target: "asset_registry",
			"on_runtime_upgrade: complete"
		);

		// Storage: AssetRegistry Metadata (r:1 w:1)
		// Storage: AssetRegistry LocationToAssetId (r:1 w:1)
		// Storage: AssetRegistry LastAssetId (r:1 w:1)
		<Runtime as frame_system::Config>::DbWeight::get().reads_writes(3u64, 3u64)
	}
}
