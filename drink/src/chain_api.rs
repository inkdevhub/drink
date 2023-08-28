//! Basic chain API.

use frame_support::{
    dispatch::Dispatchable,
    sp_runtime::{AccountId32, DispatchResultWithInfo},
    traits::tokens::currency::Currency,
};

use crate::{DrinkResult, Error, Runtime, Sandbox};

/// The runtime call type for a particular runtime.
pub type RuntimeCall<R> = <R as frame_system::Config>::RuntimeCall;

/// Interface for basic chain operations.
pub trait ChainApi<R: Runtime> {
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

    /// Run an action without modifying the storage.
    ///
    /// # Arguments
    ///
    /// * `action` - The action to run.
    fn dry_run<T>(&mut self, action: impl FnOnce(&mut Self) -> T) -> T;

    /// Execute a runtime call (dispatchable).
    ///
    /// # Arguments
    ///
    /// * `call` - The runtime call to execute.
    /// * `origin` - The origin of the call.
    fn runtime_call(
        &mut self,
        call: RuntimeCall<R>,
        origin: <RuntimeCall<R> as Dispatchable>::RuntimeOrigin,
    ) -> DispatchResultWithInfo<<RuntimeCall<R> as Dispatchable>::PostInfo>;
}

impl<R: Runtime> ChainApi<R> for Sandbox<R> {
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

    fn dry_run<T>(&mut self, action: impl FnOnce(&mut Self) -> T) -> T {
        // Make a backup of the backend.
        let backend_backup = self.externalities.as_backend();

        // Run the action, potentially modifying storage. Ensure, that there are no pending changes
        // that would affect the reverted backend.
        let result = action(self);
        self.externalities
            .commit_all()
            .expect("Failed to commit changes");

        // Restore the backend.
        self.externalities.backend = backend_backup;

        result
    }

    fn runtime_call(
        &mut self,
        call: RuntimeCall<R>,
        origin: <RuntimeCall<R> as Dispatchable>::RuntimeOrigin,
    ) -> DispatchResultWithInfo<<RuntimeCall<R> as Dispatchable>::PostInfo> {
        self.externalities.execute_with(|| call.dispatch(origin))
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        chain_api::{ChainApi, RuntimeCall},
        runtime::MinimalRuntime,
        AccountId32, Sandbox, DEFAULT_ACTOR,
    };

    #[test]
    fn dry_run_works() {
        let mut sandbox = Sandbox::<MinimalRuntime>::new().expect("Failed to create sandbox");
        let initial_balance = sandbox.balance(&DEFAULT_ACTOR);

        sandbox.dry_run(|runtime| {
            runtime.add_tokens(DEFAULT_ACTOR, 100);
            assert_eq!(runtime.balance(&DEFAULT_ACTOR), initial_balance + 100);
        });

        assert_eq!(sandbox.balance(&DEFAULT_ACTOR), initial_balance);
    }

    #[test]
    fn runtime_call_works() {
        let mut sandbox = Sandbox::<MinimalRuntime>::new().expect("Failed to create sandbox");

        const RECIPIENT: AccountId32 = AccountId32::new([2u8; 32]);
        assert_ne!(DEFAULT_ACTOR, RECIPIENT);
        let initial_balance = sandbox.balance(&RECIPIENT);

        let call = RuntimeCall::<MinimalRuntime>::Balances(
            pallet_balances::Call::<MinimalRuntime>::transfer {
                dest: RECIPIENT,
                value: 100,
            },
        );
        let result = sandbox.runtime_call(call, Some(DEFAULT_ACTOR).into());
        assert!(result.is_ok());

        let expected_balance = initial_balance + 100;
        assert_eq!(sandbox.balance(&RECIPIENT), expected_balance);
    }
}
