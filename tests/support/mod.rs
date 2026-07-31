#![allow(dead_code)]

use std::net::IpAddr;

use portman::data::network::Protocol;
use portman::data::port::PortUsage;

pub fn own_pid() -> u32 {
    std::process::id()
}

pub fn port_usage(port: u16, protocol: Protocol, pid: u32, process_name: Option<&str>, local_addr: IpAddr) -> PortUsage {
    PortUsage { port, protocol, pid, process_name: process_name.map(str::to_string), local_addr }
}

pub fn loopback_tcp(port: u16, pid: u32, process_name: &str) -> PortUsage {
    port_usage(port, Protocol::Tcp, pid, Some(process_name), IpAddr::from([127, 0, 0, 1]))
}

pub fn public_tcp(port: u16, pid: u32, process_name: &str) -> PortUsage {
    port_usage(port, Protocol::Tcp, pid, Some(process_name), IpAddr::from([0, 0, 0, 0]))
}
