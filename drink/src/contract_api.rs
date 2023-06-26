use frame_support::{sp_runtime::AccountId32, weights::Weight};
use pallet_contracts::Determinism;

use crate::{runtime::Contracts, CallResult, Sandbox, ALICE};

pub trait ContractApi {
    fn deploy_contract(
        &mut self,
        contract_bytes: Vec<u8>,
        selector: Vec<u8>,
        salt: Vec<u8>,
    ) -> AccountId32;

    fn call_contract(&mut self, address: AccountId32, selector: Vec<u8>) -> CallResult;
}

pub const GAS_LIMIT: Weight = Weight::from_parts(100_000_000_000, 3 * 1024 * 1024);

impl ContractApi for Sandbox {
    fn deploy_contract(
        &mut self,
        contract_bytes: Vec<u8>,
        selector: Vec<u8>,
        salt: Vec<u8>,
    ) -> AccountId32 {
        self.externalities.execute_with(|| {
            let result = Contracts::bare_instantiate(
                ALICE,
                0,
                GAS_LIMIT,
                None,
                contract_bytes.into(),
                selector,
                salt,
                true,
            );
            let result = result.result.unwrap();
            assert!(!result.result.did_revert());
            result.account_id
        })
    }

    fn call_contract(&mut self, address: AccountId32, selector: Vec<u8>) -> CallResult {
        self.externalities.execute_with(|| {
            let main_result = Contracts::bare_call(
                ALICE,
                address,
                0,
                GAS_LIMIT,
                None,
                selector,
                true,
                Determinism::Enforced,
            );
            let execution_result = main_result.result.unwrap();

            assert!(!execution_result.did_revert());

            CallResult {
                result: execution_result.data,
                debug_message: decode_debug_buffer(main_result.debug_message),
                gas_consumed: main_result.gas_consumed,
                gas_required: main_result.gas_required,
            }
        })
    }
}

fn decode_debug_buffer(buffer: Vec<u8>) -> Vec<String> {
    let decoded = buffer.into_iter().map(|b| b as char).collect::<String>();
    decoded.split('\n').map(|s| s.to_string()).collect()
}
