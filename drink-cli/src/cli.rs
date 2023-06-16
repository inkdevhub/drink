use clap::Parser;

#[derive(Parser)]
pub enum CliCommand {
    #[clap(alias = "b")]
    Build,
    #[clap(alias = "d")]
    Deploy,
    CallGet,
    CallFlip,
    #[clap(alias = "e")]
    Exit,
}
