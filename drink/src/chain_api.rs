//! Basic chain API.

use frame_support::{
    sp_runtime::{traits::Dispatchable, DispatchError, DispatchResultWithInfo, Saturating},
    traits::{
        fungible::{Inspect, Mutate},
        tokens::{Fortitude, Preservation},
    },
};
use frame_system::pallet_prelude::BlockNumberFor;

use crate::{runtime::AccountIdFor, DrinkResult, Error, EventRecordOf, Runtime, Sandbox};

/// Generic Time type.
pub type MomentOf<R> = <R as pallet_timestamp::Config>::Moment;

/// Generic fungible balance type.
pub type BalanceOf<R> = <R as pallet_balances::Config>::Balance;

/// The runtime call type for a particular runtime.
pub type RuntimeCall<R> = <R as frame_system::Config>::RuntimeCall;

impl<R: Runtime> Sandbox<R> {
    /// Build a new empty block and return the new height.
    pub fn build_block(&mut self) -> DrinkResult<BlockNumberFor<R>> {
        self.execute_with(|| {
            let mut current_block = frame_system::Pallet::<R>::block_number();
            let block_hash = R::finalize_block(current_block).map_err(Error::BlockFinalize)?;
            current_block.saturating_inc();
            R::initialize_block(current_block, block_hash).map_err(Error::BlockInitialize)?;
            Ok(current_block)
        })
    }
    /// Build `n` empty blocks and return the new height.
    ///
    /// # Arguments
    ///
    /// * `n` - The number of blocks to build.
    pub fn build_blocks(&mut self, n: u32) -> DrinkResult<BlockNumberFor<R>> {
        let mut last_block = None;
        for _ in 0..n {
            last_block = Some(self.build_block()?);
        }
        Ok(last_block.unwrap_or_else(|| self.current_height()))
    }
}

impl<R: frame_system::Config> Sandbox<R> {
    /// Return the current height of the chain.
    pub fn current_height(&mut self) -> BlockNumberFor<R> {
        self.execute_with(|| frame_system::Pallet::<R>::block_number())
    }

    /// Return the events of the current block so far.
    pub fn get_current_block_events(&mut self) -> Vec<EventRecordOf<R>> {
        self.execute_with(frame_system::Pallet::<R>::events)
    }

    /// Reset the events of the current block.
    pub fn reset_current_block_events(&mut self) {
        self.execute_with(frame_system::Pallet::<R>::reset_events)
    }

    /// Execute a runtime call (dispatchable).
    ///
    /// # Arguments
    ///
    /// * `call` - The runtime call to execute.
    /// * `origin` - The origin of the call.
    pub fn runtime_call(
        &mut self,
        call: RuntimeCall<R>,
        origin: <RuntimeCall<R> as Dispatchable>::RuntimeOrigin,
    ) -> DispatchResultWithInfo<<RuntimeCall<R> as Dispatchable>::PostInfo> {
        self.execute_with(|| call.dispatch(origin))
    }
}

impl<R: pallet_balances::Config> Sandbox<R> {
    /// Add tokens to an account.
    ///
    /// # Arguments
    ///
    /// * `address` - The address of the account to add tokens to.
    /// * `amount` - The number of tokens to add.
    pub fn add_tokens(
        &mut self,
        address: AccountIdFor<R>,
        amount: BalanceOf<R>,
    ) -> Result<BalanceOf<R>, DispatchError> {
        self.execute_with(|| pallet_balances::Pallet::<R>::mint_into(&address, amount))
    }

    /// Add tokens to an account.
    ///
    /// # Arguments
    ///
    /// * `address` - The address of the account to add tokens to.
    /// * `amount` - The number of tokens to add.
    pub fn balance(&mut self, address: &AccountIdFor<R>) -> BalanceOf<R> {
        self.execute_with(|| {
            pallet_balances::Pallet::<R>::reducible_balance(
                address,
                Preservation::Expendable,
                Fortitude::Force,
            )
        })
    }
}

impl<R: pallet_timestamp::Config> Sandbox<R> {
    /// Return the timestamp of the current block.
    pub fn get_timestamp(&mut self) -> MomentOf<R> {
        self.execute_with(pallet_timestamp::Pallet::<R>::get)
    }

    /// Set the timestamp of the current block.
    ///
    /// # Arguments
    ///
    /// * `timestamp` - The new timestamp to be set.
    pub fn set_timestamp(&mut self, timestamp: MomentOf<R>) {
        self.execute_with(|| pallet_timestamp::Pallet::<R>::set_timestamp(timestamp))
    }
}

#[cfg(test)]
mod tests {
    use frame_support::sp_runtime::traits::Dispatchable;

    use crate::{
        chain_api::{DispatchResultWithInfo, RuntimeCall},
        runtime::{minimal::RuntimeEvent, MinimalRuntime, Runtime},
        AccountId32, Sandbox,
    };

    fn make_transfer(
        sandbox: &mut Sandbox<MinimalRuntime>,
        dest: AccountId32,
        value: u128,
    ) -> DispatchResultWithInfo<<RuntimeCall<MinimalRuntime> as Dispatchable>::PostInfo> {
        sandbox.execute_with(|| {
            RuntimeCall::<MinimalRuntime>::Balances(
                pallet_balances::Call::<MinimalRuntime>::transfer { dest, value },
            )
            .dispatch(Some(MinimalRuntime::default_actor()).into())
        })
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
            runtime.add_tokens(actor.clone(), 100).unwrap();
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
