mod runtime;

use frame_support::{assert_ok, weights::Weight};
use sp_io::TestExternalities;
use sp_runtime::AccountId32;

use crate::runtime::*;

pub struct Sandbox {
    externalities: TestExternalities,
}

pub const ALICE: AccountId32 = AccountId32::new([1u8; 32]);
pub const GAS_LIMIT: Weight = Weight::from_parts(100_000_000_000, 3 * 1024 * 1024);

impl Sandbox {
    pub fn new() -> Self {
        let mut storage = frame_system::GenesisConfig::default()
            .build_storage::<SandboxRuntime>()
            .unwrap();
        pallet_balances::GenesisConfig::<SandboxRuntime> {
            balances: vec![(ALICE, 1_000_000_000_000_000)],
        }
        .assimilate_storage(&mut storage)
        .unwrap();

        Self {
            externalities: TestExternalities::new(storage),
        }
    }

    pub fn deploy_contract(&mut self, contract_bytes: Vec<u8>) {
        self.externalities.execute_with(|| {
            assert_ok!(Contracts::instantiate_with_code(
                RuntimeOrigin::signed(ALICE),
                0,
                GAS_LIMIT,
                None,
                contract_bytes,
                vec![155, 174, 157, 94],
                Default::default(),
            ));
        });
    }
}
