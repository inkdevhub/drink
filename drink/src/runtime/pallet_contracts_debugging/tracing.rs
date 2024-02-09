use pallet_contracts::{
    debug::{CallSpan, ExportedFunction},
    ExecReturnValue, Tracing,
};

use crate::runtime::{pallet_contracts_debugging::DrinkDebug, AccountIdFor};

impl<R: pallet_contracts::Config> Tracing<R> for DrinkDebug {
    type CallSpan = DrinkCallSpan<AccountIdFor<R>>;

    fn new_call_span(
        contract_address: &AccountIdFor<R>,
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

/// A contract's call span.
///
/// It is created just before the call is made and `Self::after_call` is called after the call is
/// done.
pub struct DrinkCallSpan<AccountId> {
    /// The address of the contract that has been called.
    pub contract_address: AccountId,
    /// The entry point that has been called (either constructor or call).
    pub entry_point: ExportedFunction,
    /// The input data of the call.
    pub input_data: Vec<u8>,
}

impl<AccountId: parity_scale_codec::Encode> CallSpan for DrinkCallSpan<AccountId> {
    fn after_call(self, output: &ExecReturnValue) {
        crate::runtime::pallet_contracts_debugging::runtime::contract_call_debugger::after_call(
            self.contract_address.encode(),
            matches!(self.entry_point, ExportedFunction::Call),
            self.input_data.to_vec(),
            output.data.clone(),
        );
    }
}
