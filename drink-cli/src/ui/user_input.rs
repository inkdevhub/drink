use ratatui::{
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph, Widget},
};

use crate::app_state::{AppState, Mode};

pub(super) fn build(app_state: &mut AppState) -> impl Widget {
    let mut style = Style::default();
    if app_state.ui_state.mode != Mode::Drinking {
        style = style.fg(Color::DarkGray);
    }

    let block = Block::default()
        .title("User input")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .style(style);

    Paragraph::new(app_state.ui_state.user_input.clone()).block(block)
}
