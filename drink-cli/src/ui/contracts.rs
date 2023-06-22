use ratatui::{
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, List, ListItem, Padding, Widget},
};

use crate::app_state::AppState;

pub(super) fn build(app_state: &mut AppState) -> impl Widget {
    let block = Block::default()
        .title("Deployed contracts")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .padding(Padding::horizontal(1));

    let items = app_state
        .contracts
        .iter()
        .enumerate()
        .map(|(idx, contract)| {
            let style = if idx == app_state.ui_state.current_contract {
                Style::default().bg(Color::White).fg(Color::Black)
            } else {
                Style::default()
            };

            ListItem::new(Line::from(Span::styled(
                format!("{} / {}", contract.name, &contract.address.to_string()[..8],),
                style,
            )))
        })
        .collect::<Vec<_>>();

    List::new(items).block(block)
}
