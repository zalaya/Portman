mod support;

use crossterm::event::{ KeyCode, KeyEvent, KeyModifiers };
use portman::app::App;
use portman::input::{ Flow, handle_key };

fn key(code: KeyCode) -> KeyEvent {
    KeyEvent::new(code, KeyModifiers::NONE)
}

fn ctrl_key(code: KeyCode) -> KeyEvent {
    KeyEvent::new(code, KeyModifiers::CONTROL)
}

#[test]
fn esc_quits_from_plain_list_navigation() -> anyhow::Result<()> {
    let mut app = App::new()?;

    assert_eq!(handle_key(&mut app, key(KeyCode::Esc))?, Flow::Quit);

    Ok(())
}

#[test]
fn esc_cancels_a_pending_kill_instead_of_quitting() -> anyhow::Result<()> {
    let mut app = App::new()?;

    app.items = vec![support::loopback_tcp(3000, 424_242, "node")];
    app.state.select(Some(0));
    app.request_kill();
    assert!(app.kill_target.is_some());

    assert_eq!(handle_key(&mut app, key(KeyCode::Esc))?, Flow::Continue, "a kill confirmation should absorb Esc, not quit");
    assert!(app.kill_target.is_none(), "Esc should cancel the pending kill");

    Ok(())
}

#[test]
fn esc_closes_the_action_menu_instead_of_quitting() -> anyhow::Result<()> {
    let mut app = App::new()?;

    app.items = vec![support::loopback_tcp(8080, 1, "dev-server")];
    app.state.select(Some(0));
    app.open_action_menu();

    assert_eq!(handle_key(&mut app, key(KeyCode::Esc))?, Flow::Continue);
    assert!(app.action_menu.is_none());

    Ok(())
}

#[test]
fn any_key_dismisses_the_help_overlay() -> anyhow::Result<()> {
    let mut app = App::new()?;
    app.toggle_help();

    assert_eq!(handle_key(&mut app, key(KeyCode::Char('z')))?, Flow::Continue);
    assert!(!app.show_help);

    Ok(())
}

#[test]
fn typed_characters_go_to_the_search_box_when_nothing_else_is_open() -> anyhow::Result<()> {
    let mut app = App::new()?;

    handle_key(&mut app, key(KeyCode::Char('r')))?;
    handle_key(&mut app, key(KeyCode::Char('e')))?;
    handle_key(&mut app, key(KeyCode::Char('d')))?;

    assert_eq!(app.filter, "red", "plain characters (no Ctrl) should type into the filter, not trigger shortcuts");

    Ok(())
}

#[test]
fn ctrl_r_refreshes_instead_of_typing_into_the_search_box() -> anyhow::Result<()> {
    let mut app = App::new()?;

    assert_eq!(handle_key(&mut app, ctrl_key(KeyCode::Char('r')))?, Flow::Continue);
    assert_eq!(app.filter, "", "Ctrl+R is a shortcut, it must not fall through to the filter");

    Ok(())
}

#[test]
fn ctrl_l_opens_the_activity_log_and_esc_closes_it() -> anyhow::Result<()> {
    let mut app = App::new()?;

    handle_key(&mut app, ctrl_key(KeyCode::Char('l')))?;
    assert!(app.show_activity);

    handle_key(&mut app, key(KeyCode::Esc))?;
    assert!(!app.show_activity, "Esc should close the activity log rather than quitting the app");

    Ok(())
}
