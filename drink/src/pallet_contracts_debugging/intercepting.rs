use ink_sandbox::AccountIdFor;
use parity_scale_codec::{Decode, Encode};

use crate::{
    pallet_contracts::{
        debug::{CallInterceptor, ExecResult, ExportedFunction},
        Config,
    },
    pallet_contracts_debugging::{runtime::contract_call_debugger, DrinkDebug},
};

impl<R: Config> CallInterceptor<R> for DrinkDebug {
    fn intercept_call(
        contract_address: &AccountIdFor<R>,
        entry_point: &ExportedFunction,
        input_data: &[u8],
    ) -> Option<ExecResult> {
        // Pass the data to the runtime interface. The data must be encoded (only simple types are
        // supported).
        contract_call_debugger::intercept_call(
            contract_address.encode(),
            matches!(*entry_point, ExportedFunction::Call),
            input_data.to_vec(),
        )
        .and_then(|intercepting_result| {
            Decode::decode(&mut intercepting_result.as_slice()).expect("Decoding should succeed")
        })
    }
}
