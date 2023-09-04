//! The drink crate provides a sandboxed runtime for testing smart contracts without a need for
//! a running node.

#![warn(missing_docs)]

pub mod chain_api;
pub mod contract_api;
mod error;
pub mod runtime;
#[cfg(feature = "session")]
pub mod session;
use std::marker::PhantomData;

pub use error::Error;
use frame_support::{sp_io::TestExternalities, sp_runtime::BuildStorage};
pub use frame_support::{sp_runtime::AccountId32, weights::Weight};
use frame_system::{EventRecord, GenesisConfig};

use crate::pallet_contracts_debugging::DebugExt;
use crate::runtime::pallet_contracts_debugging::NoopDebugExt;
use crate::runtime::*;

/// Main result type for the drink crate.
pub type DrinkResult<T> = std::result::Result<T, Error>;

/// Copied from pallet-contracts.
pub type EventRecordOf<T> =
    EventRecord<<T as frame_system::Config>::RuntimeEvent, <T as frame_system::Config>::Hash>;

/// A sandboxed runtime.
pub struct Sandbox<R: Runtime> {
    externalities: TestExternalities,
    _phantom: PhantomData<R>,
}

/// Default extrinsic origin.
pub const DEFAULT_ACTOR: AccountId32 = AccountId32::new([1u8; 32]);
/// Default gas limit.
pub const DEFAULT_GAS_LIMIT: Weight = Weight::from_parts(100_000_000_000, 3 * 1024 * 1024);

impl<R: Runtime> Sandbox<R> {
    /// Creates a new sandbox.
    ///
    /// Returns an error if the storage could not be initialized.
    ///
    /// The storage is initialized with a genesis block with a single account `DEFAULT_ACTOR` with
    /// `INITIAL_BALANCE`.
    pub fn new() -> DrinkResult<Self> {
        let mut storage = GenesisConfig::<R>::default()
            .build_storage()
            .map_err(Error::StorageBuilding)?;

        R::initialize_storage(&mut storage).map_err(Error::StorageBuilding)?;

        let mut sandbox = Self {
            externalities: TestExternalities::new(storage),
            _phantom: PhantomData,
        };

        sandbox
            .externalities
            // We start the chain from the 1st block, so that events are collected (they are not
            // recorded for the genesis block...).
            .execute_with(|| R::initialize_block(1, Default::default()))
            .map_err(Error::BlockInitialize)?;

        sandbox.override_debug_handle(DebugExt(Box::new(NoopDebugExt {})));

        Ok(sandbox)
    }

    pub fn override_debug_handle(&mut self, d: DebugExt) {
        self.externalities.register_extension(d);
    }
}
