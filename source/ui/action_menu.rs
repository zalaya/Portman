use ratatui::Frame;
use ratatui::layout::{ Alignment, Rect };
use ratatui::style::{ Modifier, Style };
use ratatui::text::{ Line, Span };
use ratatui::widgets::{ Block, Borders, Clear, Padding, Paragraph };

use super::{ panel, theme };
use crate::app::ActionMenu;

pub fn render(frame: &mut Frame, area: Rect, menu: &ActionMenu) {
    let longest_label = menu.actions.iter().map(|action| action.label().len()).max().unwrap_or(0);
    let width = ((menu.label.len().max(longest_label + 2)) as u16 + 8).clamp(36, area.width.saturating_sub(4));
    let height = menu.actions.len() as u16 + 4;
    let popup = panel::centered(area, width, height);

    let mut lines = vec![
        Line::from(Span::styled(menu.label.clone(), Style::default().add_modifier(Modifier::BOLD))).alignment(Alignment::Center),
        Line::from(""),
    ];

    for (index, action) in menu.actions.iter().enumerate() {
        let focused = index == menu.selected;
        let marker = if focused { "▸ " } else { "  " };
        let style = if focused {
            Style::default().fg(theme::PRIMARY).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(theme::MUTED)
        };

        lines.push(Line::from(Span::styled(format!("{marker}{}", action.label()), style)));
    }

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme::PRIMARY))
        .title(Line::from(" Actions ").alignment(Alignment::Center))
        .title_style(Style::default().fg(theme::PRIMARY).add_modifier(Modifier::BOLD))
        .padding(Padding::uniform(1));

    frame.render_widget(Clear, popup);
    frame.render_widget(Paragraph::new(lines).block(block), popup);
}
