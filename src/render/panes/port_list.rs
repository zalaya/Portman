use ratatui::Frame;
use ratatui::layout::{ Alignment, Constraint, Rect };
use ratatui::style::{ Color, Modifier, Style };
use ratatui::text::Line;
use ratatui::widgets::{ Block, Borders, Cell, Padding, Row, Table, TableState };

use crate::session::{ self, Session, SortKey };
use crate::scanning::port::{ PortUsage, Risk };
use crate::render::theme;

pub fn render(frame: &mut Frame, area: Rect, session: &Session) {
    let rows = session::filtered_items(&session.items, &session.filter, session.sort)
        .into_iter()
        .map(|usage| row_for(usage, session.new_keys.contains(&(usage.address(), usage.pid))))
        .collect::<Vec<_>>();
    let widths = [
        Constraint::Length(3),
        Constraint::Length(11),
        Constraint::Length(15),
        Constraint::Min(20),
        Constraint::Length(8),
    ];

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme::muted()))
        .title(Line::from(" Ports ").alignment(Alignment::Left))
        .title(Line::from(format!(" sort: {} ", session.sort.label())).alignment(Alignment::Right))
        .padding(Padding::horizontal(1));

    let table = Table::new(rows, widths)
        .header(header(session.sort))
        .block(block)
        .row_highlight_style(Style::default().bg(theme::primary()).fg(Color::White).add_modifier(Modifier::BOLD));

    frame.render_stateful_widget(table, area, &mut TableState::default().with_selected(session.selected));
}

fn header(sort: SortKey) -> Row<'static> {
    let column = |label: &str, key: SortKey| if sort == key { format!("{label} ▾") } else { label.to_string() };

    Row::new(vec![
        column("", SortKey::Risk),
        column("PORT", SortKey::Port),
        column("BIND", SortKey::Bind),
        column("PROCESS", SortKey::Process),
        column("PID", SortKey::Pid),
    ])
    .style(Style::default().fg(theme::secondary()).add_modifier(Modifier::BOLD))
}

fn row_for(usage: &PortUsage, is_new: bool) -> Row<'_> {
    let style = if usage.process_name.is_none() {
        Style::default().fg(theme::muted()).add_modifier(Modifier::ITALIC)
    } else if is_new {
        Style::default().fg(theme::success()).add_modifier(Modifier::BOLD)
    } else {
        Style::default()
    };

    let bind_style = if usage.is_exposed() {
        Style::default().fg(theme::danger()).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(theme::muted())
    };

    let risk = usage.risk();
    let risk_style = Style::default().fg(risk_color(risk));

    Row::new(vec![
        Cell::from(risk.icon()).style(risk_style),
        Cell::from(usage.address()),
        Cell::from(usage.bind_label()).style(bind_style),
        Cell::from(usage.process_label()),
        Cell::from(usage.pid.to_string()),
    ])
    .style(style)
}

fn risk_color(risk: Risk) -> Color {
    match risk {
        Risk::Safe => theme::success(),
        Risk::Watch => theme::secondary(),
        Risk::Critical => theme::danger(),
    }
}
