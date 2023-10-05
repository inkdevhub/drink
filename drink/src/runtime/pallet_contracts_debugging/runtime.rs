use sp_externalities::{decl_extension, ExternalitiesExt};
use sp_runtime_interface::runtime_interface;

/// Contracts pallet outsources debug callbacks through this runtime interface.
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

/// This trait describes the interface of a runtime extension that can be used to debug contract
/// calls.
pub trait DebugExtT {
    /// Called after a contract call is made.
    fn after_call(
        &self,
        _contract_address: Vec<u8>,
        _is_call: bool,
        _input_data: Vec<u8>,
        _result: Vec<u8>,
    ) {
    }
}

decl_extension! {
    /// A wrapper type for the `DebugExtT` debug extension.
    pub struct DebugExt(Box<dyn DebugExtT + Send>);
}

/// The simplest debug extension - does nothing.
pub struct NoopDebugExt {}
impl DebugExtT for NoopDebugExt {}
