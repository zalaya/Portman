use std::fmt;
use std::net::IpAddr;

use anyhow::Result;
use netstat2::{
    AddressFamilyFlags, ProtocolFlags, ProtocolSocketInfo, SocketInfo, TcpState, get_sockets_info,
};

#[derive(Debug)]
pub struct Listener {
    pub port: u16,
    pub protocol: Protocol,
    pub pid: u32,
    pub local_addr: IpAddr,
}

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

pub fn scan() -> Result<Vec<Listener>> {
    let address_family_flags = AddressFamilyFlags::IPV4 | AddressFamilyFlags::IPV6;
    let protocol_flags = ProtocolFlags::TCP | ProtocolFlags::UDP;
    let sockets = get_sockets_info(address_family_flags, protocol_flags)?;

    Ok(sockets.into_iter().filter_map(as_listener).collect())
}

fn as_listener(socket: SocketInfo) -> Option<Listener> {
    let pid = *socket.associated_pids.first()?;

    let (port, protocol, local_addr) = match socket.protocol_socket_info {
        ProtocolSocketInfo::Tcp(tcp) if tcp.state == TcpState::Listen => {
            (tcp.local_port, Protocol::Tcp, tcp.local_addr)
        }
        ProtocolSocketInfo::Udp(udp) => (udp.local_port, Protocol::Udp, udp.local_addr),
        _ => return None,
    };

    Some(Listener {
        port,
        protocol,
        pid,
        local_addr,
    })
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
