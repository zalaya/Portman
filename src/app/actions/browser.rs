use std::net::IpAddr;

use crate::app::App;
use crate::browser;
use crate::data::port::PortUsage;

impl App {
    pub(super) fn open_selected_in_browser(&mut self) {
        let Some(usage) = self.selected_usage() else {
            return;
        };

        let url = browser_url(usage);

        match browser::open(&url) {
            Ok(()) => self.status = Some(format!("Opened {url}")),
            Err(error) => self.status = Some(format!("Could not open {url}: {error}")),
        }
    }
}

fn browser_url(usage: &PortUsage) -> String {
    format!("http://{}:{}", host(&usage.local_addr), usage.port)
}

fn host(local_addr: &IpAddr) -> String {
    if local_addr.is_loopback() || local_addr.is_unspecified() {
        "localhost".to_string()
    } else {
        match local_addr {
            IpAddr::V4(v4) => v4.to_string(),
            IpAddr::V6(v6) => format!("[{v6}]"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::network::Protocol;

    fn usage(port: u16, local_addr: [u8; 4]) -> PortUsage {
        PortUsage { port, protocol: Protocol::Tcp, pid: 1, process_name: None, local_addr: IpAddr::from(local_addr) }
    }

    #[test]
    fn loopback_uses_localhost() {
        assert_eq!(browser_url(&usage(3000, [127, 0, 0, 1])), "http://localhost:3000");
    }

    #[test]
    fn unspecified_uses_localhost_too_since_it_includes_loopback() {
        assert_eq!(browser_url(&usage(8080, [0, 0, 0, 0])), "http://localhost:8080");
    }

    #[test]
    fn a_specific_interface_uses_its_own_address() {
        assert_eq!(browser_url(&usage(9000, [192, 168, 1, 5])), "http://192.168.1.5:9000");
    }
}
