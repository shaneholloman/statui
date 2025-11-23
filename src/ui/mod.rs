mod widgets;

use ratatui::prelude::*;

use crate::state::App;
use widgets::{render_table, render_welcome_message};

/// Renders the UI widgets for the application.
pub fn render_ui(frame: &mut Frame, mut app: &mut App) {
    if app.endpoint_order.is_empty() {
        render_welcome_message(frame);
        return;
    }

    render_table(frame, &mut app)
}
