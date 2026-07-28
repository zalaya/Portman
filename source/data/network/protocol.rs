use std::fmt;

#[derive(Debug)]
pub enum Protocol {
    Tcp,
    Udp,
}

impl fmt::Display for Protocol {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Protocol::Tcp => write!(formatter, "TCP"),
            Protocol::Udp => write!(formatter, "UDP"),
        }
    }
}
