use ratatui::Frame;
use ratatui::layout::{ Alignment, Constraint, Rect };
use ratatui::style::{ Color, Modifier, Style };
use ratatui::text::Line;
use ratatui::widgets::{ Block, BorderType, Borders, Cell, Padding, Row, Table };

use super::theme;
use crate::app::{ self, App, SortKey };
use crate::data::port_usage::PortUsage;

pub fn render(frame: &mut Frame, area: Rect, app: &mut App) {
    let rows = app::filtered_items(&app.items, &app.filter, app.sort)
        .into_iter()
        .map(|usage| row_for(usage, app.new_keys.contains(&(usage.address(), usage.pid))))
        .collect::<Vec<_>>();
    let widths = [Constraint::Length(14), Constraint::Length(12), Constraint::Min(20), Constraint::Length(8)];

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(theme::MUTED))
        .title(Line::from(format!(" sort: {} ", app.sort.label())).alignment(Alignment::Right))
        .padding(Padding::horizontal(1));

    let table = Table::new(rows, widths)
        .header(header(app.sort))
        .block(block)
        .row_highlight_style(Style::default().bg(theme::PRIMARY).fg(Color::White).add_modifier(Modifier::BOLD));

    frame.render_stateful_widget(table, area, &mut app.state);
}

fn header(sort: SortKey) -> Row<'static> {
    let column = |label: &str, key: SortKey| if sort == key { format!("{label} ▾") } else { label.to_string() };

    Row::new(vec![
        column("PORT", SortKey::Port),
        column("BIND", SortKey::Bind),
        column("PROCESS", SortKey::Process),
        column("PID", SortKey::Pid),
    ])
    .style(Style::default().fg(theme::SECONDARY).add_modifier(Modifier::BOLD))
}

fn row_for(usage: &PortUsage, is_new: bool) -> Row<'_> {
    let style = if usage.process_name.is_none() {
        Style::default().fg(theme::MUTED).add_modifier(Modifier::ITALIC)
    } else if is_new {
        Style::default().fg(theme::SECONDARY).add_modifier(Modifier::BOLD)
    } else {
        Style::default()
    };

    let bind_style = if usage.is_exposed() {
        Style::default().fg(theme::DANGER).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(theme::MUTED)
    };

    Row::new(vec![
        Cell::from(usage.address()),
        Cell::from(usage.bind_label()).style(bind_style),
        Cell::from(usage.process_label()),
        Cell::from(usage.pid.to_string()),
    ])
    .style(style)
}
