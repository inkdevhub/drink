//! Module containing the [`Runtime`](Runtime) trait and its example implementations. You can use
//! `drink` with any runtime that implements the `Runtime` trait.

pub mod minimal;
pub use frame_metadata::RuntimeMetadataPrefixed;
pub use minimal::MinimalRuntime;
pub mod pallet_contracts_debugging;

/// The type of an account identifier.
pub type AccountIdFor<R> = <R as frame_system::Config>::AccountId;

/// The type of a hash.
pub type HashFor<R> = <R as frame_system::Config>::Hash;
