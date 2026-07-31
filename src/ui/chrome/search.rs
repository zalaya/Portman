use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::Style;
use ratatui::text::{ Line, Span };

use crate::app::App;
use crate::theme;
use crate::ui::widgets::panel;

pub fn render(frame: &mut Frame, area: Rect, app: &App) {
    let text = Line::from(vec![
        Span::styled("> ", Style::default().fg(theme::secondary())),
        Span::styled(app.filter.clone(), Style::default().fg(theme::primary())),
    ]);

    panel::single_line(frame, area, text, theme::muted());
}
