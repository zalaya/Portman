use anyhow::Result;

#[derive(Debug)]
pub struct Listener {
    pub port: u16,
    pub pid: u32,
}

impl Listener {
    pub const fn new(port: u16, pid: u32) -> Self {
        Self { port, pid }
    }
}

pub fn scan() -> Result<Vec<Listener>> {
    Ok(vec![
        Listener::new(8080, 1234),
        Listener::new(3000, 5678),
    ])
}
