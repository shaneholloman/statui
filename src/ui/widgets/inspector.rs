use ratatui::{
    Frame,
    layout::Rect,
    style::{Style, Stylize},
    symbols,
    text::Line,
    widgets::{Block, Paragraph, Wrap},
};

use crate::state::{App, EndpointState};

pub fn render_inspector(frame: &mut Frame, app: &mut App, chunk: Rect) {
    let Some(selected) = app.table_state.selected() else {
        return;
    };

    let Some(endpoint_name) = app.endpoint_order.get(selected) else {
        return;
    };

    let Some(endpoint_state) = app.endpoint_states.get(endpoint_name) else {
        return;
    };

    let title_block = Block::bordered()
        .title(
            Line::from(format!("Inspector: {}", endpoint_name))
                .left_aligned()
                .style(Style::new().red().italic()),
        )
        .border_set(symbols::border::DOUBLE)
        .border_style(Style::new().yellow());

    let par = Paragraph::new(create_lines(endpoint_state))
        .block(title_block)
        .gray()
        .centered()
        .wrap(Wrap { trim: true });

    frame.render_widget(par, chunk);
}

fn create_lines(endpoint_state: &EndpointState) -> Vec<Line<'static>> {
    vec![
        Line::from(format!("URL: {}", endpoint_state.url)).left_aligned(),
        Line::from(format!("Method: {}", endpoint_state.method)).left_aligned(),
        Line::from(""),
        Line::from(""),
    ]
}
