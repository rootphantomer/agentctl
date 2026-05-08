//! Gateway registry - defines all known Agent Gateway types and their detection keywords

use serde::{Deserialize, Serialize};

/// Represents a known Agent Gateway type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Gateway {
    /// Human-readable name
    pub name: String,
    /// Unique identifier for CLI display
    pub id: String,
    /// Keywords to match in process name or command line
    pub keywords: Vec<String>,
    /// URL pattern for web interface (optional)
    pub web_url: Option<String>,
    /// Default port (optional)
    pub default_port: Option<u16>,
}

impl Gateway {
    /// Create a new Gateway definition
    pub fn new(
        name: &'static str,
        id: &'static str,
        keywords: Vec<&'static str>,
        web_url: Option<&'static str>,
        default_port: Option<u16>,
    ) -> Self {
        Self {
            name: name.to_string(),
            id: id.to_string(),
            keywords: keywords.into_iter().map(String::from).collect(),
            web_url: web_url.map(String::from),
            default_port,
        }
    }
}

/// Returns the registry of all known Agent Gateways
pub fn gateways() -> Vec<Gateway> {
    vec![
        // 1. MCP Gateway - Model Context Protocol
        Gateway::new(
            "MCP Gateway",
            "mcp",
            vec![
                "mcp-gateway",
                "mcp_server",
                "mcp-proxy",
                "mcp-host",
                "@anthropic/mcp",
                "mcp-server",
                "mcp_gateway",
                "model-context-protocol",
            ],
            Some("http://localhost:8080"),
            Some(8080),
        ),
        // 2. A2A Gateway - Agent-to-Agent
        Gateway::new(
            "A2A Gateway",
            "a2a",
            vec![
                "a2a-gateway",
                "a2a_server",
                "a2a-server",
                "google-a2a",
                "a2a-proxy",
                "agent-to-agent",
            ],
            Some("http://localhost:4120"),
            Some(4120),
        ),
        // 3. OpenAI Agent Gateway
        Gateway::new(
            "OpenAI Agent Gateway",
            "openai-agent",
            vec![
                "openai-agent-gateway",
                "agentgateway",
                "openai/agents",
                "openai-agent-gw",
                "openai-agent-gateway",
            ],
            None,
            None,
        ),
        // 4. Dify - Open source LLM app development platform
        Gateway::new(
            "Dify",
            "dify",
            vec![
                "dify-api",
                "dify-worker",
                "dify-web",
                "dify",
                "dify-api",
                "dify-worker",
            ],
            Some("http://localhost:80"),
            Some(80),
        ),
        // 5. Coze - ByteDance AI platform
        Gateway::new(
            "Coze",
            "coze",
            vec![
                "coze-agent",
                "coze-gateway",
                "coze-server",
                "coze-bot",
                "coze",
            ],
            None,
            None,
        ),
        // 6. n8n - Workflow automation
        Gateway::new(
            "n8n",
            "n8n",
            vec![
                "n8n",
                "n8n-server",
            ],
            Some("http://localhost:5678"),
            Some(5678),
        ),
        // 7. LangGraph Server
        Gateway::new(
            "LangGraph Server",
            "langgraph",
            vec![
                "langgraph-server",
                "langgraph_api",
                "langgraph_api",
                "langgraph-server",
                "langgraph",
            ],
            Some("http://localhost:5432"),
            Some(5432),
        ),
        // 8. Flowise - Visual LLM flow builder
        Gateway::new(
            "Flowise",
            "flowise",
            vec![
                "flowise",
                "flowise-ai",
            ],
            Some("http://localhost:3000"),
            Some(3000),
        ),
        // 9. AutoGen Studio - Microsoft AutoGen
        Gateway::new(
            "AutoGen Studio",
            "autogen",
            vec![
                "autogen-studio",
                "autogenstudio",
                "autogen-studio",
            ],
            Some("http://localhost:8080"),
            Some(8080),
        ),
        // 10. CrewAI
        Gateway::new(
            "CrewAI",
            "crewai",
            vec![
                "crewai-server",
                "crewai_server",
                "crewai",
            ],
            None,
            None,
        ),
        // 11. Semantic Kernel
        Gateway::new(
            "Semantic Kernel",
            "semantic-kernel",
            vec![
                "semantic-kernel",
                "semantickernel",
                "semantic_kernel",
            ],
            None,
            None,
        ),
        // 12. Haystack - deepset AI
        Gateway::new(
            "Haystack",
            "haystack",
            vec![
                "haystack-ai",
                "deepset-haystack",
                "haystack",
                "deepset",
            ],
            Some("http://localhost:8000"),
            Some(8000),
        ),
        // 13. FastAPI Agent Gateway (generic)
        Gateway::new(
            "FastAPI Agent",
            "fastapi-agent",
            vec![
                "fastapi-agent",
                "fastapi-agent-server",
                "uvicorn-agent",
            ],
            None,
            None,
        ),
        // 14. Ollama (LLM runtime, often used with agents)
        Gateway::new(
            "Ollama",
            "ollama",
            vec![
                "ollama",
            ],
            Some("http://localhost:11434"),
            Some(11434),
        ),
        // 15. LocalAI
        Gateway::new(
            "LocalAI",
            "localai",
            vec![
                "localai",
                "local-ai",
            ],
            Some("http://localhost:8080"),
            Some(8080),
        ),
        // 16. WorkBuddy - AI agent development platform
        Gateway::new(
            "WorkBuddy",
            "workbuddy",
            vec![
                "workbuddy",
                "workbuddy-agent",
                "workbuddy-server",
                "workbuddy-gateway",
            ],
            None,
            None,
        ),
        // 17. OpenClaw - Open-source AI agent platform
        Gateway::new(
            "OpenClaw",
            "openclaw",
            vec![
                "openclaw",
                "openclaw-server",
                "openclaw-gateway",
                "openclaw-daemon",
            ],
            None,
            None,
        ),
        // 18. DeepSeek TUI - Terminal UI for DeepSeek
        Gateway::new(
            "DeepSeek TUI",
            "deepseek-tui",
            vec![
                "deepseek-tui",
                "deepseek_tui",
                "deepseek-tui-server",
                "deepseek-tui-gateway",
            ],
            None,
            None,
        ),
        // 19. Hermes - AI agent platform
        Gateway::new(
            "Hermes",
            "hermes",
            vec![
                "hermes",
                "hermes-agent",
                "hermes-server",
                "hermes-gateway",
                "hermes-daemon",
            ],
            None,
            None,
        ),
        // 20. QClaw - AI agent platform
        Gateway::new(
            "QClaw",
            "qclaw",
            vec![
                "qclaw",
                "qclaw-server",
                "qclaw-gateway",
                "qclaw-daemon",
            ],
            None,
            None,
        ),
    ]
}

/// Check if a string matches any keyword (case-insensitive)
pub fn matches_keyword(text: &str, keywords: &[String]) -> bool {
    let text_lower = text.to_lowercase();
    keywords.iter().any(|kw| text_lower.contains(&kw.to_lowercase()))
}
