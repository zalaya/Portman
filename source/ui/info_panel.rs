use ratatui::Frame;
use ratatui::layout::{ Alignment, Rect };
use ratatui::style::{ Color, Modifier, Style };
use ratatui::text::{ Line, Span };
use ratatui::widgets::{ Block, Borders, Clear, Padding, Paragraph, Wrap };

use super::{ panel, theme };
use crate::app::InfoPanel;

pub fn render(frame: &mut Frame, area: Rect, info: &InfoPanel) {
    let mut lines = Vec::new();

    lines.push(section("Parent"));

    match &info.parent {
        Some(parent) => lines.push(entry(&parent.name, parent.pid, theme::MUTED)),
        None => lines.push(muted("  none — this is a root process")),
    }

    lines.push(Line::from(""));
    lines.push(section("Selected"));
    lines.push(entry(&info.current.name, info.current.pid, theme::PRIMARY));
    lines.push(Line::from(""));
    lines.push(section(&format!("Children ({})", info.children.len())));

    if info.children.is_empty() {
        lines.push(muted("  none"));
    } else {
        for child in &info.children {
            lines.push(entry(&child.name, child.pid, theme::SECONDARY));
        }
    }

    let content_width = lines.iter().map(line_width).max().unwrap_or(0) as u16;
    let width = content_width.max(info.title.len() as u16).saturating_add(6).clamp(44, area.width.saturating_sub(4));
    let height = (lines.len() as u16 + 4).min(area.height.saturating_sub(2));
    let popup = panel::centered(area, width, height);

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme::PRIMARY))
        .title(Line::from(format!(" {} ", info.title)).alignment(Alignment::Center))
        .title_style(Style::default().fg(theme::PRIMARY).add_modifier(Modifier::BOLD))
        .padding(Padding::uniform(1));

    frame.render_widget(Clear, popup);
    frame.render_widget(Paragraph::new(lines).block(block).wrap(Wrap { trim: false }), popup);
}

fn section(label: &str) -> Line<'static> {
    Line::from(Span::styled(label.to_string(), Style::default().fg(theme::SECONDARY).add_modifier(Modifier::BOLD)))
}

fn entry(name: &str, pid: u32, color: Color) -> Line<'static> {
    Line::from(vec![
        Span::raw("  "),
        Span::styled(name.to_string(), Style::default().fg(color).add_modifier(Modifier::BOLD)),
        Span::styled(format!(" ({pid})"), Style::default().fg(theme::MUTED)),
    ])
}

fn muted(text: &str) -> Line<'static> {
    Line::from(Span::styled(text.to_string(), Style::default().fg(theme::MUTED)))
}

fn line_width(line: &Line) -> usize {
    line.spans.iter().map(|span| span.content.chars().count()).sum()
}
