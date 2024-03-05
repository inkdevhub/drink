//! The drink crate provides a sandboxed runtime for testing smart contracts without a need for
//! a running node.

#![warn(missing_docs)]

pub mod errors;
mod mock;
pub mod runtime;
pub mod sandbox;
pub use sandbox::*;
#[cfg(feature = "session")]
pub mod session;

#[cfg(feature = "session")]
pub use drink_test_macro::{contract_bundle_provider, test};
pub use errors::Error;
pub use frame_support::{
    sp_runtime::{AccountId32, DispatchError},
    weights::Weight,
};
use frame_system::EventRecord;
pub use mock::{mock_message, ContractMock, MessageMock, MockedCallResult, Selector};
use pallet_contracts::{debug::ExecResult, ExecReturnValue};
use pallet_contracts_uapi::ReturnFlags;
/// Export pallets that are used in the minimal runtime.
pub use {frame_support, frame_system, pallet_balances, pallet_contracts, pallet_timestamp};

pub use crate::runtime::minimal::{self, MinimalRuntime};

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
