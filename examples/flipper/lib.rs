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
    use std::{error::Error, fs, path::PathBuf, rc::Rc};

    use drink::{
        runtime::MinimalRuntime,
        session::{contract_transcode::ContractMessageTranscoder, Session},
    };

    fn transcoder() -> Option<Rc<ContractMessageTranscoder>> {
        Some(Rc::new(
            ContractMessageTranscoder::load(PathBuf::from("./target/ink/flipper.json"))
                .expect("Failed to create transcoder"),
        ))
    }

    fn bytes() -> Vec<u8> {
        fs::read("./target/ink/flipper.wasm").expect("Failed to find or read contract file")
    }

    #[test]
    fn initialization() -> Result<(), Box<dyn Error>> {
        let init_value: bool = Session::<MinimalRuntime>::new(transcoder())?
            .deploy_and(bytes(), "new", &["true".to_string()], vec![], None)?
            .call_and("get", &[], None)?
            .last_call_return()
            .expect("Call was successful, so there should be a return")
            .expect("Call was successful");

        assert_eq!(init_value, true);

        Ok(())
    }

    #[test]
    fn flipping() -> Result<(), Box<dyn Error>> {
        let init_value: bool = Session::<MinimalRuntime>::new(transcoder())?
            .deploy_and(bytes(), "new", &["true".to_string()], vec![], None)?
            .call_and("flip", &[], None)?
            .call_and("flip", &[], None)?
            .call_and("flip", &[], None)?
            .call_and("get", &[], None)?
            .last_call_return()
            .expect("Call was successful, so there should be a return")
            .expect("Call was successful");

        assert_eq!(init_value, false);

        Ok(())
    }
}
