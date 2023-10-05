//! Module gathering common error and result types.

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

/// Every contract message wraps its return value in `Result<T, LangResult>`. This is the error
/// type.
///
/// Copied from ink primitives.
#[non_exhaustive]
#[repr(u32)]
#[derive(
    Debug,
    Copy,
    Clone,
    PartialEq,
    Eq,
    parity_scale_codec::Encode,
    parity_scale_codec::Decode,
    scale_info::TypeInfo,
    Error,
)]
pub enum LangError {
    /// Failed to read execution input for the dispatchable.
    #[error("Failed to read execution input for the dispatchable.")]
    CouldNotReadInput = 1u32,
}

/// The `Result` type for ink! messages.
pub type MessageResult<T> = Result<T, LangError>;
