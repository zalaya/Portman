use std::fmt;

use anyhow::Result;
use serde::Serialize;

use crate::data;
use crate::data::port::{ PortUsage, Risk };

pub const USAGE: &str = "Usage: portman [check] [--json] [--help]

  (no command)   Launch the interactive TUI
  check          Scan once, exit 1 if a Critical port is exposed.
  --json         Print machine-readable JSON instead of text.
  -h, --help     Print this message";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Command {
    Interactive,
    Help,
    Scan { json: bool },
    Check { json: bool },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArgError(String);

impl fmt::Display for ArgError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "unknown argument: {}\n\n{USAGE}", self.0)
    }
}

pub fn parse_args(args: &[String]) -> Result<Command, ArgError> {
    if args.iter().any(|arg| arg == "-h" || arg == "--help") {
        return Ok(Command::Help);
    }

    let json = args.iter().any(|arg| arg == "--json");
    let positional = args.iter().find(|arg| !arg.starts_with('-'));

    match positional.map(String::as_str) {
        Some("check") => Ok(Command::Check { json }),
        Some(other) => Err(ArgError(other.to_string())),
        None if json => Ok(Command::Scan { json: true }),
        None => Ok(Command::Interactive),
    }
}

#[derive(Serialize)]
pub struct PortReport {
    pub port: u16,
    pub protocol: String,
    pub pid: u32,
    pub process: String,
    pub bind: String,
    pub exposed: bool,
    pub risk: String,
}

impl From<&PortUsage> for PortReport {
    fn from(usage: &PortUsage) -> Self {
        PortReport {
            port: usage.port,
            protocol: usage.protocol.to_string(),
            pid: usage.pid,
            process: usage.process_label().to_string(),
            bind: usage.bind_label(),
            exposed: usage.is_exposed(),
            risk: usage.risk().label().to_string(),
        }
    }
}

#[derive(Serialize)]
pub struct CheckReport {
    pub ok: bool,
    pub critical: Vec<PortReport>,
}

pub fn build_reports(items: &[PortUsage]) -> Vec<PortReport> {
    items.iter().map(PortReport::from).collect()
}

pub fn evaluate(items: &[PortUsage]) -> CheckReport {
    let critical: Vec<PortReport> = items.iter().filter(|usage| usage.risk() == Risk::Critical).map(PortReport::from).collect();
    let ok = critical.is_empty();

    CheckReport { ok, critical }
}

pub fn run_scan(json: bool) -> Result<()> {
    let items = data::scan_ports()?;
    let reports = build_reports(&items);

    if json {
        println!("{}", serde_json::to_string_pretty(&reports)?);
    } else if reports.is_empty() {
        println!("No open ports found.");
    } else {
        for report in &reports {
            println!("{:<6} {:<5} {:<20} {:<8} {}", report.port, report.protocol, report.process, report.pid, report.risk);
        }
    }

    Ok(())
}

pub fn run_check(json: bool) -> Result<bool> {
    let items = data::scan_ports()?;
    let report = evaluate(&items);

    if json {
        println!("{}", serde_json::to_string_pretty(&report)?);
    } else if report.ok {
        println!("No critical ports exposed ({} total).", items.len());
    } else {
        println!("{} critical port(s) exposed:", report.critical.len());

        for port in &report.critical {
            println!("  {:<6} {:<5} {} ({})", port.port, port.protocol, port.process, port.pid);
        }
    }

    Ok(report.ok)
}

#[cfg(test)]
mod tests {
    use std::net::IpAddr;

    use super::*;
    use crate::data::network::Protocol;

    fn args(values: &[&str]) -> Vec<String> {
        values.iter().map(|value| value.to_string()).collect()
    }

    #[test]
    fn no_arguments_launches_the_interactive_tui() {
        assert_eq!(parse_args(&args(&[])), Ok(Command::Interactive));
    }

    #[test]
    fn json_flag_alone_means_a_one_shot_scan() {
        assert_eq!(parse_args(&args(&["--json"])), Ok(Command::Scan { json: true }));
    }

    #[test]
    fn check_without_json_is_human_readable() {
        assert_eq!(parse_args(&args(&["check"])), Ok(Command::Check { json: false }));
    }

    #[test]
    fn check_and_json_can_be_combined_in_any_order() {
        assert_eq!(parse_args(&args(&["check", "--json"])), Ok(Command::Check { json: true }));
        assert_eq!(parse_args(&args(&["--json", "check"])), Ok(Command::Check { json: true }));
    }

    #[test]
    fn help_flag_wins_over_everything_else() {
        assert_eq!(parse_args(&args(&["check", "--help"])), Ok(Command::Help));
    }

    #[test]
    fn unknown_positional_argument_is_rejected() {
        assert_eq!(parse_args(&args(&["scan"])), Err(ArgError("scan".to_string())));
    }

    fn usage(port: u16, pid: u32, local_addr: [u8; 4]) -> PortUsage {
        PortUsage { port, protocol: Protocol::Tcp, pid, process_name: Some("test".to_string()), local_addr: IpAddr::from(local_addr) }
    }

    #[test]
    fn evaluate_is_ok_when_nothing_is_exposed() {
        let items = vec![usage(3000, 1, [127, 0, 0, 1])];

        assert!(evaluate(&items).ok);
        assert!(evaluate(&items).critical.is_empty());
    }

    #[test]
    fn evaluate_flags_exposed_sensitive_ports_as_not_ok() {
        let items = vec![usage(6379, 1, [0, 0, 0, 0])];

        let report = evaluate(&items);

        assert!(!report.ok);
        assert_eq!(report.critical.len(), 1);
        assert_eq!(report.critical[0].pid, 1);
    }

    #[test]
    fn evaluate_ignores_exposed_but_non_sensitive_ports() {
        let items = vec![usage(8080, 1, [0, 0, 0, 0])];

        assert!(evaluate(&items).ok, "an exposed dev server isn't Critical unless it's on a sensitive port");
    }

    #[test]
    fn build_reports_covers_every_item() {
        let items = vec![usage(3000, 1, [127, 0, 0, 1]), usage(6379, 2, [0, 0, 0, 0])];

        assert_eq!(build_reports(&items).len(), 2);
    }
}
