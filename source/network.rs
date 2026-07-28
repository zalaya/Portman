use anyhow::Result;
use netstat2::{ AddressFamilyFlags, ProtocolFlags, ProtocolSocketInfo, SocketInfo, TcpState, get_sockets_info };

#[derive(Debug)]
pub struct Listener {
    pub port: u16,
    pub pid: u32,
}

impl Listener {
    pub const fn new(port: u16, pid: u32) -> Self {
        Self {
            port,
            pid
        }
    }
}

pub fn scan() -> Result<Vec<Listener>> {
    let address_family_flags = AddressFamilyFlags::IPV4 | AddressFamilyFlags::IPV6;
    let protocol_flags = ProtocolFlags::TCP;
    let sockets = get_sockets_info(address_family_flags, protocol_flags)?;

    Ok(sockets.into_iter().filter_map(as_listener).collect())
}

fn as_listener(socket: SocketInfo) -> Option<Listener> {
    let ProtocolSocketInfo::Tcp(tcp) = socket.protocol_socket_info else {
        return None;
    };

    if tcp.state != TcpState::Listen {
        return None;
    }

    let pid = *socket.associated_pids.first()?;

    Some(Listener::new(tcp.local_port, pid))
}
