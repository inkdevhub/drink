use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;

use crate::ui::run_ui;

mod app_state;
mod cli;
mod executor;
mod ui;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Starts the CLI in the provided directory
    #[arg(short, long, value_name = "DIRECTORY", )]
    path: Option<PathBuf>,
}

fn main() -> Result<()> {
    let args = Args::parse();
    run_ui(args.path)
}
