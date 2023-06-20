use clap::Parser;

#[derive(Parser)]
pub enum CliCommand {
    #[clap(alias = "c")]
    Clear,
    #[clap(alias = "cd")]
    ChangeDir {
        path: String,
    },

    #[clap(alias = "b")]
    Build,
    #[clap(alias = "d")]
    Deploy {
        #[clap(default_value = "new")]
        constructor: String,
        #[clap(default_values_t = Vec::<u8>::new())]
        salt: Vec<u8>,
    },
    CallGet,
    CallFlip,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_cli() {
        use clap::CommandFactory;
        CliCommand::command().debug_assert()
    }
}
