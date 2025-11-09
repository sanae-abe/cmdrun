# AI Integration Proposal for cmdrun

## Status

**Proposal Status**: Draft
**Created**: 2025-11-09
**Target Phase**: Phase 2 (Feature Enhancement)
**Priority**: High

## Executive Summary

This proposal outlines AI-powered features for cmdrun to enhance developer productivity through intelligent error diagnosis and history-based command suggestions. The implementation leverages cmdrun's existing plugin system and SQLite-based history storage, ensuring modularity and security.

## Table of Contents

- [Motivation](#motivation)
- [Priority Features](#priority-features)
  - [1. Smart Error Diagnosis](#1-smart-error-diagnosis)
  - [2. History-Based Command Suggestions](#2-history-based-command-suggestions)
- [Additional Features](#additional-features)
- [Technical Architecture](#technical-architecture)
- [Implementation Plan](#implementation-plan)
- [Security and Privacy](#security-and-privacy)
- [Alternative Approaches](#alternative-approaches)
- [References](#references)

---

## Motivation

### Current Pain Points

1. **Error Resolution Time**: Developers spend significant time diagnosing command failures and searching for solutions
2. **Context Switching**: Moving between terminal, documentation, and search engines breaks flow
3. **Repetitive Patterns**: Common command sequences are not recognized or suggested
4. **Learning Curve**: New users struggle to discover optimal command patterns

### Benefits of AI Integration

- **Reduced Time-to-Resolution**: AI-powered error diagnosis provides immediate, contextualized solutions
- **Improved Productivity**: History-based suggestions reduce repetitive typing and command lookups
- **Enhanced Developer Experience**: Intelligent assistance lowers barriers to entry
- **Data-Driven Insights**: Learn from execution patterns to optimize workflows

### Alignment with cmdrun's Philosophy

✅ **Performance**: Plugin-based architecture ensures zero overhead when disabled
✅ **Security**: Privacy-first design with local-first and opt-in cloud options
✅ **Cross-platform**: LLM integration works on all supported platforms
✅ **Developer Experience**: Seamless integration via existing CLI and TUI interfaces

---

## Priority Features

### 1. Smart Error Diagnosis

#### Overview

When a command fails, automatically analyze the error and provide actionable solutions using LLM-powered diagnostics.

#### User Experience

```bash
$ cmdrun run deploy
Error: Command 'deploy' failed with exit code 1
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
🤖 AI Diagnosis (powered by claude-3.5-sonnet)

Issue: Authentication failure - SSH key not found

Likely Causes:
  1. SSH key not configured in ~/.ssh/config
  2. SSH agent not running
  3. Incorrect key permissions

Suggested Solutions:
  ① Check SSH key exists:
     ls -la ~/.ssh/id_rsa

  ② Start SSH agent and add key:
     eval $(ssh-agent -s)
     ssh-add ~/.ssh/id_rsa

  ③ Fix key permissions:
     chmod 600 ~/.ssh/id_rsa

Would you like to:
  [1] Run suggested command ①
  [2] Run all suggested commands in sequence
  [3] Show full error details
  [4] Disable AI diagnosis for this session
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Choice [1-4]:
```

#### Key Features

**Automatic Triggering**:
- Activated on command failure (non-zero exit code)
- Configurable threshold (e.g., only for errors, not warnings)
- Respects user preferences (can be disabled globally or per-command)

**Contextual Analysis**:
- Captures stdout, stderr, exit code
- Includes command name, arguments, environment
- Analyzes working directory and related files
- Reviews recent command history for context

**Intelligent Suggestions**:
- Ranked by likelihood of success
- Executable commands (not just explanations)
- Links to relevant documentation
- Similar error resolution from history

**Privacy Controls**:
- Local-first: Use local LLMs (Ollama, llama.cpp) by default
- Opt-in cloud: Claude, GPT-4, etc. for enhanced accuracy
- Sensitive data filtering (API keys, passwords, secrets)
- Configurable data sharing policies

#### Configuration

```toml
# commands.toml
[config.ai]
enabled = true
provider = "local"  # or "anthropic", "openai", "custom"
model = "llama3:8b"  # Local model via Ollama

# Privacy settings
[config.ai.privacy]
send_environment_vars = false  # Don't send env vars to LLM
send_working_dir = true        # Send working directory path
filter_secrets = true          # Automatically filter secrets
max_context_lines = 50         # Limit error context size

# Trigger settings
[config.ai.triggers]
on_error = true
on_warning = false
min_exit_code = 1

# Per-command override
[commands.deploy]
cmd = "ssh user@server 'cd /app && git pull'"
ai_diagnosis = false  # Disable AI for this command
```

#### Implementation Details

**Plugin Architecture**:
```rust
// src/plugin/builtin/ai_diagnostics.rs
pub struct AIDiagnosticsPlugin {
    config: AIConfig,
    llm_client: Box<dyn LLMClient>,
    history_analyzer: HistoryAnalyzer,
}

impl Plugin for AIDiagnosticsPlugin {
    fn on_error(&self, context: &PluginContext, error: &CmdrunError) -> Result<()> {
        // 1. Collect error context
        let error_context = self.collect_context(context, error)?;

        // 2. Check privacy filters
        let filtered_context = self.apply_privacy_filters(error_context)?;

        // 3. Query LLM for diagnosis
        let diagnosis = self.llm_client.diagnose(filtered_context).await?;

        // 4. Present interactive suggestions
        self.present_suggestions(diagnosis)?;

        Ok(())
    }
}
```

**LLM Client Abstraction**:
```rust
// src/ai/llm_client.rs
pub trait LLMClient: Send + Sync {
    async fn diagnose(&self, context: ErrorContext) -> Result<Diagnosis>;
    async fn suggest_next_command(&self, history: &[HistoryEntry]) -> Result<Vec<CommandSuggestion>>;
}

// Implementations
pub struct OllamaClient { /* ... */ }
pub struct AnthropicClient { /* ... */ }
pub struct OpenAIClient { /* ... */ }
```

**Prompt Engineering**:
```rust
const ERROR_DIAGNOSIS_PROMPT: &str = r#"
You are a command-line expert assistant analyzing a failed command execution.

## Context
- Command: {command_name}
- Arguments: {args}
- Exit Code: {exit_code}
- Working Directory: {working_dir}
- Environment: {filtered_env}

## Error Output
{stderr}

## Your Task
1. Identify the root cause of the failure
2. Provide 2-3 specific, actionable solutions
3. Rank solutions by likelihood of success
4. Include copy-paste ready commands

## Output Format
Issue: [one-line summary]

Likely Causes:
1. [cause 1]
2. [cause 2]

Suggested Solutions:
① [solution 1 with command]
② [solution 2 with command]
"#;
```

---

### 2. History-Based Command Suggestions

#### Overview

Analyze command execution history to predict and suggest the next likely command, reducing repetitive typing and improving workflow efficiency.

#### User Experience

**Interactive Mode (TUI)**:
```
┌─ cmdrun (Interactive Mode) ─────────────────────────────────┐
│                                                              │
│ 🔍 Search: _                                                 │
│                                                              │
│ ┌─ Suggested Next Actions (AI-powered) ───────────────────┐ │
│ │ Based on your current workflow:                          │ │
│ │                                                           │ │
│ │ ⚡ test (85% confidence)                                  │ │
│ │    Usually run after 'build' in this project             │ │
│ │                                                           │ │
│ │ 📦 deploy-staging (60% confidence)                        │ │
│ │    Typical sequence: build → test → deploy-staging       │ │
│ │                                                           │ │
│ │ 🔍 check-logs (45% confidence)                            │ │
│ │    Often checked after deployment commands               │ │
│ └──────────────────────────────────────────────────────────┘ │
│                                                              │
│ ┌─ All Commands ──────────────────────────────────────────┐ │
│ │   build                                                  │ │
│ │ › test                                                   │ │
│ │   deploy-staging                                         │ │
│ │   deploy-production                                      │ │
│ └──────────────────────────────────────────────────────────┘ │
│                                                              │
│ [↑/↓] Navigate  [Enter] Execute  [Esc] Quit                 │
└──────────────────────────────────────────────────────────────┘
```

**CLI Mode**:
```bash
$ cmdrun suggest
🤖 Based on your recent activity, you might want to run:

  1. test (85% confidence)
     Last run: 2 minutes ago
     Typical after: build

  2. deploy-staging (60% confidence)
     Last run: 1 day ago
     Typical sequence: build → test → deploy-staging

  3. check-logs (45% confidence)
     Last run: 5 minutes ago
     Often follows deployment

Run a suggestion: cmdrun run test
Disable suggestions: cmdrun config set ai.suggestions false
```

**Proactive Suggestions**:
```bash
$ cmdrun run build
✓ Command 'build' completed successfully (2.3s)

💡 Suggestion: Run 'test' next? (You usually do this after 'build')
   [Y] Yes, run test
   [n] No, skip
   [d] Don't ask again for this sequence

Choice [Y/n/d]:
```

#### Key Features

**Pattern Recognition**:
- Sequence patterns (build → test → deploy)
- Temporal patterns (time-of-day, day-of-week)
- Contextual patterns (working directory, Git branch)
- Project-specific patterns

**Machine Learning Approach**:
- **Phase 1 (Simple)**: Markov chain-based prediction using command sequences
- **Phase 2 (Advanced)**: Fine-tuned LLM on user's command history
- **Phase 3 (Personalized)**: Reinforcement learning from user feedback

**Confidence Scoring**:
- Based on frequency of pattern occurrence
- Weighted by recency (recent patterns rank higher)
- Adjusted for context similarity
- User feedback incorporated

**Privacy-Preserving**:
- All analysis performed locally on SQLite database
- No command history sent to cloud by default
- Opt-in for cloud-enhanced suggestions
- Anonymization before any external transmission

#### Configuration

```toml
[config.ai.suggestions]
enabled = true
mode = "local"  # "local", "cloud", "hybrid"
confidence_threshold = 0.5  # Only show suggestions above 50%
max_suggestions = 3
proactive = true  # Show suggestions after command completion

[config.ai.suggestions.patterns]
sequence_length = 5  # Analyze last 5 commands
temporal_weight = 0.3  # How much to weight time patterns
context_weight = 0.4   # How much to weight context (dir, branch)
frequency_weight = 0.3 # How much to weight raw frequency
```

#### Implementation Details

**History Analyzer**:
```rust
// src/ai/history_analyzer.rs
pub struct HistoryAnalyzer {
    storage: Arc<HistoryStorage>,
    pattern_cache: Arc<RwLock<PatternCache>>,
}

impl HistoryAnalyzer {
    /// Predict next commands based on history
    pub async fn suggest_next_commands(
        &self,
        context: &CommandContext,
        max_suggestions: usize,
    ) -> Result<Vec<CommandSuggestion>> {
        // 1. Get recent history
        let recent_history = self.storage.get_recent(50)?;

        // 2. Analyze patterns
        let patterns = self.analyze_patterns(&recent_history, context)?;

        // 3. Score and rank
        let mut suggestions = self.score_patterns(patterns)?;
        suggestions.sort_by(|a, b| b.confidence.partial_cmp(&a.confidence).unwrap());

        // 4. Filter by confidence threshold
        let threshold = self.config.confidence_threshold;
        suggestions.retain(|s| s.confidence >= threshold);

        Ok(suggestions.into_iter().take(max_suggestions).collect())
    }

    /// Analyze command sequence patterns
    fn analyze_patterns(
        &self,
        history: &[HistoryEntry],
        context: &CommandContext,
    ) -> Result<Vec<Pattern>> {
        let mut patterns = Vec::new();

        // Sequence patterns (Markov chain)
        patterns.extend(self.find_sequence_patterns(history)?);

        // Temporal patterns
        patterns.extend(self.find_temporal_patterns(history, context)?);

        // Contextual patterns (working dir, Git branch)
        patterns.extend(self.find_contextual_patterns(history, context)?);

        Ok(patterns)
    }
}

#[derive(Debug)]
pub struct CommandSuggestion {
    pub command: String,
    pub confidence: f64,
    pub reason: SuggestionReason,
    pub last_run: Option<DateTime<Utc>>,
}

#[derive(Debug)]
pub enum SuggestionReason {
    Sequence { follows: Vec<String> },
    Temporal { pattern: String },
    Contextual { context: String },
    Combined,
}
```

**Pattern Cache**:
```rust
// src/ai/pattern_cache.rs
pub struct PatternCache {
    sequences: HashMap<Vec<String>, SequencePattern>,
    last_updated: DateTime<Utc>,
}

#[derive(Debug)]
pub struct SequencePattern {
    sequence: Vec<String>,
    next_command: String,
    frequency: usize,
    last_seen: DateTime<Utc>,
    confidence: f64,
}

impl PatternCache {
    /// Rebuild cache from history database
    pub fn rebuild(&mut self, history: &[HistoryEntry]) -> Result<()> {
        self.sequences.clear();

        // Sliding window over history
        for window in history.windows(self.config.sequence_length + 1) {
            let sequence: Vec<String> = window[..window.len() - 1]
                .iter()
                .map(|e| e.command.clone())
                .collect();
            let next = window.last().unwrap().command.clone();

            self.sequences.entry(sequence.clone())
                .and_modify(|p| {
                    p.frequency += 1;
                    p.last_seen = window.last().unwrap().start_time_as_datetime();
                })
                .or_insert_with(|| SequencePattern {
                    sequence,
                    next_command: next,
                    frequency: 1,
                    last_seen: window.last().unwrap().start_time_as_datetime(),
                    confidence: 0.0,
                });
        }

        // Calculate confidence scores
        self.calculate_confidence();
        self.last_updated = Utc::now();

        Ok(())
    }
}
```

---

## Additional Features

### 3. Natural Language Command Generation

**Future Enhancement**: Convert natural language to cmdrun commands.

```bash
$ cmdrun ai "deploy to production"
🤖 Suggested command: deploy-production

Confidence: High
Reasoning: Found exact match for production deployment

Run this command? [Y/n]:
```

### 4. Interactive Smart Completion

**Future Enhancement**: AI-powered tab completion with context awareness.

```bash
$ cmdrun run dep<TAB>
🤖 Suggestions:
   deploy-staging   (you usually run this at 2pm)
   deploy-production (requires confirmation)

$ cmdrun run deploy-<TAB>
🤖 Which environment?
   staging     (safe, can rollback)
   production  (requires: test passed, staging deployed)
```

---

## Technical Architecture

### System Design

```
┌──────────────────────────────────────────────────────────────┐
│                     cmdrun Core                               │
├──────────────────────────────────────────────────────────────┤
│  CLI Commands  │  TUI Interface  │  Command Executor         │
└────────┬─────────────────┬────────────────┬──────────────────┘
         │                 │                │
         │                 │                │
    ┌────▼─────────────────▼────────────────▼──────────────────┐
    │              Plugin System (Existing)                     │
    │  • Hook system (pre/post execute, on_error)              │
    │  • Thread-safe plugin management                         │
    │  • Dynamic library loading                               │
    └────┬──────────────────────────────────────────────────────┘
         │
         │
    ┌────▼──────────────────────────────────────────────────────┐
    │         AI Integration Plugin (New)                       │
    ├───────────────────────────────────────────────────────────┤
    │                                                            │
    │  ┌─────────────────────┐  ┌──────────────────────────┐   │
    │  │ Error Diagnosis     │  │ History-Based Suggestions │   │
    │  │  • Context capture  │  │  • Pattern recognition    │   │
    │  │  • Privacy filter   │  │  • Confidence scoring     │   │
    │  │  • LLM query        │  │  • Cache management       │   │
    │  │  • UI presentation  │  │  • Proactive suggestions  │   │
    │  └─────────┬───────────┘  └──────────┬───────────────┘   │
    │            │                          │                   │
    │            └────────┬─────────────────┘                   │
    │                     │                                     │
    │            ┌────────▼─────────────────────────────┐       │
    │            │    LLM Client Abstraction            │       │
    │            │  • Local (Ollama, llama.cpp)         │       │
    │            │  • Cloud (Anthropic, OpenAI)         │       │
    │            │  • Custom (user-provided endpoint)   │       │
    │            └────────┬─────────────────────────────┘       │
    │                     │                                     │
    └─────────────────────┼─────────────────────────────────────┘
                          │
         ┌────────────────┼────────────────┐
         │                │                │
    ┌────▼─────┐   ┌─────▼──────┐   ┌────▼──────┐
    │  Local   │   │   Cloud    │   │  Custom   │
    │  LLMs    │   │   APIs     │   │  Endpoint │
    │ (Ollama) │   │ (Anthropic)│   │ (User-def)│
    └──────────┘   └────────────┘   └───────────┘
```

### Data Flow

**Error Diagnosis Flow**:
```
Command Failure
      ↓
on_error Hook Triggered
      ↓
AI Diagnostics Plugin
      ↓
1. Collect Context (stdout, stderr, env, history)
      ↓
2. Apply Privacy Filters (remove secrets, sensitive data)
      ↓
3. Build Prompt (error context + system prompt)
      ↓
4. Query LLM (via client abstraction)
      ↓
5. Parse Response (extract solutions)
      ↓
6. Present to User (interactive menu)
      ↓
User Selects Solution
      ↓
Execute Suggested Command (optional)
```

**Suggestion Flow**:
```
Command Completion
      ↓
post_execute Hook Triggered
      ↓
AI Suggestions Plugin
      ↓
1. Retrieve Recent History (SQLite query)
      ↓
2. Analyze Patterns (sequence, temporal, contextual)
      ↓
3. Calculate Confidence Scores
      ↓
4. Rank and Filter (threshold, max count)
      ↓
5. Present Suggestions (TUI panel or CLI prompt)
      ↓
User Accepts/Rejects
      ↓
Record Feedback (improve future suggestions)
```

### Module Structure

```
src/
├── ai/                          # New: AI integration module
│   ├── mod.rs                   # Module exports
│   ├── llm_client.rs            # LLM client abstraction
│   ├── history_analyzer.rs      # Pattern analysis
│   ├── privacy_filter.rs        # Sensitive data filtering
│   ├── prompt_templates.rs      # Prompt engineering
│   ├── ollama.rs                # Ollama integration
│   ├── anthropic.rs             # Claude integration
│   └── openai.rs                # OpenAI integration
│
├── plugin/builtin/              # New: Built-in plugins
│   ├── mod.rs
│   ├── ai_diagnostics.rs        # Error diagnosis plugin
│   └── ai_suggestions.rs        # History-based suggestions plugin
│
└── tui/                         # Existing, extend for AI
    ├── mod.rs
    ├── suggestions_panel.rs     # New: Suggestions UI
    └── diagnosis_view.rs        # New: Error diagnosis UI
```

---

## Implementation Plan

### Phase 1: Foundation (Milestone 1.1)

**Duration**: 2-3 weeks
**Goal**: Core infrastructure and local-only error diagnosis

**Tasks**:
1. ✅ Create `src/ai/` module structure
2. ✅ Implement LLM client abstraction (`LLMClient` trait)
3. ✅ Implement Ollama client (local LLM support)
4. ✅ Create privacy filter module
5. ✅ Implement error diagnosis plugin
6. ✅ Add configuration schema for AI features
7. ✅ Unit tests for all components
8. ✅ Documentation: AI integration guide

**Deliverables**:
- Working error diagnosis with local LLMs
- Configuration system for AI features
- Privacy controls implemented
- Basic documentation

**Acceptance Criteria**:
- Error diagnosis triggers on command failure
- Ollama integration works with llama3:8b model
- Privacy filters remove secrets (tested with 20+ patterns)
- Configuration validation prevents misuse
- Zero dependencies added to core (plugin-only)

---

### Phase 2: Cloud Integration (Milestone 1.2)

**Duration**: 1-2 weeks
**Goal**: Add cloud LLM providers and enhanced diagnosis

**Tasks**:
1. ✅ Implement Anthropic client (Claude API)
2. ✅ Implement OpenAI client (GPT-4 API)
3. ✅ Add API key management (secure storage)
4. ✅ Implement provider fallback (local → cloud)
5. ✅ Add usage tracking and cost estimation
6. ✅ Enhance prompt templates
7. ✅ Integration tests with real APIs
8. ✅ User guide: Cloud provider setup

**Deliverables**:
- Multi-provider support (Ollama, Claude, GPT-4)
- Secure API key storage
- Cost estimation and limits
- Provider comparison documentation

**Acceptance Criteria**:
- All 3 providers work correctly
- API keys stored securely (OS keychain integration)
- Usage limits prevent runaway costs
- Fallback works seamlessly
- Response quality metrics documented

---

### Phase 3: History-Based Suggestions (Milestone 1.3)

**Duration**: 2-3 weeks
**Goal**: Intelligent command suggestions from history analysis

**Tasks**:
1. ✅ Implement pattern cache system
2. ✅ Build sequence pattern analyzer (Markov chain)
3. ✅ Build temporal pattern analyzer
4. ✅ Build contextual pattern analyzer
5. ✅ Implement confidence scoring algorithm
6. ✅ Create suggestions plugin
7. ✅ Integrate with TUI (suggestions panel)
8. ✅ Integrate with CLI (proactive prompts)
9. ✅ Add feedback collection mechanism
10. ✅ Performance optimization (cache management)
11. ✅ Benchmarks and tests
12. ✅ User guide: Suggestions system

**Deliverables**:
- History-based suggestions in TUI and CLI
- Pattern analysis with confidence scoring
- Proactive suggestions after command execution
- Performance benchmarks

**Acceptance Criteria**:
- Suggestion accuracy ≥ 70% for top-3 suggestions
- Response time < 50ms (local analysis)
- Cache rebuild < 100ms for 1000 history entries
- TUI integration seamless (no UI lag)
- User feedback properly recorded

---

### Phase 4: Advanced Features (Milestone 2.0)

**Duration**: 3-4 weeks
**Goal**: Natural language commands and smart completion

**Tasks**:
1. ✅ Natural language parser (intent detection)
2. ✅ Command generation from NL
3. ✅ Interactive completion system
4. ✅ Context-aware suggestions
5. ✅ Fine-tuning support (user-specific models)
6. ✅ Reinforcement learning from feedback
7. ✅ Multi-language support (i18n for AI features)
8. ✅ Advanced analytics dashboard
9. ✅ Comprehensive benchmarks
10. ✅ Advanced user guide

**Deliverables**:
- Natural language command interface
- Smart tab completion
- Personalized suggestions
- Analytics dashboard
- Complete documentation

**Acceptance Criteria**:
- NL command generation accuracy ≥ 85%
- Personalization improves accuracy by ≥ 15%
- Completion suggestions < 30ms latency
- Multi-language support for EN, JA, ZH
- Full test coverage (>90%)

---

## Security and Privacy

### Privacy-First Design Principles

1. **Local-First**:
   - All analysis performed locally by default
   - History database never leaves user's machine
   - Pattern cache stored locally

2. **Opt-In Cloud**:
   - Cloud LLMs require explicit configuration
   - Clear consent flow for data transmission
   - Per-command override (disable cloud for sensitive commands)

3. **Sensitive Data Filtering**:
   - Automatic detection of secrets (API keys, passwords, tokens)
   - Configurable filter patterns
   - Whitelist/blacklist for environment variables

4. **Transparency**:
   - Show exactly what data will be sent to LLM
   - User can review and edit before transmission
   - Audit log of all LLM queries

### Sensitive Data Patterns

```rust
// src/ai/privacy_filter.rs
const SENSITIVE_PATTERNS: &[&str] = &[
    // API Keys
    r"(?i)(api[_-]?key|apikey)[\s:=]+['\"]?([a-zA-Z0-9_-]{20,})",

    // Tokens
    r"(?i)(token|auth[_-]?token)[\s:=]+['\"]?([a-zA-Z0-9_-]{20,})",

    // Passwords
    r"(?i)(password|passwd|pwd)[\s:=]+['\"]?([^\s'\"]{8,})",

    // AWS credentials
    r"AKIA[0-9A-Z]{16}",

    // GitHub tokens
    r"ghp_[a-zA-Z0-9]{36}",

    // Private keys
    r"-----BEGIN [A-Z ]+ PRIVATE KEY-----",

    // Credit cards
    r"\b\d{4}[\s-]?\d{4}[\s-]?\d{4}[\s-]?\d{4}\b",
];

pub fn filter_sensitive_data(text: &str) -> String {
    let mut filtered = text.to_string();
    for pattern in SENSITIVE_PATTERNS {
        let re = Regex::new(pattern).unwrap();
        filtered = re.replace_all(&filtered, "[FILTERED]").to_string();
    }
    filtered
}
```

### Configuration Examples

**Maximum Privacy (Local-Only)**:
```toml
[config.ai]
enabled = true
provider = "local"
model = "llama3:8b"

[config.ai.privacy]
send_environment_vars = false
send_working_dir = false
send_command_args = false
filter_secrets = true
allow_cloud = false
```

**Balanced (Local with Cloud Fallback)**:
```toml
[config.ai]
enabled = true
provider = "hybrid"
local_model = "llama3:8b"
cloud_model = "claude-3.5-sonnet"

[config.ai.privacy]
send_environment_vars = false
send_working_dir = true
send_command_args = true
filter_secrets = true
allow_cloud = true
cloud_requires_confirmation = true
```

**Cloud-First (Enhanced Accuracy)**:
```toml
[config.ai]
enabled = true
provider = "anthropic"
model = "claude-3.5-sonnet"

[config.ai.privacy]
send_environment_vars = false  # Still filter env vars
send_working_dir = true
send_command_args = true
filter_secrets = true
allow_cloud = true
cloud_requires_confirmation = false  # Trust cloud by default

[config.ai.anthropic]
api_key_source = "keychain"  # OS keychain, not plaintext
max_tokens_per_day = 100000  # Cost control
```

### Data Minimization

**What is Sent to LLM**:
```json
{
  "command": "deploy",
  "exit_code": 1,
  "stderr": "Permission denied (publickey)",
  "context": {
    "working_dir": "/home/user/project",
    "recent_commands": ["build", "test", "deploy"]
  }
}
```

**What is NOT Sent**:
- Full command history (only last 5 commands)
- Environment variables (unless explicitly allowed)
- File contents
- User's home directory path
- Usernames, emails, IP addresses
- Any detected secrets

---

## Alternative Approaches

### Approach 1: Standalone AI Assistant (Rejected)

**Description**: Separate binary (`cmdrun-ai`) that wraps `cmdrun`.

**Pros**:
- Complete separation of concerns
- No core dependency changes
- Easier to disable

**Cons**:
- Poor integration (no access to internal state)
- Duplication of configuration and history
- Inconsistent UX (two tools instead of one)
- More complex installation

**Decision**: Rejected in favor of plugin-based approach

---

### Approach 2: Cloud-Only Service (Rejected)

**Description**: All AI features require cloud service subscription.

**Pros**:
- Best accuracy (latest models)
- No local compute requirements
- Centralized updates

**Cons**:
- Privacy concerns (all data sent to cloud)
- Requires internet connection
- Subscription cost barrier
- Against cmdrun's local-first philosophy

**Decision**: Rejected in favor of local-first with opt-in cloud

---

### Approach 3: Embedded Local Models (Considered)

**Description**: Bundle small LLM with cmdrun binary.

**Pros**:
- Zero configuration for basic AI features
- Complete offline functionality
- Best privacy

**Cons**:
- Massive binary size increase (1GB+)
- Limited model quality (small models less accurate)
- Platform-specific builds (GPU acceleration)
- Update challenges (model improvements)

**Decision**: Not implemented in Phase 1, revisit for Phase 4

---

## Dependencies

### New Dependencies

```toml
# AI integration
reqwest = { version = "0.11", features = ["json"] }  # HTTP client for cloud APIs
serde_json = "1.0"  # JSON parsing
tokio-stream = "0.1"  # Async streaming for LLM responses

# Optional: Local LLM support
ollama-rs = { version = "0.1", optional = true }  # Ollama client

# Security
ring = "0.17"  # Cryptography for API key encryption
keyring = "2.0"  # OS keychain integration
```

### Binary Size Impact

- **Core cmdrun**: ~5 MB (current)
- **With AI plugin**: ~7 MB (+2 MB for HTTP client, JSON)
- **With local LLM**: Not bundled (users install Ollama separately)

---

## Success Metrics

### Phase 1 (Error Diagnosis)

- **Activation Rate**: % of errors that trigger AI diagnosis
- **Resolution Rate**: % of errors resolved using AI suggestions
- **Time Savings**: Avg time saved vs manual troubleshooting
- **User Satisfaction**: Survey rating (1-5 scale)

**Targets**:
- Activation: >80% (opt-out defaults)
- Resolution: >60% (first attempt)
- Time Savings: >3 minutes average
- Satisfaction: >4.0/5.0

### Phase 3 (Suggestions)

- **Suggestion Accuracy**: Top-1 / Top-3 / Top-5 accuracy
- **Acceptance Rate**: % of suggestions accepted by user
- **Workflow Acceleration**: Reduction in command typing
- **Pattern Coverage**: % of user patterns detected

**Targets**:
- Top-1 Accuracy: >50%
- Top-3 Accuracy: >70%
- Acceptance Rate: >40%
- Pattern Coverage: >80%

---

## Future Enhancements

### v2.0+

1. **Multi-Agent Collaboration**:
   - Multiple specialized AI agents (deploy, debug, performance)
   - Agent orchestration and handoff

2. **Learning from Team**:
   - Opt-in shared pattern learning (anonymized)
   - Team-specific model fine-tuning

3. **Proactive Monitoring**:
   - Predict failures before they occur
   - Suggest preventive maintenance

4. **Visual Explanations**:
   - Dependency graph visualization
   - Execution flow diagrams
   - Performance flame graphs

5. **Voice Interface**:
   - Voice command execution
   - Voice-guided troubleshooting

---

## References

### Related Work

- **GitHub Copilot CLI**: AI-powered terminal assistance
- **Warp Terminal**: AI command search and suggestions
- **Fig**: Terminal autocomplete with AI
- **OpenCommit**: AI commit message generation

### Academic Research

- "Code Generation from Natural Language" (Chen et al., 2021)
- "Learning from Execution History for Program Synthesis" (Pu et al., 2022)
- "Privacy-Preserving Machine Learning" (Dwork et al., 2020)

### Technical Documentation

- [Anthropic API Documentation](https://docs.anthropic.com/)
- [OpenAI API Documentation](https://platform.openai.com/docs)
- [Ollama Documentation](https://ollama.ai/docs)
- [cmdrun Plugin API](../plugins/API.md)

---

## Conclusion

This proposal outlines a comprehensive AI integration strategy for cmdrun that:

✅ **Prioritizes Privacy**: Local-first with opt-in cloud
✅ **Enhances Productivity**: Smart error diagnosis and history-based suggestions
✅ **Maintains Performance**: Plugin-based, zero overhead when disabled
✅ **Ensures Security**: Sensitive data filtering and transparent data usage
✅ **Follows Best Practices**: Modular architecture, extensive testing, clear documentation

The phased implementation plan allows for incremental delivery of value while maintaining high quality standards.

---

**Questions or Feedback?**

Please open a GitHub issue or discussion at: https://github.com/sanae-abe/cmdrun/discussions

**Author**: Claude (AI Assistant)
**Reviewed by**: [Pending]
**Approved by**: [Pending]
**Last Updated**: 2025-11-09
