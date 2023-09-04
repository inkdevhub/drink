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
