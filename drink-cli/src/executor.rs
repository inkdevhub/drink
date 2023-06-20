use anyhow::Result;

use crate::app_state::AppState;

pub fn execute(app_state: &mut AppState) -> Result<()> {
    let command = app_state.ui_state.user_input.clone();
    app_state
        .ui_state
        .output
        .push(format!("Executing: {}", command));
    Ok(())
}
