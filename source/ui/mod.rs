mod footer;
mod search;
mod table;
mod theme;

use ratatui::Frame;
use ratatui::layout::{ Constraint, Direction, Layout };

use crate::app::App;

pub fn draw(frame: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0), Constraint::Length(3)])
        .split(frame.area());

    search::render(frame, chunks[0], &app.filter);
    table::render(frame, chunks[1], app);
    footer::render(frame, chunks[2]);
}
