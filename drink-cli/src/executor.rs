use std::{env, fs, process::Command};

use anyhow::Result;
use clap::Parser;
use drink::chain_api::ChainApi;
use sp_runtime::{app_crypto::sp_core::blake2_256, AccountId32};

use crate::{
    app_state::{AppState, Contract},
    cli::CliCommand,
};

pub fn execute(app_state: &mut AppState) -> Result<()> {
    let command = app_state.ui_state.user_input.clone();
    app_state.print_command(&command);

    let command = command
        .split_ascii_whitespace()
        .map(|a| a.trim())
        .collect::<Vec<_>>();
    let cli_command = match CliCommand::try_parse_from([vec![""], command].concat()) {
        Ok(cli_command) => cli_command,
        Err(_) => {
            app_state.print_error("Invalid command");
            return Ok(());
        }
    };

    match cli_command {
        CliCommand::Clear => app_state.ui_state.output.clear(),
        CliCommand::ChangeDir { path } => {
            let target_dir = app_state.ui_state.pwd.join(path);
            match env::set_current_dir(target_dir) {
                Ok(_) => {
                    app_state.ui_state.pwd =
                        env::current_dir().expect("Failed to get current directory");
                    app_state.print("Directory changed");
                }
                Err(err) => app_state.print_error(&err.to_string()),
            }
        }

        CliCommand::NextBlock { count } => build_blocks(app_state, count),
        CliCommand::AddTokens { recipient, value } => add_tokens(app_state, recipient, value),

        CliCommand::Build => build_contract(app_state),
        CliCommand::Deploy { constructor, salt } => deploy_contract(app_state, constructor, salt),
        CliCommand::Call { message } => call_contract(app_state, message),
    }

    Ok(())
}

fn build_contract(app_state: &mut AppState) {
    let output = Command::new("cargo-contract")
        .arg("contract")
        .arg("build")
        .arg("--release")
        .output()
        .expect("Failed to execute 'cargo contract' command");

    if output.status.success() {
        app_state.print("Contract built successfully");
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        app_state.print_error(&format!(
            "Error executing 'cargo contract' command:\n{stderr}"
        ));
    }
}

fn deploy_contract(app_state: &mut AppState, constructor: String, salt: Vec<u8>) {
    let contract_bytes_path = match fs::read_dir(app_state.ui_state.pwd.join("target/ink")) {
        Ok(entries) => {
            match entries
                .into_iter()
                .filter_map(|e| e.ok())
                .find(|e| e.path().extension().unwrap_or_default() == "wasm")
            {
                None => {
                    app_state.print_error("Failed to find contract file");
                    return;
                }
                Some(file) => {
                    app_state.ui_state.contract_project_name = file
                        .file_name()
                        .to_str()
                        .unwrap()
                        .strip_suffix(".wasm")
                        .unwrap()
                        .to_string();
                    file.path()
                }
            }
        }
        Err(err) => {
            app_state.print_error(&format!("Failed to find contract file\n{err}"));
            return;
        }
    };

    let contract_bytes = match fs::read(contract_bytes_path) {
        Ok(bytes) => bytes,
        Err(err) => {
            app_state.print_error(&format!("Failed to read contract bytes\n{err}"));
            return;
        }
    };

    let account_id =
        app_state
            .sandbox
            .deploy_contract(contract_bytes, compute_selector(constructor), salt);

    app_state.print("Contract deployed successfully");

    app_state.chain_info.deployed_contracts += 1;
    app_state.contracts.push_front(Contract {
        name: app_state.ui_state.contract_project_name.clone(),
        address: account_id,
        base_path: app_state.ui_state.pwd.clone(),
    });
}

fn call_contract(app_state: &mut AppState, message: String) {
    let account_id = match app_state.contracts.get(0).map(|c| &c.address) {
        Some(account_id) => account_id.clone(),
        None => {
            app_state.print_error("No deployed contract");
            return;
        }
    };

    let result = app_state
        .sandbox
        .call_contract(account_id, compute_selector(message));
    app_state.print(&format!("Contract called successfully.\n\n{result}"));
}

fn compute_selector(name: String) -> Vec<u8> {
    let name = name.as_bytes();
    blake2_256(name)[..4].to_vec()
}

fn build_blocks(app_state: &mut AppState, count: u64) {
    for _ in 0..count {
        app_state.sandbox.build_block();
    }

    app_state.chain_info.block_height += count;

    app_state.print(&format!("{count} blocks built"));
}

fn add_tokens(app_state: &mut AppState, recipient: AccountId32, value: u128) {
    app_state.sandbox.add_tokens(recipient.clone(), value);
    app_state.print(&format!("{value} tokens added to {recipient}",));
}
