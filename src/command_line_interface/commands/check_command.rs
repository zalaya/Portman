use anyhow::Result;

use crate::command_line_interface::report::evaluate;
use crate::domain::port;

pub fn run(json: bool) -> Result<bool> {
    let items = port::scan()?;
    let report = evaluate(&items);

    if json {
        println!("{}", serde_json::to_string_pretty(&report)?);
    } else if report.ok {
        println!("No critical ports exposed ({} total).", items.len());
    } else {
        println!("{} critical port(s) exposed:", report.critical.len());

        for port in &report.critical {
            println!("  {:<6} {:<5} {} ({})", port.port, port.protocol, port.process, port.pid);
        }
    }

    Ok(report.ok)
}
