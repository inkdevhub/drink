#![cfg_attr(not(feature = "std"), no_std, no_main)]

use ink::env::call::Selector;

const CALLEE_SELECTOR: Selector = Selector::new(ink::selector_bytes!("callee"));

#[ink::contract]
mod contract {
    use ink::env::{
        call::{build_call, ExecutionInput},
        DefaultEnvironment,
    };

    use crate::CALLEE_SELECTOR;

    #[ink(storage)]
    pub struct Contract {}

    impl Contract {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {}
        }

        #[ink(message)]
        pub fn delegate_call(&self, callee: AccountId) -> (u16, u16) {
            build_call::<DefaultEnvironment>()
                .call(callee)
                .gas_limit(0)
                .transferred_value(0)
                .exec_input(ExecutionInput::new(CALLEE_SELECTOR).push_arg(41u8))
                .returns::<(u16, u16)>()
                .invoke()
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{error::Error, fs, path::PathBuf, rc::Rc};

    use drink::{
        mock::{ContractMock, MessageMockBuilder, MockingApi},
        runtime::MinimalRuntime,
        session::{contract_transcode::ContractMessageTranscoder, Session, NO_ARGS},
    };

    use crate::CALLEE_SELECTOR;

    fn transcoder() -> Rc<ContractMessageTranscoder> {
        Rc::new(
            ContractMessageTranscoder::load(PathBuf::from("./target/ink/mocking.json"))
                .expect("Failed to create transcoder"),
        )
    }

    fn bytes() -> Vec<u8> {
        fs::read("./target/ink/mocking.wasm").expect("Failed to find or read contract file")
    }

    #[test]
    fn call_mocked_message() -> Result<(), Box<dyn Error>> {
        let mut session = Session::<MinimalRuntime>::new()?;

        // Firstly, we are creating our mocked contract.

        let mocked_message = MessageMockBuilder::new()
            .with_selector(CALLEE_SELECTOR.to_bytes())
            .with_body(Box::new(|_: u8| (4, 1)))
            .build();

        let mocked_contract = ContractMock::new().with_message(mocked_message);

        // Secondly, we are deploying it, similarly to a standard deployment action..
        let mock_address = session.mocking_api().deploy_mock(mocked_contract);

        // Now, we can deploy our proper contract and invoke it.
        let result: (u16, u16) = session
            .deploy_and(bytes(), "new", NO_ARGS, vec![], None, &transcoder())?
            .call_and("delegate_call", &[mock_address.to_string()], None)?
            .last_call_return()
            .expect("Call was successful, so there should be a return")
            .expect("Call was successful");
        assert_eq!(result, (4, 1));

        Ok(())
    }
}
