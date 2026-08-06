use std::net::IpAddr;

use anyhow::Result;

use crate::scanning::network::{ self, Listener, Protocol };
use crate::scanning::process::{ self, ProcessTable };

const SENSITIVE_PORTS: &[u16] = &[
    21, 22, 23, 25, 111, 135, 139, 445, 1433, 1521, 2375, 2376, 3306, 3389, 5432, 5900, 5901, 6379, 7001, 8020, 9200, 9300, 11211, 27017,
    27018, 50070,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Risk {
    Safe,
    Watch,
    Critical,
}

impl Risk {
    pub fn classify(is_exposed: bool, port: u16) -> Self {
        if !is_exposed {
            Risk::Safe
        } else if SENSITIVE_PORTS.contains(&port) {
            Risk::Critical
        } else {
            Risk::Watch
        }
    }

    pub fn icon(self) -> &'static str {
        "●"
    }

    pub fn label(self) -> &'static str {
        match self {
            Risk::Safe => "Safe",
            Risk::Watch => "Watch",
            Risk::Critical => "Critical",
        }
    }

    pub fn description(self) -> &'static str {
        match self {
            Risk::Safe => "Only reachable from this machine",
            Risk::Watch => "Reachable from the network",
            Risk::Critical => "Reachable from the network on a sensitive port",
        }
    }

    pub fn severity(self) -> u8 {
        match self {
            Risk::Critical => 0,
            Risk::Watch => 1,
            Risk::Safe => 2,
        }
    }
}

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

    pub fn bind_label(&self) -> String {
        if self.local_addr.is_loopback() {
            "Localhost".to_string()
        } else if self.local_addr.is_unspecified() {
            "Public".to_string()
        } else {
            self.local_addr.to_string()
        }
    }

    pub fn bind_description(&self) -> String {
        if self.local_addr.is_loopback() {
            format!("Localhost only ({}) — Not reachable from the network", self.local_addr)
        } else if self.local_addr.is_unspecified() {
            format!("Public ({}) — Reachable from the network", self.local_addr)
        } else {
            format!("Specific interface ({}) — Reachable from that network", self.local_addr)
        }
    }

    pub fn is_exposed(&self) -> bool {
        !self.local_addr.is_loopback()
    }

    pub fn risk(&self) -> Risk {
        Risk::classify(self.is_exposed(), self.port)
    }

    pub fn risk_severity(&self) -> u8 {
        self.risk().severity()
    }

    pub fn matches(&self, lowercase_needle: &str) -> bool {
        self.process_label().to_lowercase().contains(lowercase_needle)
            || self.address().to_lowercase().contains(lowercase_needle)
            || self.pid.to_string().contains(lowercase_needle)
            || self.bind_label().to_lowercase().contains(lowercase_needle)
    }
}

pub fn scan() -> Result<Vec<PortUsage>> {
    let processes = process::ProcessTable::snapshot();

    Ok(network::scan()?.into_iter().map(|listener| PortUsage::resolve(&processes, listener)).collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn usage(local_addr: [u8; 4]) -> PortUsage {
        PortUsage { port: 8080, protocol: Protocol::Tcp, pid: 1, process_name: Some("test".to_string()), local_addr: IpAddr::from(local_addr) }
    }

    #[test]
    fn address_combines_port_and_protocol() {
        assert_eq!(usage([127, 0, 0, 1]).address(), "8080/TCP");
    }

    #[test]
    fn process_label_falls_back_when_the_process_is_unresolved() {
        let mut orphaned = usage([127, 0, 0, 1]);
        orphaned.process_name = None;

        assert_eq!(orphaned.process_label(), "(exited?)");
    }

    #[test]
    fn loopback_is_not_exposed() {
        assert!(!usage([127, 0, 0, 1]).is_exposed());
        assert_eq!(usage([127, 0, 0, 1]).bind_label(), "Localhost");
    }

    #[test]
    fn unspecified_is_exposed_and_labeled_public() {
        assert!(usage([0, 0, 0, 0]).is_exposed());
        assert_eq!(usage([0, 0, 0, 0]).bind_label(), "Public");
    }

    #[test]
    fn a_specific_interface_is_exposed_and_shown_as_its_address() {
        let bound = usage([192, 168, 1, 5]);

        assert!(bound.is_exposed());
        assert_eq!(bound.bind_label(), "192.168.1.5");
    }

    #[test]
    fn matches_is_case_insensitive_across_process_address_pid_and_bind() {
        let item = usage([127, 0, 0, 1]);

        assert!(item.matches("test"), "should match process name");
        assert!(item.matches("8080"), "should match port");
        assert!(item.matches("tcp"), "should match protocol");
        assert!(item.matches("1"), "should match pid");
        assert!(item.matches("localhost"), "should match bind label");
        assert!(!item.matches("nope"), "should not match unrelated text");
    }

    #[test]
    fn loopback_ports_are_always_safe_even_on_a_sensitive_port() {
        assert_eq!(Risk::classify(false, 22), Risk::Safe);
    }

    #[test]
    fn exposed_ordinary_port_is_watch() {
        assert_eq!(Risk::classify(true, 8080), Risk::Watch);
    }

    #[test]
    fn exposed_sensitive_port_is_critical() {
        assert_eq!(Risk::classify(true, 6379), Risk::Critical);
    }

    #[test]
    fn severity_orders_critical_before_watch_before_safe() {
        assert!(Risk::Critical.severity() < Risk::Watch.severity());
        assert!(Risk::Watch.severity() < Risk::Safe.severity());
    }
}
