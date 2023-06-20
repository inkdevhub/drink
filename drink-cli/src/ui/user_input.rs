use ratatui::widgets::{Block, BorderType, Borders, Paragraph, Widget};

use crate::app_state::AppState;

pub(super) fn build(app_state: &mut AppState) -> impl Widget {
    let block = Block::default()
        .title("User input")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);

    Paragraph::new(app_state.ui_state.user_input.clone()).block(block)
}
