use anyhow::Result;

use super::action::Action;
use crate::app::App;
use crate::domain::network::Protocol;
use crate::domain::process::KillSignal;

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
        let mut actions = vec![Action::Terminate, Action::Kill];

        if usage.protocol == Protocol::Tcp {
            actions.push(Action::OpenInBrowser);
        }

        actions.push(Action::CopyPid);
        actions.push(Action::CopyAddress);

        if self.details.as_ref().is_some_and(|details| !details.process.cmd.is_empty()) {
            actions.push(Action::CopyCommand);
        }

        actions.push(Action::Refresh);

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
            Action::Terminate => {
                self.request_kill(KillSignal::Terminate);
                return Ok(());
            }
            Action::Kill => {
                self.request_kill(KillSignal::Force);
                return Ok(());
            }
            Action::OpenInBrowser => {
                self.open_selected_in_browser();
                return Ok(());
            }
            Action::Refresh => return self.refresh(),
            _ => {}
        }

        let value = match action {
            Action::CopyPid => self.selected_usage().map(|usage| usage.pid.to_string()),
            Action::CopyAddress => self.selected_usage().map(|usage| usage.address()),
            Action::CopyCommand => self.details.as_ref().map(|details| details.process.cmd.join(" ")),
            Action::Terminate | Action::Kill | Action::OpenInBrowser | Action::Refresh => unreachable!(),
        };

        self.copy_to_clipboard(action.label(), value);

        Ok(())
    }
}
