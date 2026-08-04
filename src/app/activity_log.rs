use std::time::Instant;

use crate::app::App;

const MAX_EVENTS: usize = 100;

pub struct Event {
    pub at: Instant,
    pub message: String,
}

impl App {
    pub(super) fn log_event(&mut self, message: String) {
        self.events.push_front(Event { at: Instant::now(), message });
        self.events.truncate(MAX_EVENTS);
    }

    pub fn toggle_activity(&mut self) {
        self.show_activity = !self.show_activity;
    }

    pub fn close_activity(&mut self) {
        self.show_activity = false;
    }
}
