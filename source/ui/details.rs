use ratatui::Frame;
use ratatui::layout::{ Alignment, Rect };
use ratatui::style::{ Modifier, Style };
use ratatui::text::{ Line, Span };
use ratatui::widgets::{ Block, BorderType, Borders, Padding, Paragraph, Wrap };

use super::{ panel, theme };
use crate::app::Details;

pub fn render_breadcrumb(frame: &mut Frame, area: Rect, details: &Details) {
    let text = Line::from(vec![
        Span::styled("← ", Style::default().fg(theme::MUTED)),
        Span::styled(&details.process.name, Style::default().fg(theme::PRIMARY).add_modifier(Modifier::BOLD)),
        Span::raw(" "),
        Span::styled(format!("({})", details.address), Style::default().fg(theme::MUTED)),
    ]);

    panel::single_line(frame, area, text, theme::MUTED);
}

pub fn render(frame: &mut Frame, area: Rect, details: &Details) {
    let process = &details.process;
    let mut lines = vec![
        row("Address", details.address.clone()),
        row("PID", process.pid.to_string()),
        row("Status", process.status.clone()),
        row("Uptime", format_duration(process.run_time_secs)),
        row("Memory", format_bytes(process.memory_bytes)),
    ];

    if let Some(parent_pid) = process.parent_pid {
        lines.push(row("Parent PID", parent_pid.to_string()));
    }

    if let Some(exe) = &process.exe {
        lines.push(row("Executable", exe.clone()));
    }

    if let Some(cwd) = &process.cwd {
        lines.push(row("Working dir", cwd.clone()));
    }

    if !process.cmd.is_empty() {
        lines.push(row("Command", process.cmd.join(" ")));
    }

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(theme::MUTED))
        .title(Line::from(" Process details ").alignment(Alignment::Center))
        .padding(Padding::uniform(1));

    frame.render_widget(Paragraph::new(lines).block(block).wrap(Wrap { trim: false }), area);
}

fn row(label: &'static str, value: String) -> Line<'static> {
    Line::from(vec![
        Span::styled(format!("{label:<12}"), Style::default().fg(theme::SECONDARY).add_modifier(Modifier::BOLD)),
        Span::raw(value),
    ])
}

fn format_bytes(bytes: u64) -> String {
    const KB: f64 = 1024.0;
    let bytes = bytes as f64;

    if bytes < KB {
        format!("{bytes} B")
    } else if bytes < KB * KB {
        format!("{:.1} KB", bytes / KB)
    } else if bytes < KB * KB * KB {
        format!("{:.1} MB", bytes / (KB * KB))
    } else {
        format!("{:.2} GB", bytes / (KB * KB * KB))
    }
}

fn format_duration(seconds: u64) -> String {
    let hours = seconds / 3600;
    let minutes = (seconds % 3600) / 60;
    let seconds = seconds % 60;

    if hours > 0 {
        format!("{hours}h {minutes}m {seconds}s")
    } else if minutes > 0 {
        format!("{minutes}m {seconds}s")
    } else {
        format!("{seconds}s")
    }
}
