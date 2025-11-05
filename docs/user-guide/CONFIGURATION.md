# Configuration Guide

Complete reference for `cmdrun` TOML configuration files.

## Table of Contents

- [Configuration Files](#configuration-files)
- [Global Configuration](#global-configuration)
- [Command Definitions](#command-definitions)
- [Aliases](#aliases)
- [Hooks](#hooks)
- [Variable Expansion](#variable-expansion)
- [Platform-Specific Commands](#platform-specific-commands)
- [Environment Variables](#environment-variables)
- [Complete Examples](#complete-examples)

## Configuration Files

### File Locations and Priority

`cmdrun` searches for configuration files in the following order:

1. **Project directory** (current directory and parent directories)
   - `commands.toml`
   - `.cmdrun.toml`
   - `cmdrun.toml`

2. **User home directory**
   - `~/.cmdrun/commands.toml`
   - `~/.cmdrun/.cmdrun.toml`
   - `~/.cmdrun/cmdrun.toml`

3. **Explicit path** (via `--config` flag)
   ```bash
   cmdrun --config /path/to/config.toml run build
   ```

### Search Behavior

- **Upward search**: Starts from the current directory and searches up to the root
- **First match wins**: Uses the first configuration file found
- **No auto-merge**: Only one configuration file is loaded (future versions may support merging)

---

## Global Configuration

The `[config]` section defines global settings that apply to all commands.

### Basic Structure

```toml
[config]
shell = "bash"              # Default shell
strict_mode = true          # Strict variable expansion
parallel = false            # Default parallel execution
timeout = 300               # Default timeout in seconds
working_dir = "."           # Default working directory
language = "english"        # UI language (english/japanese)
```

### Configuration Fields

#### `shell`

**Type**: String
**Default**:
- Unix/Linux/macOS: `"bash"`
- Windows: `"pwsh"`

**Description**: Default shell for command execution.

**Supported shells**:
- `bash` - Bourne Again SHell
- `zsh` - Z Shell
- `fish` - Friendly Interactive SHell
- `pwsh` - PowerShell (cross-platform)
- `sh` - POSIX shell
- `cmd` - Windows Command Prompt (Windows only)

**Example**:
```toml
[config]
shell = "zsh"
```

#### `strict_mode`

**Type**: Boolean
**Default**: `true`

**Description**: Controls variable expansion behavior.

- `true`: Undefined variables cause errors
- `false`: Undefined variables expand to empty strings

**Example**:
```toml
[config]
strict_mode = false  # Allow undefined variables
```

#### `parallel`

**Type**: Boolean
**Default**: `false`

**Description**: Default parallel execution mode for commands with multiple steps.

**Example**:
```toml
[config]
parallel = true  # Run multi-step commands in parallel by default
```

#### `timeout`

**Type**: Integer (seconds)
**Default**: `300` (5 minutes)

**Description**: Default timeout for command execution. Use `0` for no timeout.

**Example**:
```toml
[config]
timeout = 600  # 10 minutes
```

#### `working_dir`

**Type**: String (path)
**Default**: `"."`

**Description**: Default working directory for all commands.

**Example**:
```toml
[config]
working_dir = "./src"
```

#### `language`

**Type**: String
**Default**: `"english"`

**Options**: `"english"` | `"japanese"`

**Description**: UI language for messages and prompts.

**Example**:
```toml
[config]
language = "japanese"
```

### Global Environment Variables

Define environment variables that apply to all commands:

```toml
[config.env]
NODE_ENV = "development"
RUST_BACKTRACE = "1"
DATABASE_URL = "postgresql://localhost/mydb"
```

**Note**: Command-specific environment variables override global ones.

---

## Command Definitions

Commands are defined in the `[commands.*]` section.

### Basic Command

```toml
[commands.hello]
description = "Print hello message"
cmd = "echo 'Hello, World!'"
```

### Command Fields

#### `description`

**Type**: String
**Required**: No (but recommended)

**Description**: Human-readable description displayed in `cmdrun list`.

```toml
[commands.build]
description = "Build the project for production"
cmd = "cargo build --release"
```

#### `cmd`

**Type**: String | Array | Platform-specific
**Required**: Yes

**Description**: Command(s) to execute. Supports three formats:

**1. Single command (String)**:
```toml
[commands.dev]
cmd = "npm run dev"
```

**2. Multiple commands (Array)** - executed sequentially:
```toml
[commands.build]
cmd = [
    "npm run type-check",
    "npm run lint",
    "npm run build",
]
```

**3. Platform-specific (Table)**:
```toml
[commands.open]
cmd.unix = "open http://localhost:3000"
cmd.linux = "xdg-open http://localhost:3000"
cmd.macos = "open http://localhost:3000"
cmd.windows = "start http://localhost:3000"
```

#### `env`

**Type**: Table (key-value pairs)
**Required**: No

**Description**: Command-specific environment variables.

```toml
[commands.dev]
cmd = "npm run dev"
env = { PORT = "3000", NODE_ENV = "development" }
```

**Alternative syntax**:
```toml
[commands.dev]
cmd = "npm run dev"

[commands.dev.env]
PORT = "3000"
NODE_ENV = "development"
```

#### `working_dir`

**Type**: String (path)
**Required**: No

**Description**: Override the global working directory for this command.

```toml
[commands.api]
description = "Start API server"
cmd = "cargo run"
working_dir = "./backend"
```

#### `deps`

**Type**: Array of strings
**Required**: No

**Description**: List of commands to run before this command.

```toml
[commands.test]
description = "Run tests"
cmd = "cargo test"
deps = ["build"]  # Run 'build' before 'test'

[commands.build]
cmd = "cargo build --release"
```

**Dependency features**:
- Commands run in dependency order
- Circular dependencies are detected and cause errors
- Transitive dependencies are supported

#### `platform`

**Type**: Array of strings
**Required**: No

**Description**: Restrict command to specific platforms.

**Supported platforms**:
- `"unix"` - All Unix-like systems (Linux, macOS, etc.)
- `"linux"` - Linux only
- `"macos"` - macOS only
- `"windows"` - Windows only

```toml
[commands.coverage]
description = "Generate code coverage"
cmd = "cargo tarpaulin"
platform = ["unix", "linux"]  # Unix/Linux only
```

**Empty array** (default): Command runs on all platforms.

#### `tags`

**Type**: Array of strings
**Required**: No

**Description**: Metadata tags for organizing commands.

```toml
[commands.test]
cmd = "cargo test"
tags = ["testing", "ci", "quality"]
```

**Future feature**: Filter commands by tags in `cmdrun list --tag testing`.

#### `timeout`

**Type**: Integer (seconds)
**Required**: No

**Description**: Override the global timeout for this command.

```toml
[commands."test:e2e"]
cmd = "playwright test"
timeout = 1800  # 30 minutes for long-running tests
```

Use `0` for no timeout:
```toml
[commands.watch]
cmd = "cargo watch -x run"
timeout = 0  # Run indefinitely
```

#### `parallel`

**Type**: Boolean
**Required**: No
**Default**: Inherits from `[config]`

**Description**: Run multiple commands in parallel (for array `cmd`).

```toml
[commands.lint]
description = "Run all linters"
parallel = true
cmd = [
    "cargo fmt -- --check",
    "cargo clippy",
    "eslint src/",
]
```

**Serial execution** (default):
```toml
[commands.deploy]
parallel = false
cmd = [
    "npm run build",
    "scp -r dist/ server:/var/www",
]
```

#### `confirm`

**Type**: Boolean
**Required**: No
**Default**: `false`

**Description**: Require user confirmation before execution.

```toml
[commands.deploy]
description = "Deploy to production"
cmd = "kubectl apply -f production.yaml"
confirm = true  # Ask for confirmation
```

**Interactive prompt**:
```bash
$ cmdrun run deploy
⚠️  This command requires confirmation
   Description: Deploy to production
   Command: kubectl apply -f production.yaml

   Continue? (y/n):
```

---

## Aliases

Shortcuts for frequently used commands.

### Basic Aliases

```toml
[aliases]
d = "dev"
b = "build"
t = "test"
l = "lint"
```

**Usage**:
```bash
cmdrun run d    # Same as: cmdrun run dev
cmdrun run t    # Same as: cmdrun run test
```

### Advanced Aliases

Aliases can reference any command, including those with special characters:

```toml
[aliases]
start = "dev"
check = "lint:all"
db = "docker:db:start"
```

**Alias resolution**:
1. Check if alias exists
2. Resolve to target command
3. Execute target command (including its dependencies)

---

## Hooks

Execute commands before and after other commands.

### Global Hooks

Run before/after **all** commands:

```toml
[hooks]
pre_run = "echo 'Starting command...'"
post_run = "echo 'Command completed'"
```

**Example output**:
```bash
$ cmdrun run build
Starting command...
[build output...]
Command completed
```

### Command-Specific Hooks

Run before/after specific commands:

```toml
[hooks.commands.deploy]
pre_run = "git diff --exit-code"  # Ensure no uncommitted changes
post_run = "echo 'Deployed at $(date)' >> deploy.log"
```

**Multiple commands**:
```toml
[hooks.commands.deploy.pre_run]
cmd = [
    "git diff --exit-code",
    "npm run test",
]

[hooks.commands.deploy]
post_run = "notify-send 'Deployment complete'"
```

### Hook Execution Order

```
1. Global pre_run hook
2. Command-specific pre_run hook
3. Dependency commands (with their hooks)
4. Main command
5. Command-specific post_run hook
6. Global post_run hook
```

### Hook Features

- **Exit codes**: Non-zero exit code in `pre_run` aborts execution
- **Variable expansion**: Hooks support variable expansion
- **Environment**: Hooks inherit command's environment variables

---

## Variable Expansion

Secure variable expansion without `eval()` or shell injection vulnerabilities.

### Supported Syntax

#### Basic Expansion: `${VAR}`

```toml
[commands.greet]
cmd = "echo 'Hello, ${USER}!'"
```

**Result**:
```bash
$ USER=Alice cmdrun run greet
Hello, Alice!
```

#### Default Value: `${VAR:-default}`

Use default value if variable is undefined or empty:

```toml
[commands.serve]
cmd = "python -m http.server ${PORT:-8080}"
```

**Results**:
```bash
$ cmdrun run serve
# Uses port 8080 (default)

$ PORT=3000 cmdrun run serve
# Uses port 3000
```

#### Required Variable: `${VAR:?error_message}`

Fail if variable is not set:

```toml
[commands.deploy]
cmd = "scp dist/ ${DEPLOY_USER:?DEPLOY_USER not set}@${DEPLOY_HOST:?}:/var/www"
```

**Results**:
```bash
$ cmdrun run deploy
Error: DEPLOY_USER not set

$ DEPLOY_USER=admin cmdrun run deploy
Error: DEPLOY_HOST not set

$ DEPLOY_USER=admin DEPLOY_HOST=prod.example.com cmdrun run deploy
# Executes successfully
```

#### Conditional Substitution: `${VAR:+value_if_set}`

Replace with value only if variable is set:

```toml
[commands.build]
cmd = "cargo build ${RELEASE:+--release}"
```

**Results**:
```bash
$ cmdrun run build
cargo build

$ RELEASE=1 cmdrun run build
cargo build --release
```

### Nested Expansion

Variables can reference other variables (up to 10 levels deep):

```toml
[config.env]
BASE_PATH = "/var/www"
PROJECT_PATH = "${BASE_PATH}/myapp"
DEPLOY_PATH = "${PROJECT_PATH}/current"

[commands.deploy]
cmd = "rsync -av dist/ server:${DEPLOY_PATH}"
```

**Result**: Expands to `/var/www/myapp/current`

### Variable Sources

Variables are resolved in order:

1. **Command-specific env** (`commands.*.env`)
2. **Global env** (`config.env`)
3. **System environment variables**

```toml
[config.env]
API_URL = "http://localhost:3000"

[commands.test]
env = { API_URL = "http://test.example.com" }
cmd = "curl ${API_URL}"
```

**Result**: Uses `http://test.example.com` (command-specific overrides global)

### Strict Mode

Control behavior for undefined variables:

```toml
[config]
strict_mode = true  # Undefined variables cause errors

[commands.example]
cmd = "echo ${UNDEFINED_VAR}"  # Error!
```

```toml
[config]
strict_mode = false  # Undefined variables become empty

[commands.example]
cmd = "echo ${UNDEFINED_VAR}"  # Expands to empty string
```

### Security Features

- **No shell evaluation**: Variables are replaced before shell execution
- **No code injection**: Special characters are not interpreted
- **Recursive limit**: Maximum 10 levels of nested expansion
- **Type safety**: All expansions are string-based

**Safe example**:
```bash
$ MALICIOUS='; rm -rf /' cmdrun run example
# ${MALICIOUS} is safely replaced, not executed as shell code
```

---

## Platform-Specific Commands

Define different commands for different operating systems.

### Platform Table Syntax

```toml
[commands.open-browser]
description = "Open browser"
cmd.unix = "open http://localhost:3000"
cmd.linux = "xdg-open http://localhost:3000"
cmd.macos = "open http://localhost:3000"
cmd.windows = "start http://localhost:3000"
```

### Platform Detection

**Current platform**:
- `windows` - Windows
- `macos` - macOS
- `linux` - Linux
- `unix` - Other Unix-like systems (FreeBSD, etc.)

**Fallback behavior**:
- Linux/macOS commands fall back to `unix` if platform-specific not found
- Windows has no fallback

### Platform Fields

#### Available platform fields:

- `cmd.unix` - Unix-like systems (fallback for Linux/macOS)
- `cmd.linux` - Linux only
- `cmd.macos` - macOS only
- `cmd.windows` - Windows only

### Platform Restriction

Combine platform-specific commands with platform restrictions:

```toml
[commands.install]
description = "Install dependencies"
platform = ["unix", "windows"]  # Only these platforms
cmd.unix = "make install"
cmd.windows = ".\install.ps1"
```

### Examples

#### Cross-platform build:
```toml
[commands.build]
description = "Build native binary"
cmd.unix = "cargo build --release --target x86_64-unknown-linux-gnu"
cmd.macos = "cargo build --release --target x86_64-apple-darwin"
cmd.windows = "cargo build --release --target x86_64-pc-windows-msvc"
```

#### Platform-specific tools:
```toml
[commands.clipboard]
description = "Copy to clipboard"
cmd.macos = "pbcopy"
cmd.linux = "xclip -selection clipboard"
cmd.windows = "clip"
```

---

## Environment Variables

### Global Environment

Defined in `[config.env]`, available to all commands:

```toml
[config.env]
NODE_ENV = "development"
RUST_BACKTRACE = "1"
DATABASE_URL = "postgresql://localhost/mydb"
LOG_LEVEL = "debug"
```

### Command Environment

Defined per-command, overrides global environment:

```toml
[commands.prod-test]
cmd = "npm test"
env = { NODE_ENV = "production" }  # Overrides global NODE_ENV
```

### Environment Priority

1. **Command-specific** (highest priority)
2. **Global config**
3. **System environment**

```toml
[config.env]
PORT = "3000"

[commands.dev]
cmd = "npm run dev"
env = { PORT = "8080" }  # Uses 8080

[commands.start]
cmd = "npm run dev"
# Uses 3000 (from global)
```

### Variable Expansion in Environment

Environment variables can reference other variables:

```toml
[config.env]
BASE_URL = "http://localhost"
API_URL = "${BASE_URL}:3000/api"
WEB_URL = "${BASE_URL}:8080"

[commands.test]
cmd = "curl ${API_URL}/health"
```

### Inline Environment Syntax

```toml
[commands.dev]
cmd = "npm run dev"
env = { PORT = "3000", NODE_ENV = "development" }
```

**Equivalent block syntax**:
```toml
[commands.dev]
cmd = "npm run dev"

[commands.dev.env]
PORT = "3000"
NODE_ENV = "development"
```

---

## Complete Examples

### Web Development Project

```toml
[config]
shell = "bash"
language = "english"
timeout = 300

[config.env]
NODE_ENV = "development"
API_URL = "http://localhost:3000"

# Development
[commands.dev]
description = "Start development server"
cmd = "npm run dev"
env = { PORT = "3000" }

[commands."dev:api"]
description = "Start API server"
cmd = "npm run api"
working_dir = "./api"
env = { PORT = "3001" }

# Building
[commands.build]
description = "Production build"
cmd = [
    "npm run type-check",
    "npm run lint",
    "npm run build",
]
env = { NODE_ENV = "production" }

# Testing
[commands.test]
description = "Run tests"
cmd = "npm test"
deps = ["build"]

[commands."test:watch"]
description = "Run tests in watch mode"
cmd = "npm test -- --watch"
timeout = 0

# Quality checks
[commands.lint]
description = "Run linters"
parallel = true
cmd = [
    "eslint src/",
    "stylelint src/**/*.css",
]

[commands."lint:fix"]
description = "Auto-fix linting issues"
cmd = [
    "eslint src/ --fix",
    "stylelint src/**/*.css --fix",
]

# Deployment
[commands.deploy]
description = "Deploy to production"
cmd = "npm run build && firebase deploy"
deps = ["test"]
confirm = true
env = { NODE_ENV = "production" }

# Aliases
[aliases]
d = "dev"
b = "build"
t = "test"
l = "lint"

# Hooks
[hooks]
pre_run = "echo '🚀 Starting...'"
post_run = "echo '✓ Done!'"

[hooks.commands.deploy]
pre_run = "git diff --exit-code"
post_run = "echo 'Deployed at $(date)' >> deploy.log"
```

### Rust CLI Project

```toml
[config]
shell = "bash"
strict_mode = true

[config.env]
RUST_BACKTRACE = "1"
CARGO_TERM_COLOR = "always"

# Development
[commands.dev]
description = "Development with auto-reload"
cmd = "cargo watch -x run"
timeout = 0

[commands.run]
description = "Run the application"
cmd = "cargo run"

# Building
[commands.build]
description = "Build release binary"
cmd = "cargo build --release"

[commands."build:debug"]
description = "Build debug binary"
cmd = "cargo build"

# Testing
[commands.test]
description = "Run all tests"
cmd = "cargo test --all-features"

[commands."test:unit"]
description = "Run unit tests"
cmd = "cargo test --lib"

[commands."test:integration"]
description = "Run integration tests"
cmd = "cargo test --test '*'"
deps = ["build"]

[commands.coverage]
description = "Generate test coverage"
cmd = "cargo tarpaulin --out Html"
platform = ["unix", "linux"]

# Code quality
[commands.check]
description = "Check code quality"
parallel = true
cmd = [
    "cargo fmt -- --check",
    "cargo clippy -- -D warnings",
]

[commands.fmt]
description = "Format code"
cmd = "cargo fmt"

[commands.clippy]
description = "Run Clippy linter"
cmd = "cargo clippy --all-targets --all-features -- -D warnings"

# Security
[commands.audit]
description = "Security audit"
cmd = "cargo audit"

[commands.outdated]
description = "Check for outdated dependencies"
cmd = "cargo outdated"

# Release
[commands.release]
description = "Create release build"
cmd = [
    "cargo test --all-features",
    "cargo build --release",
    "strip target/release/myapp",
]
confirm = true
timeout = 1800

# Benchmarks
[commands.bench]
description = "Run benchmarks"
cmd = "cargo bench"

# Aliases
[aliases]
d = "dev"
b = "build"
t = "test"
c = "check"
r = "run"
```

### Monorepo with Multiple Services

```toml
[config]
shell = "bash"

[config.env]
COMPOSE_PROJECT_NAME = "myapp"

# Root commands
[commands.install]
description = "Install all dependencies"
cmd.unix = "npm install && cd api && cargo build"
cmd.windows = "npm install; cd api; cargo build"

[commands.clean]
description = "Clean all build artifacts"
cmd = [
    "rm -rf node_modules",
    "rm -rf api/target",
    "rm -rf web/dist",
]

# Web frontend
[commands."web:dev"]
description = "Start web frontend"
cmd = "npm run dev"
working_dir = "./web"
env = { PORT = "3000" }

[commands."web:build"]
description = "Build web frontend"
cmd = "npm run build"
working_dir = "./web"

# API backend
[commands."api:dev"]
description = "Start API server"
cmd = "cargo run"
working_dir = "./api"
env = { RUST_LOG = "debug" }

[commands."api:build"]
description = "Build API server"
cmd = "cargo build --release"
working_dir = "./api"

# Database
[commands."db:start"]
description = "Start database"
cmd = "docker-compose up -d postgres"

[commands."db:migrate"]
description = "Run database migrations"
cmd = "diesel migration run"
working_dir = "./api"
deps = ["db:start"]

[commands."db:reset"]
description = "Reset database"
cmd = "diesel database reset"
working_dir = "./api"
confirm = true

# Docker
[commands."docker:up"]
description = "Start all services"
cmd = "docker-compose up -d"

[commands."docker:down"]
description = "Stop all services"
cmd = "docker-compose down"

[commands."docker:build"]
description = "Build all images"
cmd = "docker-compose build"

# Development
[commands.dev]
description = "Start all development servers"
parallel = true
cmd = [
    "cmdrun run web:dev",
    "cmdrun run api:dev",
]
deps = ["db:start"]

# Testing
[commands.test]
description = "Run all tests"
parallel = true
cmd = [
    "cd web && npm test",
    "cd api && cargo test",
]

# CI/CD
[commands.ci]
description = "Full CI pipeline"
cmd = [
    "cmdrun run lint",
    "cmdrun run test",
    "cmdrun run build",
]

[commands.build]
description = "Build all services"
parallel = true
cmd = [
    "cmdrun run web:build",
    "cmdrun run api:build",
]

[commands.lint]
description = "Lint all code"
parallel = true
cmd = [
    "cd web && npm run lint",
    "cd api && cargo clippy",
]

# Hooks
[hooks]
pre_run = "echo '🚀 Running command...'"

[hooks.commands.deploy]
pre_run = "git diff --exit-code || (echo 'Uncommitted changes!' && exit 1)"
post_run = "echo 'Deployed successfully at $(date)' | tee -a deploy.log"
```

### Platform-Specific Development

```toml
[config]
language = "english"

# Cross-platform build
[commands.build]
description = "Build for current platform"
cmd.linux = "cargo build --release --target x86_64-unknown-linux-gnu"
cmd.macos = "cargo build --release --target x86_64-apple-darwin"
cmd.windows = "cargo build --release --target x86_64-pc-windows-msvc"

# Platform-specific tools
[commands."open:browser"]
description = "Open development server"
cmd.unix = "open http://localhost:3000"
cmd.linux = "xdg-open http://localhost:3000"
cmd.windows = "start http://localhost:3000"

[commands."open:editor"]
description = "Open in default editor"
cmd.macos = "open -a 'Visual Studio Code' ."
cmd.linux = "code ."
cmd.windows = "code ."

# System-specific setup
[commands.setup]
description = "Setup development environment"
cmd.macos = "brew install rust nodejs postgresql"
cmd.linux = "sudo apt-get install -y rustc nodejs postgresql"
cmd.windows = "choco install rust nodejs postgresql"
confirm = true

# Cross-compilation
[commands."build:all"]
description = "Build for all platforms"
parallel = true
cmd = [
    "cargo build --release --target x86_64-unknown-linux-gnu",
    "cargo build --release --target x86_64-apple-darwin",
    "cargo build --release --target x86_64-pc-windows-msvc",
]
platform = ["linux", "macos"]  # Can't cross-compile from Windows easily
```

---

## Best Practices

### 1. Use Descriptive Names

```toml
# ✅ Good
[commands."test:integration"]
[commands."docker:build"]
[commands."deploy:staging"]

# ❌ Avoid
[commands.t1]
[commands.x]
```

### 2. Add Descriptions

```toml
# ✅ Good
[commands.deploy]
description = "Deploy to production (requires confirmation)"
cmd = "kubectl apply -f production.yaml"
confirm = true

# ❌ Avoid
[commands.deploy]
cmd = "kubectl apply -f production.yaml"
```

### 3. Use Dependencies

```toml
# ✅ Good
[commands.deploy]
deps = ["test", "build"]
cmd = "scp dist/ server:/var/www"

# ❌ Avoid (manual dependency management)
[commands.deploy]
cmd = "cmdrun run test && cmdrun run build && scp dist/ server:/var/www"
```

### 4. Leverage Variable Expansion

```toml
# ✅ Good
[commands.deploy]
cmd = "scp dist/ ${DEPLOY_HOST:?DEPLOY_HOST required}:/var/www"

# ❌ Avoid (hardcoded values)
[commands.deploy]
cmd = "scp dist/ prod.example.com:/var/www"
```

### 5. Use Strict Mode

```toml
[config]
strict_mode = true  # Catch undefined variables early
```

### 6. Set Appropriate Timeouts

```toml
[commands.watch]
timeout = 0  # Long-running watcher

[commands.build]
timeout = 600  # 10 minutes for complex build

[commands.test]
timeout = 300  # 5 minutes default
```

### 7. Confirm Dangerous Operations

```toml
[commands."db:reset"]
description = "Reset database (deletes all data!)"
cmd = "diesel database reset"
confirm = true  # Prevent accidents
```

### 8. Use Hooks for Common Tasks

```toml
[hooks]
pre_run = "echo '$(date) - Starting: ${CMDRUN_COMMAND}' >> .cmdrun.log"
post_run = "echo '$(date) - Completed: ${CMDRUN_COMMAND}' >> .cmdrun.log"
```

---

## Troubleshooting

### Configuration Not Found

**Error**: `Configuration file not found`

**Solution**: Ensure one of these files exists:
- `commands.toml`
- `.cmdrun.toml`
- `~/.cmdrun/commands.toml`

### Parse Errors

**Error**: `Failed to parse TOML`

**Common causes**:
- Missing quotes around strings with special characters
- Incorrect indentation
- Missing closing brackets

**Example**:
```toml
# ❌ Wrong
[commands.test]
cmd = echo hello  # Missing quotes

# ✅ Correct
[commands.test]
cmd = "echo hello"
```

### Variable Expansion Errors

**Error**: `Undefined variable: VAR_NAME`

**Solution**: Set the variable or use default value:
```toml
cmd = "${VAR_NAME:-default_value}"
```

Or disable strict mode:
```toml
[config]
strict_mode = false
```

### Circular Dependencies

**Error**: `Circular dependency detected`

**Solution**: Remove circular references in `deps`:
```toml
# ❌ Circular
[commands.a]
deps = ["b"]

[commands.b]
deps = ["a"]

# ✅ Fixed
[commands.a]
deps = ["b"]

[commands.b]
cmd = "..."
```

---

## See Also

- [CLI Reference](CLI.md) - Command-line usage
- [Installation Guide](INSTALLATION.md) - Installation instructions
- [Internationalization](I18N.md) - Language settings

---

**Next**: [CLI Reference](CLI.md) - Learn how to use cmdrun commands
