//! Module containing the [`Runtime`](Runtime) trait and its example implementations. You can use
//! `drink` with any runtime that implements the `Runtime` trait.

pub mod minimal;

use frame_support::sp_runtime::{AccountId32, Storage};
pub use minimal::MinimalRuntime;

/// A runtime to use.
///
/// Must contain at least system, balances and contracts pallets.
pub trait Runtime:
    frame_system::Config<
        AccountId = AccountId32,
        Block = frame_system::mocking::MockBlock<MinimalRuntime>,
    > + pallet_balances::Config<Balance = u128>
    + pallet_contracts::Config<Currency = pallet_balances::Pallet<Self>>
{
    /// Initialize the storage at the genesis block.
    fn initialize_storage(_storage: &mut Storage) -> Result<(), String> {
        Ok(())
    }

    /// Initialize a new block at particular height.
    fn initialize_block(
        _height: u64,
        _parent_hash: <Self as frame_system::Config>::Hash,
    ) -> Result<(), String> {
        Ok(())
    }

    /// Finalize a block at particular height.
    fn finalize_block(_height: u64) -> Result<<Self as frame_system::Config>::Hash, String> {
        Ok(Default::default())
    }
}
