use std::io::stdout;
use std::process::ExitCode;

use anyhow::Result;
use crossterm::execute;
use crossterm::terminal::{ EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode };
use portman::app::App;
use portman::cli::{ self, Command };
use portman::terminal::event_loop;
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;

fn main() -> Result<ExitCode> {
    let args: Vec<String> = std::env::args().skip(1).collect();

    match cli::parse_args(&args) {
        Err(error) => {
            eprintln!("{error}");
            return Ok(ExitCode::from(2));
        }
        Ok(Command::Help) => {
            println!("{}", cli::USAGE);
            return Ok(ExitCode::SUCCESS);
        }
        Ok(Command::Scan { json }) => {
            cli::run_scan(json)?;
            return Ok(ExitCode::SUCCESS);
        }
        Ok(Command::Check { json }) => {
            let ok = cli::run_check(json)?;
            return Ok(if ok { ExitCode::SUCCESS } else { ExitCode::FAILURE });
        }
        Ok(Command::Interactive) => {}
    }

    enable_raw_mode()?;
    execute!(stdout(), EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout());
    let mut terminal = Terminal::new(backend)?;
    let mut app = App::new()?;
    let result = event_loop::run(&mut terminal, &mut app);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    result?;

    Ok(ExitCode::SUCCESS)
}
