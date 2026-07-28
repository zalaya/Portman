mod network;
mod process;

use anyhow::Result;
use process::Process;

fn main() -> Result<()> {
    let listeners = network::scan()?;

    for listener in listeners {
        let process = Process::from_pid(listener.pid)?;

        println!("{} -> {} ({})", listener.port, process.name, process.pid);
    }

    Ok(())
}
