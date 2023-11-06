//! Basic chain API.

use frame_support::{
    sp_runtime::{traits::Dispatchable, DispatchResultWithInfo, Saturating},
    traits::tokens::currency::Currency,
};
use frame_system::pallet_prelude::BlockNumberFor;

use crate::{runtime::AccountIdFor, DrinkResult, Error, EventRecordOf, Runtime, Sandbox};

/// The runtime call type for a particular runtime.
pub type RuntimeCall<R> = <R as frame_system::Config>::RuntimeCall;

/// Interface for basic chain operations.
pub trait ChainApi<R: Runtime> {
    /// Return the current height of the chain.
    fn current_height(&mut self) -> BlockNumberFor<R>;

    /// Build a new empty block and return the new height.
    fn build_block(&mut self) -> DrinkResult<BlockNumberFor<R>>;

    /// Build `n` empty blocks and return the new height.
    ///
    /// # Arguments
    ///
    /// * `n` - The number of blocks to build.
    fn build_blocks(&mut self, n: u32) -> DrinkResult<BlockNumberFor<R>> {
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
    fn add_tokens(&mut self, address: AccountIdFor<R>, amount: u128);

    /// Return the balance of an account.
    ///
    /// # Arguments
    ///
    /// * `address` - The address of the account to query.
    fn balance(&mut self, address: &AccountIdFor<R>) -> u128;

    /// Return the timestamp of the current block.
    fn get_timestamp(&mut self) -> <R as pallet_timestamp::Config>::Moment;

    /// Set the timestamp of the current block.
    ///
    /// # Arguments
    ///
    /// * `timestamp` - The new timestamp to be set.
    fn set_timestamp(&mut self, timestamp: <R as pallet_timestamp::Config>::Moment);

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

    /// Return the events of the current block so far.
    fn get_current_block_events(&mut self) -> Vec<EventRecordOf<R>>;

    /// Reset the events of the current block.
    fn reset_current_block_events(&mut self);

    /// Execute the given closure with the inner externallities.
    ///
    /// Returns the result of the given closure.
    fn execute_with<T>(&mut self, execute: impl FnOnce() -> T) -> T;
}

impl<R: Runtime> ChainApi<R> for Sandbox<R> {
    fn current_height(&mut self) -> BlockNumberFor<R> {
        self.externalities
            .execute_with(|| frame_system::Pallet::<R>::block_number())
    }

    fn build_block(&mut self) -> DrinkResult<BlockNumberFor<R>> {
        let mut current_block = self.current_height();
        self.externalities.execute_with(|| {
            let block_hash = R::finalize_block(current_block).map_err(Error::BlockFinalize)?;
            current_block.saturating_inc();
            R::initialize_block(current_block, block_hash).map_err(Error::BlockInitialize)?;
            Ok(current_block)
        })
    }

    fn add_tokens(&mut self, address: AccountIdFor<R>, amount: u128) {
        self.externalities.execute_with(|| {
            let _ = pallet_balances::Pallet::<R>::deposit_creating(&address, amount);
        });
    }

    fn balance(&mut self, address: &AccountIdFor<R>) -> u128 {
        self.externalities
            .execute_with(|| pallet_balances::Pallet::<R>::free_balance(address))
    }

    fn get_timestamp(&mut self) -> <R as pallet_timestamp::Config>::Moment {
        self.externalities
            .execute_with(|| pallet_timestamp::Pallet::<R>::get())
    }

    fn set_timestamp(&mut self, timestamp: <R as pallet_timestamp::Config>::Moment) {
        self.externalities
            .execute_with(|| pallet_timestamp::Pallet::<R>::set_timestamp(timestamp));
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

    fn get_current_block_events(&mut self) -> Vec<EventRecordOf<R>> {
        self.externalities
            .execute_with(|| frame_system::Pallet::<R>::events())
    }

    fn reset_current_block_events(&mut self) {
        self.externalities
            .execute_with(|| frame_system::Pallet::<R>::reset_events())
    }

    fn execute_with<T>(&mut self, execute: impl FnOnce() -> T) -> T {
        self.externalities.execute_with(execute)
    }
}

#[cfg(test)]
mod tests {
    use frame_support::sp_runtime::traits::Dispatchable;

