mod risk;
mod usage;

use anyhow::Result;

pub use risk::Risk;
pub use usage::PortUsage;

use crate::domain::{ network, process };

pub fn scan() -> Result<Vec<PortUsage>> {
    let processes = process::ProcessTable::snapshot();

    Ok(network::scan()?.into_iter().map(|listener| PortUsage::resolve(&processes, listener)).collect())
}
