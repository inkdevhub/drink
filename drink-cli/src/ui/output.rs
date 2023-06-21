use ratatui::widgets::{Block, BorderType, Borders, Padding, Paragraph, Widget};

use crate::app_state::AppState;

pub(super) fn build(app_state: &AppState) -> impl Widget {
    let block = Block::default()
        .title("Output")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .padding(Padding::horizontal(1));

    Paragraph::new(app_state.ui_state.output.clone())
        .block(block)
        .scroll((app_state.ui_state.output_offset.max(0) as u16, 0))
}
