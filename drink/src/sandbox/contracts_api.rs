//! Contracts API for the sandbox.
use std::ops::Not;

use frame_support::weights::Weight;
use frame_system::Config as SysConfig;
use pallet_contracts::{
    Code, CodeUploadResult, CollectEvents, ContractInstantiateResult, DebugInfo, Determinism,
};
use parity_scale_codec::Decode as _;

use crate::{
    runtime::AccountIdFor, BalanceOf, ContractExecResultFor, ContractInstantiateResultFor,
    EventRecordOf, Sandbox,
};

impl<Config: crate::SandboxConfig> Sandbox<Config>
where
    Config::Runtime: pallet_contracts::Config,
{
    /// Interface for `bare_instantiate` contract call with a simultaneous upload.
    ///
    /// # Arguments
    ///
    /// * `contract_bytes` - The contract code.
    /// * `value` - The number of tokens to be transferred to the contract.
    /// * `data` - The input data to be passed to the contract (including constructor name).
    /// * `salt` - The salt to be used for contract address derivation.
    /// * `origin` - The sender of the contract call.
    /// * `gas_limit` - The gas limit for the contract call.
    /// * `storage_deposit_limit` - The storage deposit limit for the contract call.
    #[allow(clippy::type_complexity, clippy::too_many_arguments)]
    pub fn deploy_contract(
        &mut self,
        contract_bytes: Vec<u8>,
        value: BalanceOf<Config::Runtime>,
        data: Vec<u8>,
        salt: Vec<u8>,
        origin: AccountIdFor<Config::Runtime>,
        gas_limit: Weight,
        storage_deposit_limit: Option<BalanceOf<Config::Runtime>>,
    ) -> ContractInstantiateResultFor<Config::Runtime> {
        self.externalities.execute_with(|| {
            pallet_contracts::Pallet::<Config::Runtime>::bare_instantiate(
                origin,
                value,
                gas_limit,
                storage_deposit_limit,
                Code::Upload(contract_bytes),
                data,
                salt,
                DebugInfo::UnsafeDebug,
                CollectEvents::UnsafeCollect,
            )
        })
    }

    /// Interface for `bare_instantiate` contract call for a previously uploaded contract.
    ///
    /// # Arguments
    ///
    /// * `code_hash` - The code hash of the contract to instantiate.
    /// * `value` - The number of tokens to be transferred to the contract.
    /// * `data` - The input data to be passed to the contract (including constructor name).
    /// * `salt` - The salt to be used for contract address derivation.
    /// * `origin` - The sender of the contract call.
    /// * `gas_limit` - The gas limit for the contract call.
    /// * `storage_deposit_limit` - The storage deposit limit for the contract call.
    #[allow(clippy::type_complexity, clippy::too_many_arguments)]
    pub fn instantiate_contract(
        &mut self,
        code_hash: Vec<u8>,
        value: BalanceOf<Config::Runtime>,
        data: Vec<u8>,
        salt: Vec<u8>,
        origin: AccountIdFor<Config::Runtime>,
        gas_limit: Weight,
        storage_deposit_limit: Option<BalanceOf<Config::Runtime>>,
    ) -> ContractInstantiateResult<
        AccountIdFor<Config::Runtime>,
        BalanceOf<Config::Runtime>,
        EventRecordOf<Config::Runtime>,
    > {
        let mut code_hash = &code_hash[..];
        self.externalities.execute_with(|| {
            pallet_contracts::Pallet::<Config::Runtime>::bare_instantiate(
                origin,
                value,
                gas_limit,
                storage_deposit_limit,
                Code::Existing(
                    <Config::Runtime as SysConfig>::Hash::decode(&mut code_hash)
                        .expect("Invalid code hash"),
                ),
                data,
                salt,
                DebugInfo::UnsafeDebug,
                CollectEvents::UnsafeCollect,
            )
        })
    }

    /// Interface for `bare_upload_code` contract call.
    ///
    /// # Arguments
    ///
    /// * `contract_bytes` - The contract code.
    /// * `origin` - The sender of the contract call.
    /// * `storage_deposit_limit` - The storage deposit limit for the contract call.
    pub fn upload_contract(
        &mut self,
        contract_bytes: Vec<u8>,
        origin: AccountIdFor<Config::Runtime>,
        storage_deposit_limit: Option<BalanceOf<Config::Runtime>>,
        determinism: Determinism,
    ) -> CodeUploadResult<<Config::Runtime as frame_system::Config>::Hash, BalanceOf<Config::Runtime>>
    {
        self.externalities.execute_with(|| {
            pallet_contracts::Pallet::<Config::Runtime>::bare_upload_code(
                origin,
                contract_bytes,
                storage_deposit_limit,
                determinism,
            )
        })
    }

    /// Interface for `bare_call` contract call.
    ///
    /// # Arguments
    ///
    /// * `address` - The address of the contract to be called.
    /// * `value` - The number of tokens to be transferred to the contract.
    /// * `data` - The input data to be passed to the contract (including message name).
    /// * `origin` - The sender of the contract call.
    /// * `gas_limit` - The gas limit for the contract call.
    /// * `storage_deposit_limit` - The storage deposit limit for the contract call.
    #[allow(clippy::too_many_arguments)]
    pub fn call_contract(
        &mut self,
        address: AccountIdFor<Config::Runtime>,
        value: BalanceOf<Config::Runtime>,
        data: Vec<u8>,
        origin: AccountIdFor<Config::Runtime>,
        gas_limit: Weight,
        storage_deposit_limit: Option<BalanceOf<Config::Runtime>>,
        determinism: Determinism,
    ) -> ContractExecResultFor<Config::Runtime> {
        self.externalities.execute_with(|| {
            pallet_contracts::Pallet::<Config::Runtime>::bare_call(
                origin,
                address,
                value,
                gas_limit,
                storage_deposit_limit,
                data,
                DebugInfo::UnsafeDebug,
                CollectEvents::UnsafeCollect,
                determinism,
            )
        })
    }
}

