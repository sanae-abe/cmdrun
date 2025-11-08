# cmdrun

[English](README.md) | [日本語](README.ja.md)

> **A personal global command manager for your frequently used commands**
>
> Register your commands once, run them from anywhere. Fast, secure, and cross-platform.

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
- **Zero `eval()`** - No dynamic code execution
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

cmdrun is a **personal global command manager** that allows you to register and run frequently used commands from anywhere on your system.

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

<img src="docs/screenshots/add.webp" alt="Adding Commands" width="600">

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

<img src="docs/screenshots/run.webp" alt="Running Commands" width="600">

<img src="docs/screenshots/list.webp" alt="Listing Commands" width="600">

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

### Language Settings (i18n)

cmdrun supports internationalization with English and Japanese languages. Configure the language in your `commands.toml`:

```toml
[config]
language = "japanese"  # or "english" (default)
```

**Supported Messages:**
- Command execution (Running, Completed, Error)
- Interactive prompts (Command ID, Description, etc.)
- Success/error messages (Command added, Command not found, etc.)
- Validation errors (Empty input, duplicate commands, etc.)

**Example (Japanese):**
```bash
$ cmdrun add test-ja "echo テスト" "日本語テストコマンド"
📝 コマンドを追加中 'test-ja' ...
✓ コマンドを追加しました 'test-ja'
  説明: 日本語テストコマンド
  コマンド: echo テスト
```

**Example (English):**
```bash
$ cmdrun add test-en "echo test" "English test command"
📝 Adding command 'test-en' ...
✓ Command added successfully 'test-en'
  Description: English test command
  Command: echo test
```

**Currently Supported Commands:**
- `cmdrun add` - Fully localized (prompts, messages, errors)
- More commands will be localized in future releases

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
