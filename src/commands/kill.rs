//! Kill command - terminate a specific gateway process

use crate::detector::{detect_gateways, kill_process};
use crate::output::{print_error, print_info, print_success, print_warning};

/// Execute the kill command for a specific PID
pub fn kill_command(pid: u32, force: bool) -> i32 {
    // First verify the process exists and is a gateway
    let gateways = detect_gateways();
    let target = gateways.iter().find(|g| g.pid == pid);

    match target {
        Some(gw) => {
            print_info(&format!("Terminating {} (PID: {})", gw.gateway.name, pid));

            match kill_process(pid, force) {
                Ok(killed) => {
                    if killed {
                        print_success(&format!("Successfully terminated {} (PID: {})", gw.gateway.name, pid));
                        0
                    } else {
                        print_error(&format!("Failed to terminate PID {} - process may require force kill", pid));
                        if !force {
                            print_info("Use --force or -f to send SIGKILL");
                        }
                        1
                    }
                }
                Err(e) => {
                    print_error(&format!("Error terminating PID {}: {}", pid, e));
                    1
                }
            }
        }
        None => {
            print_warning(&format!("PID {} is not a known Agent Gateway", pid));
            print_info("Use `ps` to check if this PID exists");
            1
        }
    }
}
