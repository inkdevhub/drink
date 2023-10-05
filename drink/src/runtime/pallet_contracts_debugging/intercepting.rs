use pallet_contracts::debug::{CallInterceptor, ExecResult, ExportedFunction};
use parity_scale_codec::{Decode, Encode};

use crate::runtime::{
    pallet_contracts_debugging::{runtime::contract_call_debugger, DrinkDebug},
    AccountIdFor, Runtime,
};

impl<R: Runtime> CallInterceptor<R> for DrinkDebug {
    fn intercept_call(
        contract_address: &AccountIdFor<R>,
        entry_point: &ExportedFunction,
        input_data: &[u8],
    ) -> Option<ExecResult> {
        // Pass the data to the runtime interface. The data must be encoded (only simple types are
        // supported).
        let intercepting_result = contract_call_debugger::intercept_call(
            contract_address.encode(),
            matches!(*entry_point, ExportedFunction::Call),
            input_data.to_vec(),
        );

        Decode::decode(&mut intercepting_result.as_slice()).expect("Decoding should succeed")
    }
}
