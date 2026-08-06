#![allow(dead_code)]

use std::net::IpAddr;

use portman::session::Details;
use portman::scanning::network::Protocol;
use portman::scanning::port::{ PortUsage, Risk };
use portman::scanning::process::ProcessDetails;

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

pub fn details_with_command(pid: u32, cmd: Vec<&str>) -> Details {
    Details {
        address: "8080/TCP".to_string(),
        bind: "Localhost only (127.0.0.1) — Not reachable from the network".to_string(),
        exposed: false,
        risk: Risk::Safe,
        process: ProcessDetails {
            pid,
            name: "test".to_string(),
            status: "Running".to_string(),
            run_time_secs: 0,
            memory_bytes: 0,
            parent_pid: None,
            user: None,
            exe: None,
            cwd: None,
            cmd: cmd.into_iter().map(str::to_string).collect(),
        },
    }
}
