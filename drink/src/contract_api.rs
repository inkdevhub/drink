use frame_support::{sp_runtime::AccountId32, weights::Weight};
use pallet_contracts::Determinism;
use pallet_contracts_primitives::{ContractExecResult, ContractInstantiateResult};

use crate::{runtime::Contracts, Sandbox};

pub trait ContractApi {
    fn deploy_contract(
        &mut self,
        contract_bytes: Vec<u8>,
        data: Vec<u8>,
        salt: Vec<u8>,
        origin: AccountId32,
        gas_limit: Weight,
    ) -> ContractInstantiateResult<AccountId32, u128>;

    fn call_contract(
        &mut self,
        address: AccountId32,
        data: Vec<u8>,
        origin: AccountId32,
        gas_limit: Weight,
    ) -> ContractExecResult<u128>;
}

pub const GAS_LIMIT: Weight = Weight::from_parts(100_000_000_000, 3 * 1024 * 1024);

impl ContractApi for Sandbox {
    fn deploy_contract(
        &mut self,
        contract_bytes: Vec<u8>,
        data: Vec<u8>,
        salt: Vec<u8>,
        origin: AccountId32,
        gas_limit: Weight,
    ) -> ContractInstantiateResult<AccountId32, u128> {
        self.externalities.execute_with(|| {
            Contracts::bare_instantiate(
                origin,
                0,
                gas_limit,
                None,
                contract_bytes.into(),
                data,
                salt,
                true,
            )
        })
    }

    fn call_contract(
        &mut self,
        address: AccountId32,
        data: Vec<u8>,
        origin: AccountId32,
        gas_limit: Weight,
    ) -> ContractExecResult<u128> {
        self.externalities.execute_with(|| {
            Contracts::bare_call(
                origin,
                address,
                0,
                gas_limit,
                None,
                data,
                true,
                Determinism::Enforced,
            )
        })
    }
}

pub fn decode_debug_buffer(buffer: &[u8]) -> Vec<String> {
    let decoded = buffer.iter().map(|b| *b as char).collect::<String>();
    decoded.split('\n').map(|s| s.to_string()).collect()
}
