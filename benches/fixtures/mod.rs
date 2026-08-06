#![allow(dead_code)]

use std::net::IpAddr;
use std::time::Duration;

use criterion::{Criterion, PlottingBackend};
use portman::scanning::network::Protocol;
use portman::scanning::port::PortUsage;

pub fn config() -> Criterion {
    Criterion::default()
        .plotting_backend(PlottingBackend::None)
        .warm_up_time(Duration::from_millis(300))
        .measurement_time(Duration::from_millis(700))
        .sample_size(20)
        .nresamples(2_000)
}

pub fn fixture_items(count: usize) -> Vec<PortUsage> {
    (0..count)
        .map(|i| PortUsage {
            port: (1024 + (i % 60_000)) as u16,
            protocol: if i % 2 == 0 {
                Protocol::Tcp
            } else {
                Protocol::Udp
            },
            pid: i as u32,
            process_name: Some(format!("process-{i}")),
            local_addr: if i % 3 == 0 {
                IpAddr::from([0, 0, 0, 0])
            } else {
                IpAddr::from([127, 0, 0, 1])
            },
        })
        .collect()
}
