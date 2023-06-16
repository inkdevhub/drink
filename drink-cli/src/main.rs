use std::io;
use clap::Parser;
use cli::CliCommand;

mod cli;

fn main() {
    loop {
        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).expect("Failed to get user input");

        let cli_command = match CliCommand::try_parse_from(&["", user_input.trim()]) {
            Ok(cli_command) => cli_command,
            Err(_) => {
                eprintln!("Invalid command");
                continue;
            }
        };

        match cli_command {
            CliCommand::Build => {
                println!("Build");
            }
            CliCommand::Deploy => {
                println!("Deploy");
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
