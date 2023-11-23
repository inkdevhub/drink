#![cfg_attr(not(feature = "std"), no_std, no_main)]

mod chain_extension_ink_side;
#[cfg(test)]
mod chain_extension_runtime_side;

#[ink::contract(env = StakingEnvironment)]
mod contract_calling_chain_extension {
    use crate::chain_extension_ink_side::StakingEnvironment;

    #[ink(storage)]
    pub struct ContractCallingChainExtension {}

    impl ContractCallingChainExtension {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {}
        }

        #[ink(message)]
        pub fn call_ce(&self) {
            self.env().extension().get_num_of_validators().unwrap();
        }
    }
}

#[cfg(test)]
mod tests {}
