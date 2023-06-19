use std::env;

use ratatui::{
    layout::Alignment,
    widgets::{Block, BorderType, Borders, Padding, Paragraph, Widget, Wrap},
};
use sp_runtime::AccountId32;

use crate::app_state::AppState;

pub(super) fn build() -> impl Widget {
    let block = Block::default()
        .title("Current environment")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .padding(Padding::horizontal(1));

    let app_state = AppState {
        pwd: env::current_dir().unwrap(),
        block_height: 1,
        deployed_contracts: 0,
        current_contract_address: AccountId32::new([0; 32]),
    };

    Paragraph::new(format!(
        r#"Current working directory: {}
Block height: {}
Deployed contracts: {}
Current contract address: {}"#,
        app_state.pwd.to_str().unwrap(),
        app_state.block_height,
        app_state.deployed_contracts,
        app_state.current_contract_address
    ))
    .alignment(Alignment::Left)
    .wrap(Wrap { trim: false })
    .block(block)
}
