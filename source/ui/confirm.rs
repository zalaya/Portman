use ratatui::Frame;
use ratatui::layout::{ Alignment, Rect };
use ratatui::style::{ Modifier, Style };
use ratatui::text::{ Line, Span };
use ratatui::widgets::{ Block, Borders, Clear, Padding, Paragraph };

use super::{ panel, theme };
use crate::app::KillTarget;

pub fn render(frame: &mut Frame, area: Rect, target: &KillTarget) {
    let question = format!("Kill {}?", target.label);
    let width = (question.len() as u16 + 8).clamp(36, area.width.saturating_sub(4));
    let popup = panel::centered(area, width, 7);

    let text = vec![
        Line::from(Span::styled(question, Style::default().add_modifier(Modifier::BOLD))).alignment(Alignment::Center),
        Line::from(""),
        Line::from(vec![
            Span::styled("y", Style::default().fg(theme::DANGER).add_modifier(Modifier::BOLD)),
            Span::styled(" yes", Style::default().fg(theme::MUTED)),
            Span::raw("    "),
            Span::styled("n", Style::default().fg(theme::PRIMARY).add_modifier(Modifier::BOLD)),
            Span::styled(" no", Style::default().fg(theme::MUTED)),
        ])
        .alignment(Alignment::Center),
    ];

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme::DANGER))
        .title(Line::from(" ⚠ Confirm ").alignment(Alignment::Center))
        .title_style(Style::default().fg(theme::DANGER).add_modifier(Modifier::BOLD))
        .padding(Padding::uniform(1));

    frame.render_widget(Clear, popup);
    frame.render_widget(Paragraph::new(text).block(block), popup);
}