/// Converts bytes to a '\n'-split string, ignoring empty lines.
pub fn decode_debug_buffer(buffer: &[u8]) -> Vec<String> {
    let decoded = buffer.iter().map(|b| *b as char).collect::<String>();
    decoded
        .split('\n')
        .filter_map(|s| s.is_empty().not().then_some(s.to_string()))
        .collect()
}

#[cfg(test)]
mod tests {
    use frame_support::sp_runtime::traits::Hash;
    use pallet_contracts::Origin;

    use super::*;
    use crate::{
        minimal::RuntimeEvent, sandbox::SandboxConfig, session::NO_SALT, MinimalRuntime,
        DEFAULT_GAS_LIMIT,
    };

    fn compile_module(contract_name: &str) -> Vec<u8> {
        let path = [
            std::env::var("CARGO_MANIFEST_DIR")
                .as_deref()
                .unwrap_or("drink"),
            "/test-resources/",
            contract_name,
            ".wat",
        ]
        .concat();
        wat::parse_file(path).expect("Failed to parse wat file")
    }

    #[test]
    fn can_upload_code() {
        let mut sandbox = Sandbox::<MinimalRuntime>::new().unwrap();
        let wasm_binary = compile_module("dummy");
        let hash = <<MinimalRuntime as frame_system::Config>::Hashing>::hash(&wasm_binary);

        let result = sandbox.upload_contract(
            wasm_binary,
            MinimalRuntime::default_actor(),
            None,
            Determinism::Enforced,
        );

        assert!(result.is_ok());
        assert_eq!(hash, result.unwrap().code_hash);
    }

    #[test]
    fn can_deploy_contract() {
        let mut sandbox = Sandbox::<MinimalRuntime>::new().unwrap();
        let wasm_binary = compile_module("dummy");

        let events_before = sandbox.events();
        assert!(events_before.is_empty());

        let result = sandbox.deploy_contract(
            wasm_binary,
            0,
            vec![],
            NO_SALT,
            MinimalRuntime::default_actor(),
            DEFAULT_GAS_LIMIT,
            None,
        );
        assert!(result.result.is_ok());
        assert!(!result.result.unwrap().result.did_revert());

        let events = result.events.expect("Drink should collect events");
        let event_count = events.len();
        let instantiation_event = events[event_count - 2].clone();
        assert!(matches!(
            instantiation_event.event,
            RuntimeEvent::Contracts(pallet_contracts::Event::<MinimalRuntime>::Instantiated { .. })
        ));
        let deposit_event = events[event_count - 1].clone();
        assert!(matches!(
            deposit_event.event,
            RuntimeEvent::Contracts(
                pallet_contracts::Event::<MinimalRuntime>::StorageDepositTransferredAndHeld { .. }
            )
        ));
    }

    #[test]
    fn can_call_contract() {
        let mut sandbox = Sandbox::<MinimalRuntime>::new().unwrap();
        let actor = MinimalRuntime::default_actor();
        let wasm_binary = compile_module("dummy");

        let result = sandbox.deploy_contract(
            wasm_binary,
            0,
            vec![],
            NO_SALT,
            actor.clone(),
            DEFAULT_GAS_LIMIT,
            None,
        );

        let contract_address = result
            .result
            .expect("Contract should be deployed")
            .account_id;

        sandbox.reset_events();

        let result = sandbox.call_contract(
            contract_address.clone(),
            0,
            vec![],
            actor.clone(),
            DEFAULT_GAS_LIMIT,
            None,
            Determinism::Enforced,
        );
        assert!(result.result.is_ok());
        assert!(!result.result.unwrap().did_revert());

        let events = result.events.expect("Drink should collect events");
        assert_eq!(events.len(), 2);

        assert_eq!(
            events[0].event,
            RuntimeEvent::Contracts(pallet_contracts::Event::<MinimalRuntime>::ContractEmitted {
                contract: contract_address.clone(),
                data: vec![0, 0, 0, 0],
            })
        );

        assert_eq!(
            events[1].event,
            RuntimeEvent::Contracts(pallet_contracts::Event::<MinimalRuntime>::Called {
                contract: contract_address,
                caller: Origin::Signed(actor),
            }),
        );
    }
}
