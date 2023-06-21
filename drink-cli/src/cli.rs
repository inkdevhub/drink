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
        #[clap(long, default_value = "new")]
        constructor: String,
        #[clap(long, default_values_t = Vec::<u8>::new(), value_delimiter = ',')]
        salt: Vec<u8>,
    },
    Call {
        message: String,
    },
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
