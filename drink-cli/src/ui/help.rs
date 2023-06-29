use ratatui::{
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Paragraph, Widget},
};

use crate::{app_state::AppState, ui::layout::section};

pub(super) fn build(_app_state: &AppState) -> impl Widget {
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
        command(
            "set-actor <account>",
            "set <account> as the current actor (transaction sender)",
        ),
        command(
            "set-gas-limit <ref_time> <proof_size>",
            "set gas limits to <ref_time> and <proof_size>",
        ),
    ])
    .block(section("Help"))
}

fn command(command: &'static str, description: &'static str) -> Line<'static> {
    Line::from(vec![
        Span::styled(command, Style::default().fg(Color::Green)),
        format!(": {description}").into(),
    ])
}
