use parity_scale_codec::Encode;
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
        self.extension::<TracingExt>()
            .expect("Failed to find `DebugExt` extension")
            .after_call(contract_address, is_call, input_data, result);
    }

    fn intercept_call(
        &mut self,
        contract_address: Vec<u8>,
        is_call: bool,
        input_data: Vec<u8>,
    ) -> Vec<u8> {
        self.extension::<InterceptingExt>()
            .expect("Failed to find `InterceptingExt` extension")
            .intercept_call(contract_address, is_call, input_data)
    }
}

/// This trait describes the interface of a runtime extension that can be used to debug contract
/// calls.
pub trait TracingExtT {
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
    /// A wrapper type for the `TracingExtT` debug extension.
    pub struct TracingExt(Box<dyn TracingExtT + Send>);
}

/// This trait describes the interface of a runtime extension that can be used to intercept contract
/// calls.
pub trait InterceptingExtT {
    /// Called when a contract call is made.
    fn intercept_call(
        &self,
        _contract_address: Vec<u8>,
        _is_call: bool,
        _input_data: Vec<u8>,
    ) -> Vec<u8> {
        None::<()>.encode() // do not intercept, continue standard procedure
    }
}

decl_extension! {
    /// A wrapper type for the `InterceptingExtT` debug extension.
    pub struct InterceptingExt(Box<dyn InterceptingExtT + Send>);
}

/// The simplest extension - uses default implementation.
pub struct NoopExt {}
impl TracingExtT for NoopExt {}
impl InterceptingExtT for NoopExt {}
