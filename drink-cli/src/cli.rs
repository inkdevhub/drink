use clap::Parser;

#[derive(Parser)]
pub enum CliCommand {
    Build,
    Deploy,
    Call,
    Exit,
}
