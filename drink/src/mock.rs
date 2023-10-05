mod contract;
mod error;
mod mocking_api;

use std::collections::BTreeMap;

pub use contract::{mock_message, ContractMock, MessageMock, Selector};
use error::MockingError;
pub use mocking_api::MockingApi;

/// Untyped result of a mocked call.
pub type MockedCallResult = Result<Vec<u8>, MockingError>;

/// A registry of mocked contracts.
pub(crate) struct MockRegistry<AccountId: Ord> {
    mocked_contracts: BTreeMap<AccountId, ContractMock>,
}

impl<AccountId: Ord> MockRegistry<AccountId> {
    /// Creates a new registry.
    pub fn new() -> Self {
        Self {
            mocked_contracts: BTreeMap::new(),
        }
    }

    /// Registers `mock` for `address`.
    pub fn register(&mut self, address: AccountId, mock: ContractMock) {
        self.mocked_contracts.insert(address, mock);
    }

    /// Returns the mock for `address`, if any.
    pub fn get(&self, address: &AccountId) -> Option<&ContractMock> {
        self.mocked_contracts.get(address)
    }
}
