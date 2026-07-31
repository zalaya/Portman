use anyhow::Result;

use crate::app::App;
use crate::clipboard;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Action {
    Kill,
    CopyPid,
    CopyAddress,
    Refresh,
}

impl Action {
    pub fn label(self) -> &'static str {
        match self {
            Action::Kill => "Kill process",
            Action::CopyPid => "Copy PID",
            Action::CopyAddress => "Copy address",
            Action::Refresh => "Refresh list",
        }
    }
}

pub struct ActionMenu {
    pub label: String,
    pub actions: Vec<Action>,
    pub selected: usize,
}

impl App {
    pub fn open_action_menu(&mut self) {
        let Some(usage) = self.selected_usage() else {
            return;
        };

        let label = format!("{} ({})", usage.process_label(), usage.address());
        let actions = vec![Action::Kill, Action::CopyPid, Action::CopyAddress, Action::Refresh];

        self.action_menu = Some(ActionMenu { label, actions, selected: 0 });
    }

    pub fn close_action_menu(&mut self) {
        self.action_menu = None;
    }

    pub fn action_menu_next(&mut self) {
        if let Some(menu) = &mut self.action_menu {
            menu.selected = (menu.selected + 1) % menu.actions.len();
        }
    }

    pub fn action_menu_previous(&mut self) {
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
            Action::Kill => {
                self.request_kill();
                return Ok(());
            }
            Action::Refresh => return self.refresh(),
            _ => {}
        }

        let value = match action {
            Action::CopyPid => self.selected_usage().map(|usage| usage.pid.to_string()),
            Action::CopyAddress => self.selected_usage().map(|usage| usage.address()),
            Action::Kill | Action::Refresh => unreachable!(),
        };

        self.copy_to_clipboard(action.label(), value);

        Ok(())
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
