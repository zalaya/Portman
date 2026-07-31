use ratatui::Frame;
use ratatui::layout::{ Constraint, Flex, Layout, Rect };
use ratatui::style::{ Color, Style };
use ratatui::text::Line;
use ratatui::widgets::{ Block, Borders, Padding, Paragraph };

pub fn single_line(frame: &mut Frame, area: Rect, line: Line, border_color: Color) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(border_color))
        .padding(Padding::horizontal(1));

    frame.render_widget(Paragraph::new(line).block(block), area);
}

pub fn centered(area: Rect, width: u16, height: u16) -> Rect {
    let [area] = Layout::horizontal([Constraint::Length(width)]).flex(Flex::Center).areas(area);
    let [area] = Layout::vertical([Constraint::Length(height)]).flex(Flex::Center).areas(area);

    area
}
