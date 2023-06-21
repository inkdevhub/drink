use std::{env, path::PathBuf};

use drink::Sandbox;
use ratatui::text::Line;
use sp_runtime::AccountId32;

#[derive(Clone, Eq, PartialEq, Hash, Debug, Default)]
pub struct ChainInfo {
    pub block_height: u64,
    pub deployed_contracts: u16,
    pub current_contract_address: Option<AccountId32>,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub enum Mode {
    #[default]
    Managing,
    Editing,
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct UiState {
    pub pwd: PathBuf,
    pub mode: Mode,

    pub user_input: String,

    pub output: Vec<Line<'static>>,
    pub output_offset: i16,
    pub output_scrolling: bool,
}

impl Default for UiState {
    fn default() -> Self {
        UiState {
            pwd: env::current_dir().expect("Failed to get current directory"),
            mode: Default::default(),
            user_input: Default::default(),
            output: Default::default(),
            output_offset: 0,
            output_scrolling: false,
        }
    }
}

#[derive(Default)]
pub struct AppState {
    pub sandbox: Sandbox,
    pub chain_info: ChainInfo,
    pub ui_state: UiState,
}
