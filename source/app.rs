use std::collections::HashSet;

use anyhow::Result;
use arboard::Clipboard;
use ratatui::widgets::TableState;

use crate::data::{ network, port_usage::PortUsage, process, process::ProcessSummary, process::ProcessTable };

pub struct KillTarget {
    pub pid: u32,
    pub label: String,
}

pub struct Details {
    pub address: String,
    pub bind: String,
    pub exposed: bool,
    pub process: process::ProcessDetails,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Action {
    Kill,
    ViewTree,
    CopyName,
    CopyPid,
    CopyAddress,
    CopyExecutable,
    CopyWorkingDir,
    Refresh,
}

impl Action {
    pub fn label(self) -> &'static str {
        match self {
            Action::Kill => "Kill process",
            Action::ViewTree => "View related processes",
            Action::CopyName => "Copy process name",
            Action::CopyPid => "Copy PID",
            Action::CopyAddress => "Copy address",
            Action::CopyExecutable => "Copy executable path",
            Action::CopyWorkingDir => "Copy working directory",
            Action::Refresh => "Refresh list",
        }
    }
}

pub struct ActionMenu {
    pub label: String,
    pub actions: Vec<Action>,
    pub selected: usize,
}

pub struct InfoPanel {
    pub title: String,
    pub parent: Option<ProcessSummary>,
    pub current: ProcessSummary,
    pub children: Vec<ProcessSummary>,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum SortKey {
    Port,
    Bind,
    Process,
    Pid,
}

impl SortKey {
    pub fn label(self) -> &'static str {
        match self {
            SortKey::Port => "port",
            SortKey::Bind => "bind",
            SortKey::Process => "process",
            SortKey::Pid => "pid",
        }
    }

    fn next(self) -> Self {
        match self {
            SortKey::Port => SortKey::Bind,
            SortKey::Bind => SortKey::Process,
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
    pub action_menu: Option<ActionMenu>,
    pub info_panel: Option<InfoPanel>,
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
            action_menu: None,
            info_panel: None,
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

    fn select(&mut self, pid: Option<u32>) {
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

    fn selected_usage(&self) -> Option<&PortUsage> {
        self.filtered().into_iter().nth(self.state.selected().unwrap_or(usize::MAX))
    }

    fn refresh_details(&mut self) {
        let selected = self
            .selected_usage()
            .map(|usage| (usage.pid, usage.address(), usage.bind_description(), usage.is_exposed()));

        self.details = selected.and_then(|(pid, address, bind, exposed)| {
            process::details(pid).map(|process| Details { address, bind, exposed, process })
        });
    }

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

        self.refresh()
    }

    pub fn open_action_menu(&mut self) {
        let Some(usage) = self.selected_usage() else {
            return;
        };

        let label = format!("{} ({})", usage.process_label(), usage.address());

        let mut actions = vec![Action::Kill, Action::ViewTree, Action::CopyName, Action::CopyAddress, Action::CopyPid];

        if let Some(details) = &self.details {
            if details.process.exe.is_some() {
                actions.push(Action::CopyExecutable);
            }

            if details.process.cwd.is_some() {
                actions.push(Action::CopyWorkingDir);
            }
        }

        actions.push(Action::Refresh);

        self.action_menu = Some(ActionMenu { label, actions, selected: 0 });
    }

    pub fn close_action_menu(&mut self) {
        self.action_menu = None;
    }

    pub fn action_menu_next(&mut self) {
        if let Some(menu) = &mut self.action_menu {
            menu.selected = (menu.selected + 1) % menu.actions.len();
        }
    }

    pub fn action_menu_previous(&mut self) {
        if let Some(menu) = &mut self.action_menu {
            menu.selected = (menu.selected + menu.actions.len() - 1) % menu.actions.len();
        }
    }

    pub fn confirm_action_menu(&mut self) -> Result<()> {
        let Some(menu) = self.action_menu.take() else {
            return Ok(());
        };

        let Some(action) = menu.actions.get(menu.selected).copied() else {
            return Ok(());
        };

        match action {
            Action::Kill => {
                self.request_kill();
                return Ok(());
            }
            Action::ViewTree => {
                self.show_process_tree();
                return Ok(());
            }
            Action::Refresh => return self.refresh(),
            _ => {}
        }

        let value = match action {
            Action::CopyName => self.selected_usage().map(|usage| usage.process_label().to_string()),
            Action::CopyPid => self.selected_usage().map(|usage| usage.pid.to_string()),
            Action::CopyAddress => self.selected_usage().map(|usage| usage.address()),
            Action::CopyExecutable => self.details.as_ref().and_then(|details| details.process.exe.clone()),
            Action::CopyWorkingDir => self.details.as_ref().and_then(|details| details.process.cwd.clone()),
            Action::Kill | Action::ViewTree | Action::Refresh => unreachable!(),
        };

        self.copy_to_clipboard(action.label(), value);

        Ok(())
    }

    fn copy_to_clipboard(&mut self, label: &str, value: Option<String>) {
        let Some(value) = value else {
            self.status = Some(format!("Nothing to copy for \"{label}\""));
            return;
        };

        match Clipboard::new().and_then(|mut clipboard| clipboard.set_text(value.clone())) {
            Ok(()) => self.status = Some(format!("Copied: {value}")),
            Err(error) => self.status = Some(format!("Could not copy to clipboard: {error}")),
        }
    }

    fn show_process_tree(&mut self) {
        let Some(usage) = self.selected_usage() else {
            return;
        };

        let pid = usage.pid;
        let title = format!("Related processes — {}", usage.process_label());
        let current = ProcessSummary { pid, name: usage.process_label().to_string() };
        let relatives = process::relatives(pid);

        self.info_panel = Some(InfoPanel { title, parent: relatives.parent, current, children: relatives.children });
    }

    pub fn close_info_panel(&mut self) {
        self.info_panel = None;
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
        SortKey::Bind => items.sort_by(|a, b| a.bind_label().cmp(&b.bind_label())),
        SortKey::Process => items.sort_by(|a, b| a.process_label().cmp(b.process_label())),
        SortKey::Pid => items.sort_by_key(|usage| usage.pid),
    }

    items
}
