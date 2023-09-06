//! This module provides all the necessary elements for supporting contract debugging directly in
//! the contracts pallet.
//!
//! # Smart-contract developer <-> pallet-contracts interaction flow
//!
//! The interaction between end-user and runtime is as follows:
//! 1. At some points during execution, the pallet invokes some callback through its configuration
//! parameter `Debug`.
//! 2. In order to forward the callback outside the runtime, `Debug` will call a runtime interface,
//! that will then forward the call further to the proper runtime extension.
//! 3. The runtime extension can be fully controlled by the end-user. It just has to be registered
//! in the runtime.
//!
//! So, in brief: pallet-contracts -> runtime interface -> runtime extension
//!              |<-----------runtime side-------------->|<---user side--->|
//!
//! # Passing objects between runtime and runtime extension
//!
//! Unfortunately, runtime interface that lies between runtime and the end-user accepts only
//! very simple argument types and those that implement some specific traits. This means that
//! usually, complex objects will be passed in their encoded form (`Vec<u8>` obtained with scale
//! encoding).

use pallet_contracts::debug::{CallSpan, ExportedFunction, Tracing};
use pallet_contracts_primitives::ExecReturnValue;
use sp_externalities::{decl_extension, ExternalitiesExt};
use sp_runtime_interface::runtime_interface;

use crate::runtime::{AccountId, Runtime};

/// The trait that allows injecting custom logic to handle contract debugging directly in the
/// contracts pallet.
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

#[runtime_interface]
trait ContractCallDebugger {
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

/// Configuration parameter for the contracts pallet. Provides all the necessary trait
/// implementations.
pub enum DrinkDebug {}

impl<R: Runtime> Tracing<R> for DrinkDebug {
    type CallSpan = DrinkCallSpan<AccountId<R>>;

    fn new_call_span(
        contract_address: &AccountId<R>,
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
        contract_call_debugger::after_call(
            self.contract_address.encode(),
            matches!(self.entry_point, ExportedFunction::Call),
            self.input_data.to_vec(),
            output.data.clone(),
        );
    }
}
