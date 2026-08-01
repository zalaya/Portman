use ratatui::Frame;
use ratatui::layout::{ Alignment, Rect };
use ratatui::style::{ Modifier, Style };
use ratatui::text::{ Line, Span };

use crate::app::ActionMenu;
use crate::ui::theme;
use crate::ui::widgets::popup::Popup;

pub fn render(frame: &mut Frame, area: Rect, menu: &ActionMenu) {
    let longest_label = menu.actions.iter().map(|action| action.label().len()).max().unwrap_or(0);
    let width = ((menu.label.len().max(longest_label + 2)) as u16 + 8).clamp(36, area.width.saturating_sub(4));
    let height = menu.actions.len() as u16 + 4;

    let mut lines = vec![
        Line::from(Span::styled(menu.label.clone(), Style::default().add_modifier(Modifier::BOLD))).alignment(Alignment::Center),
        Line::from(""),
    ];

    for (index, action) in menu.actions.iter().enumerate() {
        let focused = index == menu.selected;
        let marker = if focused { "▸ " } else { "  " };
        let style = if focused {
            Style::default().fg(theme::primary()).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(theme::muted())
        };

        lines.push(Line::from(Span::styled(format!("{marker}{}", action.label()), style)));
    }

    Popup { title: "Actions", border_color: theme::primary(), width, height, wrap: false }.render(frame, area, lines);
}
