use std::{env, path::PathBuf};

use sp_runtime::AccountId32;

#[derive(Clone, Eq, PartialEq, Hash, Debug, Default)]
pub struct ChainInfo {
    pub block_height: u64,
    pub deployed_contracts: u16,
    pub current_contract_address: Option<AccountId32>,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum Mode {
    Managing,
    Editing,
}

impl Default for Mode {
    fn default() -> Self {
        Mode::Managing
    }
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct UiState {
    pub pwd: PathBuf,
    pub user_input: String,
    pub mode: Mode,
    pub output: Vec<String>,
}

impl Default for UiState {
    fn default() -> Self {
        UiState {
            pwd: env::current_dir().expect("Failed to get current directory"),
            user_input: Default::default(),
            mode: Default::default(),
            output: Default::default(),
        }
    }
}

#[derive(Clone, Eq, PartialEq, Hash, Debug, Default)]
pub struct AppState {
    pub chain_info: ChainInfo,
    pub ui_state: UiState,
}
