//! System API for the sandbox.

use frame_support::sp_runtime::{traits::Dispatchable, DispatchResultWithInfo};
use frame_system::pallet_prelude::BlockNumberFor;

use super::Sandbox;
use crate::{EventRecordOf, RuntimeCall, SandboxConfig};

impl<Config: SandboxConfig> Sandbox<Config> {
    /// Return the current height of the chain.
    pub fn block_number(&mut self) -> BlockNumberFor<Config::Runtime> {
        self.execute_with(frame_system::Pallet::<Config::Runtime>::block_number)
    }

    /// Return the events of the current block so far.
    pub fn events(&mut self) -> Vec<EventRecordOf<Config::Runtime>> {
        self.execute_with(frame_system::Pallet::<Config::Runtime>::events)
    }

    /// Reset the events of the current block.
    pub fn reset_events(&mut self) {
        self.execute_with(frame_system::Pallet::<Config::Runtime>::reset_events)
    }

    /// Execute a runtime call (dispatchable).
    ///
    /// # Arguments
    ///
    /// * `call` - The runtime call to execute.
    /// * `origin` - The origin of the call.
    pub fn runtime_call<
        Origin: Into<<RuntimeCall<Config::Runtime> as Dispatchable>::RuntimeOrigin>,
    >(
        &mut self,
        call: RuntimeCall<Config::Runtime>,
        origin: Origin,
    ) -> DispatchResultWithInfo<<RuntimeCall<Config::Runtime> as Dispatchable>::PostInfo> {
        self.execute_with(|| call.dispatch(origin.into()))
    }
}

#[cfg(test)]
mod tests {
    use frame_support::sp_runtime::{traits::Dispatchable, DispatchResultWithInfo};

    use crate::{
        runtime::{minimal::RuntimeEvent, MinimalRuntime},
        AccountId32, RuntimeCall, Sandbox, SandboxConfig,
    };

    fn make_transfer(
        sandbox: &mut Sandbox<MinimalRuntime>,
        dest: AccountId32,
        value: u128,
    ) -> DispatchResultWithInfo<<RuntimeCall<MinimalRuntime> as Dispatchable>::PostInfo> {
        assert_ne!(
            MinimalRuntime::default_actor(),
            dest,
            "make_transfer should send to account different than default_actor"
        );
        sandbox.runtime_call(
            RuntimeCall::<MinimalRuntime>::Balances(
                pallet_balances::Call::<MinimalRuntime>::transfer_allow_death {
                    dest: dest.into(),
                    value,
                },
            ),
            Some(MinimalRuntime::default_actor()),
        )
    }

    #[test]
    fn dry_run_works() {
        let mut sandbox = Sandbox::<MinimalRuntime>::new().expect("Failed to create sandbox");
        let actor = MinimalRuntime::default_actor();
        let initial_balance = sandbox.free_balance(&actor);

        sandbox.dry_run(|runtime| {
            runtime.mint_into(actor.clone(), 100).unwrap();
            assert_eq!(runtime.free_balance(&actor), initial_balance + 100);
        });

        assert_eq!(sandbox.free_balance(&actor), initial_balance);
    }

    #[test]
    fn runtime_call_works() {
        let mut sandbox = Sandbox::<MinimalRuntime>::new().expect("Failed to create sandbox");

        const RECIPIENT: AccountId32 = AccountId32::new([2u8; 32]);
        let initial_balance = sandbox.free_balance(&RECIPIENT);

        let result = make_transfer(&mut sandbox, RECIPIENT, 100);
        assert!(result.is_ok());

        let expected_balance = initial_balance + 100;
        assert_eq!(sandbox.free_balance(&RECIPIENT), expected_balance);
    }

    #[test]
    fn current_events() {
        let mut sandbox = Sandbox::<MinimalRuntime>::new().expect("Failed to create sandbox");
        const RECIPIENT: AccountId32 = AccountId32::new([2u8; 32]);

        let events_before = sandbox.events();
        assert!(events_before.is_empty());

        make_transfer(&mut sandbox, RECIPIENT, 1).expect("Failed to make transfer");

        let events_after = sandbox.events();
        assert!(!events_after.is_empty());
        assert!(matches!(
            events_after.last().unwrap().event,
            RuntimeEvent::Balances(_)
        ));
    }

    #[test]
    fn resetting_events() {
        let mut sandbox = Sandbox::<MinimalRuntime>::new().expect("Failed to create sandbox");
        const RECIPIENT: AccountId32 = AccountId32::new([3u8; 32]);

        make_transfer(&mut sandbox, RECIPIENT.clone(), 1).expect("Failed to make transfer");

        assert!(!sandbox.events().is_empty());
        sandbox.reset_events();
        assert!(sandbox.events().is_empty());

        make_transfer(&mut sandbox, RECIPIENT, 1).expect("Failed to make transfer");
        assert!(!sandbox.events().is_empty());
    }
}
