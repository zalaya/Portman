mod action_menu;
mod confirm;
mod details;
mod info_panel;
mod panel;
mod search;
mod status;
mod table;
mod theme;

use ratatui::Frame;
use ratatui::layout::{ Constraint, Direction, Layout };

use crate::app::App;

pub fn draw(frame: &mut Frame, app: &mut App) {
    let has_status = app.status.is_some();

    let constraints = if has_status {
        vec![Constraint::Length(3), Constraint::Length(3), Constraint::Min(0)]
    } else {
        vec![Constraint::Length(3), Constraint::Min(0)]
    };

    let chunks = Layout::default().direction(Direction::Vertical).constraints(constraints).split(frame.area());

    search::render(frame, chunks[0], &app.filter);

    if has_status {
        status::render(frame, chunks[1], app.status.as_deref().unwrap());
    }

    let content_area = chunks[chunks.len() - 1];

    let columns = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(55), Constraint::Percentage(45)])
        .split(content_area);

    table::render(frame, columns[0], app);
    details::render(frame, columns[1], app.details.as_ref());

    if let Some(menu) = &app.action_menu {
        action_menu::render(frame, frame.area(), menu);
    }

    if let Some(info) = &app.info_panel {
        info_panel::render(frame, frame.area(), info);
    }

    if let Some(target) = &app.kill_target {
        confirm::render(frame, frame.area(), target);
    }
}
