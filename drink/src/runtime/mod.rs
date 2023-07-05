mod minimal;

use frame_support::sp_runtime::Storage;
pub use minimal::MinimalRuntime;
use minimal::*;

use super::DEFAULT_ACTOR;

/// A runtime to use.
///
/// Must contain at least system, balances and contracts pallets.
pub trait Runtime:
    frame_system::Config + pallet_balances::Config + pallet_contracts::Config
{
    /// Initialize the storage at the genesis block.
    fn initialize_storage(storage: &mut Storage) -> Result<(), String> {
        Ok(())
    }

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
        .assimilate_storage(&mut storage)
    }

    fn initialize_block(_: u64) -> Result<(), String> {
        todo!()
    }
}
