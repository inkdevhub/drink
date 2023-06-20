use ratatui::{
    layout::Alignment,
    widgets::{Block, BorderType, Borders, Paragraph, Widget},
};

use crate::app_state::{AppState, Mode};

pub(super) fn build(app_state: &AppState) -> impl Widget {
    let base = Block::default()
        .title("Help")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);

    let instruction = match app_state.ui_state.mode {
        Mode::Managing => "Press 'q' to quit. Press `i` to enter editing mode",
        Mode::Editing => "Press 'Esc' to quit editing mode",
    };

    Paragraph::new(format!("{instruction}\nMade by Aleph Zero Foundation"))
        .alignment(Alignment::Center)
        .block(base)
}
