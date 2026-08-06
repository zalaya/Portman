use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Padding, Paragraph};

use crate::render::theme;
use crate::render::widgets::panel;
use crate::scanning::port::Risk;
use crate::session::Session;

pub fn render_header(frame: &mut Frame, area: Rect, session: &Session) {
    let total = session.items.len();
    let (exposed, critical) = session
        .items
        .iter()
        .fold((0, 0), |(exposed, critical), usage| {
            (
                exposed + usize::from(usage.is_exposed()),
                critical + usize::from(usage.risk() == Risk::Critical),
            )
        });
    let new = session.new_keys.len();

    let mut spans = vec![
        Span::styled(
            " PORTMAN ",
            Style::default()
                .fg(theme::primary())
                .add_modifier(Modifier::BOLD),
        ),
        sep(),
        Span::styled(
            format!("{total} ports"),
            Style::default().fg(theme::muted()),
        ),
        sep(),
        Span::styled(
            format!("{exposed} exposed"),
            highlight(exposed, theme::danger()),
        ),
        sep(),
        Span::styled(
            format!("{critical} critical"),
            highlight(critical, theme::danger()).add_modifier(Modifier::BOLD),
        ),
    ];

    if new > 0 {
        spans.push(sep());
        spans.push(Span::styled(
            format!("{new} new"),
            Style::default()
                .fg(theme::success())
                .add_modifier(Modifier::BOLD),
        ));
    }

    spans.push(sep());
    spans.push(Span::styled(
        format!("sort: {}", session.sort.label()),
        Style::default().fg(theme::muted()),
    ));
    spans.push(sep());
    spans.push(Span::styled(
        "Ctrl+K help",
        Style::default().fg(theme::muted()),
    ));

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme::primary()))
        .padding(Padding::horizontal(1));

    frame.render_widget(Paragraph::new(Line::from(spans)).block(block), area);
}

fn sep() -> Span<'static> {
    Span::styled("  ·  ", Style::default().fg(theme::muted()))
}

fn highlight(count: usize, color: Color) -> Style {
    if count > 0 {
        Style::default().fg(color)
    } else {
        Style::default().fg(theme::muted())
    }
}

pub fn render_search(frame: &mut Frame, area: Rect, session: &Session) {
    let text = Line::from(vec![
        Span::styled("> ", Style::default().fg(theme::secondary())),
        Span::styled(
            session.filter.clone(),
            Style::default().fg(theme::primary()),
        ),
    ]);

    panel::single_line(frame, area, text, theme::muted());
}

pub fn render_status(frame: &mut Frame, area: Rect, message: &str) {
    let text = Line::from(Span::styled(
        message,
        Style::default()
            .fg(theme::danger())
            .add_modifier(Modifier::BOLD),
    ));

    panel::single_line(frame, area, text, theme::danger());
}
