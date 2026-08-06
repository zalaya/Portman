use sysinfo::{Pid, ProcessesToUpdate, Signal, System, Uid, Users};

pub struct ProcessTable(System);

impl ProcessTable {
    pub fn snapshot() -> Self {
        let mut system = System::new();

        system.refresh_processes(ProcessesToUpdate::All, true);

        Self(system)
    }

    pub fn resolve(&self, pid: u32) -> Option<String> {
        let process = self.0.process(Pid::from_u32(pid))?;

        Some(process.name().to_string_lossy().into_owned())
    }
}

pub struct UserDirectory(Users);

impl UserDirectory {
    pub fn snapshot() -> Self {
        Self(Users::new_with_refreshed_list())
    }

    fn name_for(&self, uid: &Uid) -> Option<String> {
        self.0
            .get_user_by_id(uid)
            .map(|user| user.name().to_string())
    }
}

pub struct ProcessDetails {
    pub pid: u32,
    pub name: String,
    pub status: String,
    pub run_time_secs: u64,
    pub memory_bytes: u64,
    pub parent_pid: Option<u32>,
    pub user: Option<String>,
    pub exe: Option<String>,
    pub cwd: Option<String>,
    pub cmd: Vec<String>,
}

pub fn details(pid: u32, users: &UserDirectory) -> Option<ProcessDetails> {
    let mut system = System::new();
    let sysinfo_pid = Pid::from_u32(pid);

    system.refresh_processes(ProcessesToUpdate::Some(&[sysinfo_pid]), true);

    let process = system.process(sysinfo_pid)?;
    let user = process.user_id().and_then(|uid| users.name_for(uid));

    Some(ProcessDetails {
        pid,
        name: process.name().to_string_lossy().into_owned(),
        status: process.status().to_string(),
        run_time_secs: process.run_time(),
        memory_bytes: process.memory(),
        parent_pid: process.parent().map(Pid::as_u32),
        user,
        exe: process.exe().map(|path| path.display().to_string()),
        cwd: process.cwd().map(|path| path.display().to_string()),
        cmd: process
            .cmd()
            .iter()
            .map(|arg| arg.to_string_lossy().into_owned())
            .collect(),
    })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KillSignal {
    Terminate,
    Force,
}

impl KillSignal {
    pub fn verb(self) -> &'static str {
        match self {
            KillSignal::Terminate => "Terminated",
            KillSignal::Force => "Killed",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KillOutcome {
    Succeeded,
    Failed,
    ProcessNotFound,
    SignalUnsupported,
}

pub fn kill(pid: u32, signal: KillSignal) -> KillOutcome {
    let mut system = System::new();
    let sysinfo_pid = Pid::from_u32(pid);

    system.refresh_processes(ProcessesToUpdate::Some(&[sysinfo_pid]), true);

    let Some(process) = system.process(sysinfo_pid) else {
        return KillOutcome::ProcessNotFound;
    };

    match signal {
        KillSignal::Force if process.kill() => KillOutcome::Succeeded,
        KillSignal::Force => KillOutcome::Failed,
        KillSignal::Terminate => match process.kill_with(Signal::Term) {
            Some(true) => KillOutcome::Succeeded,
            Some(false) => KillOutcome::Failed,
            None => KillOutcome::SignalUnsupported,
        },
    }
}
