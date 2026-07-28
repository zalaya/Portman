use std::collections::HashSet;

use anyhow::Result;
use ratatui::widgets::TableState;

use crate::data::{ network, port_usage::PortUsage, process, process::ProcessTable };

pub struct KillTarget {
    pub pid: u32,
    pub label: String,
}

pub struct Details {
    pub address: String,
    pub process: process::ProcessDetails,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum SortKey {
    Port,
    Process,
    Pid,
}

impl SortKey {
    pub fn label(self) -> &'static str {
        match self {
            SortKey::Port => "port",
            SortKey::Process => "process",
            SortKey::Pid => "pid",
        }
    }

    fn next(self) -> Self {
        match self {
            SortKey::Port => SortKey::Process,
            SortKey::Process => SortKey::Pid,
            SortKey::Pid => SortKey::Port,
        }
    }
}

pub struct App {
    pub items: Vec<PortUsage>,
    pub filter: String,
    pub sort: SortKey,
    pub state: TableState,
    pub kill_target: Option<KillTarget>,
    pub details: Option<Details>,
    pub status: Option<String>,
    pub new_keys: HashSet<(String, u32)>,
    seen_keys: Option<HashSet<(String, u32)>>,
}

impl App {
    pub fn new() -> Result<Self> {
        let mut app = Self {
            items: Vec::new(),
            filter: String::new(),
            sort: SortKey::Port,
            state: TableState::default(),
            kill_target: None,
            details: None,
            status: None,
            new_keys: HashSet::new(),
            seen_keys: None,
        };

        app.refresh()?;

        Ok(app)
    }

    pub fn refresh(&mut self) -> Result<()> {
        let previous_pid = self.selected_usage().map(|usage| usage.pid);
        let processes = ProcessTable::snapshot();

        self.items = network::scan()?
            .into_iter()
            .map(|listener| PortUsage::resolve(&processes, listener))
            .collect();

        let current_keys: HashSet<(String, u32)> = self.items.iter().map(|usage| (usage.address(), usage.pid)).collect();

        self.new_keys = match &self.seen_keys {
            Some(previous) => current_keys.difference(previous).cloned().collect(),
            None => HashSet::new(),
        };
        self.seen_keys = Some(current_keys);

        self.select(previous_pid);

        if let Some(details) = &self.details {
            self.details = process::details(details.process.pid)
                .map(|process| Details { address: details.address.clone(), process });
        }

        Ok(())
    }

    pub fn filtered(&self) -> Vec<&PortUsage> {
        filtered_items(&self.items, &self.filter, self.sort)
    }

    pub fn cycle_sort(&mut self) {
        self.sort = self.sort.next();
        self.select(self.selected_usage().map(|usage| usage.pid));
    }

    pub fn push_filter_char(&mut self, character: char) {
        self.filter.push(character);
        self.select(None);
    }

    pub fn pop_filter_char(&mut self) {
        self.filter.pop();
        self.select(None);
    }

    // Selects `pid` if it's still present in the filtered list, otherwise falls back to the
    // first match. `None` always jumps to the first match (filter changes, sort changes).
    fn select(&mut self, pid: Option<u32>) {
        let filtered = self.filtered();

        if filtered.is_empty() {
            self.state.select(None);
            return;
        }

        let index = pid.and_then(|pid| filtered.iter().position(|usage| usage.pid == pid)).unwrap_or(0);

        self.state.select(Some(index));
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
    }

    fn selected_usage(&self) -> Option<&PortUsage> {
        self.filtered().into_iter().nth(self.state.selected().unwrap_or(usize::MAX))
    }

    pub fn request_kill(&mut self) {
        let Some(usage) = self.selected_usage() else {
            return;
        };

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

        self.refresh()
    }

    pub fn open_details(&mut self) {
        let Some(usage) = self.selected_usage() else {
            return;
        };

        let address = usage.address();

        self.details = process::details(usage.pid).map(|process| Details { address, process });
    }

    pub fn close_details(&mut self) {
        self.details = None;
    }
}

pub fn filtered_items<'a>(items: &'a [PortUsage], filter: &str, sort: SortKey) -> Vec<&'a PortUsage> {
    let mut items: Vec<&PortUsage> = if filter.is_empty() {
        items.iter().collect()
    } else {
        let needle = filter.to_lowercase();

        items.iter().filter(|usage| usage.matches(&needle)).collect()
    };

    match sort {
        SortKey::Port => items.sort_by_key(|usage| usage.port),
        SortKey::Process => items.sort_by(|a, b| a.process_label().cmp(b.process_label())),
        SortKey::Pid => items.sort_by_key(|usage| usage.pid),
    }

    items
}
