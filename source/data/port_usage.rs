use std::fmt;

use super::network::{ Listener, Protocol };
use super::process::ProcessTable;

pub struct PortUsage {
    pub port: u16,
    pub protocol: Protocol,
    pub pid: u32,
    pub process_name: Option<String>,
}

impl PortUsage {
    pub fn resolve(processes: &ProcessTable, listener: Listener) -> Self {
        let Listener { port, protocol, pid } = listener;
        let process_name = processes.resolve(pid);

        Self { port, protocol, pid, process_name }
    }

    pub fn address(&self) -> String {
        format!("{}/{}", self.port, self.protocol)
    }

    pub fn process_label(&self) -> &str {
        self.process_name.as_deref().unwrap_or("(exited?)")
    }

    pub fn matches(&self, lowercase_needle: &str) -> bool {
        self.process_label().to_lowercase().contains(lowercase_needle)
            || self.port.to_string().contains(lowercase_needle)
            || self.pid.to_string().contains(lowercase_needle)
    }
}

impl PortUsage {
    fn fmt_found(&self, formatter: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result {
        write!(
            formatter,
            "{}/{} -> {} ({})",
            self.port, self.protocol, name, self.pid
        )
    }

    fn fmt_not_found(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{}/{} -> Process {} not found (exited?)",
            self.port, self.protocol, self.pid
        )
    }
}

impl fmt::Display for PortUsage {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.process_name {
            Some(name) => self.fmt_found(formatter, name),
            None => self.fmt_not_found(formatter),
        }
    }
}
