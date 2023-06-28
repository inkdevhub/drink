use ratatui::{
    style::{Color, Style},
    widgets::{Paragraph, Widget},
};

use crate::{
    app_state::{AppState, Mode},
    ui::layout::section,
};

pub(super) fn build(app_state: &mut AppState) -> impl Widget {
    let mut style = Style::default();
    if app_state.ui_state.mode != Mode::Drinking {
        style = style.fg(Color::DarkGray);
    }

    let block = section("User input").style(style);
    Paragraph::new(app_state.ui_state.user_input.current_input().to_string()).block(block)
}
