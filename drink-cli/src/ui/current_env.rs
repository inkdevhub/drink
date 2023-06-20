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

    Paragraph::new(format!(
        r#"Current working directory: {}
Block height: {}
Deployed contracts: {}
Current contract address: {}"#,
        app_state.ui_state.pwd.to_str().unwrap(),
        app_state.chain_info.block_height,
        app_state.chain_info.deployed_contracts,
        app_state
            .chain_info
            .current_contract_address
            .as_ref()
            .map_or("<none>".to_string(), |addr| format!("{addr}"))
    ))
    .alignment(Alignment::Left)
    .wrap(Wrap { trim: false })
    .block(block)
}
