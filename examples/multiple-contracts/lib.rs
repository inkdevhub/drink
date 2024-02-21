#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod balance_checker {
    use ink::env::{
        call::{build_call, ExecutionInput, Selector},
        DefaultEnvironment,
    };

    #[ink(storage)]
    pub struct BalanceChecker {
        account: AccountId,
        token_contract: AccountId,
    }

    impl BalanceChecker {
        #[ink(constructor)]
        pub fn new(account: AccountId, token_contract: AccountId) -> Self {
            Self {
                account,
                token_contract,
            }
        }

        #[ink(message)]
        pub fn check(&self) -> u128 {
            build_call::<DefaultEnvironment>()
                .call_v1(self.token_contract)
                .transferred_value(0)
                .exec_input(
                    ExecutionInput::new(Selector::new(ink::selector_bytes!("PSP22::balance_of")))
                        .push_arg(self.account),
                )
                .returns::<u128>()
                .invoke()
        }
    }
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use drink::session::{Session, NO_ARGS, NO_ENDOWMENT, NO_SALT};

    #[drink::contract_bundle_provider]
    enum BundleProvider {}

    #[drink::test]
    fn compile(mut session: Session) -> Result<(), Box<dyn Error>> {
        let contract = BundleProvider::local()?;
        Ok(())
    }
}
