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
    use std::{error::Error, fs, path::PathBuf};

    use drink::session::{
        contract_transcode::{ContractMessageTranscoder, Tuple, Value},
        Session,
    };

    fn transcoder() -> ContractMessageTranscoder {
        ContractMessageTranscoder::load(PathBuf::from("./target/ink/flipper.json"))
            .expect("Failed to create transcoder")
    }

    fn bytes() -> Vec<u8> {
        fs::read("./target/ink/flipper.wasm").expect("Failed to find or read contract file")
    }

    fn ok(v: Value) -> Value {
        Value::Tuple(Tuple::new(Some("Ok"), vec![v]))
    }

    #[test]
    fn initialization() -> Result<(), Box<dyn Error>> {
        let init_value = Session::new(Some(transcoder()))?
            .deploy(bytes(), "new", &["true"], vec![])?
            .call("get", &[])?
            .last_call_return()
            .expect("Call was successful");

        assert_eq!(init_value, ok(Value::Bool(true)));

        Ok(())
    }

    #[test]
    fn flipping() -> Result<(), Box<dyn Error>> {
        let init_value = Session::new(Some(transcoder()))?
            .deploy(bytes(), "new", &["true"], vec![])?
            .call("flip", &[])?
            .call("flip", &[])?
            .call("flip", &[])?
            .call("get", &[])?
            .last_call_return()
            .expect("Call was successful");

        assert_eq!(init_value, ok(Value::Bool(false)));

        Ok(())
    }
}
