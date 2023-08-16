//! Contracts API.

use frame_support::{sp_runtime::AccountId32, weights::Weight};
use pallet_contracts::{CollectEvents, DebugInfo, Determinism};
use pallet_contracts_primitives::{Code, ContractExecResult, ContractInstantiateResult};

use crate::{runtime::Runtime, EventRecordOf, Sandbox};

/// Interface for contract-related operations.
pub trait ContractApi<R: Runtime> {
    /// Interface for `bare_instantiate` contract call.
    fn deploy_contract(
        &mut self,
        contract_bytes: Vec<u8>,
        data: Vec<u8>,
        salt: Vec<u8>,
        origin: AccountId32,
        gas_limit: Weight,
    ) -> ContractInstantiateResult<AccountId32, u128, EventRecordOf<R>>;

    /// Interface for `bare_call` contract call.
    fn call_contract(
        &mut self,
        address: AccountId32,
        data: Vec<u8>,
        origin: AccountId32,
        gas_limit: Weight,
    ) -> ContractExecResult<u128, EventRecordOf<R>>;
}

impl<R: Runtime> ContractApi<R> for Sandbox<R> {
    fn deploy_contract(
        &mut self,
        contract_bytes: Vec<u8>,
        data: Vec<u8>,
        salt: Vec<u8>,
        origin: AccountId32,
        gas_limit: Weight,
    ) -> ContractInstantiateResult<AccountId32, u128, EventRecordOf<R>> {
        self.externalities.execute_with(|| {
            pallet_contracts::Pallet::<R>::bare_instantiate(
                origin,
                0,
                gas_limit,
                None,
                Code::Upload(contract_bytes),
                data,
                salt,
                DebugInfo::Skip,
                CollectEvents::Skip,
            )
        })
    }

    fn call_contract(
        &mut self,
        address: AccountId32,
        data: Vec<u8>,
        origin: AccountId32,
        gas_limit: Weight,
    ) -> ContractExecResult<u128, EventRecordOf<R>> {
        self.externalities.execute_with(|| {
            pallet_contracts::Pallet::<R>::bare_call(
                origin,
                address,
                0,
                gas_limit,
                None,
                data,
                DebugInfo::Skip,
                CollectEvents::Skip,
                Determinism::Enforced,
            )
        })
    }
}

/// Converts bytes to a '\n'-split string.
pub fn decode_debug_buffer(buffer: &[u8]) -> Vec<String> {
    let decoded = buffer.iter().map(|b| *b as char).collect::<String>();
    decoded.split('\n').map(|s| s.to_string()).collect()
}
