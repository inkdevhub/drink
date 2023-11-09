//! A sandboxed runtime.

pub mod balance_api;
pub mod contract_api;
pub mod mocking_api;
pub mod runtime_api;
pub mod system_api;
pub mod timestamp_api;

use std::sync::{Arc, Mutex};

use sp_io::TestExternalities;

use crate::{
    mock::MockRegistry,
    pallet_contracts_debugging::{InterceptingExt, TracingExt},
    runtime::*,
    MockingExtension,
};

/// A sandboxed runtime.
pub struct Sandbox<R: frame_system::Config> {
    externalities: TestExternalities,
    mock_registry: Arc<Mutex<MockRegistry<AccountIdFor<R>>>>,
    mock_counter: usize,
}

impl<R: frame_system::Config> Sandbox<R> {
    /// Execute the given closure with the inner externallities.
    ///
    /// Returns the result of the given closure.
    pub fn execute_with<T>(&mut self, execute: impl FnOnce() -> T) -> T {
        self.externalities.execute_with(execute)
    }

    /// Run an action without modifying the storage.
    ///
    /// # Arguments
    ///
    /// * `action` - The action to run.
    pub fn dry_run<T>(&mut self, action: impl FnOnce(&mut Self) -> T) -> T {
        // Make a backup of the backend.
        let backend_backup = self.externalities.as_backend();

        // Run the action, potentially modifying storage. Ensure, that there are no pending changes
        // that would affect the reverted backend.
        let result = action(self);
        self.externalities
            .commit_all()
            .expect("Failed to commit changes");

        // Restore the backend.
        self.externalities.backend = backend_backup;

        result
    }

    /// Overrides the debug extension.
    ///
    /// By default, a new `Sandbox` instance is created with a noop debug extension. This method
    /// allows to override it with a custom debug extension.
    pub fn override_debug_handle(&mut self, d: TracingExt) {
        self.externalities.register_extension(d);
    }

    /// Registers the extension for intercepting calls to contracts.
    fn setup_mock_extension(&mut self) {
        self.externalities
            .register_extension(InterceptingExt(Box::new(MockingExtension {
                mock_registry: Arc::clone(&self.mock_registry),
            })));
    }
}
