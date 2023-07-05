//! This module provides a context-aware interface for interacting with contracts.

use std::{mem, rc::Rc};

pub use contract_transcode;
use contract_transcode::{ContractMessageTranscoder, Value};
use frame_support::{dispatch::DispatchError, sp_runtime::AccountId32, weights::Weight};
use pallet_contracts_primitives::{ContractExecResult, ContractInstantiateResult};
use thiserror::Error;

use crate::{
    chain_api::ChainApi, contract_api::ContractApi, runtime::Runtime, Sandbox, DEFAULT_ACTOR,
    DEFAULT_GAS_LIMIT,
};

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

/// Wrapper around `Sandbox` that provides a convenient API for interacting with multiple contracts.
///
/// Instead of talking with a low-level `Sandbox`, you can use this struct to keep context,
/// including: origin, gas_limit, transcoder and history of results.
///
/// `Session` has two APIs: chain-ish and for singular actions. The first one can be used like:
/// ```rust, no_run
/// # use std::rc::Rc;
/// # use contract_transcode::ContractMessageTranscoder;
/// # use drink::session::Session;
/// # use drink::AccountId32;
/// #
/// # fn get_transcoder() -> Rc<ContractMessageTranscoder> {
/// #   Rc::new(ContractMessageTranscoder::load("").unwrap())
/// # }
/// # fn contract_bytes() -> Vec<u8> { vec![] }
/// # fn bob() -> AccountId32 { AccountId32::new([0; 32]) }
///
/// # fn main() -> Result<(), drink::session::SessionError> {
/// Session::new(Some(get_transcoder()))?
///     .deploy_and(contract_bytes(), "new", &[], vec![])?
///     .call_and("foo", &[])?
///     .with_actor(bob())
///     .call_and("bar", &[])?;
/// # Ok(()) }
/// ```
///
/// The second one serves for one-at-a-time actions:
/// ```rust, no_run
/// # use std::rc::Rc;
/// # use contract_transcode::ContractMessageTranscoder;
/// # use drink::session::Session;
/// # use drink::AccountId32;
/// #
/// # fn get_transcoder() -> Rc<ContractMessageTranscoder> {
/// #   Rc::new(ContractMessageTranscoder::load("").unwrap())
/// # }
/// # fn contract_bytes() -> Vec<u8> { vec![] }
/// # fn bob() -> AccountId32 { AccountId32::new([0; 32]) }
///
/// # fn main() -> Result<(), drink::session::SessionError> {
/// let mut session = Session::new(Some(get_transcoder()))?;
/// let _address = session.deploy(contract_bytes(), "new", &[], vec![])?;
/// session.call("foo", &[])?;
/// session.set_actor(bob());
/// session.call("bar", &[])?;
/// # Ok(()) }
/// ```
pub struct Session<R: Runtime> {
    sandbox: Sandbox<R>,

    actor: AccountId32,
    gas_limit: Weight,

    transcoder: Option<Rc<ContractMessageTranscoder>>,

    deploy_results: Vec<ContractInstantiateResult<AccountId32, u128>>,
    deploy_returns: Vec<AccountId32>,
    call_results: Vec<ContractExecResult<u128>>,
    call_returns: Vec<Value>,
}

impl<R: Runtime> Session<R> {
    /// Creates a new `Session` with optional reference to a transcoder.
    pub fn new(transcoder: Option<Rc<ContractMessageTranscoder>>) -> Result<Self, SessionError> {
        Ok(Self {
            sandbox: Sandbox::new().map_err(SessionError::Drink)?,
            actor: DEFAULT_ACTOR,
            gas_limit: DEFAULT_GAS_LIMIT,
            transcoder,
            deploy_results: vec![],
            deploy_returns: vec![],
            call_results: vec![],
            call_returns: vec![],
        })
    }

    /// Sets a new actor and returns updated `self`.
    pub fn with_actor(self, actor: AccountId32) -> Self {
        Self { actor, ..self }
    }

