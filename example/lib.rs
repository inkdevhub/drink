#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod example {
    use ink::env::debug_println;

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
            debug_println!("Previous value: `{}`", self.value);
            self.value = !self.value;
            debug_println!("Flipped to: `{}`", self.value);
        }

        #[ink(message)]
        pub fn get(&self) -> bool {
            self.value
        }
    }
}
