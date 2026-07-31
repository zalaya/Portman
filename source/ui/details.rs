use ratatui::Frame;
use ratatui::layout::{ Alignment, Rect };
use ratatui::style::{ Modifier, Style };
use ratatui::text::{ Line, Span };
use ratatui::widgets::{ Block, Borders, Padding, Paragraph, Wrap };

use super::theme;
use crate::app::Details;

pub fn render(frame: &mut Frame, area: Rect, details: Option<&Details>) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme::MUTED))
        .title(Line::from(" Details ").alignment(Alignment::Left))
        .padding(Padding::horizontal(1));

    let Some(details) = details else {
        let text = Line::from(Span::styled("No process selected", Style::default().fg(theme::MUTED)));

        frame.render_widget(Paragraph::new(text).block(block), area);
        return;
    };

    let process = &details.process;
    let bind_color = if details.exposed { theme::DANGER } else { theme::MUTED };

    let mut lines = vec![
        row("Process", process.name.clone()),
        row("Address", details.address.clone()),
        row_styled("Bind", details.bind.clone(), bind_color),
        row("PID", process.pid.to_string()),
        row("Status", process.status.clone()),
        row("Uptime", format_duration(process.run_time_secs)),
        row("Memory", format_bytes(process.memory_bytes)),
    ];

    if let Some(user) = &process.user {
        lines.push(row("User", user.clone()));
    }

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

    frame.render_widget(Paragraph::new(lines).block(block).wrap(Wrap { trim: false }), area);
}

fn row(label: &'static str, value: String) -> Line<'static> {
    Line::from(vec![
        Span::styled(format!("{label:<12}"), Style::default().fg(theme::SECONDARY).add_modifier(Modifier::BOLD)),
        Span::raw(value),
    ])
}

fn row_styled(label: &'static str, value: String, color: ratatui::style::Color) -> Line<'static> {
    Line::from(vec![
        Span::styled(format!("{label:<12}"), Style::default().fg(theme::SECONDARY).add_modifier(Modifier::BOLD)),
        Span::styled(value, Style::default().fg(color).add_modifier(Modifier::BOLD)),
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
