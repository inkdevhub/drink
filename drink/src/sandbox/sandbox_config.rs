//! Module containing the core trait use to customize the sandboxed runtime.

use frame_metadata::RuntimeMetadataPrefixed;
use frame_support::sp_runtime::{traits::Dispatchable, Storage};
use frame_system::pallet_prelude::BlockNumberFor;

/// The type of an account identifier.
pub type AccountIdFor<R> = <R as frame_system::Config>::AccountId;

/// A runtime to use.
pub trait SandboxConfig {
    /// The runtime associated with the sandbox.
    type Runtime: frame_system::Config;

    /// Initialize the storage at the genesis block.
    fn initialize_storage(_storage: &mut Storage) -> Result<(), String> {
        Ok(())
    }

    /// Initialize a new block at particular height.
    fn initialize_block(
        _height: BlockNumberFor<Self::Runtime>,
        _parent_hash: <Self::Runtime as frame_system::Config>::Hash,
    ) -> Result<(), String> {
        Ok(())
    }

    /// Finalize a block at particular height.
    fn finalize_block(
        _height: BlockNumberFor<Self::Runtime>,
    ) -> Result<<Self::Runtime as frame_system::Config>::Hash, String> {
        Ok(Default::default())
    }

    /// Default actor for the sandbox.
    fn default_actor() -> AccountIdFor<Self::Runtime>;

    /// Metadata of the runtime.
    fn get_metadata() -> RuntimeMetadataPrefixed;

    /// Convert an account to an call origin.
    fn convert_account_to_origin(
        account: AccountIdFor<Self::Runtime>,
    ) -> <<Self::Runtime as frame_system::Config>::RuntimeCall as Dispatchable>::RuntimeOrigin;
}
