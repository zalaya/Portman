use sysinfo::{ Pid, ProcessesToUpdate, System };

pub fn kill(pid: u32) -> bool {
    let mut system = System::new();
    let pid = Pid::from_u32(pid);

    system.refresh_processes(ProcessesToUpdate::Some(&[pid]), true);
    system.process(pid).map(|process| process.kill()).unwrap_or(false)
}
