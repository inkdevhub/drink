use crate::{
    runtime::{minimal::RuntimeEvent, AccountIdFor, MinimalRuntime, RuntimeWithContracts},
    session::BalanceOf,
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
#[derive(Default)]
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

    block_events_so_far: Option<usize>,
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
    pub fn deploy_results(&self) -> &[ContractInstantiateResult<R>] {
        &self.deploy_results
    }

    pub fn last_deploy_result(&self) -> &ContractInstantiateResult<R> {
        self.deploy_results.last().expect("No deploy results")
    }

    pub fn deploy_returns(&self) -> &[AccountIdFor<R>] {
        &self.deploy_returns
    }

    pub fn last_deploy_return(&self) -> &AccountIdFor<R> {
        self.deploy_returns.last().expect("No deploy returns")
    }

    pub fn call_results(&self) -> &[ContractExecResult<R>] {
        &self.call_results
    }

    pub fn last_call_result(&self) -> &ContractExecResult<R> {
        self.call_results.last().expect("No call results")
    }

    pub fn call_returns(&self) -> &[Vec<u8>] {
        &self.call_returns
    }

    pub fn last_call_return(&self) -> &[u8] {
        self.call_returns.last().expect("No call returns")
    }

    pub fn event_batches(&self) -> &[EventBatch<R>] {
        &self.event_batches
    }

    pub fn last_event_batch(&self) -> &EventBatch<R> {
        self.event_batches.last().expect("No event batches")
    }
}

pub struct EventBatch<R: frame_system::Config> {
    events: Vec<EventRecordOf<R>>,
}

impl<R: frame_system::Config> EventBatch<R> {
    pub fn all_events(&self) -> &[EventRecordOf<R>] {
        &self.events
    }
}

impl EventBatch<MinimalRuntime> {
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
