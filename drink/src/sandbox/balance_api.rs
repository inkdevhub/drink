//! Balance API for the sandbox.
use frame_support::{sp_runtime::DispatchError, traits::fungible::Mutate};

use super::Sandbox;
use crate::{runtime::AccountIdFor, BalanceOf};

impl<R: pallet_balances::Config> Sandbox<R> {
    /// Mint tokens to an account.
    ///
    /// # Arguments
    ///
    /// * `address` - The address of the account to add tokens to.
    /// * `amount` - The number of tokens to add.
    pub fn mint_into(
        &mut self,
        address: AccountIdFor<R>,
        amount: BalanceOf<R>,
    ) -> Result<BalanceOf<R>, DispatchError> {
        self.execute_with(|| pallet_balances::Pallet::<R>::mint_into(&address, amount))
    }

    /// Return the free balance of an account.
    ///
    /// # Arguments
    ///
    /// * `address` - The address of the account to query.
    pub fn free_balance(&mut self, address: &AccountIdFor<R>) -> BalanceOf<R> {
        self.execute_with(|| pallet_balances::Pallet::<R>::free_balance(address))
    }
}
