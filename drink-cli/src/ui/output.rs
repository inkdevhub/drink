use ratatui::widgets::{Block, BorderType, Borders, Paragraph, Widget};

use crate::app_state::AppState;

pub(super) fn build(app_state: &AppState) -> impl Widget {
    let block = Block::default()
        .title("Output")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);

    Paragraph::new(app_state.ui_state.output.clone()).block(block)
}
