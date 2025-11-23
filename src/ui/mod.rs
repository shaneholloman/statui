pub mod fx;
mod util;
mod widgets;

use ratatui::prelude::*;

use crate::state::{App, AppMode};
use widgets::{
    footer::render_footer, 
    inspector::render_inspector,
    table::render_table,
    welcome::render_welcome_message,
};

/// Renders the UI widgets for the application.
pub fn render_ui(frame: &mut Frame, mut app: &mut App) {
    if app.endpoint_order.is_empty() {
        render_welcome_message(frame);
        return;
    }

    let root = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(2)])
        .split(frame.area());

    let workspace_area = root[0];
    let footer_area = root[1];

    let constraints = match app.mode {
        AppMode::Normal => vec![Constraint::Percentage(100)],
        AppMode::Inspecting => vec![Constraint::Percentage(50), Constraint::Percentage(50)],
    };

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(constraints)
        .split(workspace_area);

    render_table(frame, &mut app, chunks[0]);

    if app.mode == AppMode::Inspecting {
        render_inspector(frame, &mut app, chunks[1]);
    }

    render_footer(frame, footer_area);
}
