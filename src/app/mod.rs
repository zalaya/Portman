mod actions;
mod activity_log;
mod help_overlay;
mod kill_workflow;
mod refresh;
mod selection;
mod sort_and_filter;

use std::collections::{ HashSet, VecDeque };

use anyhow::Result;

use crate::domain::port::PortUsage;
use crate::domain::process::UserDirectory;

pub use actions::{ Action, ActionMenu };
pub use activity_log::Event;
pub use kill_workflow::KillTarget;
pub use selection::Details;
pub use sort_and_filter::{ SortKey, filtered_items };

type ListenerKey = (String, u32);

pub struct App {
    pub items: Vec<PortUsage>,
    pub filter: String,
    pub sort: SortKey,
    pub selected: Option<usize>,
    pub kill_target: Option<KillTarget>,
    pub action_menu: Option<ActionMenu>,
    pub details: Option<Details>,
    pub status: Option<String>,
    pub new_keys: HashSet<ListenerKey>,
    pub events: VecDeque<Event>,
    pub show_activity: bool,
    pub show_help: bool,
    seen_keys: Option<HashSet<ListenerKey>>,
    users: UserDirectory,
}

impl App {
    pub fn new() -> Result<Self> {
        let mut app = Self {
            items: Vec::new(),
            filter: String::new(),
            sort: SortKey::Port,
            selected: None,
            kill_target: None,
            action_menu: None,
            details: None,
            status: None,
            new_keys: HashSet::new(),
            events: VecDeque::new(),
            show_activity: false,
            show_help: false,
            seen_keys: None,
            users: UserDirectory::snapshot(),
        };

        app.refresh()?;

        Ok(app)
    }

    pub fn filtered(&self) -> Vec<&PortUsage> {
        filtered_items(&self.items, &self.filter, self.sort)
    }

    fn selected_usage(&self) -> Option<&PortUsage> {
        self.filtered().into_iter().nth(self.selected.unwrap_or(usize::MAX))
    }
}
