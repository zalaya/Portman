mod actions;
mod activity;
mod help;
mod kill;
mod listing;
mod selection;

use std::collections::{ HashSet, VecDeque };

use anyhow::Result;
use ratatui::widgets::TableState;

use crate::data;
use crate::data::port::PortUsage;

pub use actions::{ Action, ActionMenu };
pub use activity::Event;
pub use kill::KillTarget;
pub use listing::{ SortKey, filtered_items };
pub use selection::Details;

type ListenerKey = (String, u32);

pub struct App {
    pub items: Vec<PortUsage>,
    pub filter: String,
    pub sort: SortKey,
    pub state: TableState,
    pub kill_target: Option<KillTarget>,
    pub action_menu: Option<ActionMenu>,
    pub details: Option<Details>,
    pub status: Option<String>,
    pub new_keys: HashSet<ListenerKey>,
    pub events: VecDeque<Event>,
    pub show_activity: bool,
    pub show_help: bool,
    seen_keys: Option<HashSet<ListenerKey>>,
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
            details: None,
            status: None,
            new_keys: HashSet::new(),
            events: VecDeque::new(),
            show_activity: false,
            show_help: false,
            seen_keys: None,
        };

        app.refresh()?;

        Ok(app)
    }

    pub fn refresh(&mut self) -> Result<()> {
        let previous_pid = self.selected_usage().map(|usage| usage.pid);
        let previous_items = std::mem::take(&mut self.items);
        let previous_keys = self.seen_keys.clone();

        self.items = data::scan_ports()?;

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

    pub fn filtered(&self) -> Vec<&PortUsage> {
        filtered_items(&self.items, &self.filter, self.sort)
    }

    fn selected_usage(&self) -> Option<&PortUsage> {
        self.filtered().into_iter().nth(self.state.selected().unwrap_or(usize::MAX))
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
    use crate::data::network::Protocol;

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
