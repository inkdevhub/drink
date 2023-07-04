use std::{env, fs, path::PathBuf, rc::Rc};
use std::path::Path;

use contract_transcode::ContractMessageTranscoder;
use contract_build::{BuildMode, ExecuteArgs, ManifestPath, OptimizationPasses, Verbosity};

use crate::app_state::{print::format_contract_action, AppState, Contract};

pub fn build(app_state: &mut AppState) {
    let path_to_cargo_toml = app_state.ui_state.pwd.join(Path::new("Cargo.toml"));
    let manifest_path =  match ManifestPath::new(path_to_cargo_toml.clone()) {
        Ok(mp) => mp,
        Err(err) => {
            app_state.print_error(&format!(
                "Invalid manifest path {}: {}",
                path_to_cargo_toml.display(),
                err
            ));
            return;
        },
    };

    let args = ExecuteArgs {
        manifest_path,
        build_mode: BuildMode::Release,
        optimization_passes: Some(OptimizationPasses::default()),
        verbosity: Verbosity::Quiet,
        ..Default::default()
    };

    match contract_build::execute(args) {
        Ok(build_result) => {
            let res: String = build_result
                .dest_wasm
                .expect("Wasm code artifact not generated")
                .canonicalize()
                .expect("Invalid dest bundle path")
                .to_string_lossy()
                .into();
            app_state.print(&format!("Contract built successfully: {}", res));
        }
        Err(err) => {
            app_state.print_error(&format!(
                "contract build for {} failed: {}",
                path_to_cargo_toml.display(),
                err,
            ));
        }
    };


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
