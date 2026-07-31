use ratatui::Frame;
use ratatui::layout::{ Alignment, Rect };
use ratatui::style::{ Color, Modifier, Style };
use ratatui::text::Line;
use ratatui::widgets::{ Block, Borders, Clear, Padding, Paragraph, Wrap };

use super::panel;

pub struct Popup<'a> {
    pub title: &'a str,
    pub border_color: Color,
    pub width: u16,
    pub height: u16,
    pub wrap: bool,
}

impl Popup<'_> {
    pub fn render(self, frame: &mut Frame, area: Rect, lines: Vec<Line<'static>>) {
        let popup_area = panel::centered(area, self.width, self.height);

        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(self.border_color))
            .title(Line::from(format!(" {} ", self.title)).alignment(Alignment::Center))
            .title_style(Style::default().fg(self.border_color).add_modifier(Modifier::BOLD))
            .padding(Padding::uniform(1));

        let mut paragraph = Paragraph::new(lines).block(block);

        if self.wrap {
            paragraph = paragraph.wrap(Wrap { trim: false });
        }

        frame.render_widget(Clear, popup_area);
        frame.render_widget(paragraph, popup_area);
    }
}
