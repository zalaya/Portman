mod confirm;
mod details;
mod footer;
mod panel;
mod search;
mod table;
mod theme;

use ratatui::Frame;
use ratatui::layout::{ Constraint, Direction, Layout };

use crate::app::App;

const LIST_HINTS: &[(&str, &str)] = &[
    ("↑/↓", "move"),
    ("type", "search"),
    ("tab", "sort"),
    ("→", "details"),
    ("enter", "kill"),
    ("ctrl+r", "refresh"),
    ("esc", "quit"),
];
const DETAILS_HINTS: &[(&str, &str)] = &[("←", "back"), ("ctrl+r", "refresh"), ("ctrl+c", "quit")];

pub fn draw(frame: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0), Constraint::Length(3)])
        .split(frame.area());

    match &app.details {
        Some(details) => {
            details::render_breadcrumb(frame, chunks[0], details);
            details::render(frame, chunks[1], details);
        }
        None => {
            search::render(frame, chunks[0], &app.filter);
            table::render(frame, chunks[1], app);
        }
    }

    match &app.status {
        Some(message) => footer::render_status(frame, chunks[2], message),
        None if app.details.is_some() => footer::render(frame, chunks[2], DETAILS_HINTS),
        None => footer::render(frame, chunks[2], LIST_HINTS),
    }

    if let Some(target) = &app.kill_target {
        confirm::render(frame, frame.area(), target);
    }
}
