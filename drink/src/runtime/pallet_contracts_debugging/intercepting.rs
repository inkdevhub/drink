use pallet_contracts::debug::{CallInterceptor, ExecResult, ExportedFunction};

use crate::runtime::{pallet_contracts_debugging::DrinkDebug, AccountIdFor, Runtime};

impl<R: Runtime> CallInterceptor<R> for DrinkDebug {
    fn intercept_call(
        _contract_address: &AccountIdFor<R>,
        _entry_point: &ExportedFunction,
        _input_data: &[u8],
    ) -> Option<ExecResult> {
        // We don't want to intercept any calls. At least for now.
        None
    }
}
