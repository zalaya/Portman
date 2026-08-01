use std::fmt;

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

#[cfg(test)]
mod tests {
    use super::*;

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
}
