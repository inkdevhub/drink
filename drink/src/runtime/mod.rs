//! Module containing the [`Runtime`](Runtime) trait and its example implementations. You can use
//! `drink` with any runtime that implements the `Runtime` trait.

pub mod minimal;
pub mod pallet_contracts_debugging;

pub use frame_metadata::RuntimeMetadataPrefixed;
use frame_support::{
    sp_runtime::{traits::Dispatchable, Storage},
    traits::{fungible::Inspect, Time},
};
use frame_system::pallet_prelude::BlockNumberFor;
pub use minimal::MinimalRuntime;

/// The type of an account identifier.
pub type AccountIdFor<R> = <R as frame_system::Config>::AccountId;

/// Generic fungible balance type.
pub type BalanceOf<R> =
    <<R as pallet_contracts::Config>::Currency as Inspect<AccountIdFor<R>>>::Balance;

/// Generic Time type.
pub type MomentOf<R> = <<R as pallet_contracts::Config>::Time as Time>::Moment;

/// The type of a hash.
pub type HashFor<R> = <R as frame_system::Config>::Hash;

/// Export pallets that are used in the runtime.
pub use frame_system;
pub use pallet_balances;
pub use pallet_contracts;

/// A runtime to use.
///
/// Must contain at least system, balances and contracts pallets.
pub trait Runtime:
    pallet_contracts::Config + pallet_timestamp::Config<Moment = MomentOf<Self>>
{
    /// Initialize the storage at the genesis block.
    fn initialize_storage(_storage: &mut Storage) -> Result<(), String> {
        Ok(())
    }

    /// Initialize a new block at particular height.
    fn initialize_block(
        _height: BlockNumberFor<Self>,
        _parent_hash: <Self as frame_system::Config>::Hash,
    ) -> Result<(), String> {
        Ok(())
    }

    /// Finalize a block at particular height.
    fn finalize_block(
        _height: BlockNumberFor<Self>,
    ) -> Result<<Self as frame_system::Config>::Hash, String> {
        Ok(Default::default())
    }

    /// Default actor for the runtime.
    fn default_actor() -> AccountIdFor<Self>;

    /// Metadata of the runtime.
    fn get_metadata() -> RuntimeMetadataPrefixed;

    /// Convert an account to an call origin.
    fn convert_account_to_origin(
        account: AccountIdFor<Self>,
    ) -> <<Self as frame_system::Config>::RuntimeCall as Dispatchable>::RuntimeOrigin;
}
