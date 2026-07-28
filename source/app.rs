use std::collections::HashSet;

use anyhow::Result;
use ratatui::widgets::TableState;

use crate::data::network::Protocol;
use crate::data::{ network, opener, port_usage::PortUsage, process, process::ProcessTable };

pub struct KillTarget {
    pub pid: u32,
    pub label: String,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum OpenField {
    Port,
    Command,
}

pub struct OpenPrompt {
    pub port_input: String,
    pub protocol: Protocol,
    pub command: String,
    pub focus: OpenField,
}

pub struct Details {
    pub address: String,
    pub bind: String,
    pub exposed: bool,
    pub process: process::ProcessDetails,
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
    pub details: Option<Details>,
    pub status: Option<String>,
    pub new_keys: HashSet<(String, u32)>,
    seen_keys: Option<HashSet<(String, u32)>>,
    pub open_prompt: Option<OpenPrompt>,
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
            open_prompt: None,
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
            self.details = process::details(details.process.pid).map(|process| Details {
                address: details.address.clone(),
                bind: details.bind.clone(),
                exposed: details.exposed,
                process,
            });
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

        if usage.pid == std::process::id() {
            self.status = Some("That's portman itself (a port you opened) — can't kill it from here".to_string());
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

    pub fn open_details(&mut self) {
        let Some(usage) = self.selected_usage() else {
            return;
        };

        let address = usage.address();
        let bind = usage.bind_label();
        let exposed = usage.is_exposed();

        self.details = process::details(usage.pid).map(|process| Details { address, bind, exposed, process });
    }

    pub fn close_details(&mut self) {
        self.details = None;
    }

    pub fn start_open_prompt(&mut self) {
        self.open_prompt = Some(OpenPrompt {
            port_input: String::new(),
            protocol: Protocol::Tcp,
            command: String::new(),
            focus: OpenField::Port,
        });
    }

    pub fn cancel_open_prompt(&mut self) {
        self.open_prompt = None;
    }

    pub fn toggle_open_focus(&mut self) {
        if let Some(prompt) = &mut self.open_prompt {
            prompt.focus = match prompt.focus {
                OpenField::Port => OpenField::Command,
                OpenField::Command => OpenField::Port,
            };
        }
    }

    pub fn toggle_open_protocol(&mut self) {
        if let Some(prompt) = &mut self.open_prompt {
            prompt.protocol = match prompt.protocol {
                Protocol::Tcp => Protocol::Udp,
                Protocol::Udp => Protocol::Tcp,
            };
        }
    }

    pub fn push_open_char(&mut self, character: char) {
        let Some(prompt) = &mut self.open_prompt else {
            return;
        };

        match prompt.focus {
            OpenField::Port if character.is_ascii_digit() => {
                if prompt.port_input.len() < 5 {
                    prompt.port_input.push(character);
                }
            }
            // Anything that isn't a port digit means the user has moved on to typing
            // the command — jump focus there instead of silently dropping the key.
            OpenField::Port => {
                prompt.focus = OpenField::Command;
                prompt.command.push(character);
            }
            OpenField::Command => prompt.command.push(character),
        }
    }

    pub fn pop_open_char(&mut self) {
        let Some(prompt) = &mut self.open_prompt else {
            return;
        };

        match prompt.focus {
            OpenField::Port => {
                prompt.port_input.pop();
            }
            OpenField::Command => {
                prompt.command.pop();
            }
        }
    }

    pub fn confirm_open(&mut self) -> Result<()> {
        let Some(prompt) = self.open_prompt.take() else {
            return Ok(());
        };

        let port = match prompt.port_input.parse::<u16>() {
            Ok(port) if port > 0 => port,
            _ => {
                self.status = Some("Enter a port between 1 and 65535".to_string());
                return Ok(());
            }
        };

        let socket = match opener::open(port, prompt.protocol) {
            Ok(socket) => socket,
            Err(error) => {
                self.status = Some(format!("Could not open {port}/{}: {error}", prompt.protocol));
                return Ok(());
            }
        };

        let command = prompt.command.trim();

        if command.is_empty() {
            drop(socket);
            self.status = Some("Enter a command to run on that port".to_string());
            return Ok(());
        }

        // Release the port ourselves so the launched command can bind it.
        drop(socket);

        let mut parts = command.split_whitespace();
        let program = parts.next().expect("command is non-empty after trim");

        match std::process::Command::new(program).args(parts).spawn() {
            Ok(_) => {
                self.status = Some(format!("Launched `{command}`"));
                self.refresh()
            }
            Err(error) => {
                self.status = Some(format!("Could not launch `{command}`: {error}"));
                Ok(())
            }
        }
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
