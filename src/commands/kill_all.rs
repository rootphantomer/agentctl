//! Kill-all command - terminate all detected gateway processes

use crate::detector::{detect_gateways, kill_process};
use crate::output::{print_confirm, print_error, print_info, print_success, print_warning};

/// Execute the kill-all command
pub fn kill_all_command(force: bool, yes: bool) -> i32 {
    let gateways = detect_gateways();

    if gateways.is_empty() {
        print_info("No Agent Gateways to kill");
        return 0;
    }

    // Show what we're about to kill
    print_info(&format!(
        "Found {} Agent Gateway(s) to terminate:",
        gateways.len()
    ));
    println!();

    for gw in &gateways {
        println!(
            "  - {} (PID: {}, Type: {})",
            gw.name,
            gw.pid,
            gw.gateway.name
        );
    }
    println!();

    // Confirm unless --yes is provided
    if !yes && !print_confirm("Are you sure you want to terminate all these processes?") {
        print_info("Aborted");
        return 0;
    }

    // Kill each gateway
    let mut success_count = 0;
    let mut fail_count = 0;

    for gw in &gateways {
        match kill_process(gw.pid, force) {
            Ok(killed) => {
                if killed {
                    success_count += 1;
                    print_success(&format!(
                        "Terminated {} (PID: {})",
                        gw.gateway.name, gw.pid
                    ));
                } else {
                    fail_count += 1;
                    print_warning(&format!(
                        "Process {} may still be running",
                        gw.pid
                    ));
                }
            }
            Err(e) => {
                fail_count += 1;
                print_error(&format!(
                    "Failed to terminate PID {}: {}",
                    gw.pid, e
                ));
            }
        }
    }

    // Summary
    println!();
    if fail_count == 0 {
        print_success(&format!(
            "Successfully terminated {} gateway(s)",
            success_count
        ));
        0
    } else {
        print_warning(&format!(
            "Terminated {} gateway(s), {} failed",
            success_count, fail_count
        ));
        if !force {
            print_info("Some processes may require --force to terminate");
        }
        1
    }
}
