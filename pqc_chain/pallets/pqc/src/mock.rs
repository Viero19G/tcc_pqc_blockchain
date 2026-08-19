use crate as pallet_pqc;

use frame_support::{derive_impl, traits::ConstU64};
use sp_runtime::{
	traits::IdentityLookup,
	BuildStorage,
};

type Block = frame_system::mocking::MockBlock<Test>;

frame_support::construct_runtime!(
	pub enum Test {
		System: frame_system,
		Pqc: pallet_pqc,
	}
);

#[derive_impl(frame_system::config_preludes::TestDefaultConfig)]
impl frame_system::Config for Test {
	type Block = Block;
	type AccountId = u64;
	type Lookup = IdentityLookup<u64>;
	type BlockHashCount = ConstU64<250>;
	type AccountData = ();
}

impl pallet_pqc::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = ();
}

pub fn new_test_ext() -> sp_io::TestExternalities {
	frame_system::GenesisConfig::<Test>::default().build_storage().unwrap().into()
}