    /// Sets a new actor and returns the old one.
    pub fn set_actor(&mut self, actor: AccountId32) -> AccountId32 {
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
    pub fn chain_api(&mut self) -> &mut impl ChainApi {
        &mut self.sandbox
    }

    /// Deploys a contract with a given constructor, arguments and salt. In case of a success,
    /// returns `self`.
    pub fn deploy_and(
        mut self,
        contract_bytes: Vec<u8>,
        constructor: &str,
        args: &[String],
        salt: Vec<u8>,
    ) -> Result<Self, SessionError> {
        self.deploy(contract_bytes, constructor, args, salt)
            .map(|_| self)
    }

    /// Deploys a contract with a given constructor, arguments and salt. In case of a success,
    /// returns the address of the deployed contract.
    pub fn deploy(
        &mut self,
        contract_bytes: Vec<u8>,
        constructor: &str,
        args: &[String],
        salt: Vec<u8>,
    ) -> Result<AccountId32, SessionError> {
        let data = self
            .transcoder
            .as_ref()
            .ok_or(SessionError::NoTranscoder)?
            .encode(constructor, args)
            .map_err(|err| SessionError::Encoding(err.to_string()))?;

        let result = self.sandbox.deploy_contract(
            contract_bytes,
            data,
            salt,
            self.actor.clone(),
            self.gas_limit,
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
    pub fn call_and(mut self, message: &str, args: &[String]) -> Result<Self, SessionError> {
        self.call_internal(None, message, args).map(|_| self)
    }

    /// Calls the last deployed contract. In case of a successful call, returns `self`.
    pub fn call_with_address_and(
        mut self,
        address: AccountId32,
        message: &str,
        args: &[String],
    ) -> Result<Self, SessionError> {
        self.call_internal(Some(address), message, args)
            .map(|_| self)
    }

    /// Calls the last deployed contract. In case of a successful call, returns the decoded result.
    pub fn call(&mut self, message: &str, args: &[String]) -> Result<Value, SessionError> {
        self.call_internal(None, message, args)
    }

    /// Calls a contract with a given address. In case of a successful call, returns the decoded
    /// result.
    pub fn call_with_address(
        &mut self,
        address: AccountId32,
        message: &str,
        args: &[String],
    ) -> Result<Value, SessionError> {
        self.call_internal(Some(address), message, args)
    }

    fn call_internal(
        &mut self,
        address: Option<AccountId32>,
        message: &str,
        args: &[String],
    ) -> Result<Value, SessionError> {
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

        let result = self
            .sandbox
            .call_contract(address, data, self.actor.clone(), self.gas_limit);

        let ret = match &result.result {
            Ok(exec_result) if exec_result.did_revert() => Err(SessionError::CallReverted),
            Ok(exec_result) => {
                let decoded = self
                    .transcoder
                    .as_ref()
                    .ok_or(SessionError::NoTranscoder)?
                    .decode_return(message, &mut exec_result.data.as_slice())
                    .map_err(|err| SessionError::Decoding(err.to_string()))?;

                self.call_returns.push(decoded.clone());
                Ok(decoded)
            }
            Err(err) => Err(SessionError::CallFailed(*err)),
        };

        self.call_results.push(result);
        ret
    }

    /// Returns the last result of deploying a contract.
    pub fn last_deploy_result(&self) -> Option<&ContractInstantiateResult<AccountId32, u128>> {
        self.deploy_results.last()
    }

    /// Returns the address of the last deployed contract.
    pub fn last_deploy_return(&self) -> Option<AccountId32> {
        self.deploy_returns.last().cloned()
    }

    /// Returns the last result of calling a contract.
    pub fn last_call_result(&self) -> Option<&ContractExecResult<u128>> {
        self.call_results.last()
    }

    /// Returns the last value returned from calling a contract.
    pub fn last_call_return(&self) -> Option<Value> {
        self.call_returns.last().cloned()
    }
}
