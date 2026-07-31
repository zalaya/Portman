use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn displays_as_uppercase_abbreviation() {
        assert_eq!(Protocol::Tcp.to_string(), "TCP");
        assert_eq!(Protocol::Udp.to_string(), "UDP");
    }
}
