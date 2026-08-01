use sysinfo::{ Pid, ProcessesToUpdate, System, Uid, Users };

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

    pub(super) fn name_for(&self, uid: &Uid) -> Option<String> {
        self.0.get_user_by_id(uid).map(|user| user.name().to_string())
    }
}
