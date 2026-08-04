use anyhow::Result;
use crossterm::event::{ KeyCode, KeyEvent, KeyModifiers };

use crate::app::App;
use crate::domain::process::KillSignal;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Flow {
    Continue,
    Quit,
}

pub fn handle_key(app: &mut App, key: KeyEvent) -> Result<Flow> {
    app.status = None;

    if app.kill_target.is_some() {
        return handle_kill_confirmation(app, key.code);
    }

    if app.show_help {
        app.close_help();
        return Ok(Flow::Continue);
    }

    if app.show_activity {
        return Ok(handle_activity_log(app, key.code));
    }

    if app.action_menu.is_some() {
        return handle_action_menu(app, key.code);
    }

    if key.modifiers.contains(KeyModifiers::CONTROL)
        && let Some(flow) = handle_shortcut(app, key.code)?
    {
        return Ok(flow);
    }

    Ok(handle_list_navigation(app, key.code))
}

fn handle_kill_confirmation(app: &mut App, code: KeyCode) -> Result<Flow> {
    match code {
        KeyCode::Char('y') | KeyCode::Enter => app.confirm_kill()?,
        KeyCode::Char('n') | KeyCode::Esc => app.cancel_kill(),
        _ => {}
    }

    Ok(Flow::Continue)
}

fn handle_activity_log(app: &mut App, code: KeyCode) -> Flow {
    match code {
        KeyCode::Esc | KeyCode::Enter => app.close_activity(),
        _ => {}
    }

    Flow::Continue
}

fn handle_action_menu(app: &mut App, code: KeyCode) -> Result<Flow> {
    match code {
        KeyCode::Up => app.action_menu_previous(),
        KeyCode::Down => app.action_menu_next(),
        KeyCode::Enter => app.confirm_action_menu()?,
        KeyCode::Esc => app.close_action_menu(),
        _ => {}
    }

    Ok(Flow::Continue)
}

fn handle_shortcut(app: &mut App, code: KeyCode) -> Result<Option<Flow>> {
    let flow = match code {
        KeyCode::Char('c') => Flow::Quit,
        KeyCode::Char('r') => {
            app.refresh()?;
            Flow::Continue
        }
        KeyCode::Char('l') => {
            app.toggle_activity();
            Flow::Continue
        }
        KeyCode::Char('k') => {
            app.toggle_help();
            Flow::Continue
        }
        _ => return Ok(None),
    };

    Ok(Some(flow))
}

fn handle_list_navigation(app: &mut App, code: KeyCode) -> Flow {
    match code {
        KeyCode::Esc => return Flow::Quit,
        KeyCode::Down => app.next(),
        KeyCode::Up => app.previous(),
        KeyCode::Enter => app.open_action_menu(),
        KeyCode::Delete => app.request_kill(KillSignal::Force),
        KeyCode::Tab => app.cycle_sort(),
        KeyCode::Backspace => app.pop_filter_char(),
        KeyCode::Char(character) => app.push_filter_char(character),
        _ => {}
    }

    Flow::Continue
}
