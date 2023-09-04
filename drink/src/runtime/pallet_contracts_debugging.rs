use pallet_contracts::debug::{CallSpan, ExportedFunction, Tracing};
use pallet_contracts_primitives::ExecReturnValue;

use crate::runtime::Runtime;

type CodeHash<R> = <R as frame_system::Config>::Hash;

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

impl<CodeHash> CallSpan for DrinkCallSpan<CodeHash> {
    fn after_call(self, output: &ExecReturnValue) {
        todo!()
    }
}
