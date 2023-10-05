//! Mocking contract feature.

mod contract;
mod error;
mod mocking_api;

use std::collections::BTreeMap;

pub use contract::{ContractMock, MessageMock, Selector};
use error::MockingError;
pub use mocking_api::MockingApi;

pub type MockedCallResult = Result<Vec<u8>, MockingError>;

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

    pub fn get(&self, address: &AccountId) -> Option<&ContractMock> {
        self.mocked_contracts.get(address)
    }
}
