use ratatui::{
    text::Line,
    widgets::{Block, BorderType, Borders, Padding, Paragraph, Widget},
};

use crate::app_state::AppState;

pub(super) fn build(app_state: &AppState) -> impl Widget {
    let block = Block::default()
        .title("Output")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .padding(Padding::horizontal(1));

    let output = app_state
        .ui_state
        .output
        .iter()
        .rev()
        .map(|s| Line::from(s.clone()))
        .collect::<Vec<_>>();

    Paragraph::new(output)
        .block(block)
        .scroll((app_state.ui_state.output_offset, 0))
}
