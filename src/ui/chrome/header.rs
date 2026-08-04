use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{ Color, Modifier, Style };
use ratatui::text::{ Line, Span };
use ratatui::widgets::{ Block, Borders, Padding, Paragraph };

use crate::app::App;
use crate::domain::port::Risk;
use crate::ui::theme;

pub fn render(frame: &mut Frame, area: Rect, app: &App) {
    let total = app.items.len();
    let (exposed, critical) = app.items.iter().fold((0, 0), |(exposed, critical), usage| {
        (exposed + usize::from(usage.is_exposed()), critical + usize::from(usage.risk() == Risk::Critical))
    });
    let new = app.new_keys.len();

    let mut spans = vec![
        Span::styled(" PORTMAN ", Style::default().fg(theme::primary()).add_modifier(Modifier::BOLD)),
        sep(),
        Span::styled(format!("{total} ports"), Style::default().fg(theme::muted())),
        sep(),
        Span::styled(format!("{exposed} exposed"), highlight(exposed, theme::danger())),
        sep(),
        Span::styled(format!("{critical} critical"), highlight(critical, theme::danger()).add_modifier(Modifier::BOLD)),
    ];

    if new > 0 {
        spans.push(sep());
        spans.push(Span::styled(format!("{new} new"), Style::default().fg(theme::success()).add_modifier(Modifier::BOLD)));
    }

    spans.push(sep());
    spans.push(Span::styled(format!("sort: {}", app.sort.label()), Style::default().fg(theme::muted())));
    spans.push(sep());
    spans.push(Span::styled("Ctrl+K help", Style::default().fg(theme::muted())));

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
    if count > 0 { Style::default().fg(color) } else { Style::default().fg(theme::muted()) }
}
