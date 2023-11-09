//! Mocking API for the sandbox.
use crate::{
    mock::ContractMock,
    runtime::{AccountIdFor, Runtime},
    Sandbox, DEFAULT_GAS_LIMIT,
};

impl<R: Runtime + pallet_contracts::Config> Sandbox<R> {
    /// Deploy `mock` as a standard contract. Returns the address of the deployed contract.
    pub fn deploy(&mut self, mock: ContractMock) -> AccountIdFor<R> {
        // We have to deploy some contract. We use a dummy contract for that. Thanks to that, we
        // ensure that the pallet will treat our mock just as a regular contract, until we actually
        // call it.
        let mock_bytes = wat::parse_str(DUMMY_CONTRACT).expect("Dummy contract should be valid");
        let mock_address = self
            .deploy_contract(
                mock_bytes,
                0u32.into(),
                vec![],
                // We have to use a different account ID for each contract.
                vec![self.mock_counter as u8],
                R::default_actor(),
                DEFAULT_GAS_LIMIT,
                None,
            )
            .result
            .expect("Deployment of a dummy contract should succeed")
            .account_id;

        self.mock_counter += 1;
        self.mock_registry
            .lock()
            .expect("Should be able to acquire lock on registry")
            .register(mock_address.clone(), mock);

        mock_address
    }

    /// Mock part of an existing contract. In particular, allows to override real behavior of
    /// deployed contract's messages.
    pub fn mock_existing_contract(&mut self, _mock: ContractMock, _address: AccountIdFor<R>) {
        todo!("soon")
    }
}

/// A dummy contract that is used to deploy a mock.
///
/// Has a single noop constructor and a single panicking message.
const DUMMY_CONTRACT: &str = r#"
(module
	(import "env" "memory" (memory 1 1))
	(func (export "deploy"))
	(func (export "call") (unreachable))
)"#;
