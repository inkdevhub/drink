use crate::{
    mock::ContractMock,
    runtime::{AccountIdFor, Runtime},
    Sandbox,
};

pub trait MockingApi<R: Runtime> {
    /// Register a new contract mock.
    fn register_mock(&mut self, mock: ContractMock<AccountIdFor<R>>);
}

impl<R: Runtime> MockingApi<R> for Sandbox<R> {
    fn register_mock(&mut self, mock: ContractMock<AccountIdFor<R>>) {
        self.mock_registry.register_mock(mock);
    }
}
