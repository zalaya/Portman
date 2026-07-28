use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::Style;
use ratatui::text::{ Line, Span };
use ratatui::widgets::{ Block, BorderType, Borders, Padding, Paragraph };

use super::theme;

pub fn render(frame: &mut Frame, area: Rect, filter: &str) {
    let text = Line::from(vec![
        Span::styled("> ", Style::default().fg(theme::SECONDARY)),
        Span::styled(filter, Style::default().fg(theme::PRIMARY)),
    ]);

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(theme::MUTED))
        .padding(Padding::horizontal(1));

    frame.render_widget(Paragraph::new(text).block(block), area);
}
