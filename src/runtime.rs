use std::io::Stdout;
use std::time::Duration;

use anyhow::Result;
use crossterm::event::{ self, Event, KeyEventKind };
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;

use crate::app::App;
use crate::input::{ self, Flow };
use crate::ui;

const REFRESH_INTERVAL: Duration = Duration::from_secs(2);

pub fn run(terminal: &mut Terminal<CrosstermBackend<Stdout>>, app: &mut App) -> Result<()> {
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

        if input::handle_key(app, key)? == Flow::Quit {
            return Ok(());
        }
    }
}
