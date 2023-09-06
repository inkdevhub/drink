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

use crate::{
    pallet_contracts_debugging::DebugExt,
    runtime::{pallet_contracts_debugging::NoopDebugExt, *},
};

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

/// Default gas limit.
pub const DEFAULT_GAS_LIMIT: Weight = Weight::from_parts(100_000_000_000, 3 * 1024 * 1024);

impl<R: Runtime> Sandbox<R> {
    /// Creates a new sandbox.
    ///
    /// Returns an error if the storage could not be initialized.
    ///
    /// The storage is initialized with a genesis block with a single account `R::default_actor()` with
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
            .execute_with(|| R::initialize_block(1u32.into(), Default::default()))
            .map_err(Error::BlockInitialize)?;

        // We register a noop debug extension by default.
        sandbox.override_debug_handle(DebugExt(Box::new(NoopDebugExt {})));

        Ok(sandbox)
    }

    /// Overrides the debug extension.
    ///
    /// By default, a new `Sandbox` instance is created with a noop debug extension. This method
    /// allows to override it with a custom debug extension.
    pub fn override_debug_handle(&mut self, d: DebugExt) {
        self.externalities.register_extension(d);
    }
}
