//! This module provides a context-aware interface for interacting with contracts.

use std::{mem, rc::Rc, fmt::Debug};

pub use contract_transcode;
use contract_transcode::ContractMessageTranscoder;
use frame_support::{sp_runtime::DispatchError, weights::Weight};
use pallet_contracts_primitives::{ContractExecResult, ContractInstantiateResult};
use parity_scale_codec::Decode;
use thiserror::Error;

use crate::{
    chain_api::ChainApi,
    contract_api::ContractApi,
    pallet_contracts_debugging::DebugExt,
    runtime::{AccountIdFor, Runtime},
    EventRecordOf, Sandbox, DEFAULT_GAS_LIMIT,
};

type Balance = u128;

const ZERO_TRANSFER: Balance = 0;
const DEFAULT_STORAGE_DEPOSIT_LIMIT: Option<Balance> = None;

/// Convenient value for an empty sequence of call/instantiation arguments.
///
/// Without it, you would have to specify explicitly a compatible type, like:
/// `session.call::<String>(.., &[], ..)`.
pub const NO_ARGS: &[String] = &[];

/// Session specific errors.
#[derive(Error, Debug)]
pub enum SessionError {
    /// Encoding data failed.
    #[error("Encoding call data failed: {0}")]
    Encoding(String),
    /// Decoding data failed.
    #[error("Decoding call data failed: {0}")]
    Decoding(String),
    /// Crate-specific error.
    #[error("{0:?}")]
    Drink(#[from] crate::Error),
    /// Deployment has been reverted by the contract.
    #[error("Contract deployment has been reverted")]
    DeploymentReverted,
    /// Deployment failed (aborted by the pallet).
    #[error("Contract deployment failed before execution: {0:?}")]
    DeploymentFailed(DispatchError),
    /// Call has been reverted by the contract.
    #[error("Contract call has been reverted")]
    CallReverted,
    /// Contract call failed (aborted by the pallet).
    #[error("Contract call failed before execution: {0:?}")]
    CallFailed(DispatchError),
    /// There is no deployed contract to call.
    #[error("No deployed contract")]
    NoContract,
    /// There is no transcoder to encode/decode contract messages.
    #[error("Missing transcoder")]
    NoTranscoder,
}

/// Every contract message wraps its return value in `Result<T, LangResult>`. This is the error
/// type.
///
/// Copied from ink primitives.
#[non_exhaustive]
#[repr(u32)]
#[derive(
    Debug,
    Copy,
    Clone,
    PartialEq,
    Eq,
    parity_scale_codec::Encode,
    parity_scale_codec::Decode,
    scale_info::TypeInfo,
    Error,
)]
pub enum LangError {
    /// Failed to read execution input for the dispatchable.
    #[error("Failed to read execution input for the dispatchable.")]
    CouldNotReadInput = 1u32,
}

/// The `Result` type for ink! messages.
pub type MessageResult<T> = Result<T, LangError>;

/// Wrapper around `Sandbox` that provides a convenient API for interacting with multiple contracts.
///
/// Instead of talking with a low-level `Sandbox`, you can use this struct to keep context,
/// including: origin, gas_limit, transcoder and history of results.
///
/// `Session` has two APIs: chain-ish and for singular actions. The first one can be used like:
/// ```rust, no_run
/// # use std::rc::Rc;
/// # use contract_transcode::ContractMessageTranscoder;
/// # use drink::{
/// #   session::Session,
/// #   AccountId32,
/// #   session::NO_ARGS,
/// #   runtime::MinimalRuntime
/// # };
/// #
/// # fn get_transcoder() -> Rc<ContractMessageTranscoder> {
/// #   Rc::new(ContractMessageTranscoder::load("").unwrap())
/// # }
/// # fn contract_bytes() -> Vec<u8> { vec![] }
/// # fn bob() -> AccountId32 { AccountId32::new([0; 32]) }
///
/// # fn main() -> Result<(), drink::session::SessionError> {
///
/// Session::<MinimalRuntime>::new(Some(get_transcoder()))?
///     .deploy_and(contract_bytes(), "new", NO_ARGS, vec![], None)?
///     .call_and("foo", NO_ARGS, None)?
///     .with_actor(bob())
///     .call_and("bar", NO_ARGS, None)?;
/// # Ok(()) }
/// ```
///
/// The second one serves for one-at-a-time actions:
/// ```rust, no_run
/// # use std::rc::Rc;
/// # use contract_transcode::ContractMessageTranscoder;
/// # use drink::{
/// #   session::Session,
/// #   AccountId32,
/// #   runtime::MinimalRuntime,
/// #   session::NO_ARGS
/// # };
/// # fn get_transcoder() -> Rc<ContractMessageTranscoder> {
/// #   Rc::new(ContractMessageTranscoder::load("").unwrap())
/// # }
/// # fn contract_bytes() -> Vec<u8> { vec![] }
/// # fn bob() -> AccountId32 { AccountId32::new([0; 32]) }
///
/// # fn main() -> Result<(), drink::session::SessionError> {
///
/// let mut session = Session::<MinimalRuntime>::new(Some(get_transcoder()))?;
/// let _address = session.deploy(contract_bytes(), "new", NO_ARGS, vec![], None)?;
/// session.call("foo", NO_ARGS, None)?;
/// session.set_actor(bob());
/// session.call("bar", NO_ARGS, None)?;
/// # Ok(()) }
/// ```
pub struct Session<R: Runtime> {
    sandbox: Sandbox<R>,

