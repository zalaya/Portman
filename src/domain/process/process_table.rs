use sysinfo::{ Pid, ProcessesToUpdate, System };

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
