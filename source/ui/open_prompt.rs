use ratatui::Frame;
use ratatui::layout::{ Alignment, Rect };
use ratatui::style::{ Modifier, Style };
use ratatui::text::{ Line, Span };
use ratatui::widgets::{ Block, BorderType, Borders, Clear, Padding, Paragraph };

use super::{ panel, theme };
use crate::app::{ OpenField, OpenPrompt };

pub fn render(frame: &mut Frame, area: Rect, prompt: &OpenPrompt) {
    let popup = panel::centered(area, 52, 9);
    let port_text = if prompt.port_input.is_empty() { "_".to_string() } else { prompt.port_input.clone() };
    let command_text = if prompt.command.is_empty() { "e.g. python3 -m http.server 8080".to_string() } else { prompt.command.clone() };

    let text = vec![
        field_line("Port", port_text, prompt.focus == OpenField::Port),
        Line::from(Span::styled(format!("{}", prompt.protocol), Style::default().fg(theme::SECONDARY).add_modifier(Modifier::BOLD)))
            .alignment(Alignment::Center),
        Line::from(""),
        field_line("Command", command_text, prompt.focus == OpenField::Command),
        Line::from(""),
        Line::from(vec![
            hint("tab", "field"),
            Span::raw("  "),
            hint("←/→", "protocol"),
            Span::raw("  "),
            hint("enter", "open"),
            Span::raw("  "),
            hint("esc", "cancel"),
        ])
        .alignment(Alignment::Center),
    ];

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Thick)
        .border_style(Style::default().fg(theme::PRIMARY))
        .title(Line::from(" Open port ").alignment(Alignment::Center))
        .title_style(Style::default().fg(theme::PRIMARY).add_modifier(Modifier::BOLD))
        .padding(Padding::uniform(1));

    frame.render_widget(Clear, popup);
    frame.render_widget(Paragraph::new(text).block(block), popup);
}

fn field_line(label: &str, value: String, focused: bool) -> Line<'static> {
    let marker = if focused { "▸ " } else { "  " };
    
    let value_style = if focused {
        Style::default().fg(theme::PRIMARY).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(theme::MUTED)
    };

    Line::from(vec![
        Span::styled(marker, Style::default().fg(theme::PRIMARY)),
        Span::styled(format!("{label}: "), Style::default().fg(theme::MUTED)),
        Span::styled(value, value_style),
    ])
}

fn hint(key: &str, label: &str) -> Span<'static> {
    Span::styled(format!("{key} {label}"), Style::default().fg(theme::SECONDARY))
}
