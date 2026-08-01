use sysinfo::{ Pid, ProcessesToUpdate, Signal, System };

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
