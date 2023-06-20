use ratatui::{
    style::{Color, Modifier, Style},
    text::Span,
};

use crate::app_state::AppState;

impl AppState {
    pub fn print_command(&mut self, command: &str) {
        self.ui_state.output.push("".into());
        self.ui_state.output.push(
            Span::styled(
                format!("Executing `{command}`"),
                Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::BOLD)
                    .add_modifier(Modifier::ITALIC),
            )
            .into(),
        );
    }

    pub fn print(&mut self, msg: &str) {
        self.ui_state.output.push(
            Span::styled(
                msg.to_string(),
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            )
            .into(),
        );
    }

    pub fn print_error(&mut self, err: &str) {
        for line in err.split('\n') {
            self.ui_state.output.push(
                Span::styled(
                    line.to_string(),
                    Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
                )
                .into(),
            )
        }
    }
}
