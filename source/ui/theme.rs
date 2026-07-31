use ratatui::style::Color;

/// Brand accent — selection highlights, focus, primary actions.
pub const PRIMARY: Color = Color::Rgb(129, 140, 248);
/// Secondary accent — labels, sort indicators, headers.
pub const SECONDARY: Color = Color::Rgb(45, 212, 191);
/// Newly-appeared items since the last refresh.
pub const SUCCESS: Color = Color::Rgb(74, 222, 128);
/// Borders, placeholder text, anything de-emphasized.
pub const MUTED: Color = Color::Rgb(100, 112, 134);
/// Destructive actions, exposed/reachable binds, errors.
pub const DANGER: Color = Color::Rgb(248, 113, 113);
