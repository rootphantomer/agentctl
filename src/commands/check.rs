//! Check command - quick check if any gateway is running (exit code based)

use crate::detector::{count_gateways, has_gateways_running};
use crate::output::{print_info, OutputFormat};

/// Execute the check command
/// Returns exit code: 0 = no gateways running, 1 = gateways running
pub fn check_command(format: OutputFormat) -> i32 {
    let running = has_gateways_running();
    let count = count_gateways();

    match format {
        OutputFormat::Compact | OutputFormat::Table => {
            if running {
                print_info(&format!("Found {} Agent Gateway(s) running", count));
                println!("\n  Run `agentctl list` to see details");
                println!("  Run `agentctl kill-all` to terminate all");
            } else {
                println!("No Agent Gateways detected - all clear!");
            }
        }
        OutputFormat::Json => {
            let msg = if running {
                format!("Found {} gateway(s)", count)
            } else {
                "No gateways detected".to_string()
            };
            println!(r#"{{"running": {}, "count": {}, "message": "{}"}}"#, running, count, msg);
        }
    }

    // Exit code: 0 = no gateways, 1 = gateways running
    if running { 1 } else { 0 }
}
