use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{ Modifier, Style };
use ratatui::text::{ Line, Span };

use super::{ panel, theme };

pub fn render(frame: &mut Frame, area: Rect, message: &str) {
    let text = Line::from(Span::styled(message, Style::default().fg(theme::DANGER).add_modifier(Modifier::BOLD)));

    panel::single_line(frame, area, text, theme::DANGER);
}
