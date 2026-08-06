use std::net::IpAddr;

use anyhow::Result;

use crate::platform::{browser, clipboard};
use crate::scanning::network::Protocol;
use crate::scanning::port::PortUsage;
use crate::scanning::process::KillSignal;
use crate::session::Session;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Action {
    Terminate,
    Kill,
    OpenInBrowser,
    CopyPid,
    CopyAddress,
    CopyCommand,
    Refresh,
}

impl Action {
    pub fn label(self) -> &'static str {
        match self {
            Action::Terminate => "Terminate (SIGTERM)",
            Action::Kill => "Force kill (SIGKILL)",
            Action::OpenInBrowser => "Open in browser",
            Action::CopyPid => "Copy PID",
            Action::CopyAddress => "Copy address",
            Action::CopyCommand => "Copy full command",
            Action::Refresh => "Refresh list",
        }
    }
}

pub struct ActionMenu {
    pub label: String,
    pub actions: Vec<Action>,
    pub selected: usize,
}

impl Session {
    pub fn open_action_menu(&mut self) {
        let Some(usage) = self.selected_usage() else {
            return;
        };

        let label = format!("{} ({})", usage.process_label(), usage.address());
        let mut actions = vec![Action::Terminate, Action::Kill];

        if usage.protocol == Protocol::Tcp {
            actions.push(Action::OpenInBrowser);
        }

        actions.push(Action::CopyPid);
        actions.push(Action::CopyAddress);

        if self
            .details
            .as_ref()
            .is_some_and(|details| !details.process.cmd.is_empty())
        {
            actions.push(Action::CopyCommand);
        }

        actions.push(Action::Refresh);

        self.action_menu = Some(ActionMenu {
            label,
            actions,
            selected: 0,
        });
    }

    pub fn close_action_menu(&mut self) {
        self.action_menu = None;
    }

    pub fn select_next_action(&mut self) {
        if let Some(menu) = &mut self.action_menu {
            menu.selected = (menu.selected + 1) % menu.actions.len();
        }
    }

    pub fn select_previous_action(&mut self) {
        if let Some(menu) = &mut self.action_menu {
            menu.selected = (menu.selected + menu.actions.len() - 1) % menu.actions.len();
        }
    }

    pub fn confirm_action_menu(&mut self) -> Result<()> {
        let Some(menu) = self.action_menu.take() else {
            return Ok(());
        };

        let Some(action) = menu.actions.get(menu.selected).copied() else {
            return Ok(());
        };

        match action {
            Action::Terminate => self.request_kill(KillSignal::Terminate),
            Action::Kill => self.request_kill(KillSignal::Force),
            Action::OpenInBrowser => self.open_selected_in_browser(),
            Action::Refresh => return self.refresh(),
            Action::CopyPid => {
                let value = self.selected_usage().map(|usage| usage.pid.to_string());
                self.copy_to_clipboard(action.label(), value);
            }
            Action::CopyAddress => {
                let value = self.selected_usage().map(|usage| usage.address());
                self.copy_to_clipboard(action.label(), value);
            }
            Action::CopyCommand => {
                let value = self
                    .details
                    .as_ref()
                    .map(|details| details.process.cmd.join(" "));
                self.copy_to_clipboard(action.label(), value);
            }
        }

        Ok(())
    }

    fn open_selected_in_browser(&mut self) {
        let Some(usage) = self.selected_usage() else {
            return;
        };

        let url = browser_url(usage);

        match browser::open(&url) {
            Ok(()) => self.status = Some(format!("Opened {url}")),
            Err(error) => self.status = Some(format!("Could not open {url}: {error}")),
        }
    }

    fn copy_to_clipboard(&mut self, label: &str, value: Option<String>) {
        let Some(value) = value else {
            self.status = Some(format!("Nothing to copy for \"{label}\""));
            return;
        };

        match clipboard::copy(value.clone()) {
            Ok(()) => {
                self.status = Some(format!("Copied: {value}"));
                self.log_event(format!("Copied {label} — {value}"));
            }
            Err(error) => self.status = Some(format!("Could not copy to clipboard: {error}")),
        }
    }
}

fn browser_url(usage: &PortUsage) -> String {
    format!("http://{}:{}", host(&usage.local_addr), usage.port)
}

fn host(local_addr: &IpAddr) -> String {
    if local_addr.is_loopback() || local_addr.is_unspecified() {
        "localhost".to_string()
    } else {
        match local_addr {
            IpAddr::V4(v4) => v4.to_string(),
            IpAddr::V6(v6) => format!("[{v6}]"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scanning::network::Protocol;

    fn usage(port: u16, local_addr: [u8; 4]) -> PortUsage {
        PortUsage {
            port,
            protocol: Protocol::Tcp,
            pid: 1,
            process_name: None,
            local_addr: IpAddr::from(local_addr),
        }
    }

    #[test]
    fn loopback_uses_localhost() {
        assert_eq!(
            browser_url(&usage(3000, [127, 0, 0, 1])),
            "http://localhost:3000"
        );
    }

    #[test]
    fn unspecified_uses_localhost_too_since_it_includes_loopback() {
        assert_eq!(
            browser_url(&usage(8080, [0, 0, 0, 0])),
            "http://localhost:8080"
        );
    }

    #[test]
    fn a_specific_interface_uses_its_own_address() {
        assert_eq!(
            browser_url(&usage(9000, [192, 168, 1, 5])),
            "http://192.168.1.5:9000"
        );
    }
}
