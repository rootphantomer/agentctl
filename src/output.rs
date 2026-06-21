//! Output formatting for different display modes

use crate::detector::DetectedGateway;
use clap::ValueEnum;
use colored::Colorize;
use serde::Serialize;

/// Output format options
#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum OutputFormat {
    /// Human-readable table format
    Table,
    /// JSON format for scripting
    Json,
    /// Compact single-line format
    Compact,
}

impl Default for OutputFormat {
    fn default() -> Self {
        Self::Table
    }
}

/// Output result structure for JSON serialization
#[derive(Debug, Serialize)]
pub struct OutputResult {
    pub success: bool,
    pub gateways: Vec<GatewayOutput>,
    pub count: usize,
    pub message: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct GatewayOutput {
    pub pid: u32,
    pub name: String,
    pub gateway_type: String,
    pub gateway_id: String,
    pub cmd: Vec<String>,
    pub memory_mb: Option<f64>,
    pub cpu_percent: Option<f32>,
    pub uptime_seconds: Option<u64>,
    pub uptime_human: Option<String>,
    pub web_url: Option<String>,
    pub default_port: Option<u16>,
}

impl From<&DetectedGateway> for GatewayOutput {
    fn from(gw: &DetectedGateway) -> Self {
        let (uptime_secs, uptime_str) = match gw.start_time {
            Some(start) => {
                let now = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs();
                let secs = now.saturating_sub(start);
                (Some(secs), Some(format_uptime(start)))
            }
            None => (None, None),
        };

        Self {
            pid: gw.pid,
            name: gw.name.clone(),
            gateway_type: gw.gateway.name.clone(),
            gateway_id: gw.gateway.id.clone(),
            cmd: gw.cmd.clone(),
            memory_mb: gw.memory.map(|m| m as f64 / 1_048_576.0),
            cpu_percent: gw.cpu,
            uptime_seconds: uptime_secs,
            uptime_human: uptime_str,
            web_url: gw.gateway.web_url.clone(),
            default_port: gw.gateway.default_port,
        }
    }
}

/// Format memory in human-readable form
pub fn format_memory(bytes: u64) -> String {
    const KB: f64 = 1_024.0;
    const MB: f64 = KB * 1_024.0;
    const GB: f64 = MB * 1_024.0;

    let bytes = bytes as f64;
    if bytes >= GB {
        format!("{:.1} GB", bytes / GB)
    } else if bytes >= MB {
        format!("{:.1} MB", bytes / MB)
    } else if bytes >= KB {
        format!("{:.1} KB", bytes / KB)
    } else {
        format!("{} B", bytes as u64)
    }
}

/// Format uptime from Unix timestamp (seconds since epoch)
pub fn format_uptime(start_time: u64) -> String {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    let elapsed = now.saturating_sub(start_time);
    let days = elapsed / 86400;
    let hours = (elapsed % 86400) / 3600;
    let minutes = (elapsed % 3600) / 60;

    if days > 0 {
        format!("{}d{:02}h{:02}m", days, hours, minutes)
    } else if hours > 0 {
        format!("{:02}h{:02}m", hours, minutes)
    } else {
        format!("{:02}m", minutes)
    }
}

/// Simple ASCII table printer with configurable column max-width.
///
/// Each column is exactly `col_width` characters wide. Cells are
/// left-padded with spaces to fill the column. Separators use `│`
/// without extra spaces — all padding is in the cell itself.
struct AsciiTable {
    headers: Vec<String>,
    rows: Vec<Vec<String>>,
    col_widths: Vec<usize>,
    /// Per-column max-width override; None means use the default (200)
    col_max_overrides: Vec<Option<usize>>,
}

impl AsciiTable {
    fn new(headers: Vec<String>) -> Self {
        let col_widths = headers.iter().map(|h| h.len()).collect();
        let col_max_overrides = vec![None; headers.len()];
        Self {
            headers,
            rows: Vec::new(),
            col_widths,
            col_max_overrides,
        }
    }

    /// Set a per-column max-width cap. Use `None` for the default (200).
    fn set_col_max(&mut self, col: usize, max: Option<usize>) {
        if col < self.col_max_overrides.len() {
            self.col_max_overrides[col] = max;
        }
    }

    fn add_row(&mut self, row: Vec<String>) {
        for (i, cell) in row.iter().enumerate() {
            if let Some(width) = self.col_widths.get_mut(i) {
                let max = self.col_max_overrides[i].unwrap_or(200);
                *width = (*width).max(cell.len().min(max));
            }
        }
        self.rows.push(row);
    }

    fn print(&self) {
        // Line width: each column width + │ between + 2 outer │
        let line_w = self.col_widths.iter().sum::<usize>() + self.col_widths.len() + 1;

        println!("┌{}┐", "─".repeat(line_w.saturating_sub(2)));

        // Header row
        let hcells: Vec<String> = self
            .headers
            .iter()
            .enumerate()
            .map(|(i, h)| self.pad_cell(h, self.col_widths[i]))
            .collect();
        println!("│{}│", hcells.join("│"));

        // Separator between header and rows
        println!(
            "├{}┤",
            self.col_widths
                .iter()
                .map(|w| "─".repeat(*w))
                .collect::<Vec<_>>()
                .join("┼")
        );

        // Data rows
        for row in &self.rows {
            let cells: Vec<String> = row
                .iter()
                .enumerate()
                .map(|(i, c)| self.pad_cell(c, self.col_widths[i]))
                .collect();
            println!("│{}│", cells.join("│"));
        }

        // Bottom border
        println!("└{}┘", "─".repeat(line_w.saturating_sub(2)));
    }

