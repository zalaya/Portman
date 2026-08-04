use anyhow::Result;

use crate::command_line_interface::report::build_reports;
use crate::domain::port;

pub fn run(json: bool) -> Result<()> {
    let items = port::scan()?;
    let reports = build_reports(&items);

    if json {
        println!("{}", serde_json::to_string_pretty(&reports)?);
    } else if reports.is_empty() {
        println!("No open ports found.");
    } else {
        for report in &reports {
            println!("{:<6} {:<5} {:<20} {:<8} {}", report.port, report.protocol, report.process, report.pid, report.risk);
        }
    }

    Ok(())
}
