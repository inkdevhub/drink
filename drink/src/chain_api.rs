//! Basic chain API.

use frame_support::{sp_runtime::AccountId32, traits::tokens::currency::Currency};

use crate::{Runtime, Sandbox};

/// Interface for basic chain operations.
pub trait ChainApi {
    /// Build a new empty block.
    fn build_block(&mut self);

    /// Build `n` empty blocks.
    fn build_blocks(&mut self, n: u32) {
        for _ in 0..n {
            self.build_block();
        }
    }

    /// Add tokens to an account.
    fn add_tokens(&mut self, address: AccountId32, amount: u128);
}

impl<R: Runtime> ChainApi for Sandbox<R> {
    fn build_block(&mut self) {
        // let new_block = self.externalities.execute_with(|| {
        //     let current_block = System::block_number();
        //
        //     Contracts::on_finalize(current_block);
        //     Timestamp::on_finalize(current_block);
        //     Balances::on_finalize(current_block);
        //
        //     let parent_hash = if current_block > 1 {
        //         System::finalize().hash()
        //     } else {
        //         System::parent_hash()
        //     };
        //
        //     System::initialize(&(current_block + 1), &parent_hash, &Default::default());
        //
        //     current_block + 1
        // });

        // self.init_block(new_block);
    }

    fn add_tokens(&mut self, address: AccountId32, amount: u128) {
        self.externalities.execute_with(|| {
            let _ = pallet_balances::Pallet::<R>::deposit_creating(&address, amount);
        });
    }
}
