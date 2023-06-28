use ratatui::{
    style::{Color, Style},
    text::{Line, Span},
    widgets::{List, ListItem, Widget},
};

use crate::{
    app_state::{AppState, ContractIndex},
    ui::layout::section,
};

pub(super) fn build(app_state: &mut AppState) -> impl Widget {
    let items = app_state
        .contracts
        .get_all()
        .iter()
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

    List::new(items).block(section("Deployed contracts"))
}
