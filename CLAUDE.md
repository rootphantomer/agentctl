# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

`agentctl` is a cross-platform CLI tool that detects and manages Agent Gateway processes (MCP, A2A, Dify, n8n, Ollama, etc.) by scanning running processes via keyword matching.

## Build & Development Commands

```bash
# Build release binary
cargo build --release

# Build debug binary
cargo build

# Run all tests
cargo test

# Run a specific test
cargo test test_name

# Run tests with output
cargo test -- --nocapture

# Check code without building
cargo check

# Format code
cargo fmt

# Lint with clippy
cargo clippy

# Run the binary
cargo run -- list
cargo run -- check
cargo run -- types
```

## Release Process

```bash
# Create a new release (updates Cargo.toml, package.json, commits, tags)
make release VERSION=x.y.z

# Push tag to trigger CI auto-publish
make release-push

# View current version
make version
```

## Architecture

```
src/
├── main.rs          # CLI entry point (clap derive), command dispatch, AsciiTable for `types`
├── commands/        # Subcommand implementations
│   ├── mod.rs       # Re-exports all command functions
│   ├── list.rs      # List detected gateways (calls detect_gateways + output_gateways)
│   ├── check.rs     # Exit-code check (0=none, 1=gateways running)
│   ├── kill.rs      # Kill single PID (validates it's a gateway first)
│   └── kill_all.rs  # Kill all gateways with confirmation prompt
├── detector.rs      # Process scanning via sysinfo crate, keyword matching, kill logic
├── gateway.rs       # Gateway type registry (20 types with keywords, ports, URLs)
└── output.rs        # Table/JSON/Compact formatting, memory/uptime helpers
```

## Key Design Decisions

- **Detection**: Uses `sysinfo` crate to enumerate all processes, matches against keyword lists in `gateway.rs`. Keywords are case-insensitive substring matches against process name and command line.
- **Gateway registry**: All 20 gateway types are defined in `gateway.rs` as a `Vec<Gateway>` returned by `gateways()`. Each has `name`, `id`, `keywords`, optional `web_url`, and optional `default_port`.
- **Output formats**: Three modes (Table, JSON, Compact) controlled by `--format` flag. Table uses custom `AsciiTable` struct (not external crate).
- **Process kill**: On Unix, sends SIGTERM first; `--force` escalates to SIGKILL after 3s timeout. Waits up to 3s for graceful shutdown.
- **Exit codes**: `check` returns 0 (no gateways) or 1 (gateways running). `kill` and `kill-all` return 0 on success, 1 on failure.
- **npm packaging**: The Rust binary is distributed via npm with a `postinstall` script that compiles from source. `package.json` is synced from `Cargo.toml` via `scripts/sync-version.js`.

## Dependencies

- `clap` 4.5 (derive) — CLI argument parsing
- `sysinfo` 0.33 — Process enumeration and management
- `serde`/`serde_json` — JSON output serialization
- `colored` — Terminal colors
- `anyhow` — Error handling

## Adding New Gateway Types

Edit `src/gateway.rs` and add a new `Gateway::new(...)` entry to the `gateways()` function. Keywords should be lowercase and include common process name variants.

## Cross-Compilation Targets

CI builds for: `x86_64-apple-darwin`, `aarch64-apple-darwin`, `x86_64-unknown-linux-gnu`, `aarch64-unknown-linux-gnu`. Linux ARM64 requires `gcc-aarch64-linux-gnu` for cross-compilation.
