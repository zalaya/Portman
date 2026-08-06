use ratatui::Frame;
use ratatui::layout::{Alignment, Constraint, Flex, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::Line;
use ratatui::widgets::{Block, Borders, Clear, Padding, Paragraph, Wrap};

pub mod panel {
    use super::{
        Block, Borders, Color, Constraint, Flex, Frame, Layout, Line, Padding, Paragraph, Rect,
        Style,
    };

    pub fn single_line(frame: &mut Frame, area: Rect, line: Line, border_color: Color) {
        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(border_color))
            .padding(Padding::horizontal(1));

        frame.render_widget(Paragraph::new(line).block(block), area);
    }

    pub fn centered(area: Rect, width: u16, height: u16) -> Rect {
        let [area] = Layout::horizontal([Constraint::Length(width)])
            .flex(Flex::Center)
            .areas(area);
        let [area] = Layout::vertical([Constraint::Length(height)])
            .flex(Flex::Center)
            .areas(area);

        area
    }
}

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
            .title_style(
                Style::default()
                    .fg(self.border_color)
                    .add_modifier(Modifier::BOLD),
            )
            .padding(Padding::uniform(1));

        let mut paragraph = Paragraph::new(lines).block(block);

        if self.wrap {
            paragraph = paragraph.wrap(Wrap { trim: false });
        }

        frame.render_widget(Clear, popup_area);
        frame.render_widget(paragraph, popup_area);
    }
}
