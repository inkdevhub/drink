use std::{env, path::PathBuf};

use contract_transcode::ContractMessageTranscoder;
use drink::Sandbox;
use ratatui::text::Line;
use sp_core::crypto::AccountId32;

use crate::user_input::UserInput;

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
    pub contract_project_name: String,
    pub mode: Mode,

    pub user_input: UserInput,
    pub current_contract: usize,

    pub show_help: bool,
    pub output: Vec<Line<'static>>,
    pub output_offset: i16,
    pub output_scrolling: bool,
}

impl Default for UiState {
    fn default() -> Self {
        UiState {
            pwd: env::current_dir().expect("Failed to get current directory"),
            contract_project_name: Default::default(),
            mode: Default::default(),
            user_input: Default::default(),
            current_contract: 0,
            show_help: false,
            output: Default::default(),
            output_offset: 0,
            output_scrolling: false,
        }
    }
}

pub struct Contract {
    pub name: String,
    pub address: AccountId32,
    pub base_path: PathBuf,
    pub transcode: ContractMessageTranscoder,
}

pub struct AppState {
    pub sandbox: Sandbox,
    pub chain_info: ChainInfo,
    pub ui_state: UiState,
    pub contracts: Vec<Contract>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            sandbox: Sandbox::new().expect("Failed to create sandbox"),
            chain_info: Default::default(),
            ui_state: Default::default(),
            contracts: vec![],
        }
    }
}