    use crate::{
        chain_api::{ChainApi, DispatchResultWithInfo, RuntimeCall},
        runtime::{minimal::RuntimeEvent, MinimalRuntime, Runtime},
        AccountId32, Sandbox,
    };

    fn make_transfer(
        sandbox: &mut Sandbox<MinimalRuntime>,
        dest: AccountId32,
        value: u128,
    ) -> DispatchResultWithInfo<<RuntimeCall<MinimalRuntime> as Dispatchable>::PostInfo> {
        let call = RuntimeCall::<MinimalRuntime>::Balances(
            pallet_balances::Call::<MinimalRuntime>::transfer { dest, value },
        );
        sandbox.runtime_call(call, Some(MinimalRuntime::default_actor()).into())
    }

    #[test]
    fn getting_and_setting_timestamp_works() {
        let mut sandbox = Sandbox::<MinimalRuntime>::new().expect("Failed to create sandbox");
        for timestamp in 0..10 {
            assert_ne!(sandbox.get_timestamp(), timestamp);
            sandbox.set_timestamp(timestamp);
            assert_eq!(sandbox.get_timestamp(), timestamp);

            sandbox.build_block().expect("Failed to build block");
        }
    }

    #[test]
    fn dry_run_works() {
        let mut sandbox = Sandbox::<MinimalRuntime>::new().expect("Failed to create sandbox");
        let actor = MinimalRuntime::default_actor();
        let initial_balance = sandbox.balance(&actor);

        sandbox.dry_run(|runtime| {
            runtime.add_tokens(actor.clone(), 100);
            assert_eq!(runtime.balance(&actor), initial_balance + 100);
        });

        assert_eq!(sandbox.balance(&actor), initial_balance);
    }

    #[test]
    fn runtime_call_works() {
        let mut sandbox = Sandbox::<MinimalRuntime>::new().expect("Failed to create sandbox");

        const RECIPIENT: AccountId32 = AccountId32::new([2u8; 32]);
        assert_ne!(MinimalRuntime::default_actor(), RECIPIENT);
        let initial_balance = sandbox.balance(&RECIPIENT);

        let result = make_transfer(&mut sandbox, RECIPIENT, 100);
        assert!(result.is_ok());

        let expected_balance = initial_balance + 100;
        assert_eq!(sandbox.balance(&RECIPIENT), expected_balance);
    }

    #[test]
    fn current_events() {
        let mut sandbox = Sandbox::<MinimalRuntime>::new().expect("Failed to create sandbox");

        let events_before = sandbox.get_current_block_events();
        assert!(events_before.is_empty());

        make_transfer(&mut sandbox, MinimalRuntime::default_actor(), 1)
            .expect("Failed to make transfer");

        let events_after = sandbox.get_current_block_events();
        assert_eq!(events_after.len(), 1);
        assert!(matches!(events_after[0].event, RuntimeEvent::Balances(_)));
    }

    #[test]
    fn resetting_events() {
        let mut sandbox = Sandbox::<MinimalRuntime>::new().expect("Failed to create sandbox");
        let actor = MinimalRuntime::default_actor();

        make_transfer(&mut sandbox, actor.clone(), 1).expect("Failed to make transfer");

        assert!(!sandbox.get_current_block_events().is_empty());
        sandbox.reset_current_block_events();
        assert!(sandbox.get_current_block_events().is_empty());

        make_transfer(&mut sandbox, actor, 1).expect("Failed to make transfer");
        assert!(!sandbox.get_current_block_events().is_empty());
    }
}
