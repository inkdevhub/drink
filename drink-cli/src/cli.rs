use clap::Parser;
use sp_core::crypto::{AccountId32, Ss58Codec};

#[derive(Parser)]
pub enum CliCommand {
    #[clap(alias = "c")]
    Clear,
    #[clap(alias = "cd")]
    ChangeDir {
        path: String,
    },

    #[clap(alias = "nb")]
    NextBlock {
        #[clap(default_value = "1")]
        count: u64,
    },
    AddTokens {
        #[clap(value_parser = AccountId32::from_ss58check)]
        recipient: AccountId32,
        value: u128,
    },
    SetActor {
        #[clap(value_parser = AccountId32::from_ss58check)]
        actor: AccountId32,
    },
    SetGasLimit {
        ref_time: u64,
        proof_size: u64,
    },

    #[clap(alias = "b")]
    Build,
    #[clap(alias = "d")]
    Deploy {
        #[clap(long, default_value = "new")]
        constructor: String,
        args: Vec<String>,
        #[clap(long, default_values_t = Vec::<u8>::new(), value_delimiter = ',')]
        salt: Vec<u8>,
    },
    Call {
        message: String,
        args: Vec<String>,
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
