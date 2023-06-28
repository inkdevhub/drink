#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod flipper {
    use ink::env::debug_println;

    #[ink(storage)]
    pub struct Flipper {
        value: bool,
    }

    impl Flipper {
        #[ink(constructor)]
        pub fn new(init: bool) -> Self {
            Self { value: init }
        }

        #[ink(message)]
        pub fn flip(&mut self) {
            debug_println!("Previous value: `{}`", self.value);
            self.value = !self.value;
            debug_println!("Flipped to:     `{}`", self.value);
        }

        #[ink(message)]
        pub fn get(&self) -> bool {
            debug_println!("Reading value from storage");
            self.value
        }
    }
}
