//! Mocking contract feature.

mod builder_utils;
mod contract;
mod message;
mod mocking_api;

use std::collections::BTreeMap;

pub use contract::ContractMock;
pub use message::{MessageMock, MessageMockBuilder};
pub use mocking_api::MockingApi;

pub struct MockRegistry<AccountId: Ord> {
    mocked_contracts: BTreeMap<AccountId, ContractMock>,
}

impl<AccountId: Ord> MockRegistry<AccountId> {
    pub fn new() -> Self {
        Self {
            mocked_contracts: BTreeMap::new(),
        }
    }

    pub fn register(&mut self, address: AccountId, mock: ContractMock) {
        self.mocked_contracts.insert(address, mock);
    }
}
