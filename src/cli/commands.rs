use anyhow::Result;

use crate::cli::report::{ build_reports, evaluate };
use crate::data;

pub fn run_scan(json: bool) -> Result<()> {
    let items = data::scan_ports()?;
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

pub fn run_check(json: bool) -> Result<bool> {
    let items = data::scan_ports()?;
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
