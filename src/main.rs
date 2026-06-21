//! agentctl - CLI tool to detect and manage running Agent Gateway processes
//!
//! # Overview
//! agentctl helps you detect, monitor, and manage Agent Gateway processes
//! running on your system. It supports detecting various popular agent gateways
//! including MCP, A2A, Dify, n8n, LangGraph, and more.
//!
//! # Quick Start
//! ```bash
//! # List all running gateways
//! agentctl list
//!
//! # Quick check (exit code based)
//! agentctl check && echo "All clear!" || echo "Gateways running!"
//!
//! # Kill a specific gateway
//! agentctl kill 12345
//!
//! # Kill all gateways (with confirmation)
//! agentctl kill-all
//! ```

mod commands;
mod detector;
mod gateway;
mod output;

use clap::{Parser, Subcommand};

use crate::output::OutputFormat;

/// agentctl - Detect and manage running Agent Gateway processes
///
/// This tool helps you identify Agent Gateway processes (MCP, A2A, Dify, n8n, etc.)
/// running on your system and optionally terminate them.
#[derive(Parser)]
#[command(
    name = "agentctl",
    about = "Detect and manage running Agent Gateway processes",
    long_about = None,
    version,
    author = "Agentctl Team",
)]
struct Cli {
    /// Enable verbose output
    #[arg(short, long, global = true)]
    verbose: bool,

    /// Output format
    #[arg(long, value_enum, default_value = "table", global = true)]
    format: OutputFormat,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// List all detected Agent Gateway processes
    List {
        /// Shorthand alias
        #[arg(alias = "ls")]
        _alias: Option<bool>,
    },
    /// Quick check if any gateway is running
    Check,
    /// Kill a specific gateway process by PID
    Kill {
        /// Process ID to terminate
        #[arg(value_name = "PID", required = true)]
        pid: u32,

        /// Send SIGKILL instead of SIGTERM (force kill)
        #[arg(short, long)]
        force: bool,
    },
    /// Kill all detected Agent Gateway processes
    KillAll {
        /// Skip confirmation prompt
        #[arg(short = 'y', long)]
        yes: bool,

        /// Send SIGKILL instead of SIGTERM
        #[arg(short, long)]
        force: bool,
    },
    /// Show supported gateway types
    Types,
}

fn main() {
    let cli = Cli::parse();

    // Set verbose mode for output module
    if cli.verbose {
        std::env::set_var("AGENTCTL_VERBOSE", "1");
    }

    let exit_code = match cli.command {
        Some(Commands::List { .. }) => {
            commands::list_command(cli.format, cli.verbose);
            0
        }
        Some(Commands::Check) => commands::check_command(cli.format),
        Some(Commands::Kill { pid, force }) => commands::kill_command(pid, force),
        Some(Commands::KillAll { yes, force }) => commands::kill_all_command(force, yes),
        Some(Commands::Types) => {
            show_types();
            0
        }
        None => {
            // Default: run check command
            commands::check_command(cli.format)
        }
    };

    std::process::exit(exit_code);
}

/// Simple ASCII table printer (for `types` command).
///
/// Each column is exactly `col_width` characters wide. Cells are
/// left-padded with spaces. Separators use `│` without extra spaces.
struct AsciiTable {
    headers: Vec<String>,
    rows: Vec<Vec<String>>,
    col_widths: Vec<usize>,
    col_max: Vec<Option<usize>>,
}

impl AsciiTable {
    fn new(headers: Vec<String>) -> Self {
        let col_widths = headers.iter().map(|h| h.len()).collect();
        let col_max = vec![None; headers.len()];
        Self {
            headers,
            rows: Vec::new(),
            col_widths,
            col_max,
        }
    }

    fn set_col_max(&mut self, col: usize, max: Option<usize>) {
        if col < self.col_max.len() {
            self.col_max[col] = max;
        }
    }

    fn add_row(&mut self, row: Vec<String>) {
        for (i, cell) in row.iter().enumerate() {
            if let Some(width) = self.col_widths.get_mut(i) {
                let max = self.col_max[i].unwrap_or(200);
                *width = (*width).max(cell.len().min(max));
            }
        }
        self.rows.push(row);
    }

    fn print(&self) {
        let line_w = self.col_widths.iter().sum::<usize>() + self.col_widths.len() + 1;
        println!("┌{}┐", "─".repeat(line_w.saturating_sub(2)));

        let hcells: Vec<String> = self
            .headers
            .iter()
            .enumerate()
            .map(|(i, h)| self.pad_cell(h, self.col_widths[i]))
            .collect();
        println!("│{}│", hcells.join("│"));

        println!(
            "├{}┤",
            self.col_widths
                .iter()
                .map(|w| "─".repeat(*w))
                .collect::<Vec<_>>()
                .join("┼")
        );

        for row in &self.rows {
            let cells: Vec<String> = row
                .iter()
                .enumerate()
                .map(|(i, c)| self.pad_cell(c, self.col_widths[i]))
                .collect();
            println!("│{}│", cells.join("│"));
        }

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

/// Display all supported gateway types
fn show_types() {
    use colored::Colorize;

    let gateways = gateway::gateways();

    println!();
    println!("{}", "Supported Agent Gateway Types".bold().cyan());
    println!("{}", "─".repeat(60).cyan());
    println!();

    let mut table = AsciiTable::new(vec![
        "ID".to_string(),
        "Name".to_string(),
        "Keywords".to_string(),
        "Port".to_string(),
    ]);
    table.set_col_max(0, Some(20));
    table.set_col_max(2, None); // Keywords - full width

    for gw in &gateways {
        table.add_row(vec![
            gw.id.clone(),
            gw.name.clone(),
            gw.keywords.join(", "),
            gw.default_port.map(|p| p.to_string()).unwrap_or_else(|| "N/A".to_string()),
        ]);
    }

    table.print();
    println!();
    println!("  {} supported gateway types", gateways.len().to_string().green());
}
