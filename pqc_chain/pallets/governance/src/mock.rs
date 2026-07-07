use crate as pallet_governance;

use frame_support::{derive_impl, parameter_types, traits::ConstU32};
use sp_runtime::{traits::IdentityLookup, Permill};

type Block = frame_system::mocking::MockBlock<Test>;

frame_support::construct_runtime!(
	pub enum Test {
		System: frame_system,
		Balances: pallet_balances,
		Governance: pallet_governance,
	}
);

#[derive_impl(frame_system::config_preludes::TestDefaultConfig)]
impl frame_system::Config for Test {
	type Block = Block;
	type AccountId = u64;
	type Lookup = IdentityLookup<u64>;
	type BlockHashCount = ConstU32<250>;
	type AccountData = pallet_balances::AccountData<u128>;
}

parameter_types! {
	pub const ExistentialDeposit: u128 = 1;
}

#[derive_impl(pallet_balances::config_preludes::TestDefaultConfig)]
impl pallet_balances::Config for Test {
	type AccountStore = System;
	type ExistentialDeposit = ExistentialDeposit;
}

parameter_types! {
	pub const MinProposalDeposit: u128 = 100;
	pub const VotingPeriod: u64 = 10;
	pub const MinTurnout: Permill = Permill::from_percent(10);
	pub const MinApproval: Permill = Permill::from_percent(51);
}

impl pallet_governance::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type RuntimeCall = RuntimeCall;
	type Currency = Balances;
	type WeightInfo = ();
	type MinProposalDeposit = MinProposalDeposit;
	type VotingPeriod = VotingPeriod;
	type MinTurnout = MinTurnout;
	type MinApproval = MinApproval;
}

pub fn new_test_ext() -> sp_io::TestExternalities {
	let mut t = frame_system::GenesisConfig::<Test>::default().build_storage().unwrap();
	pallet_balances::GenesisConfig::<Test> {
		balances: vec![(1, 1_000), (2, 500), (3, 500)],
		..Default::default()
	}
	.assimilate_storage(&mut t)
	.unwrap();
	t.into()
}
