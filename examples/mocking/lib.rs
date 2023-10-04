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
        pub fn delegate_call(&self, callee: AccountId, arg: u32) -> (u16, u16) {
            build_call::<DefaultEnvironment>()
                .call(callee)
                .gas_limit(0)
                .transferred_value(0)
                .exec_input(ExecutionInput::new(CALLEE_SELECTOR).push_arg(arg))
                .returns::<(u16, u16)>()
                .invoke()
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{error::Error, fs, path::PathBuf, rc::Rc};

    use drink::{
        mock::{ContractMock, MockingApi},
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
    fn initialization() -> Result<(), Box<dyn Error>> {
        let mut session = Session::<MinimalRuntime>::new()?;
        session.deploy(bytes(), "new", NO_ARGS, vec![], None, &transcoder())?;

        let mock = MockBuilder::new()
            .with_selector(CALLEE_SELECTOR)
            .with_matcher(|arg: u32| arg == 42)
            .with_return((1, 2))
            .build();

        session.mocking_api().register_mock(todo!());

        Ok(())
    }
}
