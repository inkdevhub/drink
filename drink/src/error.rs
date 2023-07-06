use thiserror::Error;

/// Main error type for the drink crate.
#[derive(Error, Debug)]
pub enum Error {
    /// Externalities could not be initialized.
    #[error("Failed to build storage: {0}")]
    StorageBuilding(String),
    /// Block couldn't have been initialized.
    #[error("Failed to initialize block: {0}")]
    BlockInitialize(String),
    /// Block couldn't have been finalized.
    #[error("Failed to finalize block: {0}")]
    BlockFinalize(String),
}
