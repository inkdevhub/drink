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
    pub mode: Mode,

    pub user_input: String,

    pub output: Vec<String>,
    pub output_offset: u16,
}

impl Default for UiState {
    fn default() -> Self {
        UiState {
            pwd: env::current_dir().expect("Failed to get current directory"),
            mode: Default::default(),
            user_input: Default::default(),
            output: Default::default(),
            output_offset: 0,
        }
    }
}

#[derive(Clone, Eq, PartialEq, Hash, Debug, Default)]
pub struct AppState {
    pub chain_info: ChainInfo,
    pub ui_state: UiState,
}
