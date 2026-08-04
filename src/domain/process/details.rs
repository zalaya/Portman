use sysinfo::{ Pid, ProcessesToUpdate, System };

use super::user_directory::UserDirectory;

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
        parent_pid: process.parent().map(|parent| parent.as_u32()),
        user,
        exe: process.exe().map(|path| path.display().to_string()),
        cwd: process.cwd().map(|path| path.display().to_string()),
        cmd: process.cmd().iter().map(|arg| arg.to_string_lossy().into_owned()).collect(),
    })
}
