use std::collections::BTreeMap;

use parity_scale_codec::{Decode, Encode};

use crate::{
    errors::LangError,
    mock::{error::MockingError, MockedCallResult},
};

/// Alias for a 4-byte selector.
pub type Selector = [u8; 4];
/// An untyped message mock.
///
/// Notice that in the end, we cannot operate on specific argument/return types. Rust won't let us
/// have a collection of differently typed closures. Fortunately, we can assume that all types are
/// en/decodable, so we can use `Vec<u8>` as a common denominator.
pub type MessageMock = Box<dyn Fn(Vec<u8>) -> MockedCallResult + Send + Sync>;

/// A contract mock.
pub struct ContractMock {
    messages: BTreeMap<Selector, MessageMock>,
}

impl ContractMock {
    /// Creates a new mock without any message.
    pub fn new() -> Self {
        Self {
            messages: BTreeMap::new(),
        }
    }

    /// Adds a message mock.
    pub fn with_message(mut self, selector: Selector, message: MessageMock) -> Self {
        self.messages.insert(selector, message);
        self
    }

    /// Try to call a message mock. Returns an error if there is no message mock for `selector`.
    pub fn call(&self, selector: Selector, input: Vec<u8>) -> MockedCallResult {
        match self.messages.get(&selector) {
            None => Err(MockingError::MessageNotFound(selector)),
            Some(message) => message(input),
        }
    }
}

impl Default for ContractMock {
    fn default() -> Self {
        Self::new()
    }
}

/// A helper function to create a message mock out of a typed closure.
///
/// In particular, it takes care of decoding the input and encoding the output. Also, wraps the
/// return value in a `Result`, which is normally done implicitly by ink!.
pub fn mock_message<Args: Decode, Ret: Encode, Body: Fn(Args) -> Ret + Send + Sync + 'static>(
    body: Body,
) -> MessageMock {
    Box::new(move |encoded_input| {
        let input = Decode::decode(&mut &*encoded_input).map_err(MockingError::ArgumentDecoding)?;
        Ok(Ok::<Ret, LangError>(body(input)).encode())
    })
}
