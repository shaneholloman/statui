use ratatui::{
    Frame,
    layout::Alignment,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

use crate::{
    config,
    ui::{theme::Theme, util},
};

pub fn render_welcome_message(frame: &mut Frame) {
    let text = vec![
        Line::from("Welcome to Statui!").style(
            Style::default()
                .fg(Theme::BORDER_FOCUSED)
                .add_modifier(Modifier::BOLD),
        ),
        Line::from(""),
        Line::from("No endpoints loaded."),
        Line::from(""),
        Line::from(vec![
            Span::raw("To get started, create a "),
            Span::styled(
                "statui.toml",
                Style::default()
                    .fg(Theme::BORDER_FOCUSED)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw(" file in this directory,"),
        ]),
        // The Solution (Global)
        Line::from(vec![
            Span::raw("or a "),
            Span::styled(
                "config.toml",
                Style::default()
                    .fg(Theme::BORDER_FOCUSED)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw(" file in your app config folder:"),
        ]),
        Line::from(Span::styled(
            config::get_default_config_dir(),
            Style::default().fg(Theme::DESC_FG),
        )),
        Line::from(""),
        Line::from("Add your endpoints to the file and restart the app."),
        Line::from(""),
        Line::from(vec![
            Span::raw("Press "),
            Span::styled(
                "q",
                Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
            ),
            Span::raw(" to quit."),
        ]),
    ];
    let h = text.len() as u16;
    let w = text.iter().map(|line| line.width()).max().unwrap_or(0) as u16;

    let outer_border = Block::default()
        .title("Welcome")
        .borders(Borders::ALL)
        .border_style(Theme::table_border_style());

    let paragraph = Paragraph::new(text)
        .block(Block::default())
        .alignment(Alignment::Center);

    let popup_area = util::centered_rect(70, 60, frame.area());
    let text_area = util::centered_area(h + 2, w + 2, popup_area);
    frame.render_widget(outer_border, popup_area);
    frame.render_widget(paragraph, text_area);
}
