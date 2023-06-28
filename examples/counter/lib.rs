#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod counter {
    use ink::env::debug_println;

    #[ink(storage)]
    pub struct Counter {
        value: u32,
    }

    impl Counter {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self { value: 0 }
        }

        #[ink(message)]
        pub fn bump(&mut self, by: u32) {
            debug_println!("Previous value: `{}`", self.value);
            self.value += by;
            debug_println!("Bumped to:      `{}`", self.value);
        }

        #[ink(message)]
        pub fn get(&self) -> u32 {
            debug_println!("Reading value from storage");
            self.value
        }
    }
}
