use ratatui::widgets::{Paragraph, Widget};

use crate::{app_state::AppState, ui::layout::section};

pub(super) fn build(app_state: &AppState) -> impl Widget {
    Paragraph::new(app_state.ui_state.output.clone())
        .block(section("Output"))
        .scroll((app_state.ui_state.output_offset.max(0) as u16, 0))
}
