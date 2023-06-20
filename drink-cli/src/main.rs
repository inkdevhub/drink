use anyhow::Result;

use crate::ui::run_ui;

mod app_state;
mod cli;
mod executor;
mod ui;

fn main() -> Result<()> {
    run_ui()
}
