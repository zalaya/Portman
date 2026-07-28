use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::Style;
use ratatui::text::{ Line, Span };

use super::{ panel, theme };

pub fn render(frame: &mut Frame, area: Rect, filter: &str) {
    let text = Line::from(vec![
        Span::styled("> ", Style::default().fg(theme::SECONDARY)),
        Span::styled(filter, Style::default().fg(theme::PRIMARY)),
    ]);

    panel::single_line(frame, area, text, theme::MUTED);
}
