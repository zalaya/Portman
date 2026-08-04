use crate::app::App;
use crate::platform::clipboard;

impl App {
    pub(super) fn copy_to_clipboard(&mut self, label: &str, value: Option<String>) {
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
