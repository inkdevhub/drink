//! A sandboxed runtime.

mod sandbox_core;
pub use sandbox_core::SandboxConfig;
pub mod balance_api;
pub mod contract_api;
pub mod runtime_api;
pub mod system_api;
pub mod timestamp_api;

use std::any::Any;

use sp_externalities::Extension;
use sp_io::TestExternalities;

/// A sandboxed runtime.
pub struct Sandbox<Config: SandboxConfig> {
    externalities: TestExternalities,
    _phantom: std::marker::PhantomData<Config>,
}

impl<Config: SandboxConfig> Sandbox<Config> {
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

    /// Registers an extension.
    pub fn register_extension<E: Any + Extension>(&mut self, ext: E) {
        self.externalities.register_extension(ext);
    }
}
