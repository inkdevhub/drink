//! Module exposing errors and result types for the session API.

use frame_support::sp_runtime::DispatchError;
use parity_scale_codec::Decode;
use thiserror::Error;

use crate::errors::MessageResult;

/// Session specific errors.
#[derive(Clone, Error, Debug)]
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
    #[error("Contract call has been reverted. Encoded error: {0:?}")]
    CallReverted(Vec<u8>),
    /// Contract call failed (aborted by the pallet).
    #[error("Contract call failed before execution: {0:?}")]
    CallFailed(DispatchError),
    /// There is no deployed contract to call.
    #[error("No deployed contract")]
    NoContract,
    /// There is no registered transcoder to encode/decode messages for the called contract.
    #[error("Missing transcoder")]
    NoTranscoder,
}

impl SessionError {
    /// Check if the error is a revert error and if so, decode the error message.
    pub fn decode_revert<T: Decode>(&self) -> Result<MessageResult<T>, Self> {
        match self {
            SessionError::CallReverted(error) => {
                Ok(MessageResult::decode(&mut &error[..]).expect("Failed to decode error"))
            }
            _ => Err(self.clone()),
        }
    }
}
