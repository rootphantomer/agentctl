# agentctl

**检测和管理系统中运行的所有 Agent Gateway 进程。**

`agentctl` 是一个跨平台 CLI 工具，通过扫描系统进程来识别流行的 Agent Gateway 框架——包括 MCP、A2A、Dify、n8n、LangGraph、Ollama 等——然后你可以列出、查看或终止它们。

---

## 功能

- **自动检测** 20+ 种 Agent Gateway，通过进程名和命令行关键词匹配
- **列出** 运行中的网关，显示 PID、类型、内存、CPU 和运行时长
- **终止** 单个进程（按 PID）或一键终止所有检测到的网关
- **检查** 是否有网关在运行（基于退出码，适合脚本）
- **多种输出格式** — 表格（默认）、JSON、紧凑模式
- **Verbose 模式** 展示更多详情（进程名、默认端口、Web URL）
- **跨平台** — macOS、Linux、Windows

---

## 安装

### 从源码编译

```bash
git clone https://github.com/agentctl/agentctl.git
cd agentctl
cargo build --release
cp target/release/agentctl ~/.local/bin/
```

> 需要 [Rust](https://rustup.rs/) 1.75+ 及以上版本。

### 预编译二进制

*(即将推出)*

---

## 使用示例

### 列出运行中的网关

```bash
# 默认表格视图
agentctl list

# Verbose 模式 — 显示进程名、端口、Web URL
agentctl list --verbose

# JSON 输出，便于脚本解析
agentctl list --format json

# 紧凑格式，每行一个进程
agentctl list --format compact
```

示例输出：

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

### 快速检查

```bash
agentctl check
# No Agent Gateways detected - all clear!
```

退出码：`0` 表示未检测到网关，`1` 表示有网关在运行。适合与 shell 条件判断搭配：

```bash
agentctl check && echo "All clear!" || echo "Gateways running!"
```

### 终止指定进程

```bash
agentctl kill 68356           # 发送 SIGTERM（优雅终止）
agentctl kill 68356 --force   # 发送 SIGKILL（强制终止）
```

### 一键终止所有网关

```bash
agentctl kill-all             # 需要确认
agentctl kill-all -y          # 跳过确认
agentctl kill-all -y --force  # 强制终止所有
```

### 查看支持的网关类型

```bash
agentctl types
```

---

## 支持的网关类型（20 种）

| ID | 名称 | 默认端口 |
|----|------|---------|
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

检测使用不区分大小写的关键词匹配，覆盖进程名和命令行参数。如需新增，编辑 `src/gateway.rs` 即可。

---

## 输出格式

### Table（默认）

人类可读的 ASCII 表格，包含 PID、类型、资源使用情况、命令行。

### JSON

结构化输出，包含完整信息：

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

每行一个进程，适合快速浏览或管道处理：

```
[68356] (deepseek-tui) mem=81.8 MB up=24m node node /usr/bin/deepseek-tui
```

---

## 从源码构建

```bash
git clone https://github.com/agentctl/agentctl.git
cd agentctl
cargo build --release
```

编译产物位于 `target/release/agentctl`。

运行测试：

```bash
cargo test
```

---

## 项目结构

```
src/
├── main.rs          # CLI 入口（clap 参数解析，子命令分发）
├── commands/        # 各子命令实现
│   ├── mod.rs
│   ├── list.rs
│   ├── check.rs
│   ├── kill.rs
│   └── kill_all.rs
├── detector.rs      # 进程扫描与关键词匹配
├── gateway.rs       # 20 种网关类型定义
└── output.rs        # 表格 / JSON / 紧凑三种输出格式
```

---

## 开源协议

MIT
