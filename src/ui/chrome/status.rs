use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{ Modifier, Style };
use ratatui::text::{ Line, Span };

use crate::ui::theme;
use crate::ui::widgets::panel;

pub fn render(frame: &mut Frame, area: Rect, message: &str) {
    let text = Line::from(Span::styled(message, Style::default().fg(theme::danger()).add_modifier(Modifier::BOLD)));

    panel::single_line(frame, area, text, theme::danger());
}
