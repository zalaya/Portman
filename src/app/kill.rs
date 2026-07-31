use anyhow::Result;

use crate::app::App;
use crate::data::process;

pub struct KillTarget {
    pub pid: u32,
    pub label: String,
}

impl App {
    pub fn request_kill(&mut self) {
        let Some(usage) = self.selected_usage() else {
            return;
        };

        if usage.pid == std::process::id() {
            self.status = Some("That's portman itself — can't kill it from here".to_string());
            return;
        }

        self.kill_target = Some(KillTarget {
            pid: usage.pid,
            label: format!("{} ({})", usage.process_label(), usage.address()),
        });
    }

    pub fn cancel_kill(&mut self) {
        self.kill_target = None;
    }

    pub fn confirm_kill(&mut self) -> Result<()> {
        let Some(target) = self.kill_target.take() else {
            return Ok(());
        };

        if !process::kill(target.pid) {
            self.status = Some(format!("Could not kill {} — check permissions", target.label));
            return Ok(());
        }

        self.log_event(format!("Killed {}", target.label));
        self.refresh()
    }
}
