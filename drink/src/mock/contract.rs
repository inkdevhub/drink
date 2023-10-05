use std::collections::BTreeMap;

use parity_scale_codec::{Decode, Encode};

use crate::{
    mock::{error::MockingError, MockedCallResult},
    session::errors::LangError,
};

pub type Selector = [u8; 4];
pub type MessageMock = Box<dyn Fn(Vec<u8>) -> MockedCallResult + Send + Sync>;

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
            Some(message) => message(input),
        }
    }
}

pub fn mock_message<Args: Decode, Ret: Encode, Body: Fn(Args) -> Ret + Send + Sync + 'static>(
    body: Body,
) -> MessageMock {
    Box::new(move |encoded_input| {
        let input = Decode::decode(&mut &*encoded_input).map_err(MockingError::ArgumentDecoding)?;
        Ok(Ok::<Ret, LangError>(body(input)).encode())
    })
}
