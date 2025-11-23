use color_eyre::Result;
use std::time::Duration;

use ratatui::{
    Terminal,
    backend::Backend,
    crossterm::event::{self, Event, KeyCode},
};

use tokio::sync::mpsc::Receiver;

use crate::backend::CheckResult;
use crate::{state::App, ui};

/// TUI entry point that handles drawing the ui, handling input, and displaying
/// results of a check.
pub async fn run_app(
    mut app: &mut App,
    terminal: &mut Terminal<impl Backend>,
    mut rx: Receiver<CheckResult>,
) -> Result<()> {
    loop {
        // 1. Draw the UI
        terminal.draw(|f| ui::render_ui(f, &mut app))?;

        // 2. Handle input
        // Simple input handling to quit on q
        // TODO: replace with keymap and actions model later
        // to handle all types of key inputs
        if event::poll(Duration::from_millis(250))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    // 'q' was pressed, so we quit
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Char('k') => app.previous_row(),
                    KeyCode::Char('j') => app.next_row(),
                    _ => continue,
                }
            }
        }

        // 3. Handle messages from the backend
        // After drawing and handling input, we check for new messages.
        // We use 'try_recv' in a loop to drain the channel of all
        // pending messages without blocking.
        while let Ok(result) = rx.try_recv() {
            app.on_result(result);
        }
    }
}
