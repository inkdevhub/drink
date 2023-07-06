use std::{env, path::PathBuf};

pub use contracts::{Contract, ContractIndex, ContractRegistry};
use drink::{runtime::MinimalRuntime, session::Session, Weight, DEFAULT_ACTOR, DEFAULT_GAS_LIMIT};
use sp_core::crypto::AccountId32;
pub use user_input::UserInput;

use crate::app_state::output::Output;

mod contracts;
mod output;
pub mod print;
mod user_input;

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct ChainInfo {
    pub block_height: u64,
    pub actor: AccountId32,
    pub gas_limit: Weight,
}

impl Default for ChainInfo {
    fn default() -> Self {
        Self {
            block_height: 0,
            actor: DEFAULT_ACTOR,
            gas_limit: DEFAULT_GAS_LIMIT,
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub enum Mode {
    #[default]
    Managing,
    Drinking,
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct UiState {
    pub pwd: PathBuf,
    pub mode: Mode,

    pub user_input: UserInput,
    pub output: Output,

    pub show_help: bool,
}

impl Default for UiState {
    fn default() -> Self {
        UiState {
            pwd: env::current_dir().expect("Failed to get current directory"),
            mode: Default::default(),
            user_input: Default::default(),
            output: Default::default(),
            show_help: false,
        }
    }
}

pub struct AppState {
    pub session: Session<MinimalRuntime>,
    pub chain_info: ChainInfo,
    pub ui_state: UiState,
    pub contracts: ContractRegistry,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            session: Session::new(None).expect("Failed to create drinking session"),
            chain_info: Default::default(),
            ui_state: Default::default(),
            contracts: Default::default(),
        }
    }
}
