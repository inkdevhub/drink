use ratatui::{
    layout::Alignment,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph, Widget},
};

use crate::app_state::{AppState, Mode};

pub(super) fn build(app_state: &AppState) -> impl Widget {
    let base = Block::default()
        .title("Help")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);

    let instruction: Line = match app_state.ui_state.mode {
        Mode::Managing => vec![
            Span::raw("Use arrows to scroll through output. Press "),
            Span::styled("'q'", Style::default().fg(Color::Yellow)),
            Span::raw(" to quit. Press "),
            Span::styled("'i'", Style::default().fg(Color::Yellow)),
            Span::raw(" to enter editing mode"),
        ],
        Mode::Editing => vec![
            Span::raw("Press "),
            Span::styled("'Esc'", Style::default().fg(Color::Yellow)),
            Span::raw(" to quit editing mode"),
        ],
    }
    .into();

    Paragraph::new(vec![
        instruction,
        Span::styled(
            "Made by Aleph Zero Foundation",
            Style::default().add_modifier(Modifier::ITALIC),
        )
        .into(),
    ])
    .alignment(Alignment::Center)
    .block(base)
}