    actor: AccountIdFor<R>,
    gas_limit: Weight,

    transcoder: Option<Rc<ContractMessageTranscoder>>,

    deploy_results: Vec<ContractInstantiateResult<AccountIdFor<R>, Balance, EventRecordOf<R>>>,
    deploy_returns: Vec<AccountIdFor<R>>,
    call_results: Vec<ContractExecResult<Balance, EventRecordOf<R>>>,
    call_returns: Vec<Vec<u8>>,
}

impl<R: Runtime> Session<R> {
    /// Creates a new `Session` with optional reference to a transcoder.
    pub fn new(transcoder: Option<Rc<ContractMessageTranscoder>>) -> Result<Self, SessionError> {
        Ok(Self {
            sandbox: Sandbox::new().map_err(SessionError::Drink)?,
            actor: R::default_actor(),
            gas_limit: DEFAULT_GAS_LIMIT,
            transcoder,
            deploy_results: vec![],
            deploy_returns: vec![],
            call_results: vec![],
            call_returns: vec![],
        })
    }

    /// Sets a new actor and returns updated `self`.
    pub fn with_actor(self, actor: AccountIdFor<R>) -> Self {
        Self { actor, ..self }
    }

    /// Sets a new actor and returns the old one.
    pub fn set_actor(&mut self, actor: AccountIdFor<R>) -> AccountIdFor<R> {
        mem::replace(&mut self.actor, actor)
    }

    /// Sets a new gas limit and returns updated `self`.
    pub fn with_gas_limit(self, gas_limit: Weight) -> Self {
        Self { gas_limit, ..self }
    }

    /// Sets a new gas limit and returns the old one.
    pub fn set_gas_limit(&mut self, gas_limit: Weight) -> Weight {
        mem::replace(&mut self.gas_limit, gas_limit)
    }

    /// Sets a new transcoder and returns updated `self`.
    pub fn with_transcoder(self, transcoder: Option<Rc<ContractMessageTranscoder>>) -> Self {
        Self { transcoder, ..self }
    }

    /// Sets a new transcoder and returns the old one.
    pub fn set_transcoder(
        &mut self,
        transcoder: Option<Rc<ContractMessageTranscoder>>,
    ) -> Option<Rc<ContractMessageTranscoder>> {
        mem::replace(&mut self.transcoder, transcoder)
    }

    /// Returns a reference for basic chain API.
    pub fn chain_api(&mut self) -> &mut impl ChainApi<R> {
        &mut self.sandbox
    }

    /// Returns a reference for basic contracts API.
    pub fn contracts_api(&mut self) -> &mut impl ContractApi<R> {
        &mut self.sandbox
    }

    /// Deploys a contract with a given constructor, arguments, salt and endowment. In case of
    /// success, returns `self`.
    pub fn deploy_and<S: AsRef<str> + Debug>(
        mut self,
        contract_bytes: Vec<u8>,
        constructor: &str,
        args: &[S],
        salt: Vec<u8>,
        endowment: Option<Balance>,
    ) -> Result<Self, SessionError> {
        self.deploy(contract_bytes, constructor, args, salt, endowment)
            .map(|_| self)
    }

    /// Deploys a contract with a given constructor, arguments, salt and endowment. In case of
    /// success, returns the address of the deployed contract.
    pub fn deploy<S: AsRef<str> + Debug>(
        &mut self,
        contract_bytes: Vec<u8>,
        constructor: &str,
        args: &[S],
        salt: Vec<u8>,
        endowment: Option<Balance>,
    ) -> Result<AccountIdFor<R>, SessionError> {
        let data = self
            .transcoder
            .as_ref()
            .ok_or(SessionError::NoTranscoder)?
            .encode(constructor, args)
            .map_err(|err| SessionError::Encoding(err.to_string()))?;

        let result = self.sandbox.deploy_contract(
            contract_bytes,
            endowment.unwrap_or(ZERO_TRANSFER),
            data,
            salt,
            self.actor.clone(),
            self.gas_limit,
            DEFAULT_STORAGE_DEPOSIT_LIMIT,
        );

        let ret = match &result.result {
            Ok(exec_result) if exec_result.result.did_revert() => {
                Err(SessionError::DeploymentReverted)
            }
            Ok(exec_result) => {
                let address = exec_result.account_id.clone();
                self.deploy_returns.push(address.clone());
                Ok(address)
            }
            Err(err) => Err(SessionError::DeploymentFailed(*err)),
        };

        self.deploy_results.push(result);
        ret
    }

