//! Module exposing errors and result types for the session API.

use thiserror::Error;
use frame_support::sp_runtime::DispatchError;

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
    /// Code upload failed (aborted by the pallet).
    #[error("Code upload failed: {0:?}")]
    UploadFailed(DispatchError),
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
