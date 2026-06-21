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
        keywords: &[&'static str],
        web_url: Option<&'static str>,
        default_port: Option<u16>,
    ) -> Self {
        Self {
            name: name.to_string(),
            id: id.to_string(),
            keywords: keywords.iter().map(|s| s.to_string()).collect(),
            web_url: web_url.map(String::from),
            default_port,
        }
    }
}

/// Returns the registry of all known Agent Gateways
pub fn gateways() -> Vec<Gateway> {
    vec![
        // MCP Gateway - Model Context Protocol
        Gateway::new("MCP Gateway", "mcp", &["mcp-gateway", "mcp_server", "mcp-server", "mcp_gateway"], Some("http://localhost:8080"), Some(8080)),
        // A2A Gateway - Agent-to-Agent
        Gateway::new("A2A Gateway", "a2a", &["a2a-gateway", "a2a_server", "a2a-server", "agent-to-agent"], Some("http://localhost:4120"), Some(4120)),
        // OpenAI Agent Gateway
        Gateway::new("OpenAI Agent Gateway", "openai-agent", &["openai-agent-gateway", "agentgateway", "openai/agents"], None, None),
        // Dify - Open source LLM app development platform
        Gateway::new("Dify", "dify", &["dify-api", "dify-worker", "dify-web", "dify"], Some("http://localhost:80"), Some(80)),
        // Coze - ByteDance AI platform
        Gateway::new("Coze", "coze", &["coze-agent", "coze-gateway", "coze-server", "coze-bot", "coze"], None, None),
        // n8n - Workflow automation
        Gateway::new("n8n", "n8n", &["n8n", "n8n-server"], Some("http://localhost:5678"), Some(5678)),
        // LangGraph Server
        Gateway::new("LangGraph Server", "langgraph", &["langgraph-server", "langgraph_api", "langgraph"], Some("http://localhost:5432"), Some(5432)),
        // Flowise - Visual LLM flow builder
        Gateway::new("Flowise", "flowise", &["flowise", "flowise-ai"], Some("http://localhost:3000"), Some(3000)),
        // AutoGen Studio - Microsoft AutoGen
        Gateway::new("AutoGen Studio", "autogen", &["autogen-studio", "autogenstudio"], Some("http://localhost:8080"), Some(8080)),
        // CrewAI
        Gateway::new("CrewAI", "crewai", &["crewai-server", "crewai_server", "crewai"], None, None),
        // Semantic Kernel
        Gateway::new("Semantic Kernel", "semantic-kernel", &["semantic-kernel", "semantickernel", "semantic_kernel"], None, None),
        // Haystack - deepset AI
        Gateway::new("Haystack", "haystack", &["haystack-ai", "deepset-haystack", "haystack", "deepset"], Some("http://localhost:8000"), Some(8000)),
        // FastAPI Agent Gateway (generic)
        Gateway::new("FastAPI Agent", "fastapi-agent", &["fastapi-agent", "fastapi-agent-server", "uvicorn-agent"], None, None),
        // Ollama (LLM runtime, often used with agents)
        Gateway::new("Ollama", "ollama", &["ollama"], Some("http://localhost:11434"), Some(11434)),
        // LocalAI
        Gateway::new("LocalAI", "localai", &["localai", "local-ai"], Some("http://localhost:8080"), Some(8080)),
        // WorkBuddy - AI agent development platform
        Gateway::new("WorkBuddy", "workbuddy", &["workbuddy", "workbuddy-agent", "workbuddy-server", "workbuddy-gateway"], None, None),
        // OpenClaw - Open-source AI agent platform
        Gateway::new("OpenClaw", "openclaw", &["openclaw", "openclaw-server", "openclaw-gateway", "openclaw-daemon"], None, None),
        // DeepSeek TUI - Terminal UI for DeepSeek
        Gateway::new("DeepSeek TUI", "deepseek-tui", &["deepseek-tui", "deepseek_tui", "deepseek-tui-server", "deepseek-tui-gateway"], None, None),
        // Hermes - AI agent platform
        Gateway::new("Hermes", "hermes", &["hermes", "hermes-agent", "hermes-server", "hermes-gateway", "hermes-daemon"], None, None),
        // QClaw - AI agent platform
        Gateway::new("QClaw", "qclaw", &["qclaw", "qclaw-server", "qclaw-gateway", "qclaw-daemon"], None, None),
        // Cloudflare Workers AI Agent
        Gateway::new("Cloudflare Workers AI", "cloudflare-workers", &["wrangler", "workerd", "cloudflare-worker"], None, None),
        // LangChain
        Gateway::new("LangChain", "langchain", &["langchain", "langserve", "langchain-server"], None, None),
        // LlamaIndex
        Gateway::new("LlamaIndex", "llamaindex", &["llamaindex", "llama-index", "llama.cpp"], Some("http://localhost:8080"), Some(8080)),
    ]
}

/// Check if a string matches any keyword (case-insensitive)
pub fn matches_keyword(text: &str, keywords: &[String]) -> bool {
    let text_lower = text.to_lowercase();
    keywords.iter().any(|kw| text_lower.contains(&kw.to_lowercase()))
}
