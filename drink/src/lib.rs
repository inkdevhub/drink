pub mod chain_api;
pub mod contract_api;
mod error;
mod runtime;

use std::time::SystemTime;

pub use error::Error;
use frame_support::{sp_io::TestExternalities, sp_runtime::AccountId32, traits::Hooks};
use frame_system::GenesisConfig;

use crate::runtime::*;

pub type Result<T> = std::result::Result<T, Error>;

pub struct Sandbox {
    externalities: TestExternalities,
}

pub const ALICE: AccountId32 = AccountId32::new([1u8; 32]);

impl Sandbox {
    pub fn new() -> Result<Self> {
        let mut storage = GenesisConfig::default()
            .build_storage::<SandboxRuntime>()
            .map_err(Error::StorageBuilding)?;
        pallet_balances::GenesisConfig::<SandboxRuntime> {
            balances: vec![(ALICE, 1_000_000_000_000_000)],
        }
        .assimilate_storage(&mut storage)
        .map_err(Error::StorageBuilding)?;

        let mut sandbox = Self {
            externalities: TestExternalities::new(storage),
        };
        sandbox.init_block(0);
        Ok(sandbox)
    }

    fn init_block(&mut self, height: u64) {
        self.externalities.execute_with(|| {
            System::reset_events();

            Balances::on_initialize(height);
            Timestamp::set_timestamp(
                SystemTime::now()
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .expect("Time went backwards")
                    .as_secs(),
            );
            Timestamp::on_initialize(height);
            Contracts::on_initialize(height);
        });
    }
}
