use ratatui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout},
    widgets::{Block, BorderType, Borders, Paragraph, Widget},
    Frame,
};

use crate::ui::current_env;

pub(super) fn layout<B: Backend>(f: &mut Frame<B>) {
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

    f.render_widget(current_env::build(), chunks[0]);
    let block = Block::default()
        .title("Output")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);
    f.render_widget(block, chunks[1]);
    let block = Block::default()
        .title("User input")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);
    f.render_widget(block, chunks[2]);
    f.render_widget(build_footer(), chunks[3]);
}

fn build_footer() -> impl Widget {
    let base = Block::default()
        .title("Help")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);
    Paragraph::new("Press `q` to exit.\nMade by Aleph Zero Foundation")
        .alignment(Alignment::Center)
        .block(base)
}
