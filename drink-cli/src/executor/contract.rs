use std::{env, fmt, fs, path::PathBuf, rc::Rc};
use std::fmt::Formatter;
use std::path::Path;

use contract_transcode::ContractMessageTranscoder;
use contract_build::{BuildMode, ExecuteArgs, ManifestPath, OptimizationPasses, Verbosity};

use crate::app_state::{print::format_contract_action, AppState, Contract};

enum BuildError {
    InvalidManifest { manifest_path: PathBuf, err: anyhow::Error },
    BuildFailed { err: anyhow::Error },
    WasmNotGenerated,
    InvalidDestPath { err: std::io::Error },
}

impl fmt::Display for BuildError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            BuildError::InvalidManifest { manifest_path, err } =>
                write!(f, "Invalid manifest path {}: {}", manifest_path.display(), err),
            BuildError::BuildFailed { err } =>
                write!(f, "Contract build failed: {}", err),
            BuildError::WasmNotGenerated =>
                write!(f, "Wasm code artifact not generated"),
            BuildError::InvalidDestPath { err } =>
                write!(f, "Invalid destination bundle path: {}", err),
        }
    }
}

fn build_result(app_state: &mut AppState) -> Result<String, BuildError> {
    let path_to_cargo_toml = app_state.ui_state.pwd.join(Path::new("Cargo.toml"));
    let manifest_path =  ManifestPath::new(path_to_cargo_toml.clone()).map_err(|err| {
        BuildError::InvalidManifest { manifest_path: path_to_cargo_toml, err }
    })?;

    let args = ExecuteArgs {
        manifest_path,
        build_mode: BuildMode::Release,
        optimization_passes: Some(OptimizationPasses::default()),
        verbosity: Verbosity::Quiet,
        ..Default::default()
    };

    contract_build::execute(args)
        .map_err(|err| BuildError::BuildFailed { err })?
        .dest_wasm.ok_or(BuildError::WasmNotGenerated)?
        .canonicalize()
        .map_err(|err| BuildError::InvalidDestPath { err })
        .map(|pb| { pb.to_string_lossy().to_string() })
}

/// Build the contract in the current directory.
pub fn build(app_state: &mut AppState) {
    match build_result(app_state) {
        Ok(res) => app_state.print(&format!("Contract built successfully {res}")),
        Err(msg) => app_state.print_error(&format!("{msg}")),
    }
}

pub fn deploy(app_state: &mut AppState, constructor: String, args: Vec<String>, salt: Vec<u8>) {
    // Get raw contract bytes
    let Some((contract_name, contract_file)) = find_wasm_blob() else {
        app_state.print_error("Failed to find contract file");
        return;
    };

    let contract_bytes = match fs::read(contract_file) {
        Ok(bytes) => bytes,
        Err(err) => {
            app_state.print_error(&format!("Failed to read contract bytes\n{err}"));
            return;
        }
    };

    // Read contract metadata and prepare transcoder
    let metadata_path = app_state
        .ui_state
        .pwd
        .join(format!("target/ink/{contract_name}.json"));

    let Ok(transcoder) = ContractMessageTranscoder::load(metadata_path) else {
        app_state.print_error("Failed to create transcoder from metadata file.");
        return;
    };
    let transcoder = Rc::new(transcoder);

    app_state.session.set_transcoder(Some(transcoder.clone()));
    match app_state
        .session
        .deploy(contract_bytes, &constructor, args.as_slice(), salt)
    {
        Ok(address) => {
            app_state.contracts.add(Contract {
                name: contract_name,
                address,
                base_path: app_state.ui_state.pwd.clone(),
                transcoder,
            });
            app_state.print("Contract deployed successfully");
        }
        Err(err) => app_state.print_error(&format!("Failed to deploy contract\n{err}")),
    }

    if let Some(info) = app_state.session.last_deploy_result() {
        app_state.print(&format_contract_action(info));
    }
}

pub fn call(app_state: &mut AppState, message: String, args: Vec<String>) {
    let Some(contract) = app_state.contracts.current_contract() else {
        app_state.print_error("No deployed contract");
        return;
    };

    app_state
        .session
        .set_transcoder(Some(contract.transcoder.clone()));

    let address = contract.address.clone();
    match app_state
        .session
        .call_with_address(address, &message, &args)
    {
        Ok(result) => app_state.print(&format!("Result: {:?}", result)),
        Err(err) => app_state.print_error(&format!("Failed to call contract\n{err}")),
    };

    if let Some(info) = app_state.session.last_call_result() {
        app_state.print(&format_contract_action(info))
    }
}

fn find_wasm_blob() -> Option<(String, PathBuf)> {
    let pwd = env::current_dir().expect("Failed to get current directory");
    let Ok(entries) = fs::read_dir(pwd.join("target/ink")) else {
        return None;
    };
    let Some(file) = entries
        .into_iter()
        .filter_map(|e| e.ok())
        .find(|e| e.path().extension().unwrap_or_default() == "wasm") else {
        return None;
    };

    let raw_name = file
        .file_name()
        .into_string()
        .expect("Invalid file name")
        .strip_suffix(".wasm")
        .expect("We have just checked file extension")
        .to_string();

    Some((raw_name, file.path()))
}
