//! Mocking contract feature.

mod builder_utils;
mod contract;
mod message;
mod mocking_api;

pub use contract::ContractMock;
pub use message::{MessageMock, MessageMockBuilder};
pub use mocking_api::MockingApi;

pub struct MockRegistry {
    mocked_contracts: Vec<ContractMock>,
}

impl MockRegistry {
    pub fn new() -> Self {
        Self {
            mocked_contracts: Vec::new(),
        }
    }
}
