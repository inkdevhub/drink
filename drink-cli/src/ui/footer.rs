use ratatui::{
    layout::Alignment,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Paragraph, Widget},
};

use crate::{
    app_state::{AppState, Mode},
    ui::layout::section,
};

pub(super) fn build(app_state: &AppState) -> impl Widget {
    let instruction: Line = match app_state.ui_state.mode {
        Mode::Managing => alternate_help([
            "Use arrows to scroll through output. Press ",
            "'q'",
            " to quit. Press ",
            "'h'",
            " to see help. Press ",
            "'i'",
            " to enter editing mode.",
        ]),
        Mode::Drinking => alternate_help([
            "Press ",
            "'Esc'",
            " to quit editing mode. Use ",
            "'Tab'",
            " to switch between deployed contracts.",
        ]),
    };

    Paragraph::new(vec![
        instruction,
        Span::styled(
            "Made by Aleph Zero Foundation",
            Style::default().add_modifier(Modifier::ITALIC),
        )
        .into(),
    ])
    .alignment(Alignment::Center)
    .block(section("Help"))
}

fn alternate_help<I: IntoIterator<Item = &'static str>>(items: I) -> Line<'static> {
    items
        .into_iter()
        .enumerate()
        .map(|(idx, item)| match idx % 2 {
            0 => Span::raw(item),
            _ => Span::styled(item, Style::default().fg(Color::Yellow)),
        })
        .collect::<Vec<_>>()
        .into()
}
