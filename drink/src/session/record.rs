use parity_scale_codec::Decode;

use crate::{
    errors::MessageResult,
    runtime::{minimal::RuntimeEvent, AccountIdFor, MinimalRuntime, RuntimeWithContracts},
    session::{error::SessionError, BalanceOf},
    EventRecordOf, Sandbox,
};

type ContractInstantiateResult<R> = pallet_contracts_primitives::ContractInstantiateResult<
    AccountIdFor<R>,
    BalanceOf<R>,
    EventRecordOf<R>,
>;
type ContractExecResult<R> =
    pallet_contracts_primitives::ContractExecResult<BalanceOf<R>, EventRecordOf<R>>;

/// Data structure storing the results of contract interaction during a session.
///
/// # Naming convention
///
/// By `result` we mean the full result (enriched with some context information) of the contract
/// interaction, like `ContractExecResult`. By `return` we mean the return value of the contract
/// execution, like a value returned from a message or the address of a newly instantiated contract.
pub struct Record<R: RuntimeWithContracts> {
    /// The results of contract instantiation.
    deploy_results: Vec<ContractInstantiateResult<R>>,
    /// The return values of contract instantiation (i.e. the addresses of the newly instantiated
    /// contracts).
    deploy_returns: Vec<AccountIdFor<R>>,

    /// The results of contract calls.
    call_results: Vec<ContractExecResult<R>>,
    /// The return values of contract calls (in the SCALE-encoded form).
    call_returns: Vec<Vec<u8>>,

    /// The events emitted by the contracts.
    event_batches: Vec<EventBatch<R>>,

    /// Because `drink` normally doesn't have a continuous block production, everything implicitly
    /// happens within a single block (unless user explicitly trigger a new block). This means that
    /// all runtime events (from consecutive transactions) are stacked up in a common buffer.
    /// `Record` is capable of recording only the events that happened during a single transaction
    /// by remembering the number of events that were already in the buffer before the transaction
    /// started. However, this is must be explicitly enabled by calling `start_recording_events`
    /// before the transaction and `stop_recording_events` after the transaction.
    block_events_so_far: Option<usize>,
}

impl<R: RuntimeWithContracts> Default for Record<R> {
    fn default() -> Self {
        Self {
            deploy_results: Vec::new(),
            deploy_returns: Vec::new(),
            call_results: Vec::new(),
            call_returns: Vec::new(),
            event_batches: Vec::new(),
            block_events_so_far: None,
        }
    }
}

// API for `Session` to record results and events related to contract interaction.
impl<R: RuntimeWithContracts> Record<R> {
    pub(super) fn push_deploy_result(&mut self, result: ContractInstantiateResult<R>) {
        self.deploy_results.push(result);
    }

    pub(super) fn push_deploy_return(&mut self, return_value: AccountIdFor<R>) {
        self.deploy_returns.push(return_value);
    }

    pub(super) fn push_call_result(&mut self, result: ContractExecResult<R>) {
        self.call_results.push(result);
    }

    pub(super) fn push_call_return(&mut self, return_value: Vec<u8>) {
        self.call_returns.push(return_value);
    }

    pub(super) fn start_recording_events(&mut self, sandbox: &mut Sandbox<R>) {
        assert!(
            self.block_events_so_far.is_none(),
            "Already recording events"
        );
        self.block_events_so_far = Some(sandbox.events().len());
    }

    pub(super) fn stop_recording_events(&mut self, sandbox: &mut Sandbox<R>) {
        let start = self
            .block_events_so_far
            .take()
            .expect("Not recording events");
        let end = sandbox.events().len();
        let events = sandbox.events()[start..end].to_vec();
        self.event_batches.push(EventBatch { events });
    }
}

// API for the end user.
impl<R: RuntimeWithContracts> Record<R> {
    /// Returns all the results of contract instantiations that happened during the session.
    pub fn deploy_results(&self) -> &[ContractInstantiateResult<R>] {
        &self.deploy_results
    }

    /// Returns the last result of contract instantiation that happened during the session. Panics
    /// if there were no contract instantiations.
    pub fn last_deploy_result(&self) -> &ContractInstantiateResult<R> {
        self.deploy_results.last().expect("No deploy results")
    }

    /// Returns all the return values of contract instantiations that happened during the session.
    pub fn deploy_returns(&self) -> &[AccountIdFor<R>] {
        &self.deploy_returns
    }

    /// Returns the last return value of contract instantiation that happened during the session.
    /// Panics if there were no contract instantiations.
    pub fn last_deploy_return(&self) -> &AccountIdFor<R> {
        self.deploy_returns.last().expect("No deploy returns")
    }

    /// Returns all the results of contract calls that happened during the session.
    pub fn call_results(&self) -> &[ContractExecResult<R>] {
        &self.call_results
    }

    /// Returns the last result of contract call that happened during the session. Panics if there
    /// were no contract calls.
    pub fn last_call_result(&self) -> &ContractExecResult<R> {
        self.call_results.last().expect("No call results")
    }

    /// Returns all the (encoded) return values of contract calls that happened during the session.
    pub fn call_returns(&self) -> &[Vec<u8>] {
        &self.call_returns
    }

    /// Returns the last (encoded) return value of contract call that happened during the session.
    /// Panics if there were no contract calls.
    pub fn last_call_return(&self) -> &[u8] {
        self.call_returns.last().expect("No call returns")
    }

    /// Returns the last (decoded) return value of contract call that happened during the session.
    /// Panics if there were no contract calls.
    pub fn last_call_return_decoded<T: Decode>(&self) -> Result<MessageResult<T>, SessionError> {
        let mut raw = self.last_call_return();
        MessageResult::decode(&mut raw).map_err(|err| {
            SessionError::Decoding(format!(
                "Failed to decode the result of calling a contract: {err:?}"
            ))
        })
    }

    /// Returns all the event batches that were recorded for contract interactions during the
    /// session.
    pub fn event_batches(&self) -> &[EventBatch<R>] {
        &self.event_batches
    }

    /// Returns the last event batch that was recorded for contract interactions during the session.
    /// Panics if there were no event batches.
    pub fn last_event_batch(&self) -> &EventBatch<R> {
        self.event_batches.last().expect("No event batches")
    }
}

/// A batch of runtime events that were emitted during a single contract interaction.
pub struct EventBatch<R: frame_system::Config> {
    events: Vec<EventRecordOf<R>>,
}

impl<R: frame_system::Config> EventBatch<R> {
    /// Returns all the events that were emitted during the contract interaction.
    pub fn all_events(&self) -> &[EventRecordOf<R>] {
        &self.events
    }
}

impl EventBatch<MinimalRuntime> {
    /// Returns all the contract events that were emitted during the contract interaction.
    ///
    /// We have to match against static enum variant, and thus (at least for now) we support only
    /// `MinimalRuntime`.
    pub fn contract_events(&self) -> Vec<&[u8]> {
        self.events
            .iter()
            .filter_map(|event| match &event.event {
                RuntimeEvent::Contracts(
                    pallet_contracts::Event::<MinimalRuntime>::ContractEmitted { data, .. },
                ) => Some(data.as_slice()),
                _ => None,
            })
            .collect()
    }
}
