mod app;
mod data;
mod ui;

use std::io::{ Stdout, stdout };
use std::time::Duration;

use anyhow::Result;
use app::App;
use crossterm::event::{ self, Event, KeyCode, KeyEventKind, KeyModifiers };
use crossterm::execute;
use crossterm::terminal::{ EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode };
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;

const REFRESH_INTERVAL: Duration = Duration::from_secs(2);

fn main() -> Result<()> {
    enable_raw_mode()?;
    execute!(stdout(), EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout());
    let mut terminal = Terminal::new(backend)?;
    let mut app = App::new()?;
    let result = run(&mut terminal, &mut app);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    result
}

fn run(terminal: &mut Terminal<CrosstermBackend<Stdout>>, app: &mut App) -> Result<()> {
    loop {
        terminal.draw(|frame| ui::draw(frame, app))?;

        if !event::poll(REFRESH_INTERVAL)? {
            if app.kill_target.is_none() {
                app.refresh()?;
            }

            continue;
        }

        let Event::Key(key) = event::read()? else {
            continue;
        };

        if key.kind != KeyEventKind::Press {
            continue;
        }

        app.status = None;

        if app.kill_target.is_some() {
            match key.code {
                KeyCode::Char('y') | KeyCode::Enter => app.confirm_kill()?,
                KeyCode::Char('n') | KeyCode::Esc => app.cancel_kill(),
                _ => {}
            }

            continue;
        }

        if app.info_panel.is_some() {
            match key.code {
                KeyCode::Enter | KeyCode::Esc => app.close_info_panel(),
                _ => {}
            }

            continue;
        }

        if app.action_menu.is_some() {
            match key.code {
                KeyCode::Up => app.action_menu_previous(),
                KeyCode::Down => app.action_menu_next(),
                KeyCode::Enter => app.confirm_action_menu()?,
                KeyCode::Esc => app.close_action_menu(),
                _ => {}
            }

            continue;
        }

        if key.code == KeyCode::Char('c') && key.modifiers.contains(KeyModifiers::CONTROL) {
            return Ok(());
        }

        if key.code == KeyCode::Char('r') && key.modifiers.contains(KeyModifiers::CONTROL) {
            app.refresh()?;
            continue;
        }

        match key.code {
            KeyCode::Esc => return Ok(()),
            KeyCode::Down => app.next(),
            KeyCode::Up => app.previous(),
            KeyCode::Enter => app.open_action_menu(),
            KeyCode::Delete => app.request_kill(),
            KeyCode::Tab => app.cycle_sort(),
            KeyCode::Backspace => app.pop_filter_char(),
            KeyCode::Char(character) => app.push_filter_char(character),
            _ => {}
        }
    }
}
