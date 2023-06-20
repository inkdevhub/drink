use std::{env, process::Command};

use anyhow::Result;
use clap::Parser;
use sp_runtime::AccountId32;

use crate::{app_state::AppState, cli::CliCommand};

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
        CliCommand::Clear => {
            app_state.ui_state.output.clear();
            app_state.ui_state.output_offset = 0;
        }
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

        CliCommand::Build => {
            build_contract(app_state);
        }
        CliCommand::Deploy => {
            // account_id = Some(deploy_contract(&mut sandbox));
            app_state.chain_info.deployed_contracts += 1;
            app_state.chain_info.current_contract_address = Some(AccountId32::new([0; 32]));
        }
        CliCommand::CallGet => {
            // let account_id = match account_id {
            //     Some(ref account_id) => account_id.clone(),
            //     None => {
            //         eprintln!("Contract not deployed");
            //         continue;
            //     }
            // };
            //
            // let result = sandbox.call_contract(account_id, "get".to_string());
            // println!("Contract called successfully.\n\n{result}")
        }
        CliCommand::CallFlip => {}
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

// fn deploy_contract(sandbox: &mut Sandbox) -> AccountId32 {
//     println!("Deploying contract...");
//
//     let contract_bytes_path = env::current_dir()
//         .expect("Failed to get current directory")
//         .join(CONTRACT_DIR)
//         .join("target/ink/example.wasm");
//     let contract_bytes = std::fs::read(contract_bytes_path).expect("Failed to read contract bytes");
//
//     let account_id = sandbox.deploy_contract(contract_bytes);
//
//     println!("Contract deployed successfully");
//
//     account_id
// }
//
