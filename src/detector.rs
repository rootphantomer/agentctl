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
///
/// Sends a termination signal, then waits up to 3s for the process to actually
/// disappear. With `force`, the wait is skipped and the outcome is reported
/// optimistically (see the platform notes below).
///
/// Returns `Ok(true)` when the process is confirmed gone, `Ok(false)` when it
/// was still alive after the grace period, and `Err` when it could not be
/// signalled at all.
#[cfg(unix)]
pub fn kill_process(pid: u32, force: bool) -> anyhow::Result<bool> {
    use std::time::{Duration, Instant};
    use sysinfo::Pid;

    let mut sys = System::new_all();
    let target = Pid::from_u32(pid);

    // Try to find and kill the process
    let Some(process) = sys.process(target) else {
        return Err(anyhow::anyhow!("Process {} not found", pid));
    };

    let killed = process.kill();
    if !killed {
        return Err(anyhow::anyhow!("Failed to send signal to process {}", pid));
    }

    if force {
        // Fire-and-forget: the caller asked not to wait for a graceful exit.
        return Ok(true);
    }

    // Wait for process to terminate (up to 3 seconds).
    //
    // Polling refreshes only this PID — building a whole new `System` on every
    // 100ms tick re-enumerated every process on the machine each time.
    let deadline = Instant::now() + Duration::from_secs(3);
    while Instant::now() < deadline {
        if !process_exists(&mut sys, target) {
            return Ok(true);
        }
        std::thread::sleep(Duration::from_millis(100));
    }

    Ok(false)
}

/// Kill a process by PID (non-Unix platforms).
///
/// Note: `sysinfo` implements process termination on Windows by shelling out to
/// `taskkill.exe /PID <pid> /F`, which is always a *forceful* termination — the
/// crate exposes no graceful (SIGTERM-equivalent) path. The `force` flag
/// therefore cannot select a gentler signal here; it only controls whether we
/// wait for the process to actually disappear before reporting success.
#[cfg(not(unix))]
pub fn kill_process(pid: u32, force: bool) -> anyhow::Result<bool> {
    use std::time::{Duration, Instant};
    use sysinfo::Pid;

    let mut sys = System::new_all();
    let target = Pid::from_u32(pid);

    let Some(process) = sys.process(target) else {
        return Err(anyhow::anyhow!("Process {} not found", pid));
    };

    let killed = process.kill();
    if !killed {
        return Err(anyhow::anyhow!("Failed to terminate process {}", pid));
    }

    if force {
        return Ok(true);
    }

    let deadline = Instant::now() + Duration::from_secs(3);
    while Instant::now() < deadline {
        if !process_exists(&mut sys, target) {
            return Ok(true);
        }
        std::thread::sleep(Duration::from_millis(100));
    }

    Ok(false)
}

/// Refresh a single PID and report whether the process is still present.
fn process_exists(sys: &mut System, pid: sysinfo::Pid) -> bool {
    sys.refresh_processes(sysinfo::ProcessesToUpdate::Some(&[pid]), true);
    sys.process(pid).is_some()
}

/// Check if any gateway is running
///
/// Convenience wrapper around [`detect_gateways`], kept test-only: production
/// code should call `detect_gateways()` once and derive both the count and the
/// boolean from it, rather than scanning twice.
#[cfg(test)]
pub fn has_gateways_running() -> bool {
    !detect_gateways().is_empty()
}

/// Get count of running gateways (see [`has_gateways_running`]).
#[cfg(test)]
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
        // Detect once and derive both facts from the same snapshot — the two
        // wrappers exist so this stays a single scan.
        let detected = detect_gateways();
        assert_eq!(has_gateways_running(), !detected.is_empty());
        assert_eq!(count_gateways(), detected.len());
    }
}
