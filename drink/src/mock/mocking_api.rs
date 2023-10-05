use crate::{
    contract_api::ContractApi,
    mock::ContractMock,
    runtime::{AccountIdFor, Runtime},
    Sandbox, DEFAULT_GAS_LIMIT,
};

pub trait MockingApi<R: Runtime> {
    /// Deploy `mock` as a standard contract. Returns the address of the deployed contract.
    fn deploy_mock(&mut self, mock: ContractMock) -> AccountIdFor<R>;

    /// Mock part of an existing contract. In particular, allows to override real behavior of
    /// deployed contract's messages.
    fn mock_existing_contract(&mut self, mock: ContractMock, address: AccountIdFor<R>);
}

impl<R: Runtime> MockingApi<R> for Sandbox<R> {
    fn deploy_mock(&mut self, mock: ContractMock) -> AccountIdFor<R> {
        let mock_bytes = wat::parse_str(DUMMY_CONTRACT).expect("Dummy contract should be valid");
        let mock_address = self
            .deploy_contract(
                mock_bytes,
                0,
                vec![],
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
            .as_ref()
            .lock()
            .expect("Should be able to acquire lock on registry")
            .register(mock_address.clone(), mock);

        mock_address
    }

    fn mock_existing_contract(&mut self, mock: ContractMock, address: AccountIdFor<R>) {
        todo!("soon")
    }
}

const DUMMY_CONTRACT: &str = r#"
(module
	(import "env" "memory" (memory 1 1))
	(func (export "deploy"))
	(func (export "call") (unreachable))
)"#;
