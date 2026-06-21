# agentctl

**Detect and manage Agent Gateway processes on your system.**

`agentctl` is a cross-platform CLI tool that scans running processes to identify popular Agent Gateway frameworks — MCP, A2A, Dify, n8n, LangGraph, Ollama, and more — then lets you list, inspect, or terminate them.

[🇨🇳 中文文档](./README.zh.md)

---

## Features

- **Auto-detect** 20+ agent gateway types by process name and command-line keywords
- **List** running gateways with PID, type, memory, CPU, and uptime
- **Kill** individual processes by PID or terminate all detected gateways at once
- **Check** if any gateway is running (exit-code based, ideal for scripts)
- **Multiple output formats** — table (default), JSON, compact
- **Verbose mode** for extra details (process name, default port, web URL)
- **Cross-platform** — macOS, Linux

---

## Installation

### npm（recommended）

```bash
npm install -g agentctl
```

> Compiles from source automatically via `cargo build --release` on install.
> Requires [Rust](https://rustup.rs/) toolchain and Node.js 14+. First install takes ~1-3 min (crate download + compile).

### Build from source

```bash
git clone https://github.com/agentctl/agentctl.git
cd agentctl
cargo build --release
cp target/release/agentctl ~/.local/bin/
```

---

## Usage

### List running gateways

```bash
# Default table view
agentctl list

# Verbose — show process name, port, and web URL
agentctl list --verbose

# JSON output for scripting
agentctl list --format json

# Compact one-line-per-process
agentctl list --format compact
```

Example output:

```
┌─────────────────────────────────────────────────────────────────────────────────────────────────────────────────────┐
│PID  │Type        │Memory │CPU %│Uptime  │Command                                                                    │
├─────┼────────────┼───────┼─────┼────────┼───────────────────────────────────────────────────────────────────────────┤
│68356│DeepSeek TUI│81.8 MB│0.0% │24m     │node /usr/bin/deepseek-tui                                                 │
│34232│Hermes      │30.6 MB│0.0% │22h33m  │python -m hermes_cli.main gateway run --replace                            │
│12045│Ollama      │2.1 GB │12.5%│5d14h   │ollama serve                                                               │
└─────────────────────────────────────────────────────────────────────────────────────────────────────────────────────┘

  3 detected (2 types)
```

### Quick check

```bash
agentctl check
# No Agent Gateways detected - all clear!
```

Returns exit code `0` when nothing is found, `1` when gateways are running — suitable for shell conditionals:

```bash
agentctl check && echo "All clear!" || echo "Gateways running!"
```

### Kill a specific process

```bash
agentctl kill 68356           # Send SIGTERM
agentctl kill 68356 --force   # Send SIGKILL
```

### Kill all running gateways

```bash
agentctl kill-all             # Prompts for confirmation
agentctl kill-all -y          # Skip confirmation
agentctl kill-all -y --force  # Force kill all
```

### Show supported gateway types

```bash
agentctl types
```

---

## Supported Gateways (23)

| ID | Name | Default Port |
|----|------|-------------|
| `mcp` | MCP Gateway | 8080 |
| `a2a` | A2A Gateway | 4120 |
| `openai-agent` | OpenAI Agent Gateway | — |
| `dify` | Dify | 80 |
| `coze` | Coze | — |
| `n8n` | n8n | 5678 |
| `langgraph` | LangGraph Server | 5432 |
| `flowise` | Flowise | 3000 |
| `autogen` | AutoGen Studio | 8080 |
| `crewai` | CrewAI | — |
| `semantic-kernel` | Semantic Kernel | — |
| `haystack` | Haystack | 8000 |
| `fastapi-agent` | FastAPI Agent | — |
| `ollama` | Ollama | 11434 |
| `localai` | LocalAI | 8080 |
| `workbuddy` | WorkBuddy | — |
| `openclaw` | OpenClaw | — |
| `deepseek-tui` | DeepSeek TUI | — |
| `hermes` | Hermes | — |
| `qclaw` | QClaw | — |
| `cloudflare-workers` | Cloudflare Workers AI | — |
| `langchain` | LangChain | — |
| `llamaindex` | LlamaIndex | 8080 |

Detection uses case-insensitive keyword matching against process names and command-line arguments. Add more by editing `src/gateway.rs`.

---

## Output Formats

### Table (default)

Human-readable ASCII table with PID, type, resource usage, and command line.

### JSON

Structured output with full detail:

```json
{
  "success": true,
  "count": 3,
  "gateways": [
    {
      "pid": 68356,
      "name": "node",
      "gateway_type": "DeepSeek TUI",
      "gateway_id": "deepseek-tui",
      "cmd": ["node", "/usr/bin/deepseek-tui"],
      "memory_mb": 81.8,
      "cpu_percent": 0.0,
      "uptime_seconds": 1440,
      "uptime_human": "24m",
      "web_url": null,
      "default_port": null
    }
  ]
}
```

### Compact

One line per process, suitable for quick scanning or piping:

```
[68356] (deepseek-tui) mem=81.8 MB up=24m node node /usr/bin/deepseek-tui
```

---

## Building

```bash
git clone https://github.com/agentctl/agentctl.git
cd agentctl
cargo build --release
```

The binary is at `target/release/agentctl`.

Run tests:

```bash
cargo test
```

---

## Project Structure

```
src/
├── main.rs          # CLI entry point (clap parsing, command dispatch)
├── commands/        # Subcommand implementations
│   ├── mod.rs
│   ├── list.rs
│   ├── check.rs
│   ├── kill.rs
│   └── kill_all.rs
├── detector.rs      # Process scanning and keyword matching
├── gateway.rs       # Gateway type definitions (20 types)
└── output.rs        # Table / JSON / Compact output formatting
```

---

## License

MIT
