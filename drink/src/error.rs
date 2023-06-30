use thiserror::Error;

/// Main error type for the drink crate.
#[derive(Error, Debug)]
pub enum Error {
    /// Externalities could not be initialized.
    #[error("Failed to build storage: {0}")]
    StorageBuilding(String),
}
