pub mod network;
pub mod port;
pub mod process;

pub fn scan_ports() -> anyhow::Result<Vec<port::PortUsage>> {
    let processes = process::ProcessTable::snapshot();

    Ok(network::scan()?.into_iter().map(|listener| port::PortUsage::resolve(&processes, listener)).collect())
}
