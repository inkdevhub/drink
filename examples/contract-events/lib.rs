#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod flipper {
    #[ink(event)]
    pub struct Flipped {
        new_value: bool,
    }

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
            self.value = !self.value;
            self.env().emit_event(Flipped {
                new_value: self.value,
            });
        }

        #[ink(message)]
        pub fn get(&self) -> bool {
            self.value
        }
    }
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use drink::{
        runtime::MinimalRuntime,
        session::{Session, NO_ARGS},
    };

    #[drink::contract_bundle_provider]
    enum BundleProvider {}

    #[drink::test]
    fn we_can_inspect_emitted_events() -> Result<(), Box<dyn Error>> {
        let bundle = BundleProvider::local()?;

        let events = Session::<MinimalRuntime>::new()?
            .deploy_bundle_and(bundle, "new", &["false"], vec![], None)?
            .call_and("flip", NO_ARGS, None)?
            .last_call_result()
            .expect("Call was successful, so there should be a return")
            .events
            .as_ref()
            .expect("Drink is collecting events")
            .clone();

        for event in events {
            println!("Event: {:?}", event);
        }

        Ok(())
    }
}
