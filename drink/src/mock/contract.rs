use std::collections::BTreeMap;

use crate::mock::{error::MockingError, MockedCallResult};

pub type Selector = [u8; 4];
pub type MessageMock = Box<dyn Fn(Vec<u8>) -> Vec<u8> + Send + Sync>;

pub struct ContractMock {
    messages: BTreeMap<Selector, MessageMock>,
}

impl ContractMock {
    pub fn new() -> Self {
        Self {
            messages: BTreeMap::new(),
        }
    }

    pub fn with_message(mut self, selector: Selector, message: MessageMock) -> Self {
        self.messages.insert(selector, message);
        self
    }

    pub fn call(&self, selector: Selector, input: Vec<u8>) -> MockedCallResult {
        match self.messages.get(&selector) {
            None => Err(MockingError::MessageNotFound(selector)),
            Some(message) => Ok(message(input)),
        }
    }
}
