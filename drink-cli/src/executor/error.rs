use thiserror::Error;

#[derive(Error, Debug)]
pub(crate) enum BuildError {
    #[error("Invalid manifest path {manifest_path}: {err}")]
    InvalidManifest {
        manifest_path: std::path::PathBuf,
        err: anyhow::Error,
    },
    #[error("Contract build failed: {err}")]
    BuildFailed { err: anyhow::Error },
    #[error("Wasm code artifact not generated")]
    WasmNotGenerated,
    #[error("Invalid destination bundle path: {err}")]
    InvalidDestPath { err: std::io::Error },
}