    /// Calls a contract with a given address. In case of a successful call, returns `self`.
    pub fn call_and<S: AsRef<str> + Debug>(
        mut self,
        message: &str,
        args: &[S],
        endowment: Option<Balance>,
    ) -> Result<Self, SessionError> {
        self.call_internal(None, message, args, endowment)
            .map(|_| self)
    }

    /// Calls the last deployed contract. In case of a successful call, returns `self`.
    pub fn call_with_address_and<S: AsRef<str> + Debug>(
        mut self,
        address: AccountIdFor<R>,
        message: &str,
        args: &[S],
        endowment: Option<Balance>,
    ) -> Result<Self, SessionError> {
        self.call_internal(Some(address), message, args, endowment)
            .map(|_| self)
    }

    /// Calls the last deployed contract. In case of a successful call, returns the decoded result.
    pub fn call<S: AsRef<str> + Debug>(
        &mut self,
        message: &str,
        args: &[S],
        endowment: Option<Balance>,
    ) -> Result<Vec<u8>, SessionError> {
        self.call_internal(None, message, args, endowment)
    }

    /// Calls a contract with a given address. In case of a successful call, returns the decoded
    /// result.
    pub fn call_with_address<S: AsRef<str> + Debug>(
        &mut self,
        address: AccountIdFor<R>,
        message: &str,
        args: &[S],
        endowment: Option<Balance>,
    ) -> Result<Vec<u8>, SessionError> {
        self.call_internal(Some(address), message, args, endowment)
    }

    fn call_internal<S: AsRef<str> + Debug>(
        &mut self,
        address: Option<AccountIdFor<R>>,
        message: &str,
        args: &[S],
        endowment: Option<Balance>,
    ) -> Result<Vec<u8>, SessionError> {
        let data = self
            .transcoder
            .as_ref()
            .ok_or(SessionError::NoTranscoder)?
            .encode(message, args)
            .map_err(|err| SessionError::Encoding(err.to_string()))?;

        let address = match address {
            Some(address) => address,
            None => self
                .deploy_returns
                .last()
                .ok_or(SessionError::NoContract)?
                .clone(),
        };

        let result = self.sandbox.call_contract(
            address,
            endowment.unwrap_or(ZERO_TRANSFER),
            data,
            self.actor.clone(),
            self.gas_limit,
            DEFAULT_STORAGE_DEPOSIT_LIMIT,
        );

        let ret = match &result.result {
            Ok(exec_result) if exec_result.did_revert() => Err(SessionError::CallReverted),
            Ok(exec_result) => {
                let encoded = exec_result.data.clone();
                self.call_returns.push(encoded.clone());
                Ok(encoded)
            }
            Err(err) => Err(SessionError::CallFailed(*err)),
        };

        self.call_results.push(result);
        ret
    }

    /// Returns the last result of deploying a contract.
    pub fn last_deploy_result(
        &self,
    ) -> Option<&ContractInstantiateResult<AccountIdFor<R>, Balance, EventRecordOf<R>>> {
        self.deploy_results.last()
    }

    /// Returns the address of the last deployed contract.
    pub fn last_deploy_return(&self) -> Option<AccountIdFor<R>> {
        self.deploy_returns.last().cloned()
    }

    /// Returns the addresses of all deployed contracts in the order of deploying.
    pub fn deployed_contracts(&self) -> Vec<AccountIdFor<R>> {
        self.deploy_returns.clone()
    }

    /// Returns the last result of calling a contract.
    pub fn last_call_result(&self) -> Option<&ContractExecResult<Balance, EventRecordOf<R>>> {
        self.call_results.last()
    }

    /// Returns the last value (in the encoded form) returned from calling a contract.
    ///
    /// Returns `None` if there has been no call yet.
    pub fn last_call_raw_return(&self) -> Option<Vec<u8>> {
        self.call_returns.last().cloned()
    }

    /// Returns the last value (in the decoded form) returned from calling a contract.
    ///
    /// Returns `None` if there has been no call yet, or if decoding failed.
    pub fn last_call_return<T: Decode>(&self) -> Option<MessageResult<T>> {
        let raw = self.last_call_raw_return()?;
        MessageResult::decode(&mut raw.as_slice()).ok()
    }

    /// Overrides the debug extension.
    ///
    /// By default, a new `Session` instance will use a noop debug extension. This method allows to
    /// override it with a custom debug extension.
    pub fn override_debug_handle(&mut self, d: DebugExt) {
        self.sandbox.override_debug_handle(d);
    }
}
