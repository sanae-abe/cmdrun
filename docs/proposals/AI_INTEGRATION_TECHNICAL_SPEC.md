# AI Integration Technical Specification

> **Companion Document to**: [AI_INTEGRATION.md](./AI_INTEGRATION.md)
> **Purpose**: Detailed technical specifications and implementation guidelines
> **Status**: Draft
> **Last Updated**: 2025-11-09

## Table of Contents

- [Architecture Diagrams](#architecture-diagrams)
- [API Specifications](#api-specifications)
- [Code Examples](#code-examples)
- [Database Schema](#database-schema)
- [Configuration Schema](#configuration-schema)
- [Testing Strategy](#testing-strategy)
- [Performance Benchmarks](#performance-benchmarks)

---

## Architecture Diagrams

### Error Diagnosis Flow - Detailed

```
┌─────────────────────────────────────────────────────────────┐
│ User executes command:                                      │
│ $ cmdrun run deploy                                         │
└────────────────────────┬────────────────────────────────────┘
                         │
                         ▼
┌─────────────────────────────────────────────────────────────┐
│ CommandExecutor::execute_single()                           │
│  • Spawn process: tokio::process::Command                   │
│  • Apply timeout: tokio::time::timeout()                    │
│  • Capture stdout/stderr                                    │
│  • Wait for exit code                                       │
└────────────────────────┬────────────────────────────────────┘
                         │
                         ▼ (Exit code != 0)
┌─────────────────────────────────────────────────────────────┐
│ Plugin Hook: on_error()                                     │
│  • Triggered by: PluginManager::notify_error()              │
│  • Thread: async task on tokio runtime                      │
└────────────────────────┬────────────────────────────────────┘
                         │
                         ▼
┌─────────────────────────────────────────────────────────────┐
│ AIDiagnosticsPlugin::on_error()                             │
│  1. Check if AI diagnosis enabled                           │
│  2. Check per-command override                              │
│  3. Build ErrorContext                                      │
└────────────────────────┬────────────────────────────────────┘
                         │
                         ▼
┌─────────────────────────────────────────────────────────────┐
│ collect_context()                                            │
│  ┌───────────────────────────────────────────────────────┐  │
│  │ ErrorContext {                                        │  │
│  │   command_name: "deploy",                             │  │
│  │   args: vec!["--env", "production"],                  │  │
│  │   exit_code: 1,                                       │  │
│  │   stdout: "Connecting to server...",                  │  │
│  │   stderr: "Permission denied (publickey)",            │  │
│  │   working_dir: "/home/user/project",                  │  │
│  │   environment: HashMap<...>,                          │  │
│  │   recent_history: vec![                               │  │
│  │     HistoryEntry { command: "build", ... },           │  │
│  │     HistoryEntry { command: "test", ... },            │  │
│  │   ],                                                  │  │
│  │   timestamp: Utc::now(),                              │  │
│  │ }                                                     │  │
│  └───────────────────────────────────────────────────────┘  │
└────────────────────────┬────────────────────────────────────┘
                         │
                         ▼
┌─────────────────────────────────────────────────────────────┐
│ apply_privacy_filters()                                      │
│  • Scan for secrets: SENSITIVE_PATTERNS regex               │
│  • Filter environment variables                             │
│  • Redact file paths if configured                          │
│  • Remove API keys, tokens, passwords                       │
│  ┌───────────────────────────────────────────────────────┐  │
│  │ Before:                                               │  │
│  │   env: {"API_KEY": "sk-1234567890abcdef"}            │  │
│  │ After:                                                │  │
│  │   env: {"API_KEY": "[FILTERED]"}                     │  │
│  └───────────────────────────────────────────────────────┘  │
└────────────────────────┬────────────────────────────────────┘
                         │
                         ▼
┌─────────────────────────────────────────────────────────────┐
│ build_prompt()                                               │
│  • Load template: ERROR_DIAGNOSIS_PROMPT                    │
│  • Inject context: command, error, history                  │
│  • Add system instructions                                  │
│  ┌───────────────────────────────────────────────────────┐  │
│  │ System: You are a command-line expert...             │  │
│  │                                                       │  │
│  │ Context:                                              │  │
│  │ - Command: deploy                                     │  │
│  │ - Exit Code: 1                                        │  │
│  │ - Error: Permission denied (publickey)                │  │
│  │                                                       │  │
│  │ Task: Diagnose and provide solutions...              │  │
│  └───────────────────────────────────────────────────────┘  │
└────────────────────────┬────────────────────────────────────┘
                         │
                         ▼
┌─────────────────────────────────────────────────────────────┐
│ LLMClient::diagnose()                                        │
│  • Route based on config.ai.provider:                       │
│    - "local" → OllamaClient                                 │
│    - "anthropic" → AnthropicClient                          │
│    - "openai" → OpenAIClient                                │
│    - "hybrid" → Try local first, fallback to cloud          │
└────────────────────────┬────────────────────────────────────┘
                         │
        ┌────────────────┼────────────────┐
        │                │                │
        ▼                ▼                ▼
┌──────────────┐  ┌──────────────┐  ┌──────────────┐
│ OllamaClient │  │AnthropicClient│ │ OpenAIClient │
│              │  │              │  │              │
│ POST         │  │ POST         │  │ POST         │
│ localhost:   │  │ api.anthropic│  │ api.openai.  │
│ 11434/api/   │  │ .com/v1/     │  │ com/v1/chat/ │
│ generate     │  │ messages     │  │ completions  │
└──────┬───────┘  └──────┬───────┘  └──────┬───────┘
       │                 │                 │
       └─────────────────┼─────────────────┘
                         │
                         ▼ (LLM Response)
┌─────────────────────────────────────────────────────────────┐
│ parse_diagnosis()                                            │
│  • Extract structured data from LLM response                │
│  • Parse solutions (markdown to structured format)          │
│  ┌───────────────────────────────────────────────────────┐  │
│  │ Diagnosis {                                           │  │
│  │   issue: "SSH key not configured",                    │  │
│  │   causes: vec![                                       │  │
│  │     "SSH key missing",                                │  │
│  │     "Incorrect permissions",                          │  │
│  │   ],                                                  │  │
│  │   solutions: vec![                                    │  │
│  │     Solution {                                        │  │
│  │       description: "Check SSH key exists",            │  │
│  │       command: "ls -la ~/.ssh/id_rsa",                │  │
│  │       confidence: 0.9,                                │  │
│  │     },                                                │  │
│  │     ...                                               │  │
│  │   ],                                                  │  │
│  │ }                                                     │  │
│  └───────────────────────────────────────────────────────┘  │
└────────────────────────┬────────────────────────────────────┘
                         │
                         ▼
┌─────────────────────────────────────────────────────────────┐
│ present_suggestions()                                        │
│  • Render diagnosis in terminal                             │
│  • Interactive menu (dialoguer)                             │
│  • User selects solution                                    │
│  ┌───────────────────────────────────────────────────────┐  │
│  │ 🤖 AI Diagnosis                                       │  │
│  │                                                       │  │
│  │ Issue: SSH key not configured                         │  │
│  │                                                       │  │
│  │ Solutions:                                            │  │
│  │ [1] Check SSH key exists                              │  │
│  │ [2] Start SSH agent                                   │  │
│  │ [3] Fix key permissions                               │  │
│  │                                                       │  │
│  │ Choice [1-3]:                                         │  │
│  └───────────────────────────────────────────────────────┘  │
└────────────────────────┬────────────────────────────────────┘
                         │
                         ▼
┌─────────────────────────────────────────────────────────────┐
│ execute_solution() (Optional)                                │
│  • User confirms execution                                  │
│  • Run suggested command via CommandExecutor                │
│  • Report result                                            │
└─────────────────────────────────────────────────────────────┘
```

---

## API Specifications

### LLMClient Trait

```rust
/// Abstraction over different LLM providers
#[async_trait::async_trait]
pub trait LLMClient: Send + Sync {
    /// Diagnose a command execution error
    async fn diagnose(&self, context: ErrorContext) -> Result<Diagnosis>;

    /// Suggest next commands based on history
    async fn suggest_next_commands(
        &self,
        history: &[HistoryEntry],
        context: &CommandContext,
    ) -> Result<Vec<CommandSuggestion>>;

    /// Generate command from natural language
    async fn generate_command(&self, nl_query: &str) -> Result<GeneratedCommand>;

    /// Get provider name
    fn provider_name(&self) -> &str;

    /// Estimate cost of request (in USD)
    fn estimate_cost(&self, tokens: usize) -> f64;
}

/// Error context for diagnosis
#[derive(Debug, Clone, Serialize)]
pub struct ErrorContext {
    pub command_name: String,
    pub args: Vec<String>,
    pub exit_code: i32,
    pub stdout: String,
    pub stderr: String,
    pub working_dir: PathBuf,
    pub environment: HashMap<String, String>,
    pub recent_history: Vec<HistoryEntry>,
    pub timestamp: DateTime<Utc>,
}

/// Diagnosis result from LLM
#[derive(Debug, Clone, Deserialize)]
pub struct Diagnosis {
    pub issue: String,
    pub causes: Vec<String>,
    pub solutions: Vec<Solution>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Solution {
    pub description: String,
    pub command: Option<String>,
    pub confidence: f64,
    pub docs_link: Option<String>,
}

/// Command context for suggestions
#[derive(Debug, Clone)]
pub struct CommandContext {
    pub working_dir: PathBuf,
    pub git_branch: Option<String>,
    pub time_of_day: chrono::NaiveTime,
    pub day_of_week: chrono::Weekday,
}

/// Command suggestion
#[derive(Debug, Clone)]
pub struct CommandSuggestion {
    pub command: String,
    pub confidence: f64,
    pub reason: SuggestionReason,
    pub last_run: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone)]
pub enum SuggestionReason {
    Sequence { follows: Vec<String> },
    Temporal { pattern: String },
    Contextual { context: String },
    Combined,
}

/// Generated command from natural language
#[derive(Debug, Clone)]
pub struct GeneratedCommand {
    pub command: String,
    pub confidence: f64,
    pub explanation: String,
    pub alternatives: Vec<String>,
}
```

### Ollama Client Implementation

```rust
use reqwest::Client as HttpClient;
use serde::{Deserialize, Serialize};

pub struct OllamaClient {
    http_client: HttpClient,
    base_url: String,
    model: String,
    timeout: Duration,
}

impl OllamaClient {
    pub fn new(base_url: String, model: String) -> Result<Self> {
        Ok(Self {
            http_client: HttpClient::builder()
                .timeout(Duration::from_secs(60))
                .build()?,
            base_url,
            model,
            timeout: Duration::from_secs(60),
        })
    }

    /// Test connection to Ollama server
    pub async fn health_check(&self) -> Result<bool> {
        let url = format!("{}/api/tags", self.base_url);
        let response = self.http_client.get(&url).send().await?;
        Ok(response.status().is_success())
    }
}

#[async_trait::async_trait]
impl LLMClient for OllamaClient {
    async fn diagnose(&self, context: ErrorContext) -> Result<Diagnosis> {
        // Build prompt
        let prompt = format!(
            r#"You are a command-line expert. Analyze this error:

Command: {}
Exit Code: {}
Error Output:
{}

Provide:
1. Root cause (one line)
2. 2-3 likely causes
3. 2-3 specific solutions with commands

Format your response as JSON:
{{
  "issue": "...",
  "causes": ["...", "..."],
  "solutions": [
    {{
      "description": "...",
      "command": "...",
      "confidence": 0.9
    }}
  ]
}}"#,
            context.command_name, context.exit_code, context.stderr
        );

        // Ollama API request
        #[derive(Serialize)]
        struct OllamaRequest {
            model: String,
            prompt: String,
            stream: bool,
        }

        #[derive(Deserialize)]
        struct OllamaResponse {
            response: String,
        }

        let request = OllamaRequest {
            model: self.model.clone(),
            prompt,
            stream: false,
        };

        let url = format!("{}/api/generate", self.base_url);
        let response = self
            .http_client
            .post(&url)
            .json(&request)
            .timeout(self.timeout)
            .send()
            .await
            .context("Failed to send request to Ollama")?;

        if !response.status().is_success() {
            anyhow::bail!("Ollama request failed: {}", response.status());
        }

        let ollama_response: OllamaResponse = response.json().await?;

        // Parse JSON from LLM response
        let diagnosis: Diagnosis = serde_json::from_str(&ollama_response.response)
            .context("Failed to parse diagnosis from Ollama response")?;

        Ok(diagnosis)
    }

    async fn suggest_next_commands(
        &self,
        history: &[HistoryEntry],
        context: &CommandContext,
    ) -> Result<Vec<CommandSuggestion>> {
        // Similar implementation...
        todo!("Implement history-based suggestions")
    }

    async fn generate_command(&self, nl_query: &str) -> Result<GeneratedCommand> {
        // Similar implementation...
        todo!("Implement NL command generation")
    }

    fn provider_name(&self) -> &str {
        "ollama"
    }

    fn estimate_cost(&self, _tokens: usize) -> f64 {
        0.0 // Local models are free
    }
}
```

### Anthropic Client Implementation

```rust
pub struct AnthropicClient {
    http_client: HttpClient,
    api_key: SecretString,
    model: String,
    max_tokens: usize,
}

impl AnthropicClient {
    pub fn new(api_key: SecretString, model: String) -> Result<Self> {
        Ok(Self {
            http_client: HttpClient::builder()
                .timeout(Duration::from_secs(30))
                .build()?,
            api_key,
            model,
            max_tokens: 2048,
        })
    }
}

#[async_trait::async_trait]
impl LLMClient for AnthropicClient {
    async fn diagnose(&self, context: ErrorContext) -> Result<Diagnosis> {
        #[derive(Serialize)]
        struct AnthropicRequest {
            model: String,
            max_tokens: usize,
            messages: Vec<Message>,
        }

        #[derive(Serialize)]
        struct Message {
            role: String,
            content: String,
        }

        #[derive(Deserialize)]
        struct AnthropicResponse {
            content: Vec<ContentBlock>,
        }

        #[derive(Deserialize)]
        struct ContentBlock {
            text: String,
        }

        let prompt = format!(
            r#"Analyze this command error and provide a diagnosis:

Command: {}
Exit Code: {}
Error: {}

Return JSON with: issue, causes (array), solutions (array with description, command, confidence)"#,
            context.command_name, context.exit_code, context.stderr
        );

        let request = AnthropicRequest {
            model: self.model.clone(),
            max_tokens: self.max_tokens,
            messages: vec![Message {
                role: "user".to_string(),
                content: prompt,
            }],
        };

        let response = self
            .http_client
            .post("https://api.anthropic.com/v1/messages")
            .header("x-api-key", self.api_key.expose_secret())
            .header("anthropic-version", "2023-06-01")
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            anyhow::bail!("Anthropic API error: {}", response.status());
        }

        let api_response: AnthropicResponse = response.json().await?;
        let text = &api_response.content[0].text;

        let diagnosis: Diagnosis = serde_json::from_str(text)
            .context("Failed to parse diagnosis from Anthropic response")?;

        Ok(diagnosis)
    }

    fn provider_name(&self) -> &str {
        "anthropic"
    }

    fn estimate_cost(&self, tokens: usize) -> f64 {
        // Claude 3.5 Sonnet pricing (as of 2024)
        let input_cost_per_1m = 3.0; // $3 per 1M input tokens
        let output_cost_per_1m = 15.0; // $15 per 1M output tokens

        let input_tokens = tokens;
        let output_tokens = self.max_tokens;

        (input_tokens as f64 / 1_000_000.0) * input_cost_per_1m
            + (output_tokens as f64 / 1_000_000.0) * output_cost_per_1m
    }
}
```

---

## Code Examples

### Privacy Filter Implementation

```rust
// src/ai/privacy_filter.rs

use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::HashMap;

/// Sensitive data patterns
static SENSITIVE_PATTERNS: Lazy<Vec<Regex>> = Lazy::new(|| {
    vec![
        // API Keys
        Regex::new(r"(?i)(api[_-]?key|apikey)[\s:=]+['\"]?([a-zA-Z0-9_-]{20,})").unwrap(),
        // Tokens
        Regex::new(r"(?i)(token|auth[_-]?token)[\s:=]+['\"]?([a-zA-Z0-9_-]{20,})").unwrap(),
        // Passwords
        Regex::new(r"(?i)(password|passwd|pwd)[\s:=]+['\"]?([^\s'\"]{8,})").unwrap(),
        // AWS credentials
        Regex::new(r"AKIA[0-9A-Z]{16}").unwrap(),
        // GitHub tokens
        Regex::new(r"ghp_[a-zA-Z0-9]{36}").unwrap(),
        // Private keys
        Regex::new(r"-----BEGIN [A-Z ]+ PRIVATE KEY-----").unwrap(),
        // JWT tokens
        Regex::new(r"eyJ[a-zA-Z0-9_-]+\.eyJ[a-zA-Z0-9_-]+\.[a-zA-Z0-9_-]+").unwrap(),
        // Credit cards
        Regex::new(r"\b\d{4}[\s-]?\d{4}[\s-]?\d{4}[\s-]?\d{4}\b").unwrap(),
        // Email addresses (optional, configurable)
        Regex::new(r"\b[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Z|a-z]{2,}\b").unwrap(),
    ]
});

/// Sensitive environment variable names
const SENSITIVE_ENV_VARS: &[&str] = &[
    "API_KEY",
    "SECRET",
    "TOKEN",
    "PASSWORD",
    "PASSWD",
    "AWS_ACCESS_KEY_ID",
    "AWS_SECRET_ACCESS_KEY",
    "GITHUB_TOKEN",
    "ANTHROPIC_API_KEY",
    "OPENAI_API_KEY",
];

pub struct PrivacyFilter {
    config: FilterConfig,
}

pub struct FilterConfig {
    pub filter_secrets: bool,
    pub filter_env_vars: bool,
    pub filter_file_paths: bool,
    pub custom_patterns: Vec<Regex>,
}

impl Default for FilterConfig {
    fn default() -> Self {
        Self {
            filter_secrets: true,
            filter_env_vars: true,
            filter_file_paths: false,
            custom_patterns: Vec::new(),
        }
    }
}

impl PrivacyFilter {
    pub fn new(config: FilterConfig) -> Self {
        Self { config }
    }

    /// Filter sensitive data from text
    pub fn filter_text(&self, text: &str) -> String {
        if !self.config.filter_secrets {
            return text.to_string();
        }

        let mut filtered = text.to_string();

        // Apply built-in patterns
        for pattern in SENSITIVE_PATTERNS.iter() {
            filtered = pattern.replace_all(&filtered, "[FILTERED]").to_string();
        }

        // Apply custom patterns
        for pattern in &self.config.custom_patterns {
            filtered = pattern.replace_all(&filtered, "[FILTERED]").to_string();
        }

        filtered
    }

    /// Filter environment variables
    pub fn filter_env(&self, env: &HashMap<String, String>) -> HashMap<String, String> {
        if !self.config.filter_env_vars {
            return env.clone();
        }

        env.iter()
            .map(|(k, v)| {
                let filtered_value = if self.is_sensitive_env_var(k) {
                    "[FILTERED]".to_string()
                } else {
                    self.filter_text(v)
                };
                (k.clone(), filtered_value)
            })
            .collect()
    }

    /// Check if environment variable name is sensitive
    fn is_sensitive_env_var(&self, key: &str) -> bool {
        let upper = key.to_uppercase();
        SENSITIVE_ENV_VARS
            .iter()
            .any(|pattern| upper.contains(pattern))
    }

    /// Filter file paths (replace home directory with ~)
    pub fn filter_path(&self, path: &str) -> String {
        if !self.config.filter_file_paths {
            return path.to_string();
        }

        if let Some(home) = dirs::home_dir() {
            let home_str = home.to_string_lossy();
            path.replace(home_str.as_ref(), "~")
        } else {
            path.to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filter_api_key() {
        let filter = PrivacyFilter::new(FilterConfig::default());
        let text = "API_KEY=sk-1234567890abcdefghijklmnopqrstuvwxyz";
        let filtered = filter.filter_text(text);
        assert_eq!(filtered, "[FILTERED]");
    }

    #[test]
    fn test_filter_github_token() {
        let filter = PrivacyFilter::new(FilterConfig::default());
        let text = "token: ghp_abcdefghijklmnopqrstuvwxyz123456";
        let filtered = filter.filter_text(text);
        assert_eq!(filtered, "token: [FILTERED]");
    }

    #[test]
    fn test_filter_env_vars() {
        let filter = PrivacyFilter::new(FilterConfig::default());
        let mut env = HashMap::new();
        env.insert("PATH".to_string(), "/usr/bin".to_string());
        env.insert("API_KEY".to_string(), "secret123".to_string());

        let filtered = filter.filter_env(&env);
        assert_eq!(filtered.get("PATH").unwrap(), "/usr/bin");
        assert_eq!(filtered.get("API_KEY").unwrap(), "[FILTERED]");
    }
}
```

---

## Database Schema

### AI-Specific Tables

```sql
-- Pattern cache for history-based suggestions
CREATE TABLE IF NOT EXISTS pattern_cache (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    sequence TEXT NOT NULL,           -- JSON array: ["build", "test"]
    next_command TEXT NOT NULL,
    frequency INTEGER NOT NULL DEFAULT 1,
    last_seen INTEGER NOT NULL,       -- Unix timestamp in milliseconds
    confidence REAL NOT NULL DEFAULT 0.0,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL,
    UNIQUE(sequence, next_command)
);

CREATE INDEX idx_pattern_cache_sequence ON pattern_cache(sequence);
CREATE INDEX idx_pattern_cache_frequency ON pattern_cache(frequency DESC);

-- LLM query log for analytics and cost tracking
CREATE TABLE IF NOT EXISTS llm_queries (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    provider TEXT NOT NULL,           -- "ollama", "anthropic", "openai"
    model TEXT NOT NULL,
    query_type TEXT NOT NULL,         -- "diagnosis", "suggestion", "generation"
    input_tokens INTEGER,
    output_tokens INTEGER,
    estimated_cost REAL,
    latency_ms INTEGER,
    success BOOLEAN NOT NULL,
    error_message TEXT,
    created_at INTEGER NOT NULL
);

CREATE INDEX idx_llm_queries_provider ON llm_queries(provider);
CREATE INDEX idx_llm_queries_created_at ON llm_queries(created_at);

-- User feedback for suggestion quality
CREATE TABLE IF NOT EXISTS suggestion_feedback (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    suggestion_id TEXT NOT NULL,      -- UUID
    command_suggested TEXT NOT NULL,
    accepted BOOLEAN NOT NULL,
    executed BOOLEAN NOT NULL,
    success BOOLEAN,
    feedback_score INTEGER,           -- 1-5 rating (optional)
    created_at INTEGER NOT NULL
);

CREATE INDEX idx_suggestion_feedback_command ON suggestion_feedback(command_suggested);
```

---

## Configuration Schema

### TOML Configuration

```toml
# AI Integration Configuration

[config.ai]
enabled = true
provider = "local"              # "local", "anthropic", "openai", "hybrid"
model = "llama3:8b"             # Model name

# Hybrid mode: try local first, fallback to cloud
[config.ai.hybrid]
local_model = "llama3:8b"
cloud_provider = "anthropic"
cloud_model = "claude-3.5-sonnet"
fallback_on_error = true
fallback_timeout_ms = 5000

# Privacy settings
[config.ai.privacy]
send_environment_vars = false
send_working_dir = true
send_command_args = true
filter_secrets = true
filter_file_paths = true
allow_cloud = true
cloud_requires_confirmation = true

# Sensitive data patterns (custom)
sensitive_patterns = [
    "CUSTOM_SECRET_.*",
    "INTERNAL_TOKEN_.*",
]

# Error diagnosis settings
[config.ai.diagnosis]
enabled = true
min_exit_code = 1
max_context_lines = 50
include_recent_history = 5
interactive = true
auto_execute_safe_commands = false

# History-based suggestions
[config.ai.suggestions]
enabled = true
mode = "local"                  # "local", "cloud", "hybrid"
confidence_threshold = 0.5
max_suggestions = 3
proactive = true                # Show after command completion
sequence_length = 5
temporal_weight = 0.3
context_weight = 0.4
frequency_weight = 0.3

# Provider-specific settings
[config.ai.ollama]
base_url = "http://localhost:11434"
timeout_ms = 60000

[config.ai.anthropic]
api_key_source = "keychain"     # "keychain", "env", "config"
api_key_env_var = "ANTHROPIC_API_KEY"
max_tokens = 2048
temperature = 0.7
max_tokens_per_day = 100000     # Cost control

[config.ai.openai]
api_key_source = "keychain"
api_key_env_var = "OPENAI_API_KEY"
max_tokens = 2048
temperature = 0.7
max_tokens_per_day = 100000

# Per-command overrides
[commands.deploy]
cmd = "ssh user@server 'cd /app && git pull'"
ai_diagnosis = false            # Disable AI for this command
ai_suggestions = false

[commands.build]
cmd = "cargo build --release"
ai_diagnosis = true
ai_diagnosis_provider = "anthropic"  # Override global provider
```

### Rust Configuration Structs

```rust
// src/config/ai.rs

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AIConfig {
    pub enabled: bool,
    pub provider: AIProvider,
    pub model: String,

    #[serde(default)]
    pub hybrid: Option<HybridConfig>,

    #[serde(default)]
    pub privacy: PrivacyConfig,

    #[serde(default)]
    pub diagnosis: DiagnosisConfig,

    #[serde(default)]
    pub suggestions: SuggestionsConfig,

    #[serde(default)]
    pub ollama: Option<OllamaConfig>,

    #[serde(default)]
    pub anthropic: Option<AnthropicConfig>,

    #[serde(default)]
    pub openai: Option<OpenAIConfig>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum AIProvider {
    Local,
    Anthropic,
    OpenAI,
    Hybrid,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HybridConfig {
    pub local_model: String,
    pub cloud_provider: String,
    pub cloud_model: String,
    pub fallback_on_error: bool,
    pub fallback_timeout_ms: u64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PrivacyConfig {
    #[serde(default = "default_true")]
    pub send_environment_vars: bool,

    #[serde(default = "default_true")]
    pub send_working_dir: bool,

    #[serde(default = "default_true")]
    pub send_command_args: bool,

    #[serde(default = "default_true")]
    pub filter_secrets: bool,

    #[serde(default)]
    pub filter_file_paths: bool,

    #[serde(default = "default_true")]
    pub allow_cloud: bool,

    #[serde(default = "default_true")]
    pub cloud_requires_confirmation: bool,

    #[serde(default)]
    pub sensitive_patterns: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DiagnosisConfig {
    #[serde(default = "default_true")]
    pub enabled: bool,

    #[serde(default = "default_min_exit_code")]
    pub min_exit_code: i32,

    #[serde(default = "default_max_context_lines")]
    pub max_context_lines: usize,

    #[serde(default = "default_recent_history")]
    pub include_recent_history: usize,

    #[serde(default = "default_true")]
    pub interactive: bool,

    #[serde(default)]
    pub auto_execute_safe_commands: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SuggestionsConfig {
    #[serde(default = "default_true")]
    pub enabled: bool,

    #[serde(default)]
    pub mode: AIProvider,

    #[serde(default = "default_confidence_threshold")]
    pub confidence_threshold: f64,

    #[serde(default = "default_max_suggestions")]
    pub max_suggestions: usize,

    #[serde(default = "default_true")]
    pub proactive: bool,

    #[serde(default = "default_sequence_length")]
    pub sequence_length: usize,

    #[serde(default = "default_weight")]
    pub temporal_weight: f64,

    #[serde(default = "default_weight")]
    pub context_weight: f64,

    #[serde(default = "default_weight")]
    pub frequency_weight: f64,
}

// Default value functions
fn default_true() -> bool {
    true
}

fn default_min_exit_code() -> i32 {
    1
}

fn default_max_context_lines() -> usize {
    50
}

fn default_recent_history() -> usize {
    5
}

fn default_confidence_threshold() -> f64 {
    0.5
}

fn default_max_suggestions() -> usize {
    3
}

fn default_sequence_length() -> usize {
    5
}

fn default_weight() -> f64 {
    0.33
}
```

---

## Testing Strategy

### Unit Tests

```rust
// tests/unit/ai/privacy_filter.rs

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filter_various_secrets() {
        let filter = PrivacyFilter::new(FilterConfig::default());

        let test_cases = vec![
            ("API_KEY=sk-1234567890", "[FILTERED]"),
            ("token: ghp_abcdefghij", "token: [FILTERED]"),
            ("AKIA1234567890ABCDEF", "[FILTERED]"),
            (
                "-----BEGIN RSA PRIVATE KEY-----",
                "[FILTERED]",
            ),
            (
                "user@example.com",
                "[FILTERED]",
            ),
        ];

        for (input, expected) in test_cases {
            assert_eq!(filter.filter_text(input), expected);
        }
    }
}
```

### Integration Tests

```rust
// tests/integration/ai_diagnosis.rs

#[tokio::test]
async fn test_error_diagnosis_ollama() {
    // Start mock Ollama server
    let mock_server = MockOllamaServer::start().await;

    // Configure client
    let client = OllamaClient::new(
        mock_server.url(),
        "llama3:8b".to_string(),
    )
    .unwrap();

    // Create error context
    let context = ErrorContext {
        command_name: "deploy".to_string(),
        args: vec![],
        exit_code: 1,
        stdout: String::new(),
        stderr: "Permission denied (publickey)".to_string(),
        working_dir: PathBuf::from("/test"),
        environment: HashMap::new(),
        recent_history: vec![],
        timestamp: Utc::now(),
    };

    // Diagnose
    let diagnosis = client.diagnose(context).await.unwrap();

    // Assertions
    assert!(!diagnosis.issue.is_empty());
    assert!(!diagnosis.solutions.is_empty());
    assert!(diagnosis.solutions[0].confidence > 0.5);
}
```

### End-to-End Tests

```bash
#!/bin/bash
# tests/e2e/ai_diagnosis.sh

# Start Ollama (if not running)
ollama serve &
OLLAMA_PID=$!
sleep 2

# Pull model
ollama pull llama3:8b

# Configure cmdrun for AI
cat > /tmp/cmdrun-test.toml <<EOF
[config.ai]
enabled = true
provider = "local"
model = "llama3:8b"

[commands.failing-command]
cmd = "exit 1"
EOF

# Run failing command
cmdrun --config /tmp/cmdrun-test.toml run failing-command 2>&1 | tee /tmp/output.txt

# Check for AI diagnosis
if grep -q "🤖 AI Diagnosis" /tmp/output.txt; then
    echo "✓ AI diagnosis triggered"
else
    echo "✗ AI diagnosis NOT triggered"
    exit 1
fi

# Cleanup
kill $OLLAMA_PID
```

---

## Performance Benchmarks

### Target Metrics

| Operation | Target | Acceptable | Critical |
|-----------|--------|------------|----------|
| Error diagnosis (local) | < 2s | < 5s | > 10s |
| Error diagnosis (cloud) | < 1s | < 3s | > 5s |
| History analysis | < 50ms | < 100ms | > 200ms |
| Pattern cache rebuild | < 100ms | < 200ms | > 500ms |
| Suggestion display | < 10ms | < 20ms | > 50ms |

### Benchmark Code

```rust
// benches/ai_performance.rs

use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn benchmark_privacy_filter(c: &mut Criterion) {
    let filter = PrivacyFilter::new(FilterConfig::default());
    let text = "API_KEY=sk-1234567890 and token=ghp_abcdefghij".repeat(100);

    c.bench_function("privacy_filter_100_lines", |b| {
        b.iter(|| filter.filter_text(black_box(&text)))
    });
}

fn benchmark_pattern_analysis(c: &mut Criterion) {
    let analyzer = HistoryAnalyzer::new(/* ... */);
    let history = generate_test_history(1000);

    c.bench_function("pattern_analysis_1000_entries", |b| {
        b.iter(|| {
            analyzer.analyze_patterns(
                black_box(&history),
                black_box(&CommandContext::default()),
            )
        })
    });
}

criterion_group!(
    benches,
    benchmark_privacy_filter,
    benchmark_pattern_analysis
);
criterion_main!(benches);
```

---

## Appendix

### Prompt Templates

```rust
// src/ai/prompt_templates.rs

pub const ERROR_DIAGNOSIS_PROMPT: &str = r#"
You are an expert command-line assistant specializing in diagnosing and fixing command execution errors.

## Context
- Command: {command_name}
- Arguments: {args}
- Exit Code: {exit_code}
- Working Directory: {working_dir}
- Recent Commands: {recent_history}

## Error Output
{stderr}

## Your Task
Analyze this error and provide:

1. **Issue**: One-line root cause summary
2. **Causes**: 2-3 most likely causes
3. **Solutions**: 2-3 specific, actionable solutions

For each solution, provide:
- Description (what it does)
- Command (if applicable, copy-paste ready)
- Confidence (0.0-1.0)

## Output Format (JSON)
```json
{
  "issue": "SSH authentication failed",
  "causes": [
    "SSH key not configured",
    "Incorrect file permissions on key"
  ],
  "solutions": [
    {
      "description": "Check if SSH key exists",
      "command": "ls -la ~/.ssh/id_rsa",
      "confidence": 0.9
    },
    {
      "description": "Fix SSH key permissions",
      "command": "chmod 600 ~/.ssh/id_rsa",
      "confidence": 0.8
    }
  ]
}
```

Be concise, technical, and provide copy-paste ready commands.
"#;

pub const SUGGESTION_PROMPT: &str = r#"
You are a command-line assistant that predicts the next likely command based on execution history.

## Recent Command History
{history}

## Current Context
- Working Directory: {working_dir}
- Git Branch: {git_branch}
- Time: {time_of_day}
- Day: {day_of_week}

## Your Task
Predict the 3 most likely next commands the user will run.

For each suggestion:
- Command name
- Confidence (0.0-1.0)
- Reason (why this is likely)

## Output Format (JSON)
```json
{
  "suggestions": [
    {
      "command": "test",
      "confidence": 0.85,
      "reason": "Usually follows 'build' in this project"
    }
  ]
}
```
"#;
```

---

**End of Technical Specification**
