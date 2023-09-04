use pallet_contracts::debug::{CallSpan, ExportedFunction, Tracing};
use pallet_contracts_primitives::ExecReturnValue;
use sp_externalities::{decl_extension, ExternalitiesExt};
use sp_runtime_interface::runtime_interface;

use crate::runtime::Runtime;

type CodeHash<R> = <R as frame_system::Config>::Hash;

pub trait DebugExtT {
    fn after_call(&self, code_hash: Vec<u8>, is_call: bool, input_data: Vec<u8>, result: Vec<u8>) {}
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
        code_hash: Vec<u8>,
        is_call: bool,
        input_data: Vec<u8>,
        result: Vec<u8>,
    ) {
        self.extension::<DebugExt>()
            .expect("Failed to find `DebugExt` extension")
            .after_call(code_hash, is_call, input_data, result);
    }
}

pub enum DrinkDebug {}

impl<R: Runtime> Tracing<R> for DrinkDebug {
    type CallSpan = DrinkCallSpan<CodeHash<R>>;

    fn new_call_span(
        code_hash: &CodeHash<R>,
        entry_point: ExportedFunction,
        input_data: &[u8],
    ) -> Self::CallSpan {
        DrinkCallSpan {
            code_hash: *code_hash,
            entry_point,
            input_data: input_data.to_vec(),
        }
    }
}

pub struct DrinkCallSpan<CodeHash> {
    pub code_hash: CodeHash,
    pub entry_point: ExportedFunction,
    pub input_data: Vec<u8>,
}

impl<CodeHash: AsRef<[u8]>> CallSpan for DrinkCallSpan<CodeHash> {
    fn after_call(self, output: &ExecReturnValue) {
        let raw_code_hash: &[u8] = self.code_hash.as_ref();
        contract_call_debugger::after_call(
            raw_code_hash.to_vec(),
            matches!(self.entry_point, ExportedFunction::Call),
            self.input_data.to_vec(),
            output.data.clone(),
        );
    }
}
