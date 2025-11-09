# cmdrun

[![Version](https://img.shields.io/badge/version-1.0.0-blue.svg)](https://github.com/sanae-abe/cmdrun)
[![Rust](https://img.shields.io/badge/rust-1.75%2B-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](LICENSE)

[English](README.md) | [日本語](README.ja.md) | [简体中文](README.zh-CN.md) | [繁體中文](README.zh-TW.md)

> **A fast, secure, cross-platform command runner**
>
> Register your commands once, run them from anywhere.

## Table of Contents

- [Why cmdrun?](#why-cmdrun)
- [Installation](#installation)
- [Basic Usage](#basic-usage)
- [Features](#features)
- [Configuration Examples](#configuration-examples)
- [Documentation](#documentation)
- [License](#license)

## Why cmdrun?

### 🚀 Performance
- **~29x faster startup** than Node.js-based task runners
- **4ms startup time** vs 115ms+ for npm/yarn
- **10MB memory footprint** vs 200MB+ for Node.js

### 🔒 Security
- **Zero Rust `eval()`** - No dynamic code generation in application code
- **Safe variable expansion** - No shell injection vulnerabilities
- **Dependency audit** - Built-in security scanning

### 🌍 Cross-platform
- **Supported OS**: Linux, macOS, Windows, FreeBSD
- **Shell detection**: Auto-detects bash/zsh/fish/pwsh
- **Native binaries**: No runtime dependencies

### 💎 Developer Experience
- **TOML configuration** - Type-safe, easy to read
- **Powerful features** - Dependencies, parallel execution, hooks, Watch Mode
- **Great errors** - Detailed error messages with context

### 🎯 What Makes cmdrun Special

**Unique combination of features:**
- 🔒 Zero Rust eval() with fuzzing (373,423 tests, 0 vulnerabilities)
- 🌍 4-language support (EN/JA/ZH-CN/ZH-TW)
- 🎨 Shell completion (Zsh/Bash/Fish)
- 📊 SQLite-based execution history
- 🔌 Dynamic plugin system
- 🎯 Intelligent typo detection

## Installation

#### System Requirements

- **Operating System**: Linux, macOS, Windows, FreeBSD
- **Rust**: 1.75+ (MSRV)

#### Install Rust Toolchain

```bash
# 1. Download and run Rustup (Rust installer)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 2. Load environment variables
source ~/.cargo/env

# 3. Verify installation
rustc --version
cargo --version
```

#### Build and Install cmdrun

```bash
# 1. Clone the repository
git clone git@github.com:sanae-abe/cmdrun.git
cd cmdrun

# 2. Build and install
cargo install --path .

# 3. Verify installation
cmdrun --version
cmdrun --help
```

### Update

```bash
# If installed from source
cd cmdrun  # Navigate to project directory
git pull

# Rebuild and install
cargo install --path . --force
```

### Uninstall

```bash
# 1. Remove binary
cargo uninstall cmdrun

# 2. Remove configuration files (optional)
# Linux/macOS
rm -rf ~/.config/cmdrun

# Windows (run in PowerShell)
# Remove-Item -Recurse -Force "$env:APPDATA\cmdrun"

# 3. Remove project directory (optional)
# cd ..
# rm -rf cmdrun
```

**Note:**
- `cargo uninstall cmdrun` only removes the executable
- Configuration files (commands.toml, etc.) need to be removed manually
- Skip step 2 if you want to keep your settings

<!-- Future installation methods (commented out until available)
#### Homebrew (macOS/Linux)
```bash
brew install sanae-abe/tap/cmdrun
```

#### Scoop (Windows)
```bash
scoop bucket add cmdrun https://github.com/sanae-abe/scoop-bucket
scoop install cmdrun
```
-->

## Basic Usage

cmdrun is a **fast, secure, cross-platform command runner** that allows you to register and run frequently used commands from anywhere on your system.

#### Register your frequently used commands

```bash
# Add a command interactively
cmdrun add

# Or add directly with parameters
cmdrun add dev "npm run dev" "Start development server"
cmdrun add push "git add . && git commit && git push" "Commit and push changes"
cmdrun add prod-ssh "ssh user@production-server.com" "Connect to production server"
cmdrun add docker-clean "docker system prune -af" "Clean up unused Docker resources"
cmdrun add db-backup "pg_dump mydb > backup_$(date +%Y%m%d).sql" "Backup database"
```

<!-- <img src="docs/screenshots/add.webp" alt="Adding Commands" width="600"> -->

#### Run and manage your commands

```bash
# Run a registered command
cmdrun run dev

# List all registered commands
cmdrun list

# Search for commands
cmdrun search docker

# Remove a command
cmdrun remove dev
```

<!-- <img src="docs/screenshots/run.webp" alt="Running Commands" width="600"> -->

<!-- <img src="docs/screenshots/list.webp" alt="Listing Commands" width="600"> -->

#### Configuration management

```bash
# Show all settings
cmdrun config show

# Change language
cmdrun config set language japanese

# Use custom configuration file
cmdrun --config ~/work/commands.toml list
cmdrun -c ~/.cmdrun/personal.toml run dev

# Show help
cmdrun --help
```

**Configuration file location:**
- Linux/macOS: `~/.config/cmdrun/commands.toml`
- Windows: `%APPDATA%\cmdrun\commands.toml`
- Custom path: Use `--config/-c` option to specify any path

## Features

### Variable Expansion

```toml
[commands.deploy]
cmd = "scp dist/ ${DEPLOY_USER:?DEPLOY_USER not set}@${DEPLOY_HOST:?DEPLOY_HOST not set}:${DEPLOY_PATH:-/var/www}"
```

Supported syntax:
- `${VAR}` - Basic expansion
- `${1}`, `${2}`, ... - Positional arguments
- `${VAR:-default}` - Default value
- `${VAR:?error}` - Required variable
- `${VAR:+value}` - Conditional substitution

**Positional Arguments Example:**

```toml
[commands.convert]
description = "Convert image format"
cmd = "sharp -i ${1} -f ${2:-webp} -q ${3:-80} -o ${4:-output.webp}"
```

```bash
# Usage with arguments
cmdrun run convert input.png webp 90 output.webp
# Expands to: sharp -i input.png -f webp -q 90 -o output.webp

# Using default values
cmdrun run convert input.png
# Expands to: sharp -i input.png -f webp -q 80 -o output.webp
```

### Dependencies

```toml
[commands.test]
cmd = "cargo test"
deps = ["build"]  # Run 'build' before 'test'

[commands.build]
cmd = "cargo build --release"
```

### Parallel Execution

```toml
[commands.check]
parallel = true
cmd = [
    "cargo fmt -- --check",
    "cargo clippy",
]
```

### Platform-specific Commands

```toml
[commands."open:browser"]
cmd.unix = "open http://localhost:3000"
cmd.windows = "start http://localhost:3000"
cmd.linux = "xdg-open http://localhost:3000"
```

### Hooks

```toml
[hooks]
pre_run = "echo 'Starting...'"
post_run = "echo 'Done!'"

[hooks.commands.deploy]
pre_run = "git diff --exit-code"  # Ensure no uncommitted changes
post_run = "echo 'Deployed at $(date)' >> deploy.log"
```

### Environment Variables

```toml
[config.env]
NODE_ENV = "development"
RUST_BACKTRACE = "1"

[commands.dev]
cmd = "npm run dev"
env = { PORT = "3000" }  # Command-specific env
```

### Environment Management

Easily switch between development, staging, and production environments.

```bash
# Create environments
cmdrun env create dev --description "Development environment"
cmdrun env create prod --description "Production environment"

# Switch environments
cmdrun env use dev
cmdrun run start  # Start with development settings

cmdrun env use prod
cmdrun run deploy  # Deploy with production settings

# Set environment variables
cmdrun env set API_URL https://api.staging.com --env staging
```

See [Environment Management Guide](docs/ENVIRONMENT_MANAGEMENT.md) for details.

### History & Logging

Record, search, and replay command execution history.

```bash
# Show history
cmdrun history list

# Search commands
cmdrun history search build

# Show statistics
cmdrun history stats

# Retry last failed command
cmdrun retry

# Export history
cmdrun history export --format json -o history.json
```

See [History Guide](docs/user-guide/HISTORY.md) for details.

### Template System

Use, create, and share project templates.

```bash
# List available templates
cmdrun template list

# Use a template
cmdrun template use rust-cli

# Create custom template
cmdrun template add my-template

# Export template
cmdrun template export rust-cli ./my-template.toml
```

**Built-in Templates:**
- `rust-cli` - Rust CLI development (cargo build/test/clippy/fmt)
- `nodejs-web` - Node.js web development (npm dev/build/test)
- `python-data` - Python data science (pytest/jupyter)
- `react-app` - React application (dev/build/storybook)

See [Template Feature Report](TEMPLATE_FEATURE_REPORT.md) for details.

### Plugin System

Extend functionality with external plugins.

```toml
# commands.toml
[plugins]
enabled = ["hello", "logger"]

[plugins.logger]
path = "plugins/logger_plugin.so"
log_file = "cmdrun.log"
level = "info"
```

```bash
# List plugins
cmdrun plugin list

# Show plugin details
cmdrun plugin info logger

# Enable/disable plugins
cmdrun plugin enable logger
cmdrun plugin disable logger
```

See [Plugin System Report](PLUGIN_SYSTEM_IMPLEMENTATION_REPORT.md) and [Plugin API](docs/plugins/API.md) for details.

### Watch Mode - File Watching

```toml
# Define commands as usual in commands.toml
[commands.dev]
cmd = "cargo build"

[commands.test]
cmd = "cargo test"
```

```bash
# Run with Watch Mode from command line
# Watch Rust files and build on changes
cmdrun watch dev --pattern "**/*.rs"

# Auto-run tests (with 1s debounce)
cmdrun watch test --pattern "**/*.rs" --debounce 1000

# Watch multiple directories
cmdrun watch dev --path src --path lib
```

**Watch Mode Key Features:**
- **Glob Patterns**: File filtering (e.g., `**/*.rs`, `**/*.ts`)
- **Exclude Patterns**: Exclude unwanted files/directories (defaults exclude `node_modules`, `target`, etc.)
- **Debouncing**: Prevent unnecessary executions on frequent changes (default 500ms)
- **Recursive Watching**: Automatically watch subdirectories (can disable with `--no-recursive`)
- **gitignore Integration**: Automatically respect `.gitignore` patterns

See [Watch Mode Guide](docs/user-guide/WATCH_MODE.md) for details.

### Shell Completion

cmdrun provides intelligent shell completion for Zsh, Bash, and Fish shells with command descriptions and global configuration fallback.

**Setup:**

```bash
# Zsh - Add to ~/.zshrc
eval "$(cmdrun completion zsh)"

# Bash - Add to ~/.bashrc
eval "$(cmdrun completion bash)"

# Fish - Add to ~/.config/fish/config.fish
cmdrun completion fish | source
```

**Features:**
- 🎯 **Smart Completion**: Auto-completes command names from both global and local configurations
- 📝 **Descriptions**: Shows command descriptions (Zsh/Fish) or command list (Bash)
- 🌍 **Global Fallback**: Works even without local `commands.toml` by using global config
- ⚡ **Fast**: Minimal overhead, optimized for speed

**Shell-Specific Features:**

**Zsh:**
- Press `Tab` once: Opens menu selection with descriptions
- Navigate with arrow keys or `Tab`/`Shift+Tab`
- Full description display for each command

**Bash:**
- Press `Tab` twice: Shows command name list
- No descriptions (Bash limitation)

**Fish:**
- Press `Tab`: Shows command list with descriptions
- Navigate with arrow keys
- Automatic filtering as you type

### Typo Detection

cmdrun automatically detects typos in command names and suggests corrections.

**Example:**
```bash
$ cmdrun seach docker
Error: Unknown command 'seach'

Did you mean one of these?
  → search (distance: 1)
  → watch (distance: 2)

Run 'cmdrun --help' for available commands.
```

**Configuration:**
```toml
[config]
typo_detection = true
typo_threshold = 2        # Maximum Levenshtein distance
auto_correct = false      # Set to true for automatic correction
```

**Multilingual Error Messages:**
- English: "Did you mean 'X'?"
- Japanese: "もしかして: 'X' ですか？"
- Chinese (Simplified): "您是否想输入 'X'？"
- Chinese (Traditional): "您是否想輸入 'X'？"

### Language Settings (i18n)

cmdrun supports 4 languages: **English, Japanese, Simplified Chinese (简体中文), Traditional Chinese (繁體中文)**.

**Language Configuration:**
- Uses configuration file setting (`commands.toml`)
- Change language: `cmdrun config set language <language>`
- Supported values: `english`, `japanese`, `chinese_simplified`, `chinese_traditional`

**Localized Commands (9 commands):**
- `cmdrun add`, `search`, `init`, `remove`, `info`
- `cmdrun config`, `watch`, `validate`, `edit`
- Typo suggestions with multilingual error messages

**Configuration:**
```toml
[config]
language = "english"  # or "japanese", "chinese_simplified", "chinese_traditional"
```

**Change Language:**
```bash
# Set to Japanese
cmdrun config set language japanese

# Set to Chinese (Simplified)
cmdrun config set language chinese_simplified

# Set to Chinese (Traditional)
cmdrun config set language chinese_traditional
```

**Example (Chinese - Simplified):**
```bash
$ cmdrun add test "echo 测试" "测试命令"
📝 正在添加命令 'test' ...
✓ 成功添加命令 'test'
  说明: 测试命令
  命令: echo 测试
```

**Documentation:**
- English: [README.md](README.md)
- 日本语: [README.ja.md](README.ja.md)
- 简体中文: [README.zh-CN.md](README.zh-CN.md)
- 繁體中文: [README.zh-TW.md](README.zh-TW.md)

See [I18N Guide](docs/user-guide/I18N.md) for details.

### Custom Configuration Files

You can use the `--config/-c` option to switch between multiple configuration files.

**Usage Examples:**

```bash
# Work-related commands
cmdrun --config ~/work/commands.toml list
cmdrun -c ~/work/commands.toml run deploy

# Personal commands
cmdrun -c ~/personal/commands.toml run backup

# Project-specific commands
cd ~/projects/myapp
cmdrun -c ./commands.toml run dev
```

**Use Cases:**

1. **Environment-specific configurations**
   ```bash
   # Production environment
   cmdrun -c ~/.cmdrun/production.toml run deploy

   # Staging environment
   cmdrun -c ~/.cmdrun/staging.toml run deploy

   # Development environment
   cmdrun -c ~/.cmdrun/development.toml run dev
   ```

2. **Multiple project management**
   ```bash
   # Project A
   cmdrun -c ~/projects/project-a/commands.toml run test

   # Project B
   cmdrun -c ~/projects/project-b/commands.toml run test
   ```

3. **Role-based command sets**
   ```bash
   # System administration
   cmdrun -c ~/.cmdrun/admin.toml run server-check

   # Development tasks
   cmdrun -c ~/.cmdrun/dev.toml run code-review
   ```

**For more details, see [Configuration Reference](docs/user-guide/CONFIGURATION.md#custom-configuration-file-specification).**

## Configuration Examples

You can edit the configuration file (`~/.config/cmdrun/commands.toml`) directly for advanced features:

```toml
# Commands with dependencies
[commands.deploy]
description = "Deploy to production"
cmd = "ssh user@server 'cd /app && git pull && npm install && pm2 restart app'"
deps = ["test"]  # Deploy only after tests pass
confirm = true   # Ask for confirmation before running

[commands.test]
description = "Run tests"
cmd = "npm test"

# Using environment variables
[commands.backup]
description = "Create backup"
cmd = "rsync -avz ~/projects/ ${BACKUP_PATH:?BACKUP_PATH not set}"

# Platform-specific commands
[commands.open]
description = "Open browser"
cmd.unix = "open http://localhost:3000"
cmd.windows = "start http://localhost:3000"
cmd.linux = "xdg-open http://localhost:3000"
```

## Color Output Control

cmdrun automatically detects whether it's outputting to a terminal and adjusts color output accordingly.

### Controlling Color Output

**Method 1: Using `--color` flag (recommended)**

```bash
# Disable colored output
cmdrun list --color=never

# Force colored output (even when piping)
cmdrun list --color=always | less -R

# Automatic detection (default)
cmdrun list --color=auto
```

**Method 2: Using `NO_COLOR` environment variable**

```bash
# Disable colored output for a single command
NO_COLOR=1 cmdrun list

# Disable colored output globally
export NO_COLOR=1
cmdrun list
```

**Method 3: Automatic detection (default behavior)**

```bash
# Colors enabled when outputting to terminal
cmdrun list

# Colors automatically disabled when piping
cmdrun list | grep build
cmdrun list > commands.txt
```

### CI/CD Usage

For CI/CD environments, you can disable colors to keep logs clean:

```yaml
# GitHub Actions
- name: Run tests
  run: cmdrun run test --color=never
  # or
  env:
    NO_COLOR: 1
  run: cmdrun run test

# GitLab CI
script:
  - cmdrun run deploy --color=never
```

### Priority Order

1. `--color` flag (highest priority)
2. `NO_COLOR` environment variable
3. Automatic TTY/pipe detection (default)

For more details, see the [CLI Reference](docs/user-guide/CLI.md).

## Documentation

### User Guide
- [CLI Reference](docs/user-guide/CLI.md)
- [Configuration Reference](docs/user-guide/CONFIGURATION.md)
- [Internationalization (i18n)](docs/user-guide/I18N.md)
- [Watch Mode](docs/user-guide/WATCH_MODE.md)
- [History](docs/user-guide/HISTORY.md)
- [FAQ](docs/user-guide/FAQ.md)
- [Recipes](docs/user-guide/RECIPES.md)
- [Troubleshooting](docs/user-guide/TROUBLESHOOTING.md)

### Feature Guides
- [Environment Management](docs/ENVIRONMENT_MANAGEMENT.md)
- [Template System](TEMPLATE_FEATURE_REPORT.md)
- [Plugin System](PLUGIN_SYSTEM_IMPLEMENTATION_REPORT.md)

### Plugin Development
- [Plugin API Specification](docs/plugins/API.md)
- [Plugin Development Guide](docs/plugins/DEVELOPMENT_GUIDE.md)
- [Sample Plugins](examples/plugins/README.md)

### Technical Documentation
- [Architecture](docs/technical/ARCHITECTURE.md)
- [Performance](docs/technical/PERFORMANCE.md)
- [Performance Guide](docs/technical/PERFORMANCE_GUIDE.md)
- [Security](docs/technical/SECURITY.md)
- [Cross-platform Support](docs/technical/CROSS_PLATFORM.md)
- [Distribution](docs/technical/DISTRIBUTION.md)

## License

This project is licensed under the [MIT License](LICENSE).

---
**Developer**: sanae.a.sunny@gmail.com
