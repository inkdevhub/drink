//! Basic chain API.

use frame_support::{sp_runtime::AccountId32, traits::tokens::currency::Currency};

use crate::{DrinkResult, Error, Runtime, Sandbox};

/// Interface for basic chain operations.
pub trait ChainApi {
    /// Return the current height of the chain.
    fn current_height(&mut self) -> u64;

    /// Build a new empty block and return the new height.
    fn build_block(&mut self) -> DrinkResult<u64>;

    /// Build `n` empty blocks and return the new height.
    ///
    /// # Arguments
    ///
    /// * `n` - The number of blocks to build.
    fn build_blocks(&mut self, n: u64) -> DrinkResult<u64> {
        let mut last_block = None;
        for _ in 0..n {
            last_block = Some(self.build_block()?);
        }
        Ok(last_block.unwrap_or_else(|| self.current_height()))
    }

    /// Add tokens to an account.
    ///
    /// # Arguments
    ///
    /// * `address` - The address of the account to add tokens to.
    /// * `amount` - The number of tokens to add.
    fn add_tokens(&mut self, address: AccountId32, amount: u128);

    /// Return the balance of an account.
    ///
    /// # Arguments
    ///
    /// * `address` - The address of the account to query.
    fn balance(&mut self, address: &AccountId32) -> u128;
}

impl<R: Runtime> ChainApi for Sandbox<R> {
    fn current_height(&mut self) -> u64 {
        self.externalities
            .execute_with(|| frame_system::Pallet::<R>::block_number())
    }

    fn build_block(&mut self) -> DrinkResult<u64> {
        let current_block = self.current_height();
        self.externalities.execute_with(|| {
            let block_hash = R::finalize_block(current_block).map_err(Error::BlockFinalize)?;
            R::initialize_block(current_block + 1, block_hash).map_err(Error::BlockInitialize)?;
            Ok(current_block + 1)
        })
    }

    fn add_tokens(&mut self, address: AccountId32, amount: u128) {
        self.externalities.execute_with(|| {
            let _ = pallet_balances::Pallet::<R>::deposit_creating(&address, amount);
        });
    }

    fn balance(&mut self, address: &AccountId32) -> u128 {
        self.externalities
            .execute_with(|| pallet_balances::Pallet::<R>::free_balance(address))
    }
}
