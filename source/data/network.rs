mod listener;
mod protocol;

use anyhow::Result;
use netstat2::{ AddressFamilyFlags, ProtocolFlags, ProtocolSocketInfo, SocketInfo, TcpState, get_sockets_info };

pub use listener::Listener;
pub use protocol::Protocol;

pub fn scan() -> Result<Vec<Listener>> {
    let address_family_flags = AddressFamilyFlags::IPV4 | AddressFamilyFlags::IPV6;
    let protocol_flags = ProtocolFlags::TCP | ProtocolFlags::UDP;
    let sockets = get_sockets_info(address_family_flags, protocol_flags)?;

    Ok(sockets.into_iter().filter_map(as_listener).collect())
}

fn as_listener(socket: SocketInfo) -> Option<Listener> {
    let pid = *socket.associated_pids.first()?;

    let (port, protocol) = match socket.protocol_socket_info {
        ProtocolSocketInfo::Tcp(tcp) if tcp.state == TcpState::Listen => (tcp.local_port, Protocol::Tcp),
        ProtocolSocketInfo::Udp(udp) => (udp.local_port, Protocol::Udp),
        _ => return None,
    };

    Some(Listener { port, protocol, pid })
}
