use std::io::stdout;

use anyhow::Result;
use crossterm::execute;
use crossterm::terminal::{ EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode };
use portman::app::App;
use portman::runtime;
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;

fn main() -> Result<()> {
    enable_raw_mode()?;
    execute!(stdout(), EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout());
    let mut terminal = Terminal::new(backend)?;
    let mut app = App::new()?;
    let result = runtime::run(&mut terminal, &mut app);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    result
}
