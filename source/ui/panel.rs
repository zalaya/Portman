use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{ Color, Style };
use ratatui::text::Line;
use ratatui::widgets::{ Block, BorderType, Borders, Padding, Paragraph };

pub fn single_line(frame: &mut Frame, area: Rect, line: Line, border_color: Color) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(border_color))
        .padding(Padding::horizontal(1));

    frame.render_widget(Paragraph::new(line).block(block), area);
}
