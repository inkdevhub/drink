use std::{env, path::PathBuf};

pub use contracts::{Contract, ContractIndex, ContractRegistry};
use drink::{runtime::MinimalSandbox, session::Session, Sandbox, Weight, DEFAULT_GAS_LIMIT};
use sp_core::crypto::AccountId32;
pub use user_input::UserInput;

use crate::app_state::output::Output;

mod contracts;
mod output;
pub mod print;
mod user_input;

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct ChainInfo {
    pub block_height: u32,
    pub actor: AccountId32,
    pub gas_limit: Weight,
}

impl Default for ChainInfo {
    fn default() -> Self {
        Self {
            block_height: 0,
            actor: MinimalSandbox::default_actor(),
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
    pub cwd: PathBuf,
    pub mode: Mode,

    pub user_input: UserInput,
    pub output: Output,

    pub show_help: bool,
}

impl UiState {
    pub fn new(cwd_override: Option<PathBuf>) -> Self {
        let cwd = cwd_override
            .unwrap_or_else(|| env::current_dir().expect("Failed to get current directory"));

        UiState {
            cwd,
            mode: Default::default(),
            user_input: Default::default(),
            output: Default::default(),
            show_help: false,
        }
    }
}

impl Default for UiState {
    fn default() -> Self {
        UiState::new(None)
    }
}

pub struct AppState {
    pub session: Session<MinimalSandbox>,
    pub chain_info: ChainInfo,
    pub ui_state: UiState,
    pub contracts: ContractRegistry,
}

impl AppState {
    pub fn new(cwd_override: Option<PathBuf>) -> Self {
        AppState {
            session: Session::default(),
            chain_info: Default::default(),
            ui_state: UiState::new(cwd_override),
            contracts: Default::default(),
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new(None)
    }
}
