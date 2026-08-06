mod chrome;
mod overlays;
mod panes;
mod theme;
mod widgets;

use ratatui::Frame;
use ratatui::layout::{ Constraint, Direction, Layout, Rect };

use crate::session::Session;

pub fn draw(frame: &mut Frame, session: &Session) {
    let (header_area, search_area, status_area, content_area) = layout(frame.area(), session.status.is_some());

    chrome::render_header(frame, header_area, session);
    chrome::render_search(frame, search_area, session);

    if let (Some(area), Some(status)) = (status_area, session.status.as_deref()) {
        chrome::render_status(frame, area, status);
    }

    let [list_area, details_area] =
        Layout::horizontal([Constraint::Percentage(55), Constraint::Percentage(45)]).areas(content_area);

    panes::port_list::render(frame, list_area, session);
    panes::process_details::render(frame, details_area, session.details.as_ref());

    if let Some(menu) = &session.action_menu {
        overlays::action_menu::render(frame, frame.area(), menu);
    }

    if session.show_activity {
        overlays::activity_log::render(frame, frame.area(), session);
    }

    if session.show_help {
        overlays::help::render(frame, frame.area());
    }

    if let Some(target) = &session.kill_target {
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

#[cfg(test)]
mod tests {
    use ratatui::Terminal;
    use ratatui::backend::TestBackend;

    use super::*;
    use crate::scanning::network::Protocol;
    use crate::scanning::port::PortUsage;
    use crate::scanning::process::{ KillSignal, ProcessDetails };
    use crate::session::{ Action, ActionMenu, Details, KillTarget };

    fn draw_into(width: u16, height: u16, session: &Session) {
        let backend = TestBackend::new(width, height);
        let mut terminal = Terminal::new(backend).unwrap();

        terminal.draw(|frame| draw(frame, session)).unwrap();
    }

    fn port(pid: u32) -> PortUsage {
        PortUsage { port: 8080, protocol: Protocol::Tcp, pid, process_name: Some("test".to_string()), local_addr: [0, 0, 0, 0].into() }
    }

    fn process_details() -> ProcessDetails {
        ProcessDetails {
            pid: 1,
            name: "test".to_string(),
            status: "Running".to_string(),
            run_time_secs: 90,
            memory_bytes: 2048,
            parent_pid: Some(1),
            user: Some("root".to_string()),
            exe: Some("/usr/bin/test".to_string()),
            cwd: Some("/tmp".to_string()),
            cmd: vec!["test".to_string(), "--flag".to_string()],
        }
    }

    #[test]
    fn draws_an_empty_list_without_panicking() {
        let mut session = blank_session();
        session.items = Vec::new();
        session.selected = None;

        draw_into(120, 40, &session);
    }

    #[test]
    fn draws_a_populated_list_with_a_status_message() {
        let mut session = blank_session();
        session.items = vec![port(1), port(2)];
        session.selected = Some(0);
        session.status = Some("hello".to_string());

        draw_into(120, 40, &session);
    }

    #[test]
    fn draws_every_overlay_without_panicking() {
        let mut with_action_menu = blank_session();
        with_action_menu.action_menu =
            Some(ActionMenu { label: "test".to_string(), actions: vec![Action::Terminate, Action::Refresh], selected: 0 });
        draw_into(120, 40, &with_action_menu);

        let mut with_kill_confirm = blank_session();
        with_kill_confirm.kill_target = Some(KillTarget { pid: 1, label: "test".to_string(), signal: KillSignal::Force });
        draw_into(120, 40, &with_kill_confirm);

        let mut with_help = blank_session();
        with_help.show_help = true;
        draw_into(120, 40, &with_help);

        let mut with_activity = blank_session();
        with_activity.show_activity = true;
        draw_into(120, 40, &with_activity);

        let mut with_details = blank_session();
        with_details.items = vec![port(1)];
        with_details.selected = Some(0);
        with_details.details = Some(Details {
            address: "8080/TCP".to_string(),
            bind: "Public".to_string(),
            exposed: true,
            risk: crate::scanning::port::Risk::Critical,
            process: process_details(),
        });
        draw_into(120, 40, &with_details);
    }

    #[test]
    fn survives_a_terminal_too_small_to_show_anything_useful() {
        draw_into(1, 1, &blank_session());
        draw_into(0, 0, &blank_session());
    }

    fn blank_session() -> Session {
        Session::new().expect("scanning the local machine's ports should always succeed in tests")
    }
}
