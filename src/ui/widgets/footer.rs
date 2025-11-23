use ratatui::{
    Frame,
    layout::{Alignment, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

pub fn render_footer(frame: &mut Frame, area: Rect) {
    let controls = [
        ("q", "Quit"),
        ("j/k", "Up/Down"),
        ("i", "Inspect"),
        // ("/", "Filter"),
        // ("?", "Help"),
    ];

    let spans: Vec<Span> = controls
        .iter()
        .flat_map(|(key, desc)| {
            let key_text = format!(" {} ", key);
            let desc_text = format!(" {} ", desc);

            vec![
                // Key: Black text on Cyan background 
                Span::styled(
                    key_text,
                    Style::default().fg(Color::Black).bg(Color::Cyan).add_modifier(Modifier::BOLD),
                ),

                // Description: Dark Gray text on the default background
                Span::styled(
                    desc_text,
                    Style::default().fg(Color::DarkGray).bg(Color::Reset),
                ),
                // A Spacer 
                Span::from(" "),
            ]
        })
        .collect();

    let footer = Paragraph::new(Line::from(spans))
        .alignment(Alignment::Left)
        .block(Block::default().borders(Borders::TOP));

    frame.render_widget(footer, area);
}
