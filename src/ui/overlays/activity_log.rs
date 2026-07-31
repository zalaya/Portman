use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::Style;
use ratatui::text::{ Line, Span };

use crate::app::App;
use crate::theme;
use crate::ui::widgets::popup::Popup;

pub fn render(frame: &mut Frame, area: Rect, app: &App) {
    let width = (area.width * 3 / 4).clamp(50, area.width.saturating_sub(4));
    let height = (area.height * 3 / 4).clamp(10, area.height.saturating_sub(2));

    let lines: Vec<Line> = if app.events.is_empty() {
        vec![Line::from(Span::styled("No activity yet", Style::default().fg(theme::muted())))]
    } else {
        app.events
            .iter()
            .map(|event| {
                let elapsed = format_elapsed(event.at.elapsed().as_secs());

                Line::from(vec![
                    Span::styled(format!("{elapsed:>4} ago  "), Style::default().fg(theme::muted())),
                    Span::raw(event.message.clone()),
                ])
            })
            .collect()
    };

    Popup { title: "Activity — Esc to close", border_color: theme::primary(), width, height, wrap: true }.render(frame, area, lines);
}

fn format_elapsed(seconds: u64) -> String {
    if seconds < 60 {
        format!("{seconds}s")
    } else if seconds < 3600 {
        format!("{}m", seconds / 60)
    } else {
        format!("{}h", seconds / 3600)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formats_seconds_minutes_and_hours() {
        assert_eq!(format_elapsed(45), "45s");
        assert_eq!(format_elapsed(59), "59s");
        assert_eq!(format_elapsed(60), "1m");
        assert_eq!(format_elapsed(3599), "59m");
        assert_eq!(format_elapsed(3600), "1h");
    }
}
