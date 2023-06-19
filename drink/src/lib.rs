mod runtime;

use std::fmt::Display;

use frame_support::weights::Weight;
use pallet_contracts::Determinism;
use pallet_contracts_primitives::Code;
use sp_io::TestExternalities;
use sp_runtime::AccountId32;

use crate::runtime::*;

pub struct Sandbox {
    externalities: TestExternalities,
}

pub const ALICE: AccountId32 = AccountId32::new([1u8; 32]);
pub const GAS_LIMIT: Weight = Weight::from_parts(100_000_000_000, 3 * 1024 * 1024);

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct CallResult {
    pub result: Vec<u8>,
    pub debug_message: Vec<String>,

    pub gas_consumed: Weight,
    pub gas_required: Weight,
}

impl Display for CallResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Gas consumed: {:?}", self.gas_consumed)?;
        writeln!(f, "Gas required: {:?}", self.gas_required)?;
        writeln!(f, "Result: {:?}", self.result)?;
        writeln!(f, "Debug buffer:")?;
        for line in &self.debug_message {
            writeln!(f, "  {line}")?;
        }
        Ok(())
    }
}

impl Sandbox {
    pub fn new() -> Self {
        let mut storage = frame_system::GenesisConfig::default()
            .build_storage::<SandboxRuntime>()
            .unwrap();
        pallet_balances::GenesisConfig::<SandboxRuntime> {
            balances: vec![(ALICE, 1_000_000_000_000_000)],
        }
        .assimilate_storage(&mut storage)
        .unwrap();

        Self {
            externalities: TestExternalities::new(storage),
        }
    }

    pub fn deploy_contract(&mut self, contract_bytes: Vec<u8>) -> AccountId32 {
        self.externalities.execute_with(|| {
            let result = Contracts::bare_instantiate(
                ALICE,
                0,
                GAS_LIMIT,
                None,
                Code::Upload(contract_bytes),
                vec![155, 174, 157, 94],
                Default::default(),
                true,
            );
            let result = result.result.unwrap();
            assert!(!result.result.did_revert());
            result.account_id
        })
    }

    pub fn call_contract(&mut self, address: AccountId32, msg: String) -> CallResult {
        let msg = match msg.as_str() {
            "flip" => vec![99, 58, 165, 81],
            "get" => vec![47, 134, 91, 217],
            _ => panic!("Invalid message"),
        };

        self.externalities.execute_with(|| {
            let main_result = Contracts::bare_call(
                ALICE,
                address,
                0,
                GAS_LIMIT,
                None,
                msg,
                true,
                Determinism::Deterministic,
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
