use super::protocol::Protocol;

#[derive(Debug)]
pub struct Listener {
    pub port: u16,
    pub protocol: Protocol,
    pub pid: u32,
}
