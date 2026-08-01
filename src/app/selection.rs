use crate::app::App;
use crate::data::port::Risk;
use crate::data::process;

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
            self.state.select(None);
        } else {
            let index = pid.and_then(|pid| filtered.iter().position(|usage| usage.pid == pid)).unwrap_or(0);

            self.state.select(Some(index));
        }

        self.refresh_details();
    }

    pub fn next(&mut self) {
        let len = self.filtered().len();

        if len == 0 {
            return;
        }

        let next = match self.state.selected() {
            Some(i) => (i + 1) % len,
            None => 0,
        };

        self.state.select(Some(next));
        self.refresh_details();
    }

    pub fn previous(&mut self) {
        let len = self.filtered().len();

        if len == 0 {
            return;
        }

        let previous = match self.state.selected() {
            Some(0) | None => len - 1,
            Some(i) => i - 1,
        };

        self.state.select(Some(previous));
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
