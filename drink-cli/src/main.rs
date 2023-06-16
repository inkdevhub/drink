use std::{env, io, process::Command};

use clap::Parser;
use cli::CliCommand;
use drink::Sandbox;

mod cli;

const CONTRACT_DIR: &'static str = "../example/";

fn main() {
    let mut sandbox = Sandbox::new();

    loop {
        let mut user_input = String::new();
        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to get user input");

        let cli_command = match CliCommand::try_parse_from(&["", user_input.trim()]) {
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
                deploy_contract(&mut sandbox);
            }
            CliCommand::Call => {
                println!("Call");
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

fn deploy_contract(sandbox: &mut Sandbox) {
    println!("Deploying contract...");

    let contract_bytes_path = env::current_dir()
        .expect("Failed to get current directory")
        .join(CONTRACT_DIR)
        .join("target/ink/wasm32-unknown-unknown/release/example.wasm");
    let contract_bytes = std::fs::read(contract_bytes_path).expect("Failed to read contract bytes");

    sandbox.deploy_contract(&contract_bytes);

    println!("Contract deployed successfully");
}
