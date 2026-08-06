use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{ Modifier, Style };
use ratatui::text::{ Line, Span };

use crate::render::theme;
use crate::render::widgets::Popup;

const BINDINGS: &[(&str, &str)] = &[
    ("↑ / ↓", "Move selection"),
    ("type", "Search by port, bind, process or PID"),
    ("Tab", "Cycle sort key (add Risk to the rotation)"),
    ("Enter", "Open the actions menu"),
    ("Delete", "Kill the selected process"),
    ("Ctrl+L", "Toggle the activity log"),
    ("Ctrl+R", "Refresh now"),
    ("Ctrl+K", "Toggle this help"),
    ("Esc", "Quit, or close whatever's open"),
];

pub fn render(frame: &mut Frame, area: Rect) {
    let width = 58u16.min(area.width.saturating_sub(4)).max(30);
    let height = (BINDINGS.len() as u16 + 4).min(area.height.saturating_sub(2));

    let lines: Vec<Line> = BINDINGS
        .iter()
        .map(|(key, label)| {
            Line::from(vec![
                Span::styled(format!("{key:<11}"), Style::default().fg(theme::secondary()).add_modifier(Modifier::BOLD)),
                Span::styled(label.to_string(), Style::default().fg(theme::muted())),
            ])
        })
        .collect();

    Popup { title: "Keybindings", border_color: theme::primary(), width, height, wrap: false }.render(frame, area, lines);
}
