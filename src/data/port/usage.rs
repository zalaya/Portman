use std::net::IpAddr;

use super::risk::Risk;
use crate::data::network::{ Listener, Protocol };
use crate::data::process::ProcessTable;

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
            format!("Localhost only ({}) — not reachable from the network", self.local_addr)
        } else if self.local_addr.is_unspecified() {
            format!("Public ({}) — reachable from the network", self.local_addr)
        } else {
            format!("Specific interface ({}) — reachable from that network", self.local_addr)
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
}
