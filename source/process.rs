use anyhow::{ Context, Result };
use sysinfo::{ Pid, ProcessesToUpdate, System };

#[derive(Debug)]
pub struct Process {
    pub pid: u32,
    pub name: String,
}

impl Process {
    pub fn new(pid: u32, name: impl Into<String>) -> Self {
        Self {
            pid,
            name: name.into(),
        }
    }

    pub fn from_pid(pid: u32) -> Result<Self> {
        let mut system = System::new();

        system.refresh_processes(ProcessesToUpdate::All, true);

        let process = system
            .process(Pid::from_u32(pid))
            .with_context(|| format!("Process {} not found", pid))?;

        Ok(Self::new(pid, process.name().to_string_lossy()))
    }
}
