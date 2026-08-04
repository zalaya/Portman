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
