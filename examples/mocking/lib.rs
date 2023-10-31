#![cfg_attr(not(feature = "std"), no_std, no_main)]

/// This is a fixed selector of the `callee` message.
const CALLEE_SELECTOR: [u8; 4] = ink::selector_bytes!("callee");

#[ink::contract]
mod proxy {
    use ink::env::{
        call::{build_call, ExecutionInput},
        DefaultEnvironment,
    };

    use crate::CALLEE_SELECTOR;

    #[ink(storage)]
    pub struct Proxy {}

    impl Proxy {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {}
        }

        /// Calls `callee` with the selector `CALLEE_SELECTOR` and forwards the result.
        #[ink(message)]
        pub fn forward_call(&self, callee: AccountId) -> (u8, u8) {
            build_call::<DefaultEnvironment>()
                .call(callee)
                .gas_limit(0)
                .transferred_value(0)
                .exec_input(ExecutionInput::new(CALLEE_SELECTOR.into()))
                .returns::<(u8, u8)>()
                .invoke()
        }
    }
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use drink::{
        mock_message,
        runtime::MinimalRuntime,
        session::{Session, NO_ARGS},
        ContractMock, MockingApi,
    };

    use crate::CALLEE_SELECTOR;

    #[drink::contract_bundle_provider]
    enum BundleProvider {}

    #[drink::test]
    fn call_mocked_message() -> Result<(), Box<dyn Error>> {
        let mut session = Session::<MinimalRuntime>::new()?;

        // Firstly, we create the mocked contract.
        const RETURN_VALUE: (u8, u8) = (4, 1);
        let mocked_contract =
            ContractMock::new().with_message(CALLEE_SELECTOR, mock_message(|()| RETURN_VALUE));

        // Secondly, we deploy it, similarly to a standard deployment action.
        let mock_address = session.mocking_api().deploy(mocked_contract);

        // Now, we can deploy our proper contract and verify its behavior.
        let result: (u8, u8) = session
            .deploy_bundle_and(BundleProvider::local()?, "new", NO_ARGS, vec![], None)?
            .call_and("forward_call", &[mock_address.to_string()], None)?
            .last_call_return()
            .expect("Call was successful, so there should be a return")
            .expect("Call was successful");
        assert_eq!(result, RETURN_VALUE);

        Ok(())
    }
}
