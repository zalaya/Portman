use std::process::Command;

fn portman(args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_portman")).args(args).output().expect("failed to run the portman binary")
}

#[test]
fn help_flag_prints_usage_and_exits_successfully() {
    let output = portman(&["--help"]);

    assert!(output.status.success());
    assert!(String::from_utf8_lossy(&output.stdout).contains("Usage: portman"));
}

#[test]
fn unknown_argument_is_rejected_with_exit_code_two() {
    let output = portman(&["not-a-real-command"]);

    assert_eq!(output.status.code(), Some(2));
    assert!(String::from_utf8_lossy(&output.stderr).contains("unknown argument"));
}

#[test]
fn json_flag_alone_prints_a_json_array_of_the_local_machine_s_ports() {
    let output = portman(&["--json"]);

    assert!(output.status.success());

    let parsed: serde_json::Value =
        serde_json::from_slice(&output.stdout).expect("--json should always print a single, valid JSON document");
    assert!(parsed.is_array(), "a plain scan should print a JSON array of ports");
}

#[test]
fn check_prints_a_human_readable_summary_and_exits_zero_or_one() {
    let output = portman(&["check"]);

    assert!(
        output.status.code() == Some(0) || output.status.code() == Some(1),
        "check should exit 0 (nothing critical) or 1 (critical port exposed), got {:?}",
        output.status.code()
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("No critical ports exposed") || stdout.contains("critical port(s) exposed"),
        "unexpected check output: {stdout}"
    );
}

#[test]
fn check_with_json_prints_a_valid_json_object_with_an_ok_field() {
    let output = portman(&["check", "--json"]);

    let parsed: serde_json::Value =
        serde_json::from_slice(&output.stdout).expect("check --json should always print a single, valid JSON document");
    assert!(parsed.get("ok").is_some_and(serde_json::Value::is_boolean), "the report should have a boolean \"ok\" field");
    assert!(parsed.get("critical").is_some_and(serde_json::Value::is_array), "the report should have a \"critical\" array");
}
