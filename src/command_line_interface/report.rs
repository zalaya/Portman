use serde::Serialize;

use crate::scanning::port::{ PortUsage, Risk };

#[derive(Serialize)]
pub struct PortReport {
    pub port: u16,
    pub protocol: String,
    pub pid: u32,
    pub process: String,
    pub bind: String,
    pub exposed: bool,
    pub risk: String,
}

impl From<&PortUsage> for PortReport {
    fn from(usage: &PortUsage) -> Self {
        PortReport {
            port: usage.port,
            protocol: usage.protocol.to_string(),
            pid: usage.pid,
            process: usage.process_label().to_string(),
            bind: usage.bind_label(),
            exposed: usage.is_exposed(),
            risk: usage.risk().label().to_string(),
        }
    }
}

#[derive(Serialize)]
pub struct CheckReport {
    pub ok: bool,
    pub critical: Vec<PortReport>,
}

pub fn build_reports(items: &[PortUsage]) -> Vec<PortReport> {
    items.iter().map(PortReport::from).collect()
}

pub fn evaluate(items: &[PortUsage]) -> CheckReport {
    let critical: Vec<PortReport> = items.iter().filter(|usage| usage.risk() == Risk::Critical).map(PortReport::from).collect();
    let ok = critical.is_empty();

    CheckReport { ok, critical }
}

#[cfg(test)]
mod tests {
    use std::net::IpAddr;

    use super::*;
    use crate::scanning::network::Protocol;

    fn usage(port: u16, pid: u32, local_addr: [u8; 4]) -> PortUsage {
        PortUsage { port, protocol: Protocol::Tcp, pid, process_name: Some("test".to_string()), local_addr: IpAddr::from(local_addr) }
    }

    #[test]
    fn evaluate_is_ok_when_nothing_is_exposed() {
        let items = vec![usage(3000, 1, [127, 0, 0, 1])];

        assert!(evaluate(&items).ok);
        assert!(evaluate(&items).critical.is_empty());
    }

    #[test]
    fn evaluate_flags_exposed_sensitive_ports_as_not_ok() {
        let items = vec![usage(6379, 1, [0, 0, 0, 0])];

        let report = evaluate(&items);

        assert!(!report.ok);
        assert_eq!(report.critical.len(), 1);
        assert_eq!(report.critical[0].pid, 1);
    }

    #[test]
    fn evaluate_ignores_exposed_but_non_sensitive_ports() {
        let items = vec![usage(8080, 1, [0, 0, 0, 0])];

        assert!(evaluate(&items).ok, "an exposed dev server isn't Critical unless it's on a sensitive port");
    }

    #[test]
    fn build_reports_covers_every_item() {
        let items = vec![usage(3000, 1, [127, 0, 0, 1]), usage(6379, 2, [0, 0, 0, 0])];

        assert_eq!(build_reports(&items).len(), 2);
    }
}
