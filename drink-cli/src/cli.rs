use clap::Parser;

#[derive(Parser)]
pub enum CliCommand {
    #[clap(alias = "b")]
    Build,
    #[clap(alias = "d")]
    Deploy,
    #[clap(alias = "c")]
    Call,
    #[clap(alias = "e")]
    Exit,
}
