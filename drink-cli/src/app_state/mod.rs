use std::{env, path::PathBuf};

pub use contracts::{Contract, ContractIndex, ContractRegistry};
use drink::Sandbox;
use ratatui::text::Line;
pub use user_input::UserInput;

mod contracts;
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

    pub show_help: bool,
    pub output: Vec<Line<'static>>,
    pub output_offset: u16,
    pub output_scrolling: bool,
}

impl Default for UiState {
    fn default() -> Self {
        UiState {
            pwd: env::current_dir().expect("Failed to get current directory"),
            mode: Default::default(),
            user_input: Default::default(),
            show_help: false,
            output: Default::default(),
            output_offset: 0,
            output_scrolling: false,
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
