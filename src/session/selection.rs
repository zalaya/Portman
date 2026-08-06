use crate::session::Session;
use crate::scanning::port::Risk;
use crate::scanning::process;

pub struct Details {
    pub address: String,
    pub bind: String,
    pub exposed: bool,
    pub risk: Risk,
    pub process: process::ProcessDetails,
}

impl Session {
    pub(super) fn select(&mut self, pid: Option<u32>) {
        let filtered = self.filtered();

        if filtered.is_empty() {
            self.selected = None;
        } else {
            let index = pid.and_then(|pid| filtered.iter().position(|usage| usage.pid == pid)).unwrap_or(0);

            self.selected = Some(index);
        }

        self.refresh_details();
    }

    pub fn next(&mut self) {
        let len = self.filtered().len();

        if len == 0 {
            return;
        }

        let next = match self.selected {
            Some(i) => (i + 1) % len,
            None => 0,
        };

        self.selected = Some(next);
        self.refresh_details();
    }

    pub fn previous(&mut self) {
        let len = self.filtered().len();

        if len == 0 {
            return;
        }

        let previous = match self.selected {
            Some(0) | None => len - 1,
            Some(i) => i - 1,
        };

        self.selected = Some(previous);
        self.refresh_details();
    }

    fn refresh_details(&mut self) {
        let selected = self
            .selected_usage()
            .map(|usage| (usage.pid, usage.address(), usage.bind_description(), usage.is_exposed(), usage.risk()));

        self.details = selected.and_then(|(pid, address, bind, exposed, risk)| {
            process::details(pid, &self.users).map(|process| Details { address, bind, exposed, risk, process })
        });
    }
}

#[cfg(test)]
mod tests {
    use std::net::IpAddr;

    use super::*;
    use crate::scanning::network::Protocol;

    fn usage(pid: u32) -> crate::scanning::port::PortUsage {
        crate::scanning::port::PortUsage {
            port: 3000,
            protocol: Protocol::Tcp,
            pid,
            process_name: Some("test".to_string()),
            local_addr: IpAddr::from([127, 0, 0, 1]),
        }
    }

    #[test]
    fn next_wraps_from_the_last_row_back_to_the_first() -> anyhow::Result<()> {
        let mut session = Session::new()?;
        session.items = vec![usage(1), usage(2), usage(3)];
        session.selected = Some(2);

        session.next();

        assert_eq!(session.selected, Some(0), "next() past the last row should wrap to the first");

        Ok(())
    }

    #[test]
    fn previous_wraps_from_the_first_row_back_to_the_last() -> anyhow::Result<()> {
        let mut session = Session::new()?;
        session.items = vec![usage(1), usage(2), usage(3)];
        session.selected = Some(0);

        session.previous();

        assert_eq!(session.selected, Some(2), "previous() before the first row should wrap to the last");

        Ok(())
    }

    #[test]
    fn next_and_previous_on_an_empty_list_do_nothing() -> anyhow::Result<()> {
        let mut session = Session::new()?;
        session.items = Vec::new();
        session.selected = None;

        session.next();
        assert_eq!(session.selected, None);

        session.previous();
        assert_eq!(session.selected, None);

        Ok(())
    }
}
