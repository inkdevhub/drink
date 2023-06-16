#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod example {
    #[ink(storage)]
    pub struct Example {
        value: bool,
    }

    impl Example {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self { value: false }
        }

        #[ink(message)]
        pub fn flip(&mut self) {
            self.value = !self.value;
        }

        #[ink(message)]
        pub fn get(&self) -> bool {
            self.value
        }
    }
}
