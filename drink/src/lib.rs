pub mod chain_api;
pub mod contract_api;
mod runtime;

use std::time::SystemTime;

use frame_support::{
    sp_io::TestExternalities, sp_runtime::AccountId32, traits::Hooks, weights::Weight,
};
use frame_system::GenesisConfig;

use crate::runtime::*;

pub struct Sandbox {
    externalities: TestExternalities,
}

pub const ALICE: AccountId32 = AccountId32::new([1u8; 32]);

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct CallResult {
    pub result: Vec<u8>,
    pub debug_message: Vec<String>,

    pub gas_consumed: Weight,
    pub gas_required: Weight,
}

impl Default for Sandbox {
    fn default() -> Self {
        Self::new()
    }
}

impl Sandbox {
    pub fn new() -> Self {
        let mut storage = GenesisConfig::default()
            .build_storage::<SandboxRuntime>()
            .unwrap();
        pallet_balances::GenesisConfig::<SandboxRuntime> {
            balances: vec![(ALICE, 1_000_000_000_000_000)],
        }
        .assimilate_storage(&mut storage)
        .unwrap();

        let mut slf = Self {
            externalities: TestExternalities::new(storage),
        };
        slf.init_block(0);
        slf
    }

    fn init_block(&mut self, height: u64) {
        self.externalities.execute_with(|| {
            System::reset_events();

            Balances::on_initialize(height);
            Timestamp::set_timestamp(
                SystemTime::now()
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
            );
            Timestamp::on_initialize(height);
            Contracts::on_initialize(height);
        });
    }
}
