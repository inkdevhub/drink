use crate::{
    mock::ContractMock,
    runtime::{AccountIdFor, Runtime},
    Sandbox,
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
        todo!()
    }

    fn mock_existing_contract(&mut self, mock: ContractMock, address: AccountIdFor<R>) {
        todo!("soon")
    }
}
