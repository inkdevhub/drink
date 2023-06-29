use std::{mem, rc::Rc};

pub use contract_transcode;
use contract_transcode::{ContractMessageTranscoder, Value};
use frame_support::{dispatch::DispatchError, sp_runtime::AccountId32, weights::Weight};
use pallet_contracts_primitives::{ContractExecResult, ContractInstantiateResult};
use thiserror::Error;

use crate::{
    chain_api::ChainApi, contract_api::ContractApi, Sandbox, DEFAULT_ACTOR, DEFAULT_GAS_LIMIT,
};

#[derive(Error, Debug)]
pub enum SessionError {
    #[error("Encoding call data failed: {0}")]
    Encoding(String),
    #[error("Decoding call data failed: {0}")]
    Decoding(String),
    #[error("{0:?}")]
    Drink(#[from] crate::Error),
    #[error("Contract deployment has been reverted")]
    DeploymentReverted,
    #[error("Contract deployment failed before execution: {0:?}")]
    DeploymentFailed(DispatchError),
    #[error("Contract call has been reverted")]
    CallReverted,
    #[error("Contract call failed before execution: {0:?}")]
    CallFailed(DispatchError),
    #[error("No deployed contract")]
    NoContract,
    #[error("Missing transcoder")]
    NoTranscoder,
}

pub struct Session {
    sandbox: Sandbox,

    actor: AccountId32,
    gas_limit: Weight,

    transcoder: Option<Rc<ContractMessageTranscoder>>,

    deploy_results: Vec<ContractInstantiateResult<AccountId32, u128>>,
    deploy_returns: Vec<AccountId32>,
    call_results: Vec<ContractExecResult<u128>>,
    call_returns: Vec<Value>,
}

impl Session {
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

    pub fn with_actor(self, actor: AccountId32) -> Self {
        Self { actor, ..self }
    }

    pub fn set_actor(&mut self, actor: AccountId32) -> AccountId32 {
        mem::replace(&mut self.actor, actor)
    }

    pub fn with_gas_limit(self, gas_limit: Weight) -> Self {
        Self { gas_limit, ..self }
    }

    pub fn set_gas_limit(&mut self, gas_limit: Weight) -> Weight {
        mem::replace(&mut self.gas_limit, gas_limit)
    }

    pub fn with_transcoder(self, transcoder: Option<Rc<ContractMessageTranscoder>>) -> Self {
        Self { transcoder, ..self }
    }

    pub fn set_transcoder(
        &mut self,
        transcoder: Option<Rc<ContractMessageTranscoder>>,
    ) -> Option<Rc<ContractMessageTranscoder>> {
        mem::replace(&mut self.transcoder, transcoder)
    }

    pub fn chain_api(&mut self) -> &mut impl ChainApi {
        &mut self.sandbox
    }

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

    pub fn call_and(
        mut self,
        address: Option<AccountId32>,
        message: &str,
        args: &[String],
    ) -> Result<Self, SessionError> {
        self.call(address, message, args).map(|_| self)
    }

    pub fn call(
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

    pub fn last_deploy_result(&self) -> Option<&ContractInstantiateResult<AccountId32, u128>> {
        self.deploy_results.last()
    }

    pub fn last_deploy_return(&self) -> Option<AccountId32> {
        self.deploy_returns.last().cloned()
    }

    pub fn last_call_result(&self) -> Option<&ContractExecResult<u128>> {
        self.call_results.last()
    }

    pub fn last_call_return(&self) -> Option<Value> {
        self.call_returns.last().cloned()
    }
}
