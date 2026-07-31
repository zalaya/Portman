mod chrome;
mod overlays;
mod panes;
mod widgets;

use ratatui::Frame;
use ratatui::layout::{ Constraint, Direction, Layout, Rect };

use crate::app::App;

pub fn draw(frame: &mut Frame, app: &mut App) {
    let (header_area, search_area, status_area, content_area) = layout(frame.area(), app.status.is_some());

    chrome::header::render(frame, header_area, app);
    chrome::search::render(frame, search_area, app);

    if let Some(area) = status_area {
        chrome::status::render(frame, area, app.status.as_deref().unwrap());
    }

    let [list_area, details_area] =
        Layout::horizontal([Constraint::Percentage(55), Constraint::Percentage(45)]).areas(content_area);

    panes::port_list::render(frame, list_area, app);
    panes::process_details::render(frame, details_area, app.details.as_ref());

    if let Some(menu) = &app.action_menu {
        overlays::action_menu::render(frame, frame.area(), menu);
    }

    if app.show_activity {
        overlays::activity_log::render(frame, frame.area(), app);
    }

    if app.show_help {
        overlays::help::render(frame, frame.area());
    }

    if let Some(target) = &app.kill_target {
        overlays::kill_confirm::render(frame, frame.area(), target);
    }
}

fn layout(area: Rect, has_status: bool) -> (Rect, Rect, Option<Rect>, Rect) {
    let mut constraints = vec![Constraint::Length(3), Constraint::Length(3)];

    if has_status {
        constraints.push(Constraint::Length(3));
    }

    constraints.push(Constraint::Min(0));

    let chunks = Layout::default().direction(Direction::Vertical).constraints(constraints).split(area);

    if has_status {
        (chunks[0], chunks[1], Some(chunks[2]), chunks[3])
    } else {
        (chunks[0], chunks[1], None, chunks[2])
    }
}
