use std::net::IpAddr;

use super::network::{ Listener, Protocol };
use super::process::ProcessTable;

pub struct PortUsage {
    pub port: u16,
    pub protocol: Protocol,
    pub pid: u32,
    pub process_name: Option<String>,
    pub local_addr: IpAddr,
}

impl PortUsage {
    pub fn resolve(processes: &ProcessTable, listener: Listener) -> Self {
        let Listener { port, protocol, pid, local_addr } = listener;
        let process_name = processes.resolve(pid);

        Self { port, protocol, pid, process_name, local_addr }
    }

    pub fn address(&self) -> String {
        format!("{}/{}", self.port, self.protocol)
    }

    pub fn process_label(&self) -> &str {
        self.process_name.as_deref().unwrap_or("(exited?)")
    }

    /// Short label for how reachable this socket is: loopback-only, every
    /// interface, or the specific bound IP.
    pub fn bind_label(&self) -> String {
        if self.local_addr.is_loopback() {
            "Localhost".to_string()
        } else if self.local_addr.is_unspecified() {
            "Public".to_string()
        } else {
            self.local_addr.to_string()
        }
    }

    /// Longer, human-readable explanation of the same thing, for the details panel.
    pub fn bind_description(&self) -> String {
        if self.local_addr.is_loopback() {
            format!("Localhost only ({}) — not reachable from the network", self.local_addr)
        } else if self.local_addr.is_unspecified() {
            format!("Public ({}) — reachable from the network", self.local_addr)
        } else {
            format!("Specific interface ({}) — reachable from that network", self.local_addr)
        }
    }

    /// Reachable from outside the machine (bound to all interfaces or a specific
    /// non-loopback one), as opposed to loopback-only.
    pub fn is_exposed(&self) -> bool {
        !self.local_addr.is_loopback()
    }

    pub fn matches(&self, lowercase_needle: &str) -> bool {
        self.process_label().to_lowercase().contains(lowercase_needle)
            || self.address().to_lowercase().contains(lowercase_needle)
            || self.pid.to_string().contains(lowercase_needle)
            || self.bind_label().to_lowercase().contains(lowercase_needle)
    }
}
