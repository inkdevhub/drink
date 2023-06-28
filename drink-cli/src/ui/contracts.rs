use ratatui::{
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, List, ListItem, Padding, Widget},
};

use crate::app_state::{AppState, ContractIndex};

pub(super) fn build(app_state: &mut AppState) -> impl Widget {
    let block = Block::default()
        .title("Deployed contracts")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .padding(Padding::horizontal(1));

    let items = app_state
        .contracts
        .get_all()
        .into_iter()
        .enumerate()
        .map(|(idx, contract)| {
            let style = match app_state.contracts.current_index() {
                ContractIndex::CurrentContract(cc) if cc == idx => {
                    Style::default().bg(Color::White).fg(Color::Black)
                }
                _ => Style::default(),
            };

            ListItem::new(Line::from(Span::styled(
                format!("{} / {}", contract.name, &contract.address.to_string()[..8],),
                style,
            )))
        })
        .collect::<Vec<_>>();

    List::new(items).block(block)
}
