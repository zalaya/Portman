use anyhow::Result;

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
        Ok(Self::new(pid, format!("process-{pid}")))
    }
}
