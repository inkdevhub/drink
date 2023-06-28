use ratatui::{
    layout::Alignment,
    widgets::{Block, BorderType, Borders, Padding, Paragraph, Widget, Wrap},
};

use crate::app_state::AppState;

pub(super) fn build(app_state: &mut AppState) -> impl Widget {
    let block = Block::default()
        .title("Current environment")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .padding(Padding::horizontal(1));

    let current_contract_info = match app_state.contracts.current_contract() {
        Some(contract) => format!("name: {} | address: {}", contract.name, contract.address),
        None => "No deployed contract".to_string(),
    };

    Paragraph::new(format!(
        r#"Current working directory: {}
Block height: {}
Deployed contracts: {}
Current contract: {{ {} }}"#,
        app_state.ui_state.pwd.to_str().unwrap(),
        app_state.chain_info.block_height,
        app_state.chain_info.deployed_contracts,
        current_contract_info
    ))
    .alignment(Alignment::Left)
    .wrap(Wrap { trim: false })
    .block(block)
}
