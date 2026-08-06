use std::time::Instant;

use crate::session::Session;

const MAX_EVENTS: usize = 100;

pub struct Event {
    pub at: Instant,
    pub message: String,
}

impl Session {
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

    pub fn toggle_help(&mut self) {
        self.show_help = !self.show_help;
    }

    pub fn close_help(&mut self) {
        self.show_help = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn log_event_keeps_only_the_most_recent_max_events() -> anyhow::Result<()> {
        let mut session = Session::new()?;
        session.events.clear();

        for i in 0..(MAX_EVENTS + 50) {
            session.log_event(format!("event {i}"));
        }

        assert_eq!(session.events.len(), MAX_EVENTS, "the log should never grow past its cap");
        assert_eq!(session.events.front().unwrap().message, format!("event {}", MAX_EVENTS + 49), "newest event goes first");
        assert_eq!(session.events.back().unwrap().message, "event 50", "oldest events past the cap should be dropped");

        Ok(())
    }

    #[test]
    fn toggling_activity_and_help_flips_their_own_flag_only() -> anyhow::Result<()> {
        let mut session = Session::new()?;

        session.toggle_activity();
        assert!(session.show_activity);
        assert!(!session.show_help, "toggling activity must not affect the help overlay");

        session.toggle_help();
        assert!(session.show_help);

        session.close_activity();
        session.close_help();
        assert!(!session.show_activity);
        assert!(!session.show_help);

        Ok(())
    }
}
