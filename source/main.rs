mod network;
mod process;

use anyhow::Result;
use process::Process;

fn main() -> Result<()> {
    let listeners = network::scan()?;

    for listener in listeners {
        let process = Process::from_pid(listener.pid)?;
        let port = listener.port;
        let pid = process.pid;
        let name = process.name;

        println!("{} -> {} ({})", port, name, pid);
    }

    Ok(())
}
