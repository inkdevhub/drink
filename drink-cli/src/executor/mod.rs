mod contract;
mod error;

use std::env;

use anyhow::Result;
use clap::Parser;
use drink::{chain_api::ChainApi, Weight};
use sp_core::crypto::AccountId32;

use crate::{app_state::AppState, cli::CliCommand};

pub fn execute(app_state: &mut AppState) -> Result<()> {
    let command = app_state.ui_state.user_input.current_input().to_string();
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
        CliCommand::SetActor { actor } => {
            app_state.chain_info.actor = actor;
            app_state.print("Actor was set");
        }
        CliCommand::SetGasLimit {
            ref_time,
            proof_size,
        } => {
            app_state.chain_info.gas_limit = Weight::from_parts(ref_time, proof_size);
            app_state.print("Gas limit was set");
        }

        CliCommand::Build => contract::build(app_state),
        CliCommand::Deploy {
            constructor,
            args,
            salt,
        } => contract::deploy(app_state, constructor, args, salt),
        CliCommand::Call { message, args } => contract::call(app_state, message, args),
    }

    Ok(())
}

fn build_blocks(app_state: &mut AppState, count: u64) {
    for _ in 0..count {
        app_state.session.chain_api().build_block();
    }

    app_state.chain_info.block_height += count;

    app_state.print(&format!("{count} blocks built"));
}

fn add_tokens(app_state: &mut AppState, recipient: AccountId32, value: u128) {
    app_state
        .session
        .chain_api()
        .add_tokens(recipient.clone(), value);
    app_state.print(&format!("{value} tokens added to {recipient}", ));
}