    /// Return a string exactly `width` chars long, left-aligned with
    /// right-padding spaces. Truncates with ".." if text exceeds `width`.
    fn pad_cell(&self, text: &str, width: usize) -> String {
        let display = if text.len() > width {
            let keep = width.saturating_sub(2);
            format!("{}..", &text[..keep])
        } else {
            text.to_string()
        };
        format!("{:<width$}", display, width = width)
    }
}

/// Output detected gateways in the specified format
pub fn output_gateways(gateways: &[DetectedGateway], format: OutputFormat, verbose: bool) {
    match format {
        OutputFormat::Table => output_table(gateways, verbose),
        OutputFormat::Json => output_json(gateways),
        OutputFormat::Compact => output_compact(gateways),
    }
}

/// Output in table format
fn output_table(gateways: &[DetectedGateway], verbose: bool) {
    if gateways.is_empty() {
        println!("{}", "  No Agent Gateways detected".yellow().dimmed());
        return;
    }

    let (headers, col_maxes) = if verbose {
        (
            vec!["PID", "Type", "Process", "Memory", "CPU %", "Uptime", "Port", "Web URL", "Command"],
            vec![Some(8), None, None, None, None, None, None, None, None],
        )
    } else {
        (
            vec!["PID", "Type", "Memory", "CPU %", "Uptime", "Command"],
            vec![Some(8), None, None, None, None, None],
        )
    };

    let mut table = AsciiTable::new(headers.iter().map(|h| h.to_string()).collect());
    for (i, max) in col_maxes.iter().enumerate() {
        table.set_col_max(i, *max);
    }

    for gw in gateways {
        let cmd_full = gw.cmd.join(" ");
        let uptime = gw.start_time.map(format_uptime).unwrap_or_else(|| "N/A".to_string());
        let mem = gw.memory.map(format_memory).unwrap_or_else(|| "N/A".to_string());
        let cpu = format!("{:.1}%", gw.cpu.unwrap_or(0.0));

        let mut row = vec![
            gw.pid.to_string(),
            gw.gateway.name.clone(),
            mem,
            cpu,
            uptime,
            cmd_full,
        ];

        if verbose {
            let web_url = gw.gateway.web_url.as_deref().unwrap_or("-").to_string();
            let port = gw.gateway.default_port.map(|p| p.to_string()).unwrap_or_else(|| "-".to_string());
            row.insert(2, gw.name.clone());
            row.push(port);
            row.push(web_url);
        }

        table.add_row(row);
    }

    table.print();

    // Print summary
    let count = gateways.len();
    let gateway_types: std::collections::HashSet<_> = gateways.iter().map(|g| g.gateway.id.clone()).collect();
    println!(
        "\n  {} detected ({} types)",
        count.to_string().green().bold(),
        gateway_types.len().to_string().cyan()
    );
}

/// Output in JSON format
fn output_json(gateways: &[DetectedGateway]) {
    let result = OutputResult {
        success: true,
        gateways: gateways.iter().map(GatewayOutput::from).collect(),
        count: gateways.len(),
        message: None,
    };

    println!("{}", serde_json::to_string_pretty(&result).expect("Failed to serialize JSON"));
}

/// Output in compact format (one line per gateway)
fn output_compact(gateways: &[DetectedGateway]) {
    for gw in gateways {
        println!(
            "{} {} mem={} up={} {} {}",
            format!("[{}]", gw.pid).cyan(),
            format!("({})", gw.gateway.id).green(),
            gw.memory.map(format_memory).unwrap_or_else(|| "?".to_string()).yellow(),
            gw.start_time.map(format_uptime).unwrap_or_else(|| "?".to_string()).dimmed(),
            gw.name.blue(),
            gw.cmd.join(" ").dimmed()
        );
    }

    if gateways.is_empty() {
        println!("{}", "No gateways detected".yellow());
    } else {
        println!("\nTotal: {} gateway(s)", gateways.len());
    }
}

/// Print a success message
pub fn print_success(msg: &str) {
    println!("{} {}", "✓".green(), msg);
}

/// Print an error message
pub fn print_error(msg: &str) {
    eprintln!("{} {}", "✗".red(), msg);
}

/// Print a warning message
pub fn print_warning(msg: &str) {
    println!("{} {}", "⚠".yellow(), msg);
}

/// Print info message
pub fn print_info(msg: &str) {
    println!("{} {}", "ℹ".blue(), msg);
}

/// Print confirmation prompt
pub fn print_confirm(msg: &str) -> bool {
    print!("{} {} [y/N]: ", "⚠".yellow(), msg);
    std::io::Write::flush(&mut std::io::stdout()).ok();

    let mut input = String::new();
    if std::io::stdin().read_line(&mut input).is_ok() {
        let input = input.trim().to_lowercase();
        input == "y" || input == "yes"
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_memory() {
        assert_eq!(format_memory(500), "500 B");
        assert_eq!(format_memory(1024), "1.0 KB");
        assert_eq!(format_memory(1048576), "1.0 MB");
        assert_eq!(format_memory(1073741824), "1.0 GB");
    }
}
