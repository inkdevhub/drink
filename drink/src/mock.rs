mod contract;
mod error;

use std::collections::BTreeMap;

pub use contract::{mock_message, ContractMock, MessageMock, Selector};
use error::MockingError;

/// Untyped result of a mocked call.
pub type MockedCallResult = Result<Vec<u8>, MockingError>;

/// A registry of mocked contracts.
pub(crate) struct MockRegistry<AccountId: Ord> {
    mocked_contracts: BTreeMap<AccountId, ContractMock>,
    nonce: u8,
}

impl<AccountId: Ord> MockRegistry<AccountId> {
    /// Creates a new registry.
    pub fn new() -> Self {
        Self {
            mocked_contracts: BTreeMap::new(),
            nonce: 0u8,
        }
    }

    /// Returns the salt for the next contract.
    pub fn salt(&mut self) -> Vec<u8> {
        self.nonce += 1;
        vec![self.nonce]
    }

    /// Registers `mock` for `address`. Returns the previous mock, if any.
    pub fn register(&mut self, address: AccountId, mock: ContractMock) -> Option<ContractMock> {
        self.mocked_contracts.insert(address, mock)
    }

    /// Returns the mock for `address`, if any.
    pub fn get(&self, address: &AccountId) -> Option<&ContractMock> {
        self.mocked_contracts.get(address)
    }
}
