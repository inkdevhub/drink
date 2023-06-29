use std::{env, fs, path::PathBuf, rc::Rc};

use contract_transcode::ContractMessageTranscoder;

use crate::app_state::{print::format_contract_action, AppState, Contract};

pub fn build(app_state: &mut AppState) {
    let Ok(output) = std::process::Command::new("cargo")
        .arg("contract")
        .arg("build")
        .arg("--release")
        .output() else {
        app_state.print_error("Failed to execute build command. Make sure `cargo contract` is installed. (`cargo install cargo-contract`)");
        return;
    };

    if output.status.success() {
        app_state.print("Contract built successfully");
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        app_state.print_error(&format!(
            "Failed to execute 'cargo contract' command:\n{stderr}"
        ));
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
    match app_state.session.call(Some(address), &message, &args) {
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
