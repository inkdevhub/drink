//! The drink crate provides a sandboxed runtime for testing smart contracts without a need for
//! a running node.

#![warn(missing_docs)]

pub mod chain_api;
pub mod contract_api;
pub mod errors;
mod mock;
pub mod runtime;
#[cfg(feature = "session")]
pub mod session;

use std::{
    marker::PhantomData,
    sync::{Arc, Mutex},
};

pub use errors::Error;
use frame_support::sp_runtime::{traits::One, BuildStorage};
pub use frame_support::{
    sp_runtime::{AccountId32, DispatchError},
    weights::Weight,
};
use frame_system::{pallet_prelude::BlockNumberFor, EventRecord, GenesisConfig};
pub use mock::{mock_message, ContractMock, MessageMock, MockedCallResult, MockingApi, Selector};
use pallet_contracts::debug::ExecResult;
use pallet_contracts_primitives::{ExecReturnValue, ReturnFlags};
use parity_scale_codec::{Decode, Encode};
use sp_io::TestExternalities;

use crate::{
    errors::MessageResult,
    mock::MockRegistry,
    pallet_contracts_debugging::{InterceptingExt, TracingExt},
    runtime::{
        pallet_contracts_debugging::{InterceptingExtT, NoopExt},
        *,
    },
};

/// Main result type for the drink crate.
pub type DrinkResult<T> = std::result::Result<T, Error>;

/// Copied from pallet-contracts.
pub type EventRecordOf<T> =
    EventRecord<<T as frame_system::Config>::RuntimeEvent, <T as frame_system::Config>::Hash>;

/// A sandboxed runtime.
pub struct Sandbox<R: Runtime> {
    externalities: TestExternalities,
    mock_registry: Arc<Mutex<MockRegistry<AccountIdFor<R>>>>,
    mock_counter: usize,
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
            mock_registry: Arc::new(Mutex::new(MockRegistry::new())),
            mock_counter: 0,
            _phantom: PhantomData,
        };

        sandbox
            .externalities
            // We start the chain from the 1st block, so that events are collected (they are not
            // recorded for the genesis block...).
            .execute_with(|| R::initialize_block(BlockNumberFor::<R>::one(), Default::default()))
            .map_err(Error::BlockInitialize)?;

        // We register a noop debug extension by default.
        sandbox.override_debug_handle(TracingExt(Box::new(NoopExt {})));

        sandbox.setup_mock_extension();

        Ok(sandbox)
    }

    /// Overrides the debug extension.
    ///
    /// By default, a new `Sandbox` instance is created with a noop debug extension. This method
    /// allows to override it with a custom debug extension.
    pub fn override_debug_handle(&mut self, d: TracingExt) {
        self.externalities.register_extension(d);
    }

    /// Registers the extension for intercepting calls to contracts.
    fn setup_mock_extension(&mut self) {
        self.externalities
            .register_extension(InterceptingExt(Box::new(MockingExtension {
                mock_registry: Arc::clone(&self.mock_registry),
            })));
    }
}

/// Runtime extension enabling contract call interception.
struct MockingExtension<AccountId: Ord> {
    /// Mock registry, shared with the sandbox.
    ///
    /// Potentially the runtime is executed in parallel and thus we need to wrap the registry in
    /// `Arc<Mutex>` instead of `Rc<RefCell>`.
    mock_registry: Arc<Mutex<MockRegistry<AccountId>>>,
}

impl<AccountId: Ord + Decode> InterceptingExtT for MockingExtension<AccountId> {
    fn intercept_call(
        &self,
        contract_address: Vec<u8>,
        _is_call: bool,
        input_data: Vec<u8>,
    ) -> Vec<u8> {
        let contract_address = Decode::decode(&mut &contract_address[..])
            .expect("Contract address should be decodable");

        match self
            .mock_registry
            .lock()
            .expect("Should be able to acquire registry")
            .get(&contract_address)
        {
            // There is no mock registered for this address, so we return `None` to indicate that
            // the call should be executed normally.
            None => None::<()>.encode(),
            // We intercept the call and return the result of the mock.
            Some(mock) => {
                let (selector, call_data) = input_data.split_at(4);
                let selector: Selector = selector
                    .try_into()
                    .expect("Input data should contain at least selector bytes");

                let result = mock
                    .call(selector, call_data.to_vec())
                    .expect("TODO: let the user define the fallback mechanism");

                // Although we don't know the exact type, thanks to the SCALE encoding we know
                // that `()` will always succeed (we only care about the `Ok`/`Err` distinction).
                let decoded_result: MessageResult<()> =
                    Decode::decode(&mut &result[..]).expect("Mock result should be decodable");

                let flags = match decoded_result {
                    Ok(_) => ReturnFlags::empty(),
                    Err(_) => ReturnFlags::REVERT,
                };

                let result: ExecResult = Ok(ExecReturnValue {
                    flags,
                    data: result,
                });

                Some(result).encode()
            }
        }
    }
}
