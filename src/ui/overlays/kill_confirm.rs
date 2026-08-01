use ratatui::Frame;
use ratatui::layout::{ Alignment, Rect };
use ratatui::style::{ Modifier, Style };
use ratatui::text::{ Line, Span };

use crate::app::KillTarget;
use crate::data::process::KillSignal;
use crate::theme;
use crate::ui::widgets::popup::Popup;

pub fn render(frame: &mut Frame, area: Rect, target: &KillTarget) {
    let verb = match target.signal {
        KillSignal::Terminate => "Terminate",
        KillSignal::Force => "Force kill",
    };
    let question = format!("{verb} {}?", target.label);
    let width = (question.len() as u16 + 8).clamp(36, area.width.saturating_sub(4));

    let lines = vec![
        Line::from(Span::styled(question, Style::default().add_modifier(Modifier::BOLD))).alignment(Alignment::Center),
        Line::from(""),
        Line::from(vec![
            Span::styled("y", Style::default().fg(theme::danger()).add_modifier(Modifier::BOLD)),
            Span::styled(" yes", Style::default().fg(theme::muted())),
            Span::raw("    "),
            Span::styled("n", Style::default().fg(theme::primary()).add_modifier(Modifier::BOLD)),
            Span::styled(" no", Style::default().fg(theme::muted())),
        ])
        .alignment(Alignment::Center),
    ];

    Popup { title: "⚠ Confirm", border_color: theme::danger(), width, height: 7, wrap: false }.render(frame, area, lines);
}
