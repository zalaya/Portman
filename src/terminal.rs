use std::io::Stdout;
use std::time::Duration;

use anyhow::Result;
use crossterm::event::{ self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers };
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;

use crate::render;
use crate::scanning::process::KillSignal;
use crate::session::Session;

const REFRESH_INTERVAL: Duration = Duration::from_secs(2);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Flow {
    Continue,
    Quit,
}

pub fn run(terminal: &mut Terminal<CrosstermBackend<Stdout>>, session: &mut Session) -> Result<()> {
    loop {
        terminal.draw(|frame| render::draw(frame, session))?;

        if !event::poll(REFRESH_INTERVAL)? {
            if session.kill_target.is_none() {
                session.refresh()?;
            }

            continue;
        }

        let Event::Key(key) = event::read()? else {
            continue;
        };

        if key.kind != KeyEventKind::Press {
            continue;
        }

        if handle_key(session, key)? == Flow::Quit {
            return Ok(());
        }
    }
}

pub fn handle_key(session: &mut Session, key: KeyEvent) -> Result<Flow> {
    session.status = None;

    if session.kill_target.is_some() {
        return handle_kill_confirmation(session, key.code);
    }

    if session.show_help {
        session.close_help();
        return Ok(Flow::Continue);
    }

    if session.show_activity {
        return Ok(handle_activity_log(session, key.code));
    }

    if session.action_menu.is_some() {
        return handle_action_menu(session, key.code);
    }

    if key.modifiers.contains(KeyModifiers::CONTROL)
        && let Some(flow) = handle_shortcut(session, key.code)?
    {
        return Ok(flow);
    }

    Ok(handle_list_navigation(session, key.code))
}

fn handle_kill_confirmation(session: &mut Session, code: KeyCode) -> Result<Flow> {
    match code {
        KeyCode::Char('y') | KeyCode::Enter => session.confirm_kill()?,
        KeyCode::Char('n') | KeyCode::Esc => session.cancel_kill(),
        _ => {}
    }

    Ok(Flow::Continue)
}

fn handle_activity_log(session: &mut Session, code: KeyCode) -> Flow {
    match code {
        KeyCode::Esc | KeyCode::Enter => session.close_activity(),
        _ => {}
    }

    Flow::Continue
}

fn handle_action_menu(session: &mut Session, code: KeyCode) -> Result<Flow> {
    match code {
        KeyCode::Up => session.select_previous_action(),
        KeyCode::Down => session.select_next_action(),
        KeyCode::Enter => session.confirm_action_menu()?,
        KeyCode::Esc => session.close_action_menu(),
        _ => {}
    }

    Ok(Flow::Continue)
}

fn handle_shortcut(session: &mut Session, code: KeyCode) -> Result<Option<Flow>> {
    let flow = match code {
        KeyCode::Char('c') => Flow::Quit,
        KeyCode::Char('r') => {
            session.refresh()?;
            Flow::Continue
        }
        KeyCode::Char('l') => {
            session.toggle_activity();
            Flow::Continue
        }
        KeyCode::Char('k') => {
            session.toggle_help();
            Flow::Continue
        }
        _ => return Ok(None),
    };

    Ok(Some(flow))
}

fn handle_list_navigation(session: &mut Session, code: KeyCode) -> Flow {
    match code {
        KeyCode::Esc => return Flow::Quit,
        KeyCode::Down => session.next(),
        KeyCode::Up => session.previous(),
        KeyCode::Enter => session.open_action_menu(),
        KeyCode::Delete => session.request_kill(KillSignal::Force),
        KeyCode::Tab => session.cycle_sort(),
        KeyCode::Backspace => session.pop_filter_char(),
        KeyCode::Char(character) => session.push_filter_char(character),
        _ => {}
    }

    Flow::Continue
}
