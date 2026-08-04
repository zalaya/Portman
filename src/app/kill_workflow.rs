use anyhow::Result;

use crate::app::App;
use crate::domain::process::{ self, KillOutcome, KillSignal };

pub struct KillTarget {
    pub pid: u32,
    pub label: String,
    pub signal: KillSignal,
}

impl App {
    pub fn request_kill(&mut self, signal: KillSignal) {
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
            signal,
        });
    }

    pub fn cancel_kill(&mut self) {
        self.kill_target = None;
    }

    pub fn confirm_kill(&mut self) -> Result<()> {
        let Some(target) = self.kill_target.take() else {
            return Ok(());
        };

        match process::kill(target.pid, target.signal) {
            KillOutcome::Succeeded => {
                self.log_event(format!("{} {}", target.signal.verb(), target.label));
                return self.refresh();
            }
            KillOutcome::Failed => self.status = Some(format!("Could not kill {} — check permissions", target.label)),
            KillOutcome::ProcessNotFound => self.status = Some(format!("{} is already gone", target.label)),
            KillOutcome::SignalUnsupported => self.status = Some("That signal isn't supported on this platform".to_string()),
        }

        Ok(())
    }
}
