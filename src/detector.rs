//! Process detection logic

use crate::gateway::{gateways, matches_keyword, Gateway};
use serde::{Deserialize, Serialize};
use sysinfo::System;

/// Represents a detected running Agent Gateway process
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetectedGateway {
    /// Process ID
    pub pid: u32,
    /// Process name
    pub name: String,
    /// The matched gateway type
    pub gateway: Gateway,
    /// Full command line arguments
    pub cmd: Vec<String>,
    /// Memory usage in bytes (if available)
    pub memory: Option<u64>,
    /// CPU usage percentage (if available)
    pub cpu: Option<f32>,
    /// Start time in seconds since epoch (if available)
    pub start_time: Option<u64>,
}

impl DetectedGateway {
    /// Get the full command line as a single string
    #[allow(dead_code)]
    pub fn cmd_line(&self) -> String {
        self.cmd.join(" ")
    }
}

/// Detect all running Agent Gateway processes
pub fn detect_gateways() -> Vec<DetectedGateway> {
    let mut sys = System::new_all();
    sys.refresh_all();

    let gateway_list = gateways();
    let mut detected: Vec<DetectedGateway> = Vec::new();
    let mut seen_pids: std::collections::HashSet<u32> = std::collections::HashSet::new();

    for (pid, process) in sys.processes() {
        let pid_u32 = pid.as_u32();

        // Skip if we've already seen this PID
        if seen_pids.contains(&pid_u32) {
            continue;
        }

        let process_name = process.name().to_string_lossy();
        let cmd: Vec<String> = process.cmd().iter().map(|s| s.to_string_lossy().to_string()).collect();
        let cmd_str = cmd.join(" ");

        // Check each gateway type
        for gateway in &gateway_list {
            // Check process name or command line arguments
            if matches_keyword(&process_name, &gateway.keywords)
                || (!cmd_str.is_empty() && matches_keyword(&cmd_str, &gateway.keywords))
            {
                seen_pids.insert(pid_u32);
                detected.push(DetectedGateway {
                    pid: pid_u32,
                    name: process_name.to_string(),
                    gateway: gateway.clone(),
                    cmd: cmd.clone(),
                    memory: Some(process.memory()),
                    cpu: Some(process.cpu_usage()),
                    start_time: Some(process.start_time()),
                });
                break;
            }
        }
    }

    // Sort by gateway name then PID
    detected.sort_by(|a, b| {
        a.gateway.name
            .cmp(&b.gateway.name)
            .then(a.pid.cmp(&b.pid))
    });

    detected
}

/// Kill a process by PID
#[cfg(unix)]
pub fn kill_process(pid: u32, force: bool) -> anyhow::Result<bool> {
    use std::time::{Duration, Instant};
    use sysinfo::Pid;

    let sys = System::new_all();

    // Try to find and kill the process
    if let Some(process) = sys.process(Pid::from_u32(pid)) {
        // Send SIGTERM (default)
        let killed = process.kill();
        if !killed {
            return Err(anyhow::anyhow!("Failed to send signal to process {}", pid));
        }
    } else {
        return Err(anyhow::anyhow!("Process {} not found", pid));
    }

    // Wait for process to terminate (up to 3 seconds)
    if !force {
        let start = Instant::now();
        let sys = System::new_all();

        while start.elapsed() < Duration::from_secs(3) {
            if sys.process(Pid::from_u32(pid)).is_none() {
                return Ok(true);
            }
            std::thread::sleep(Duration::from_millis(100));
        }
    }

    // If still alive and force is requested, try kill again (SIGKILL)
    if force {
        let sys = System::new_all();
        if let Some(process) = sys.process(Pid::from_u32(pid)) {
            // sysinfo's kill() sends SIGKILL on Unix when called again
            let _ = process.kill();
            std::thread::sleep(Duration::from_millis(100));
        }
        return Ok(true);
    }

    Ok(false)
}

#[cfg(not(unix))]
pub fn kill_process(pid: u32, _force: bool) -> anyhow::Result<bool> {
    use sysinfo::Pid;
    let sys = System::new_all();

    if let Some(process) = sys.process(Pid::from_u32(pid)) {
        let killed = process.kill();
        if killed {
            Ok(true)
        } else {
            Err(anyhow::anyhow!("Failed to terminate process {}", pid))
        }
    } else {
        Err(anyhow::anyhow!("Process {} not found", pid))
    }
}

/// Check if any gateway is running
pub fn has_gateways_running() -> bool {
    !detect_gateways().is_empty()
}

/// Get count of running gateways
pub fn count_gateways() -> usize {
    detect_gateways().len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_gateways() {
        let gateways = detect_gateways();
        println!("Detected {} gateways", gateways.len());
        for gw in &gateways {
            println!("  PID: {}, Type: {}, Name: {}", gw.pid, gw.gateway.name, gw.name);
        }
    }

    #[test]
    fn test_has_gateways() {
        println!("Gateways running: {}", has_gateways_running());
    }
}
