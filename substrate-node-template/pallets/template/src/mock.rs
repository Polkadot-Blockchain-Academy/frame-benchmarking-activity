use crate as pallet_template;
use frame_support::{
	derive_impl,
	traits::{ConstU128, ConstU32, ConstU64},
};
use sp_runtime::BuildStorage;

type Balance = u128;
type Block = frame_system::mocking::MockBlock<Runtime>;

frame_support::construct_runtime!(
	pub struct Runtime {
		System: frame_system,
		Balances: pallet_balances,
		TemplateModule: pallet_template,
	}
);

#[derive_impl(frame_system::config_preludes::TestDefaultConfig as frame_system::DefaultConfig)]
impl frame_system::Config for Runtime {
	type RuntimeOrigin = RuntimeOrigin;
	type RuntimeCall = RuntimeCall;
	type RuntimeEvent = RuntimeEvent;
	type PalletInfo = PalletInfo;
	type Block = Block;
	type BlockHashCount = ConstU64<250>;
	type BaseCallFilter = frame_support::traits::Everything;
	type OnSetCode = ();

	type AccountData = pallet_balances::AccountData<Balance>;
}

impl pallet_balances::Config for Runtime {
	type Balance = Balance;
	type DustRemoval = ();
	type RuntimeEvent = RuntimeEvent;
	type ExistentialDeposit = ConstU128<1>;
	type AccountStore = System;
	type WeightInfo = ();
	type MaxLocks = ConstU32<10>;
	type MaxReserves = ();
	type ReserveIdentifier = [u8; 8];
	type RuntimeHoldReason = ();
	type FreezeIdentifier = ();
	type MaxHolds = ConstU32<10>;
	type MaxFreezes = ConstU32<10>;
}

impl pallet_template::Config for Runtime {
	type Currency = Balances;
	type MaxVoters = ConstU32<100>;
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = ();
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
	frame_system::GenesisConfig::<Runtime>::default()
		.build_storage()
		.unwrap()
		.into()
}
