pub use contract_transcode;
use contract_transcode::{ContractMessageTranscoder, Value};
use frame_support::{dispatch::DispatchError, sp_runtime::AccountId32, weights::Weight};
use pallet_contracts_primitives::{ContractExecResult, ContractInstantiateResult};
use thiserror::Error;

use crate::{contract_api::ContractApi, Sandbox, DEFAULT_ACTOR, DEFAULT_GAS_LIMIT};

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

    transcoder: Option<ContractMessageTranscoder>,

    deploy_results: Vec<ContractInstantiateResult<AccountId32, u128>>,
    deploy_returns: Vec<AccountId32>,
    call_results: Vec<ContractExecResult<u128>>,
    call_returns: Vec<Value>,
}

impl Session {
    pub fn new(transcoder: Option<ContractMessageTranscoder>) -> Result<Self, SessionError> {
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

    pub fn with_gas_limit(self, gas_limit: Weight) -> Self {
        Self { gas_limit, ..self }
    }

    pub fn with_transcoder(self, transcoder: ContractMessageTranscoder) -> Self {
        Self {
            transcoder: Some(transcoder),
            ..self
        }
    }

    pub fn deploy(
        mut self,
        contract_bytes: Vec<u8>,
        constructor: &str,
        args: &[&str],
        salt: Vec<u8>,
    ) -> Result<Self, SessionError> {
        let data = self
            .transcoder
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

        match &result.result {
            Ok(exec_result) if exec_result.result.did_revert() => {
                Err(SessionError::DeploymentReverted)
            }
            Ok(exec_result) => {
                self.deploy_returns.push(exec_result.account_id.clone());
                self.deploy_results.push(result);
                Ok(self)
            }
            Err(err) => Err(SessionError::DeploymentFailed(*err)),
        }
    }

    pub fn call(mut self, message: &str, args: &[&str]) -> Result<Self, SessionError> {
        let data = self
            .transcoder
            .ok_or(SessionError::NoTranscoder)?
            .encode(message, args)
            .map_err(|err| SessionError::Encoding(err.to_string()))?;

        let address = self.last_deploy_return().ok_or(SessionError::NoContract)?;
        let result = self
            .sandbox
            .call_contract(address, data, self.actor.clone(), self.gas_limit);

        match &result.result {
            Ok(exec_result) if exec_result.did_revert() => Err(SessionError::CallReverted),
            Ok(exec_result) => {
                let decoded = self
                    .transcoder
                    .ok_or(SessionError::NoTranscoder)?
                    .decode_return(message, &mut exec_result.data.as_slice())
                    .map_err(|err| SessionError::Decoding(err.to_string()))?;

                self.call_returns.push(decoded);
                self.call_results.push(result);
                Ok(self)
            }
            Err(err) => Err(SessionError::CallFailed(*err)),
        }
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
