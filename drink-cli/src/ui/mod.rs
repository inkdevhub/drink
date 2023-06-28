mod contracts;
mod current_env;
mod footer;
mod help;
mod layout;
mod output;
mod print;
mod user_input;

use std::{io, io::Stdout};

use anyhow::{anyhow, Result};
use crossterm::{
    event,
    event::{DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use layout::layout;
use ratatui::backend::CrosstermBackend;

use crate::{
    app_state::{
        AppState,
        Mode::{Drinking, Managing},
    },
    executor::execute,
};

type Terminal = ratatui::Terminal<CrosstermBackend<Stdout>>;

pub fn run_ui() -> Result<()> {
    let mut terminal = setup_dedicated_terminal()?;
    let app_result = run_ui_app(&mut terminal);
    restore_original_terminal(terminal)?;
    app_result
}

fn setup_dedicated_terminal() -> Result<Terminal> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    Terminal::new(backend).map_err(|e| anyhow!(e))
}

fn restore_original_terminal(mut terminal: Terminal) -> Result<()> {
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor().map_err(|e| anyhow!(e))
}

fn run_ui_app(terminal: &mut Terminal) -> Result<()> {
    let mut app_state = AppState::default();

    loop {
        terminal.draw(|f| layout(f, &mut app_state))?;

        let mode = &mut app_state.ui_state.mode;
        if let Event::Key(key) = event::read()? {
            match (*mode, key.code) {
                (_, KeyCode::Esc) => *mode = Managing,

                (Managing, KeyCode::Char('q')) => break,
                (Managing, KeyCode::Char('i')) => {
                    *mode = Drinking;
                    app_state.ui_state.show_help = false;
                }
                (Managing, KeyCode::Char('h')) => {
                    app_state.ui_state.show_help = !app_state.ui_state.show_help
                }
                (Managing, KeyCode::Down) => {
                    app_state.ui_state.output_scrolling = true;
                    app_state.ui_state.output_offset += 1
                }
                (Managing, KeyCode::Up) if app_state.ui_state.output_offset > 0 => {
                    app_state.ui_state.output_scrolling = true;
                    app_state.ui_state.output_offset -= 1
                }

                (Drinking, KeyCode::Char(c)) => app_state.ui_state.user_input.push(c),
                (Drinking, KeyCode::Backspace) => {
                    app_state.ui_state.user_input.pop();
                }
                (Drinking, KeyCode::Tab) => {
                    let prev_path = match app_state.contracts.current_contract() {
                        Some(c) => c.base_path.clone(),
                        None => continue,
                    };

                    let new_path = &app_state
                        .contracts
                        .next()
                        .expect("There is at least one contract - just checked")
                        .base_path;

                    if *new_path != prev_path {
                        let base_path = new_path.to_str().unwrap();
                        app_state.ui_state.user_input.set(format!("cd {base_path}"));
                        execute(&mut app_state)?;
                        app_state.ui_state.user_input.set(String::new());
                    }
                }
                (Drinking, KeyCode::Up) => app_state.ui_state.user_input.prev_input(),
                (Drinking, KeyCode::Down) => app_state.ui_state.user_input.next_input(),
                (Drinking, KeyCode::Enter) => {
                    execute(&mut app_state)?;
                    app_state.ui_state.user_input.apply();
                    app_state.ui_state.output_scrolling = false;
                }

                _ => {}
            }
        }
    }
    Ok(())
}
