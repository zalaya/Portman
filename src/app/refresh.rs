use std::collections::HashSet;

use anyhow::Result;

use super::{ App, ListenerKey };
use crate::domain::port::{ self, PortUsage };
use crate::domain::process::UserDirectory;

impl App {
    pub fn refresh(&mut self) -> Result<()> {
        let previous_pid = self.selected_usage().map(|usage| usage.pid);
        let previous_items = std::mem::take(&mut self.items);
        let previous_keys = self.seen_keys.clone();

        self.items = port::scan()?;
        self.users = UserDirectory::snapshot();

        let current_keys: HashSet<ListenerKey> = self.items.iter().map(listener_key).collect();

        self.new_keys = match &previous_keys {
            Some(previous) => current_keys.difference(previous).cloned().collect(),
            None => HashSet::new(),
        };

        if let Some(previous_keys) = &previous_keys {
            let messages = listener_change_messages(&self.items, &self.new_keys, &previous_items, previous_keys, &current_keys);

            for message in messages {
                self.log_event(message);
            }
        }

        self.seen_keys = Some(current_keys);
        self.select(previous_pid);

        Ok(())
    }
}

fn listener_key(usage: &PortUsage) -> ListenerKey {
    (usage.address(), usage.pid)
}

fn listener_change_messages(
    current_items: &[PortUsage],
    new_keys: &HashSet<ListenerKey>,
    previous_items: &[PortUsage],
    previous_keys: &HashSet<ListenerKey>,
    current_keys: &HashSet<ListenerKey>,
) -> Vec<String> {
    let mut messages: Vec<String> = current_items
        .iter()
        .filter(|usage| new_keys.contains(&listener_key(usage)))
        .map(|usage| format!("Opened {} — {} ({})", usage.address(), usage.process_label(), usage.pid))
        .collect();

    let closed_keys: HashSet<ListenerKey> = previous_keys.difference(current_keys).cloned().collect();

    messages.extend(
        previous_items
            .iter()
            .filter(|usage| closed_keys.contains(&listener_key(usage)))
            .map(|usage| format!("Closed {} — {} ({})", usage.address(), usage.process_label(), usage.pid)),
    );

    messages
}

#[cfg(test)]
mod tests {
    use std::net::IpAddr;

    use super::*;
    use crate::domain::network::Protocol;

    fn usage(port: u16, pid: u32) -> PortUsage {
        PortUsage { port, protocol: Protocol::Tcp, pid, process_name: Some("test".to_string()), local_addr: IpAddr::from([127, 0, 0, 1]) }
    }

    #[test]
    fn reports_newly_opened_listeners() {
        let current = vec![usage(3000, 1)];
        let new_keys = HashSet::from([listener_key(&current[0])]);

        let messages = listener_change_messages(&current, &new_keys, &[], &HashSet::new(), &HashSet::from([listener_key(&current[0])]));

        assert_eq!(messages, ["Opened 3000/TCP — test (1)"]);
    }

    #[test]
    fn reports_listeners_that_disappeared() {
        let previous = vec![usage(3000, 1)];
        let previous_keys = HashSet::from([listener_key(&previous[0])]);

        let messages = listener_change_messages(&[], &HashSet::new(), &previous, &previous_keys, &HashSet::new());

        assert_eq!(messages, ["Closed 3000/TCP — test (1)"]);
    }

    #[test]
    fn reports_nothing_when_the_set_of_listeners_is_unchanged() {
        let items = vec![usage(3000, 1)];
        let keys = HashSet::from([listener_key(&items[0])]);

        let messages = listener_change_messages(&items, &HashSet::new(), &items, &keys, &keys);

        assert!(messages.is_empty());
    }
}
