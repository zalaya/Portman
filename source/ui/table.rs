use ratatui::Frame;
use ratatui::layout::{ Constraint, Rect };
use ratatui::style::{ Modifier, Style };
use ratatui::widgets::{ Block, BorderType, Borders, Cell, Row, Table };

use super::theme;
use crate::app::{ self, App };
use crate::data::port_usage::PortUsage;

pub fn render(frame: &mut Frame, area: Rect, app: &mut App) {
    let rows = app::filtered_items(&app.items, &app.filter).into_iter().map(row_for).collect::<Vec<_>>();
    let widths = [Constraint::Length(14), Constraint::Min(20), Constraint::Length(8)];

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(theme::MUTED));

    let table = Table::new(rows, widths)
        .header(header())
        .block(block)
        .row_highlight_style(Style::default().fg(theme::PRIMARY).add_modifier(Modifier::BOLD))
        .highlight_symbol("❯ ");

    frame.render_stateful_widget(table, area, &mut app.state);
}

fn header() -> Row<'static> {
    Row::new(vec!["  PORT", "PROCESS", "PID"]).style(Style::default().fg(theme::SECONDARY).add_modifier(Modifier::BOLD))
}

fn row_for(usage: &PortUsage) -> Row<'_> {
    let style = if usage.process_name.is_some() {
        Style::default()
    } else {
        Style::default().fg(theme::MUTED).add_modifier(Modifier::ITALIC)
    };

    Row::new(vec![
        Cell::from(usage.address()),
        Cell::from(usage.process_label()),
        Cell::from(usage.pid.to_string()),
    ]).style(style)
}
