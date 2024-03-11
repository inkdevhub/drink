//! Module containing a [`MinimalSandbox`] that implements the [`crate::Sandbox`] trait.
//! Also contains debugging utilities for the contracts pallet.

pub mod minimal;
pub mod pallet_contracts_debugging;
pub use frame_metadata::RuntimeMetadataPrefixed;
pub use minimal::MinimalSandbox;

/// The type of an account identifier.
pub type AccountIdFor<R> = <R as frame_system::Config>::AccountId;

/// The type of a hash.
pub type HashFor<R> = <R as frame_system::Config>::Hash;
