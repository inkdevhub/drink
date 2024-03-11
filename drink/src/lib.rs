//! The drink crate provides a sandboxed runtime for testing smart contracts without a need for
//! a running node.

#![warn(missing_docs)]

pub mod errors;
pub mod runtime;
pub mod sandbox;
pub use sandbox::*;
#[cfg(feature = "session")]
pub mod session;

#[cfg(feature = "macros")]
pub use drink_test_macro::{contract_bundle_provider, test};
pub use errors::Error;
use frame_support::traits::fungible::Inspect;
pub use frame_support::{
    sp_runtime::{AccountId32, DispatchError},
    weights::Weight,
};
use frame_system::EventRecord;
use pallet_contracts::{ContractExecResult, ContractInstantiateResult};
#[cfg(feature = "session")]
pub use session::mock::{mock_message, ContractMock, MessageMock, MockedCallResult, Selector};
/// Export pallets that are used in the minimal runtime.
pub use {
    frame_support, frame_system, pallet_balances, pallet_contracts, pallet_timestamp, paste,
    sp_externalities::Extension, sp_io::TestExternalities,
};

pub use crate::runtime::minimal::{self, MinimalSandbox};

/// Alias for `frame-system`'s `RuntimeCall` type.
pub type RuntimeCall<R> = <R as frame_system::Config>::RuntimeCall;

/// Main result type for the drink crate.
pub type DrinkResult<T> = std::result::Result<T, Error>;

/// Copied from pallet-contracts.
pub type EventRecordOf<Runtime> = EventRecord<
    <Runtime as frame_system::Config>::RuntimeEvent,
    <Runtime as frame_system::Config>::Hash,
>;

type BalanceOf<R> =
    <<R as pallet_contracts::Config>::Currency as Inspect<AccountIdFor<R>>>::Balance;

/// Copied from pallet-contracts.
pub type ContractInstantiateResultFor<Runtime> =
    ContractInstantiateResult<AccountIdFor<Runtime>, BalanceOf<Runtime>, EventRecordOf<Runtime>>;

/// Copied from pallet-contracts.
pub type ContractExecResultFor<Runtime> =
    ContractExecResult<BalanceOf<Runtime>, EventRecordOf<Runtime>>;

/// Default gas limit.
pub const DEFAULT_GAS_LIMIT: Weight = Weight::from_parts(100_000_000_000, 3 * 1024 * 1024);
