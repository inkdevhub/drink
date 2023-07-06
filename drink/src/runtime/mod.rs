//! Module containing the [`Runtime`](Runtime) trait and its example implementations. You can use
//! `drink` with any runtime that implements the `Runtime` trait.

mod minimal;

use frame_support::sp_runtime::{AccountId32, Storage};
pub use minimal::MinimalRuntime;

use super::DEFAULT_ACTOR;

/// A runtime to use.
///
/// Must contain at least system, balances and contracts pallets.
pub trait Runtime:
    frame_system::Config<AccountId = AccountId32, BlockNumber = u64>
    + pallet_balances::Config<Balance = u128>
    + pallet_contracts::Config<Currency = pallet_balances::Pallet<Self>>
{
    /// Initialize the storage at the genesis block.
    fn initialize_storage(_storage: &mut Storage) -> Result<(), String> {
        Ok(())
    }

    /// Initialize a new block at particular height.
    fn initialize_block(_height: u64) -> Result<(), String> {
        Ok(())
    }
}

/// Default initial balance for the default account.
pub const INITIAL_BALANCE: u128 = 1_000_000_000_000_000;

impl Runtime for MinimalRuntime {
    fn initialize_storage(storage: &mut Storage) -> Result<(), String> {
        pallet_balances::GenesisConfig::<Self> {
            balances: vec![(DEFAULT_ACTOR, INITIAL_BALANCE)],
        }
        .assimilate_storage(storage)
    }

    fn initialize_block(_: u64) -> Result<(), String> {
        todo!()
    }
}
