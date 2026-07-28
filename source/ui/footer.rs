use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{ Modifier, Style };
use ratatui::text::{ Line, Span };
use ratatui::widgets::{ Block, BorderType, Borders, Padding, Paragraph };

use super::theme;

pub fn render(frame: &mut Frame, area: Rect) {
    let mut spans = Vec::new();

    for (index, (key, label)) in [
        ("↑/↓", "move"),
        ("type", "search"),
        ("ctrl+r", "refresh"),
        ("esc", "quit"),
    ]
    .into_iter()
    .enumerate()
    {
        if index > 0 {
            spans.push(separator());
        }

        spans.extend(hint(key, label));
    }

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(theme::MUTED))
        .padding(Padding::horizontal(2));

    frame.render_widget(Paragraph::new(Line::from(spans)).block(block), area);
}

fn hint(key: &str, label: &str) -> [Span<'static>; 2] {
    [
        Span::styled(key.to_string(), Style::default().fg(theme::SECONDARY).add_modifier(Modifier::BOLD)),
        Span::styled(format!(" {label}"), Style::default().fg(theme::MUTED)),
    ]
}

fn separator() -> Span<'static> {
    Span::styled("   ·   ", Style::default().fg(theme::MUTED))
}
