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

#[cfg(test)]
mod tests {
    use super::*;

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
