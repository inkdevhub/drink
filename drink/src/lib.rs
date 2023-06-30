//! The drink crate provides a sandboxed runtime for testing smart contracts without a need for
//! a running node.

#![warn(missing_docs)]

pub mod chain_api;
pub mod contract_api;
mod error;
mod runtime;
#[cfg(feature = "session")]
pub mod session;

use std::time::SystemTime;

pub use error::Error;
use frame_support::{sp_io::TestExternalities, traits::Hooks};
pub use frame_support::{sp_runtime::AccountId32, weights::Weight};
use frame_system::GenesisConfig;

use crate::runtime::*;

/// Main result type for the drink crate.
pub type DrinkResult<T> = std::result::Result<T, Error>;

/// A sandboxed runtime.
pub struct Sandbox {
    externalities: TestExternalities,
}

/// Default extrinsic origin.
pub const DEFAULT_ACTOR: AccountId32 = AccountId32::new([1u8; 32]);
/// Default initial balance.
pub const INITIAL_BALANCE: u128 = 1_000_000_000_000_000;
/// Default gas limit.
pub const DEFAULT_GAS_LIMIT: Weight = Weight::from_parts(100_000_000_000, 3 * 1024 * 1024);

impl Sandbox {
    /// Creates a new sandbox.
    ///
    /// Returns an error if the storage could not be initialized.
    ///
    /// The storage is initialized with a genesis block with a single account `DEFAULT_ACTOR` with
    /// `INITIAL_BALANCE`.
    pub fn new() -> DrinkResult<Self> {
        let mut storage = GenesisConfig::default()
            .build_storage::<SandboxRuntime>()
            .map_err(Error::StorageBuilding)?;
        pallet_balances::GenesisConfig::<SandboxRuntime> {
            balances: vec![(DEFAULT_ACTOR, INITIAL_BALANCE)],
        }
        .assimilate_storage(&mut storage)
        .map_err(Error::StorageBuilding)?;

        let mut sandbox = Self {
            externalities: TestExternalities::new(storage),
        };
        sandbox.init_block(0);
        Ok(sandbox)
    }

    /// Does the block initialization work.
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
