//! Mocking contract feature.

use std::marker::PhantomData;

pub struct MockRegistry<AccountId> {
    mocked_contracts: Vec<ContractMock<AccountId>>,
}

impl<AccountId> MockRegistry<AccountId> {
    pub fn new() -> Self {
        Self {
            mocked_contracts: Vec::new(),
        }
    }

    pub fn register_mock(&mut self, contract: ContractMock<AccountId>) {
        self.mocked_contracts.push(contract);
    }
}

pub struct ContractMock<AccountId> {
    mocked_addresses: Vec<AccountId>,
    mocked_methods: Vec<Box<dyn MethodMockT>>,
}

trait MethodMockT {}
impl<Args, Ret> MethodMockT for MethodMock<Args, Ret> {}

pub struct MethodMock<Args, Ret> {
    selector: [u8; 4],
    matchers: Vec<CallMatcher<Args, Ret>>,
    _phantom: PhantomData<(Args, Ret)>,
}

pub struct CallMatcher<Args, Ret> {
    arg_matcher: Box<dyn Fn(Args) -> bool>,
    ret: Ret,
}
