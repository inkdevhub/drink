mod current_env;
mod layout;

use std::{io, io::Stdout};

use anyhow::{anyhow, Result};
use crossterm::{
    event,
    event::{DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use layout::layout;
use ratatui::backend::{Backend, CrosstermBackend};

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
    loop {
        terminal.draw(|f| layout(f))?;
        if let Event::Key(key) = event::read()? {
            if let KeyCode::Char('q') = key.code {
                break;
            }
        }
    }
    Ok(())
}
