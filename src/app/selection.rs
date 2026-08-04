use crate::app::App;
use crate::domain::port::Risk;
use crate::domain::process;

pub struct Details {
    pub address: String,
    pub bind: String,
    pub exposed: bool,
    pub risk: Risk,
    pub process: process::ProcessDetails,
}

impl App {
    pub(super) fn select(&mut self, pid: Option<u32>) {
        let filtered = self.filtered();

        if filtered.is_empty() {
            self.selected = None;
        } else {
            let index = pid.and_then(|pid| filtered.iter().position(|usage| usage.pid == pid)).unwrap_or(0);

            self.selected = Some(index);
        }

        self.refresh_details();
    }

    pub fn next(&mut self) {
        let len = self.filtered().len();

        if len == 0 {
            return;
        }

        let next = match self.selected {
            Some(i) => (i + 1) % len,
            None => 0,
        };

        self.selected = Some(next);
        self.refresh_details();
    }

    pub fn previous(&mut self) {
        let len = self.filtered().len();

        if len == 0 {
            return;
        }

        let previous = match self.selected {
            Some(0) | None => len - 1,
            Some(i) => i - 1,
        };

        self.selected = Some(previous);
        self.refresh_details();
    }

    fn refresh_details(&mut self) {
        let selected = self
            .selected_usage()
            .map(|usage| (usage.pid, usage.address(), usage.bind_description(), usage.is_exposed(), usage.risk()));

        self.details = selected.and_then(|(pid, address, bind, exposed, risk)| {
            process::details(pid, &self.users).map(|process| Details { address, bind, exposed, risk, process })
        });
    }
}
