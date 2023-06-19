use std::{env, io, process::Command};

use clap::Parser;
use cli::CliCommand;
use drink::Sandbox;
use sp_runtime::AccountId32;

mod cli;

const CONTRACT_DIR: &str = "../example/";

fn main() {
    let mut sandbox = Sandbox::new();
    let mut account_id = None;

    loop {
        println!();

        let mut user_input = String::new();
        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to get user input");

        let cli_command = match CliCommand::try_parse_from(["", user_input.trim()]) {
            Ok(cli_command) => cli_command,
            Err(_) => {
                eprintln!("Invalid command");
                continue;
            }
        };

        match cli_command {
            CliCommand::Build => {
                build_contract();
            }
            CliCommand::Deploy => {
                account_id = Some(deploy_contract(&mut sandbox));
            }
            CliCommand::CallGet => {
                let account_id = match account_id {
                    Some(ref account_id) => account_id.clone(),
                    None => {
                        eprintln!("Contract not deployed");
                        continue;
                    }
                };

                let result = sandbox.call_contract(account_id, "get".to_string());
                println!("Contract called successfully.\n\n{result}")
            }
            CliCommand::CallFlip => {
                let account_id = match account_id {
                    Some(ref account_id) => account_id.clone(),
                    None => {
                        eprintln!("Contract not deployed");
                        continue;
                    }
                };

                let result = sandbox.call_contract(account_id, "flip".to_string());
                println!("Contract called successfully.\n\n{result}")
            }
            CliCommand::Exit => {
                println!("Exit");
                break;
            }
        }
    }
}

fn build_contract() {
    println!("Building contract...");

    let current_dir = env::current_dir().expect("Failed to get current directory");
    let contract_path = current_dir.join(CONTRACT_DIR);
    env::set_current_dir(contract_path).expect("Failed to change directory");

    let output = Command::new("cargo-contract")
        .arg("contract")
        .arg("build")
        .arg("--release")
        .output()
        .expect("Failed to execute 'cargo contract' command");

    if output.status.success() {
        println!("Contract built successfully");
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!("Error executing 'cargo contract' command:\n{}", stderr);
    }
}

fn deploy_contract(sandbox: &mut Sandbox) -> AccountId32 {
    println!("Deploying contract...");

    let contract_bytes_path = env::current_dir()
        .expect("Failed to get current directory")
        .join(CONTRACT_DIR)
        .join("target/ink/example.wasm");
    let contract_bytes = std::fs::read(contract_bytes_path).expect("Failed to read contract bytes");

    let account_id = sandbox.deploy_contract(contract_bytes);

    println!("Contract deployed successfully");

    account_id
}
