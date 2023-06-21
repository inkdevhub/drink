use ratatui::{
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Padding, Paragraph, Widget},
};

use crate::app_state::AppState;

pub(super) fn build(_app_state: &AppState) -> impl Widget {
    let block = Block::default()
        .title("Help")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .padding(Padding::horizontal(1));

    Paragraph::new(vec![
        command("cd <dir>", "change directory do <dir>"),
        command("clear / c", "clear output tab"),
        command(
            "build / b",
            "build contract from the sources in the current directory",
        ),
        command(
            "deploy / d [--constructor <name>] [--salt <salt>]",
            "deploy contract using <constructor> (`new` by default) and <salt> (empty by default)",
        ),
        command("call <message>", "call contract's message"),
        command(
            "next-block / nb [count]",
            "build next <count> blocks (by default a single block)",
        ),
        command(
            "add-tokens <recipient> <value>",
            "add <value> tokens to <recipient>",
        ),
    ])
    .block(block)
}

fn command(command: &'static str, description: &'static str) -> Line<'static> {
    Line::from(vec![
        Span::styled(command, Style::default().fg(Color::Green)),
        format!(": {description}").into(),
    ])
}
