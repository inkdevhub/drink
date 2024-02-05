//! Balance API for the sandbox.
use frame_support::{sp_runtime::DispatchError, traits::fungible::Mutate};

use super::Sandbox;
use crate::{runtime::AccountIdFor, BalanceOf};

impl<Config: crate::SandboxConfig> Sandbox<Config>
where
    Config::Runtime: crate::pallet_balances::Config,
{
    /// Mint tokens to an account.
    ///
    /// # Arguments
    ///
    /// * `address` - The address of the account to add tokens to.
    /// * `amount` - The number of tokens to add.
    pub fn mint_into(
        &mut self,
        address: AccountIdFor<Config::Runtime>,
        amount: BalanceOf<Config::Runtime>,
    ) -> Result<BalanceOf<Config::Runtime>, DispatchError> {
        self.execute_with(|| {
            pallet_balances::Pallet::<Config::Runtime>::mint_into(&address, amount)
        })
    }

    /// Return the free balance of an account.
    ///
    /// # Arguments
    ///
    /// * `address` - The address of the account to query.
    pub fn free_balance(
        &mut self,
        address: &AccountIdFor<Config::Runtime>,
    ) -> BalanceOf<Config::Runtime> {
        self.execute_with(|| pallet_balances::Pallet::<Config::Runtime>::free_balance(address))
    }
}
