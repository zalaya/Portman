use std::io;
use std::net::{ TcpListener, UdpSocket };

use super::network::Protocol;

#[allow(dead_code)]
pub enum OpenSocket {
    Tcp(TcpListener),
    Udp(UdpSocket),
}

pub fn open(port: u16, protocol: Protocol) -> io::Result<OpenSocket> {
    match protocol {
        Protocol::Tcp => TcpListener::bind(("0.0.0.0", port)).map(OpenSocket::Tcp),
        Protocol::Udp => UdpSocket::bind(("0.0.0.0", port)).map(OpenSocket::Udp),
    }
}
