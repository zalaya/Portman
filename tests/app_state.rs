mod support;

use portman::app::{ App, SortKey };
use portman::domain::port::Risk;
use portman::domain::process::KillSignal;

#[test]
fn killing_portman_itself_is_blocked() -> anyhow::Result<()> {
    let mut app = App::new()?;

    app.items = vec![support::loopback_tcp(9999, support::own_pid(), "test-harness")];
    app.selected = Some(0);

    app.request_kill(KillSignal::Force);

    assert!(app.kill_target.is_none(), "should refuse to set up a kill target for portman's own process");
    assert_eq!(app.status.as_deref(), Some("That's portman itself — can't kill it from here"));

    Ok(())
}

#[test]
fn killing_another_process_asks_for_confirmation_first() -> anyhow::Result<()> {
    let mut app = App::new()?;

    app.items = vec![support::loopback_tcp(3000, 424_242, "node")];
    app.selected = Some(0);

    app.request_kill(KillSignal::Terminate);

    let target = app.kill_target.as_ref().expect("should stage a kill target for a process that isn't us");
    assert_eq!(target.pid, 424_242);
    assert_eq!(target.signal, KillSignal::Terminate);

    app.cancel_kill();
    assert!(app.kill_target.is_none(), "cancelling should clear the pending kill");

    Ok(())
}

#[test]
fn typing_into_the_filter_narrows_the_list_to_matches() -> anyhow::Result<()> {
    let mut app = App::new()?;

    app.items = vec![
        support::loopback_tcp(3000, 1, "node"),
        support::loopback_tcp(5432, 2, "postgres"),
        support::public_tcp(6379, 3, "redis-server"),
    ];
    app.selected = Some(0);

    for character in "redis".chars() {
        app.push_filter_char(character);
    }

    let visible: Vec<u32> = app.filtered().iter().map(|usage| usage.pid).collect();
    assert_eq!(visible, [3], "only the redis listener should match");

    app.pop_filter_char();
    assert_eq!(app.filter, "redi");

    Ok(())
}

#[test]
fn cycling_sort_visits_every_key_and_returns_to_port() -> anyhow::Result<()> {
    let mut app = App::new()?;
    assert_eq!(app.sort, SortKey::Port);

    for _ in 0..5 {
        app.cycle_sort();
    }

    assert_eq!(app.sort, SortKey::Port, "five cycles should be a full loop back to the start");

    Ok(())
}

#[test]
fn opening_the_action_menu_on_a_tcp_port_offers_the_browser_action() -> anyhow::Result<()> {
    let mut app = App::new()?;

    app.items = vec![support::loopback_tcp(8080, u32::MAX, "dev-server")];
    app.selected = Some(0);

    app.open_action_menu();

    let menu = app.action_menu.as_ref().expect("selecting a row and opening the menu should populate it");
    let labels: Vec<&str> = menu.actions.iter().map(|action| action.label()).collect();

    assert_eq!(
        labels,
        ["Terminate (SIGTERM)", "Force kill (SIGKILL)", "Open in browser", "Copy PID", "Copy address", "Refresh list"],
        "a fake PID has no resolvable process details, so \"Copy full command\" shouldn't appear"
    );

    app.close_action_menu();
    assert!(app.action_menu.is_none());

    Ok(())
}

#[test]
fn opening_the_action_menu_offers_copy_command_when_details_have_a_command_line() -> anyhow::Result<()> {
    let mut app = App::new()?;

    app.items = vec![support::loopback_tcp(8080, u32::MAX, "dev-server")];
    app.selected = Some(0);
    app.details = Some(support::details_with_command(u32::MAX, vec!["node", "server.js"]));

    app.open_action_menu();

    let menu = app.action_menu.as_ref().unwrap();
    assert!(menu.actions.iter().any(|action| action.label() == "Copy full command"));

    Ok(())
}

#[test]
fn selecting_a_running_process_populates_the_details_pane() -> anyhow::Result<()> {
    let mut app = App::new()?;

    app.items = vec![support::loopback_tcp(3000, support::own_pid(), "test-harness")];
    app.selected = None;
    app.next();

    let details = app.details.as_ref().expect("selecting a real, running process should populate the details pane");

    assert_eq!(details.process.pid, support::own_pid());
    assert_eq!(details.risk, Risk::Safe, "a loopback-only listener should never be flagged as reachable");
    assert!(!details.exposed);

    Ok(())
}

#[test]
fn filtering_to_no_matches_clears_selection_and_details() -> anyhow::Result<()> {
    let mut app = App::new()?;

    app.items = vec![support::loopback_tcp(3000, support::own_pid(), "test-harness")];
    app.selected = None;
    app.next();
    assert!(app.details.is_some(), "sanity check: something should be selected before we filter it away");

    for character in "this-will-never-match-anything".chars() {
        app.push_filter_char(character);
    }

    assert!(app.filtered().is_empty());
    assert!(app.selected.is_none());
    assert!(app.details.is_none());

    app.next();
    app.previous();

    Ok(())
}
