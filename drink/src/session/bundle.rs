//! This module provides simple utilities for loading and parsing `.contract` files in context of `drink` tests.

use std::{path::PathBuf, rc::Rc};

use contract_metadata::ContractMetadata;
use contract_transcode::ContractMessageTranscoder;

use super::error::SessionError;

/// A struct representing the result of parsing a `.contract` bundle file.
///
/// It can be used with the following methods of the `Session` struct:
/// - `deploy_bundle`
/// - `deploy_bundle_and`
/// - `upload_bundle`
/// - `upload_bundle_and`
pub struct ContractBundle {
    /// WASM blob of the contract
    pub bytes: Vec<u8>,
    /// Transcoder derived from the ABI/metadata
    pub transcoder: Rc<ContractMessageTranscoder>,
}

impl ContractBundle {
    /// Load and parse the information in a `.contract` bundle under `path`, producing a `ContractBundle` struct.
    pub fn load<P>(path: P) -> Result<Self, SessionError>
    where
        P: AsRef<std::path::Path>,
    {
        let metadata: ContractMetadata = ContractMetadata::load(&path).map_err(|e| {
            SessionError::BundleLoadFailed(format!("Failed to load the contract file:\n{e:?}"))
        })?;

        let ink_metadata = serde_json::from_value(serde_json::Value::Object(metadata.abi))
            .map_err(|e| {
                SessionError::BundleLoadFailed(format!(
                    "Failed to parse metadata from the contract file:\n{e:?}"
                ))
            })?;

        let transcoder = Rc::new(ContractMessageTranscoder::new(ink_metadata));

        let bytes = metadata
            .source
            .wasm
            .ok_or(SessionError::BundleLoadFailed(
                "Failed to get the WASM blob from the contract file".to_string(),
            ))?
            .0;
            
        Ok(Self { bytes, transcoder })
    }

    /// Load the `.contract` bundle (`bundle_file`) located in the `project_dir`` working directory.
    ///
    /// This is meant to be used predominantly by the `local_bundle!` macro.
    pub fn local(project_dir: &str, bundle_file: String) -> Self {
        let mut path = PathBuf::from(project_dir);
        path.push("target");
        path.push("ink");
        path.push(bundle_file);
        Self::load(path).expect("Loading the local bundle failed")
    }
}

/// A convenience macro that allows you to load a bundle found in the target directory
/// of the current project.
#[macro_export]
macro_rules! local_bundle {
    () => {
        drink::session::ContractBundle::local(
            env!("CARGO_MANIFEST_DIR"),
            env!("CARGO_CRATE_NAME").to_owned() + ".contract",
        )
    };
}
