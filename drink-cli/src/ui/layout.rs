use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    Frame,
};

use crate::{
    app_state::AppState,
    ui::{current_env, footer, output, user_input},
};

pub(super) fn layout<B: Backend>(f: &mut Frame<B>, app_state: &mut AppState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Ratio(4, 20),
                Constraint::Ratio(12, 20),
                Constraint::Ratio(2, 20),
                Constraint::Ratio(2, 20),
            ]
            .as_ref(),
        )
        .split(f.size());

    f.render_widget(current_env::build(app_state), chunks[0]);
    f.render_widget(output::build(app_state), chunks[1]);
    f.render_widget(user_input::build(app_state), chunks[2]);
    f.render_widget(footer::build(app_state), chunks[3]);
}
