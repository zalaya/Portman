use crate::session::Session;
use crate::scanning::port::PortUsage;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SortKey {
    Port,
    Bind,
    Process,
    Pid,
    Risk,
}

impl SortKey {
    pub fn label(self) -> &'static str {
        match self {
            SortKey::Port => "port",
            SortKey::Bind => "bind",
            SortKey::Process => "process",
            SortKey::Pid => "pid",
            SortKey::Risk => "risk",
        }
    }

    fn next(self) -> Self {
        match self {
            SortKey::Port => SortKey::Bind,
            SortKey::Bind => SortKey::Process,
            SortKey::Process => SortKey::Pid,
            SortKey::Pid => SortKey::Risk,
            SortKey::Risk => SortKey::Port,
        }
    }
}

impl Session {
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
}

pub fn filtered_items<'a>(items: &'a [PortUsage], filter: &str, sort: SortKey) -> Vec<&'a PortUsage> {
    let needle = filter.to_lowercase();

    let mut items: Vec<&PortUsage> = items.iter().filter(|usage| filter.is_empty() || usage.matches(&needle)).collect();

    match sort {
        SortKey::Port => items.sort_by_key(|usage| usage.port),
        SortKey::Bind => items.sort_by_cached_key(|usage| usage.bind_label()),
        SortKey::Process => items.sort_by(|a, b| a.process_label().cmp(b.process_label())),
        SortKey::Pid => items.sort_by_key(|usage| usage.pid),
        SortKey::Risk => items.sort_by_key(|usage| usage.risk_severity()),
    }

    items
}

#[cfg(test)]
mod tests {
    use std::net::IpAddr;

    use super::*;
    use crate::scanning::network::Protocol;

    fn usage(port: u16, pid: u32, process_name: &str, local_addr: [u8; 4]) -> PortUsage {
        PortUsage {
            port,
            protocol: Protocol::Tcp,
            pid,
            process_name: Some(process_name.to_string()),
            local_addr: IpAddr::from(local_addr),
        }
    }

    #[test]
    fn sort_key_cycles_through_every_variant_and_wraps() {
        let mut key = SortKey::Port;
        let mut seen = vec![key];

        for _ in 0..4 {
            key = key.next();
            seen.push(key);
        }

        assert_eq!(key.next(), SortKey::Port, "the fifth `next()` should wrap back to the start");
        assert_eq!(seen, [SortKey::Port, SortKey::Bind, SortKey::Process, SortKey::Pid, SortKey::Risk]);
    }

    #[test]
    fn empty_filter_keeps_every_item() {
        let items = vec![usage(80, 1, "nginx", [127, 0, 0, 1]), usage(443, 2, "caddy", [0, 0, 0, 0])];

        assert_eq!(filtered_items(&items, "", SortKey::Port).len(), 2);
    }

    #[test]
    fn filter_matches_process_name_case_insensitively() {
        let items = vec![usage(80, 1, "nginx", [127, 0, 0, 1]), usage(443, 2, "caddy", [0, 0, 0, 0])];

        let found = filtered_items(&items, "NGINX", SortKey::Port);

        assert_eq!(found.len(), 1);
        assert_eq!(found[0].pid, 1);
    }

    #[test]
    fn filter_also_matches_port_pid_and_bind_label() {
        let items = vec![usage(8080, 42, "node", [127, 0, 0, 1])];

        assert_eq!(filtered_items(&items, "8080", SortKey::Port).len(), 1, "should match by port");
        assert_eq!(filtered_items(&items, "42", SortKey::Port).len(), 1, "should match by pid");
        assert_eq!(filtered_items(&items, "localhost", SortKey::Port).len(), 1, "should match by bind label");
    }

    #[test]
    fn sorting_by_risk_puts_critical_ports_first() {
        let items = vec![
            usage(8080, 1, "dev-server", [0, 0, 0, 0]),
            usage(6379, 2, "redis-server", [0, 0, 0, 0]),
            usage(3000, 3, "local-only", [127, 0, 0, 1]),
        ];

        let sorted = filtered_items(&items, "", SortKey::Risk);

        assert_eq!(sorted.iter().map(|usage| usage.pid).collect::<Vec<_>>(), [2, 1, 3]);
    }
}
