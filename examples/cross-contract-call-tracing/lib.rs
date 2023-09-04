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
    use ink::storage::traits::Storable;
    use std::{error::Error, fs, path::PathBuf, rc::Rc};

    use drink::{
        runtime::{
            pallet_contracts_debugging::{DebugExt, DebugExtT},
            MinimalRuntime,
        },
        session::{
            contract_transcode::{ContractMessageTranscoder, Tuple, Value},
            Session,
        },
        AccountId32,
    };

    fn transcoder() -> Rc<ContractMessageTranscoder> {
        let path = PathBuf::from("target/ink/cross_contract_call_tracing.json");
        Rc::new(ContractMessageTranscoder::load(path).expect("Failed to create transcoder"))
    }

    fn bytes() -> Vec<u8> {
        let path = "target/ink/cross_contract_call_tracing.wasm";
        fs::read(path).expect("Failed to find or read contract file")
    }

    fn ok(v: Value) -> Value {
        Value::Tuple(Tuple::new(Some("Ok"), vec![v]))
    }

    struct TestDebugger;
    impl DebugExtT for TestDebugger {
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
                transcoder
                    .decode_return("outer_call", &mut result.as_slice())
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
        let mut session = Session::<MinimalRuntime>::new(Some(transcoder()))?;
        session.override_debug_handle(DebugExt(Box::new(TestDebugger {})));

        let outer_address = session.deploy(bytes(), "new", &[], vec![1])?;
        let middle_address = session.deploy(bytes(), "new", &[], vec![2])?;
        let inner_address = session.deploy(bytes(), "new", &[], vec![3])?;

        let value = session.call_with_address(
            outer_address,
            "outer_call",
            &[
                middle_address.to_string(),
                inner_address.to_string(),
                "7".to_string(),
            ],
        )?;

        assert_eq!(value, ok(Value::UInt(22)));

        Ok(())
    }
}
