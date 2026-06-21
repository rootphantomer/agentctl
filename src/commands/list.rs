//! List command - show all detected Agent Gateway processes

use crate::detector::detect_gateways;
use crate::output::{output_gateways, OutputFormat};

/// Execute the list command
pub fn list_command(format: OutputFormat, verbose: bool) {
    let gateways = detect_gateways();
    output_gateways(&gateways, format, verbose);

    if gateways.is_empty() {
        std::process::exit(0);
    }
}
