//! The drink crate provides a sandboxed runtime for testing smart contracts without a need for
//! a running node.

#![warn(missing_docs)]

extern crate core;

mod bundle;
pub mod errors;
mod mock;
pub mod runtime;
pub mod sandbox;
pub use sandbox::*;
#[cfg(feature = "session")]
pub mod session;

use std::sync::{Arc, Mutex};

pub use bundle::ContractBundle;
pub use drink_test_macro::{contract_bundle_provider, test};
pub use errors::Error;
pub use frame_support::{
    sp_runtime::{AccountId32, DispatchError},
    weights::Weight,
};
use frame_system::EventRecord;
pub use mock::{mock_message, ContractMock, MessageMock, MockedCallResult, Selector};
use pallet_contracts::debug::ExecResult;
use pallet_contracts_primitives::{ExecReturnValue, ReturnFlags};
use parity_scale_codec::{Decode, Encode};
/// Export pallets that are used in the minimal runtime.
pub use {frame_support, frame_system, pallet_balances, pallet_contracts, pallet_timestamp};

use crate::{
    errors::MessageResult,
    mock::MockRegistry,
    runtime::{pallet_contracts_debugging::InterceptingExtT, *},
};

/// Alias for `frame-system`'s `RuntimeCall` type.
pub type RuntimeCall<R> = <R as frame_system::Config>::RuntimeCall;

/// Alias for `pallet-balances`'s Balance type.
pub type BalanceOf<R> = <R as pallet_balances::Config>::Balance;

/// Main result type for the drink crate.
pub type DrinkResult<T> = std::result::Result<T, Error>;

/// Copied from pallet-contracts.
pub type EventRecordOf<T> =
    EventRecord<<T as frame_system::Config>::RuntimeEvent, <T as frame_system::Config>::Hash>;

/// Default gas limit.
pub const DEFAULT_GAS_LIMIT: Weight = Weight::from_parts(100_000_000_000, 3 * 1024 * 1024);

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
