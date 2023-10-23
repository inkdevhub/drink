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

#[cfg(test)]
mod tests {
    use std::error::Error;

    use drink::{
        local_contract_file,
        runtime::MinimalRuntime,
        session::{Session, NO_ARGS},
        ContractBundle,
    };

    #[drink::test]
    fn initialization() -> Result<(), Box<dyn Error>> {
        let contract = ContractBundle::load("./target/ink/flipper.contract")?;
        let init_value: bool = Session::<MinimalRuntime>::new()?
            .deploy_bundle_and(contract, "new", &["true"], vec![], None)?
            .call_and("get", NO_ARGS, None)?
            .last_call_return()
            .expect("Call was successful, so there should be a return")
            .expect("Call was successful");

        assert_eq!(init_value, true);

        Ok(())
    }

    #[drink::test]
    fn flipping() -> Result<(), Box<dyn Error>> {
        let contract = local_contract_file!();
        let init_value: bool = Session::<MinimalRuntime>::new()?
            .deploy_bundle_and(contract, "new", &["true"], vec![], None)?
            .call_and("flip", NO_ARGS, None)?
            .call_and("flip", NO_ARGS, None)?
            .call_and("flip", NO_ARGS, None)?
            .call_and("get", NO_ARGS, None)?
            .last_call_return()
            .expect("Call was successful, so there should be a return")
            .expect("Call was successful");

        assert_eq!(init_value, false);

        Ok(())
    }
}
