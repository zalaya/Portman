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
            if app.kill_target.is_none() && app.open_prompt.is_none() {
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

        if app.open_prompt.is_some() {
            match key.code {
                KeyCode::Enter => app.confirm_open()?,
                KeyCode::Esc => app.cancel_open_prompt(),
                KeyCode::Tab => app.toggle_open_focus(),
                KeyCode::Left | KeyCode::Right => app.toggle_open_protocol(),
                KeyCode::Backspace => app.pop_open_char(),
                KeyCode::Char(character) => app.push_open_char(character),
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

        if key.code == KeyCode::Char('o') && key.modifiers.contains(KeyModifiers::CONTROL) {
            app.start_open_prompt();
            continue;
        }

        if app.details.is_some() {
            match key.code {
                KeyCode::Left | KeyCode::Esc => app.close_details(),
                _ => {}
            }

            continue;
        }

        match key.code {
            KeyCode::Esc => return Ok(()),
            KeyCode::Down => app.next(),
            KeyCode::Up => app.previous(),
            KeyCode::Right => app.open_details(),
            KeyCode::Delete => app.request_kill(),
            KeyCode::Tab => app.cycle_sort(),
            KeyCode::Backspace => app.pop_filter_char(),
            KeyCode::Char(character) => app.push_filter_char(character),
            _ => {}
        }
    }
}
