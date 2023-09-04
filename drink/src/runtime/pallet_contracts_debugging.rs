use pallet_contracts::debug::{CallSpan, ExportedFunction, Tracing};
use pallet_contracts_primitives::ExecReturnValue;
use sp_externalities::{decl_extension, ExternalitiesExt};
use sp_runtime_interface::runtime_interface;

use crate::runtime::Runtime;

type AccountIdOf<R> = <R as frame_system::Config>::AccountId;

pub trait DebugExtT {
    fn after_call(
        &self,
        contract_address: Vec<u8>,
        is_call: bool,
        input_data: Vec<u8>,
        result: Vec<u8>,
    ) {
    }
}

decl_extension! {
    pub struct DebugExt(Box<dyn DebugExtT + Send>);
}

pub struct NoopDebugExt {}
impl DebugExtT for NoopDebugExt {}

#[runtime_interface]
pub trait ContractCallDebugger {
    fn after_call(
        &mut self,
        contract_address: Vec<u8>,
        is_call: bool,
        input_data: Vec<u8>,
        result: Vec<u8>,
    ) {
        self.extension::<DebugExt>()
            .expect("Failed to find `DebugExt` extension")
            .after_call(contract_address, is_call, input_data, result);
    }
}

pub enum DrinkDebug {}

impl<R: Runtime> Tracing<R> for DrinkDebug {
    type CallSpan = DrinkCallSpan<AccountIdOf<R>>;

    fn new_call_span(
        contract_address: &AccountIdOf<R>,
        entry_point: ExportedFunction,
        input_data: &[u8],
    ) -> Self::CallSpan {
        DrinkCallSpan {
            contract_address: contract_address.clone(),
            entry_point,
            input_data: input_data.to_vec(),
        }
    }
}

pub struct DrinkCallSpan<AccountId> {
    pub contract_address: AccountId,
    pub entry_point: ExportedFunction,
    pub input_data: Vec<u8>,
}

impl<AccountId: AsRef<[u8]>> CallSpan for DrinkCallSpan<AccountId> {
    fn after_call(self, output: &ExecReturnValue) {
        let raw_contract_address: &[u8] = self.contract_address.as_ref();
        contract_call_debugger::after_call(
            raw_contract_address.to_vec(),
            matches!(self.entry_point, ExportedFunction::Call),
            self.input_data.to_vec(),
            output.data.clone(),
        );
    }
}
