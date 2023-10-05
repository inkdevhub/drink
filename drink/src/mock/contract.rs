use crate::mock::message::MessageMockT;

pub struct ContractMock {
    messages: Vec<Box<dyn MessageMockT + Send + Sync>>,
}

impl ContractMock {
    pub fn new() -> Self {
        Self {
            messages: Vec::new(),
        }
    }

    pub fn with_message<M: MessageMockT + Send + Sync + 'static>(mut self, message: M) -> Self {
        self.messages.push(Box::new(message));
        self
    }
}
