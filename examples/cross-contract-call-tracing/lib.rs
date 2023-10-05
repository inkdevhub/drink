#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod contract {
    use ink::env::{
        call::{build_call, ExecutionInput, Selector},
        DefaultEnvironment,
    };

    #[ink(storage)]
    pub struct CrossCallingContract;

    impl CrossCallingContract {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {}
        }

        #[ink(message)]
        pub fn outer_call(
            &self,
            next_callee: AccountId,
            next_next_callee: AccountId,
            arg: u32,
        ) -> u32 {
            build_call::<DefaultEnvironment>()
                .call(next_callee)
                .gas_limit(0)
                .transferred_value(0)
                .exec_input(
                    ExecutionInput::new(Selector::new(ink::selector_bytes!("middle_call")))
                        .push_arg(next_next_callee)
                        .push_arg(arg),
                )
                .returns::<u32>()
                .invoke()
        }

        #[ink(message)]
        pub fn middle_call(&self, next_callee: AccountId, arg: u32) -> u32 {
            build_call::<DefaultEnvironment>()
                .call(next_callee)
                .gas_limit(0)
                .transferred_value(0)
                .exec_input(
                    ExecutionInput::new(Selector::new(ink::selector_bytes!("inner_call")))
                        .push_arg(arg),
                )
                .returns::<u32>()
                .invoke()
        }

        #[ink(message)]
        pub fn inner_call(&self, arg: u32) -> u32 {
            match arg % 2 {
                0 => arg / 2,
                _ => 3 * arg + 1,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, error::Error, fs, path::PathBuf, rc::Rc};

    use drink::{
        runtime::{
            pallet_contracts_debugging::{TracingExt, TracingExtT},
            MinimalRuntime,
        },
        session::{
            contract_transcode::{ContractMessageTranscoder, Value},
            Session, NO_ARGS,
        },
        AccountId32,
    };
    use ink::storage::traits::Storable;

    fn transcoder() -> Rc<ContractMessageTranscoder> {
        let path = PathBuf::from("target/ink/cross_contract_call_tracing.json");
        Rc::new(ContractMessageTranscoder::load(path).expect("Failed to create transcoder"))
    }

    fn bytes() -> Vec<u8> {
        let path = "target/ink/cross_contract_call_tracing.wasm";
        fs::read(path).expect("Failed to find or read contract file")
    }

    thread_local! {
        static OUTER_ADDRESS:  RefCell<Option<AccountId32>> = RefCell::new(None);
        static MIDDLE_ADDRESS:  RefCell<Option<AccountId32>> = RefCell::new(None);
        static INNER_ADDRESS:  RefCell<Option<AccountId32>> = RefCell::new(None);
    }

    struct TestDebugger;
    impl TracingExtT for TestDebugger {
        fn after_call(
            &self,
            contract_address: Vec<u8>,
            is_call: bool,
            input_data: Vec<u8>,
            result: Vec<u8>,
        ) {
            let contract_address = AccountId32::decode(&mut contract_address.as_slice())
                .expect("Failed to decode contract address");
            let transcoder = transcoder();

            let data_decoded = if is_call {
                transcoder.decode_contract_message(&mut input_data.as_slice())
            } else {
                transcoder.decode_contract_constructor(&mut input_data.as_slice())
            }
            .unwrap();

            let return_decoded = if is_call {
                let call_name = if contract_address
                    == OUTER_ADDRESS.with(|a| a.borrow().clone().unwrap())
                {
                    "outer_call"
                } else if contract_address == MIDDLE_ADDRESS.with(|a| a.borrow().clone().unwrap()) {
                    "middle_call"
                } else if contract_address == INNER_ADDRESS.with(|a| a.borrow().clone().unwrap()) {
                    "inner_call"
                } else {
                    panic!("Unexpected contract address")
                };

                transcoder
                    .decode_return(call_name, &mut result.as_slice())
                    .unwrap()
            } else {
                Value::Unit
            };

            println!(
                "Contract at address `{contract_address}` has been called with data: \
                    \n    {data_decoded}\nand returned:\n    {return_decoded}\n"
            )
        }
    }

    #[test]
    fn test() -> Result<(), Box<dyn Error>> {
        let mut session = Session::<MinimalRuntime>::new()?;
        session.override_debug_handle(TracingExt(Box::new(TestDebugger {})));

        let outer_address =
            session.deploy(bytes(), "new", NO_ARGS, vec![1], None, &transcoder())?;
        OUTER_ADDRESS.with(|a| *a.borrow_mut() = Some(outer_address.clone()));
        let middle_address =
            session.deploy(bytes(), "new", NO_ARGS, vec![2], None, &transcoder())?;
        MIDDLE_ADDRESS.with(|a| *a.borrow_mut() = Some(middle_address.clone()));
        let inner_address =
            session.deploy(bytes(), "new", NO_ARGS, vec![3], None, &transcoder())?;
        INNER_ADDRESS.with(|a| *a.borrow_mut() = Some(inner_address.clone()));

        let value = session.call_with_address(
            outer_address,
            "outer_call",
            &[
                &*middle_address.to_string(),
                &*inner_address.to_string(),
                "7",
            ],
            None,
        )?;
        let value = u32::decode(&mut value.as_slice()).expect("Failed to decode return value");

        assert_eq!(value, value);

        Ok(())
    }
}
