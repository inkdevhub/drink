//! This module provides a context-aware interface for interacting with contracts.

use std::{fmt::Debug, mem, rc::Rc};

pub use contract_transcode;
use contract_transcode::ContractMessageTranscoder;
use frame_support::weights::Weight;
use pallet_contracts_primitives::{ContractExecResult, ContractInstantiateResult};
use parity_scale_codec::Decode;

use crate::{
    chain_api::ChainApi,
    contract_api::ContractApi,
    pallet_contracts_debugging::TracingExt,
    runtime::{AccountIdFor, HashFor, Runtime},
    EventRecordOf, Sandbox, DEFAULT_GAS_LIMIT,
};

pub mod error;
mod transcoding;

use error::SessionError;

use crate::{errors::MessageResult, mock::MockingApi, session::transcoding::TranscoderRegistry};

type Balance = u128;

const ZERO_TRANSFER: Balance = 0;
const DEFAULT_STORAGE_DEPOSIT_LIMIT: Option<Balance> = None;

/// Convenient value for an empty sequence of call/instantiation arguments.
///
/// Without it, you would have to specify explicitly a compatible type, like:
/// `session.call::<String>(.., &[], ..)`.
pub const NO_ARGS: &[String] = &[];

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
/// # fn main() -> Result<(), drink::session::error::SessionError> {
///
/// Session::<MinimalRuntime>::new()?
///     .deploy_and(contract_bytes(), "new", NO_ARGS, vec![], None, &get_transcoder())?
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
/// # fn main() -> Result<(), drink::session::error::SessionError> {
///
/// let mut session = Session::<MinimalRuntime>::new()?;
/// let _address = session.deploy(contract_bytes(), "new", NO_ARGS, vec![], None, &get_transcoder())?;
/// session.call("foo", NO_ARGS, None)?;
/// session.set_actor(bob());
/// session.call("bar", NO_ARGS, None)?;
/// # Ok(()) }
/// ```
pub struct Session<R: Runtime> {
    sandbox: Sandbox<R>,

    actor: AccountIdFor<R>,
    gas_limit: Weight,

    transcoders: TranscoderRegistry<AccountIdFor<R>>,

    deploy_results: Vec<ContractInstantiateResult<AccountIdFor<R>, Balance, EventRecordOf<R>>>,
    deploy_returns: Vec<AccountIdFor<R>>,
    call_results: Vec<ContractExecResult<Balance, EventRecordOf<R>>>,
    call_returns: Vec<Vec<u8>>,
}

impl<R: Runtime> Session<R> {
    /// Creates a new `Session`.
    pub fn new() -> Result<Self, SessionError> {
        Ok(Self {
            sandbox: Sandbox::new().map_err(SessionError::Drink)?,
            actor: R::default_actor(),
            gas_limit: DEFAULT_GAS_LIMIT,
            transcoders: TranscoderRegistry::new(),
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

    /// Register a transcoder for a particular contract and returns updated `self`.
    pub fn with_transcoder(
        mut self,
        contract_address: AccountIdFor<R>,
        transcoder: &Rc<ContractMessageTranscoder>,
    ) -> Self {
        self.set_transcoder(contract_address, transcoder);
        self
    }

    /// Registers a transcoder for a particular contract.
    pub fn set_transcoder(
        &mut self,
        contract_address: AccountIdFor<R>,
        transcoder: &Rc<ContractMessageTranscoder>,
    ) {
        self.transcoders.register(contract_address, transcoder);
    }

    /// Returns a reference for basic chain API.
    pub fn chain_api(&mut self) -> &mut impl ChainApi<R> {
        &mut self.sandbox
    }

    /// Returns a reference for basic contracts API.
    pub fn contracts_api(&mut self) -> &mut impl ContractApi<R> {
        &mut self.sandbox
    }

    /// Returns a reference for mocking API.
    pub fn mocking_api(&mut self) -> &mut impl MockingApi<R> {
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
        transcoder: &Rc<ContractMessageTranscoder>,
    ) -> Result<Self, SessionError> {
        self.deploy(
            contract_bytes,
            constructor,
            args,
            salt,
            endowment,
            transcoder,
        )
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
        transcoder: &Rc<ContractMessageTranscoder>,
    ) -> Result<AccountIdFor<R>, SessionError> {
        let data = transcoder
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

                self.transcoders.register(address.clone(), transcoder);

                Ok(address)
            }
            Err(err) => Err(SessionError::DeploymentFailed(*err)),
        };

        self.deploy_results.push(result);
        ret
    }

    /// Uploads a raw contract code. In case of success, returns `self`.
    pub fn upload_and(mut self, contract_bytes: Vec<u8>) -> Result<Self, SessionError> {
        self.upload(contract_bytes).map(|_| self)
    }

    /// Uploads a raw contract code. In case of success returns the code hash.
    pub fn upload(&mut self, contract_bytes: Vec<u8>) -> Result<HashFor<R>, SessionError> {
        let result = self.sandbox.upload_contract(
            contract_bytes,
            self.actor.clone(),
            DEFAULT_STORAGE_DEPOSIT_LIMIT,
        );

        result
            .map(|upload_result| upload_result.code_hash)
            .map_err(SessionError::UploadFailed)
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
        let address = match address {
            Some(address) => address,
            None => self
                .deploy_returns
                .last()
                .ok_or(SessionError::NoContract)?
                .clone(),
        };

        let data = self
            .transcoders
            .get(&address)
            .as_ref()
            .ok_or(SessionError::NoTranscoder)?
            .encode(message, args)
            .map_err(|err| SessionError::Encoding(err.to_string()))?;

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
    pub fn override_debug_handle(&mut self, d: TracingExt) {
        self.sandbox.override_debug_handle(d);
    }
}
