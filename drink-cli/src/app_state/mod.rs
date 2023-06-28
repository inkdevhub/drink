use std::{env, path::PathBuf};

pub use contracts::{Contract, ContractIndex, ContractRegistry};
use drink::Sandbox;
pub use user_input::UserInput;

use crate::app_state::output::Output;

mod contracts;
mod output;
mod user_input;

#[derive(Clone, Eq, PartialEq, Hash, Debug, Default)]
pub struct ChainInfo {
    pub block_height: u64,
    pub deployed_contracts: u16,
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
    pub sandbox: Sandbox,
    pub chain_info: ChainInfo,
    pub ui_state: UiState,
    pub contracts: ContractRegistry,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            sandbox: Sandbox::new().expect("Failed to create sandbox"),
            chain_info: Default::default(),
            ui_state: Default::default(),
            contracts: Default::default(),
        }
    }
}
