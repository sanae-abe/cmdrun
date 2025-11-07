# cmdrun FAQ (Frequently Asked Questions)

## Table of Contents

- [General Questions](#general-questions)
- [Installation & Setup](#installation--setup)
- [Configuration](#configuration)
- [Usage](#usage)
- [Comparison with Other Tools](#comparison-with-other-tools)
- [Migration](#migration)
- [Troubleshooting](#troubleshooting)
- [Advanced Topics](#advanced-topics)

---

## General Questions

### What is cmdrun?

cmdrun is a **fast, secure, and cross-platform command runner** designed for personal global command management. It allows you to:

- Register frequently-used commands once and run them from anywhere
- Manage environment-specific commands (work, personal, projects)
- Execute commands with dependencies, parallel execution, and file watching
- Use safe variable expansion without eval() vulnerabilities

**Key Features:**
- **29x faster startup** than Node.js-based task runners (4ms vs 115ms)
- **Zero eval()** - no dynamic code execution vulnerabilities
- **Cross-platform** - Linux, macOS, Windows, FreeBSD
- **Type-safe TOML configuration** - easier to read and validate

### Why should I use cmdrun instead of npm scripts or Makefiles?

**vs npm scripts:**

| Feature | cmdrun | npm scripts |
|---------|--------|-------------|
| Startup time | 4ms | 115ms+ |
| Global commands | Yes (anywhere) | No (per-project) |
| Security | No eval() | Uses eval() |
| Configuration | TOML (readable) | JSON (limited) |
| Dependencies | Built-in | Manual |
| Watch mode | Built-in | Requires nodemon |
| Memory usage | 10MB | 200MB+ |

**vs Makefiles:**

| Feature | cmdrun | Make |
|---------|--------|------|
| Configuration | TOML (modern) | Makefile (1970s syntax) |
| Cross-platform | Excellent | Tab/shell issues |
| Variable syntax | `${VAR}` | `$(VAR)` (confusing) |
| Error messages | Clear, colored | Cryptic |
| Dependencies | Automatic | Manual `.PHONY` |
| Parallel execution | Built-in | `-j` flag |

**When to use cmdrun:**
- You want a personal command management tool
- You need global commands accessible from any directory
- You value security (no eval) and performance (4ms startup)
- You want cross-platform compatibility

**When to use alternatives:**
- Team collaboration requiring package.json (use npm scripts)
- Complex build systems with incremental compilation (use Make/Bazel)
- Project-specific tasks that shouldn't be global

### Is cmdrun production-ready?

**Yes**, cmdrun 1.0.0 is production-ready with:

- Comprehensive test suite (unit, integration, security tests)
- Memory safety guaranteed by Rust
- No eval() or dynamic code execution
- Extensive error handling
- Cross-platform support
- Security-focused design (see [SECURITY.md](../technical/SECURITY.md))

**Production usage considerations:**
- Test your commands in a non-production environment first
- Use `confirm = true` for destructive operations
- Set appropriate timeouts for long-running commands
- Review security best practices in the documentation

### What platforms are supported?

**Fully Supported:**
- Linux (x86_64, ARM64)
- macOS (Intel, Apple Silicon)
- Windows 10+ (x86_64)
- FreeBSD (x86_64)

**Shell Support:**
- bash (default on Unix)
- zsh
- fish
- PowerShell (default on Windows)
- cmd (Windows)

**Minimum Requirements:**
- Rust 1.75+ (for building from source)
- No runtime dependencies (native binary)

---

## Installation & Setup

### How do I install cmdrun?

**From Source (Current Method):**

```bash
# 1. Install Rust toolchain (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# 2. Clone and build cmdrun
git clone git@github.com:sanae-abe/cmdrun.git
cd cmdrun
cargo install --path .

# 3. Verify installation
cmdrun --version
```

**Future Distribution Methods:**
- Binary releases (GitHub Releases) - Coming soon
- Homebrew (macOS/Linux) - Planned
- Cargo (crates.io) - Planned
- APT/RPM packages (Linux) - Planned
- Chocolatey/Scoop (Windows) - Planned

See [DISTRIBUTION.md](../technical/DISTRIBUTION.md) for details.

### Where should I put my configuration file?

**Default Locations:**
- **Linux/macOS:** `~/.config/cmdrun/commands.toml`
- **Windows:** `%APPDATA%\cmdrun\commands.toml`

The config file is created automatically on first run.

**Custom Locations:**
```bash
# Use custom config file
cmdrun --config ~/work/commands.toml list

# Project-specific config
cmdrun -c ./commands.toml run dev

# Environment-specific configs
cmdrun -c ~/.cmdrun/production.toml run deploy
cmdrun -c ~/.cmdrun/staging.toml run deploy
```

**Recommended Setup:**
```
~/.config/cmdrun/
├── commands.toml       # Default/personal commands
├── work.toml           # Work-related commands
└── projects/
    ├── project-a.toml  # Project A commands
    └── project-b.toml  # Project B commands
```

### How do I upgrade cmdrun?

**From Source:**
```bash
cd cmdrun  # Your local clone
git pull origin main
cargo install --path . --force
```

**Future (with binary releases):**
```bash
# Self-update command (planned)
cmdrun self-update

# Or download latest binary from GitHub Releases
```

### How do I uninstall cmdrun?

```bash
# 1. Remove binary
cargo uninstall cmdrun

# 2. Remove configuration (optional)
# Linux/macOS
rm -rf ~/.config/cmdrun

# Windows (PowerShell)
Remove-Item -Recurse -Force "$env:APPDATA\cmdrun"

# 3. Remove source code (optional)
# cd .. && rm -rf cmdrun
```

**Note:** Configuration files are NOT removed automatically. Back them up if needed!

---

## Configuration

### How do I add my first command?

**Interactive Method (Recommended):**
```bash
cmdrun add
# Follow prompts:
# - Command ID: dev
# - Description: Start development server
# - Command: npm run dev
```

**Direct Method:**
```bash
cmdrun add dev "npm run dev" "Start development server"
```

**Manual TOML Editing:**
```toml
# ~/.config/cmdrun/commands.toml
[commands.dev]
description = "Start development server"
cmd = "npm run dev"
```

### How do I organize commands for different projects?

**Option 1: Single Config with Prefixes**
```toml
# ~/.config/cmdrun/commands.toml
[commands.webapp-dev]
description = "Web app dev server"
cmd = "cd ~/projects/webapp && npm run dev"

[commands.api-dev]
description = "API dev server"
cmd = "cd ~/projects/api && cargo run"
```

**Option 2: Separate Config Files (Recommended)**
```bash
# Create project-specific configs
cmdrun -c ~/projects/webapp/commands.toml add dev "npm run dev"
cmdrun -c ~/projects/api/commands.toml add dev "cargo run"

# Use them
cd ~/projects/webapp
cmdrun -c ./commands.toml run dev

cd ~/projects/api
cmdrun -c ./commands.toml run dev
```

**Option 3: Shell Aliases**
```bash
# ~/.bashrc or ~/.zshrc
alias webapp='cmdrun -c ~/projects/webapp/commands.toml'
alias api='cmdrun -c ~/projects/api/commands.toml'

# Usage
webapp run dev
api run dev
```

### Can I use environment variables in commands?

**Yes!** cmdrun supports safe variable expansion:

```toml
[commands.deploy]
description = "Deploy to server"
cmd = "rsync -avz dist/ ${DEPLOY_USER}@${DEPLOY_HOST}:${DEPLOY_PATH}"

[commands.backup]
description = "Backup database"
cmd = "pg_dump ${DB_NAME:-mydb} > backup_$(date +%Y%m%d).sql"
```

**Usage:**
```bash
# Set environment variables
export DEPLOY_USER="admin"
export DEPLOY_HOST="production.example.com"
export DEPLOY_PATH="/var/www"

# Run command
cmdrun run deploy
```

**Supported Syntax:**
- `${VAR}` - Basic expansion
- `${VAR:-default}` - Default value if VAR is unset
- `${VAR:?error message}` - Error if VAR is unset
- `${VAR:+value}` - Use value if VAR is set

### How do I use positional arguments?

```toml
[commands.convert]
description = "Convert image format"
cmd = "sharp -i ${1} -f ${2:-webp} -q ${3:-80} -o ${4:-output.webp}"
```

**Usage:**
```bash
# All arguments
cmdrun run convert input.png jpeg 90 output.jpg
# Expands to: sharp -i input.png -f jpeg -q 90 -o output.jpg

# With defaults
cmdrun run convert input.png
# Expands to: sharp -i input.png -f webp -q 80 -o output.webp
```

### How do I run commands before/after other commands?

**Use hooks:**

```toml
# Global hooks (apply to all commands)
[hooks]
pre_run = "echo 'Starting...'"
post_run = "echo 'Done!'"

# Command-specific hooks
[hooks.commands.deploy]
pre_run = "git diff --exit-code"  # Ensure no uncommitted changes
post_run = "echo $(date) >> deploy.log"  # Log deployment time

[commands.deploy]
description = "Deploy to production"
cmd = "scp dist/ user@server:/var/www"
```

**Execution Order:**
1. Global `pre_run`
2. Command-specific `pre_run`
3. Main command
4. Command-specific `post_run`
5. Global `post_run`

---

## Usage

### How do I list all available commands?

```bash
# List all commands
cmdrun list

# Search for specific commands
cmdrun search docker
cmdrun search deploy

# Show detailed information
cmdrun info deploy
```

### Can I run multiple commands in sequence?

**Yes, multiple ways:**

**Method 1: Multiple Commands in cmd**
```toml
[commands.test-and-build]
description = "Run tests then build"
cmd = [
    "cargo test",
    "cargo build --release"
]
```

**Method 2: Dependencies**
```toml
[commands.deploy]
description = "Deploy (after building)"
cmd = "scp target/release/app user@server:/app"
deps = ["build"]  # Run 'build' first

[commands.build]
description = "Build release"
cmd = "cargo build --release"
```

**Method 3: Shell &&**
```toml
[commands.quick-deploy]
cmd = "cargo build --release && scp target/release/app user@server:/app"
```

### Can I run commands in parallel?

**Yes:**

```toml
[commands.check-all]
description = "Run all checks in parallel"
parallel = true
cmd = [
    "cargo fmt -- --check",
    "cargo clippy",
    "cargo test"
]
```

**Usage:**
```bash
cmdrun run check-all
# All three commands run simultaneously
```

**Note:** Use parallel execution only for independent commands!

### How do I watch files and auto-run commands?

```bash
# Watch Rust files and rebuild
cmdrun watch build --pattern "**/*.rs"

# Watch tests with 1-second debounce
cmdrun watch test --pattern "**/*.rs" --debounce 1000

# Watch multiple directories
cmdrun watch dev --path src --path lib --pattern "**/*.rs"

# Custom exclusions
cmdrun watch dev --pattern "**/*.ts" --ignore "**/*.test.ts"
```

**Watch Mode Features:**
- Glob pattern filtering (`**/*.rs`, `**/*.ts`)
- Debouncing (default 500ms)
- .gitignore integration
- Recursive watching (default)
- Auto-exclude common directories (node_modules, target, .git)

See [WATCH_MODE.md](WATCH_MODE.md) for details.

### How do I change the UI language?

```toml
# ~/.config/cmdrun/commands.toml
[config]
language = "japanese"  # or "english"
```

**Or via command:**
```bash
cmdrun config set language japanese
cmdrun config set language english
```

**Supported Languages:**
- English (default)
- Japanese (日本語)

---

## Comparison with Other Tools

### cmdrun vs npm/yarn scripts

**Use cmdrun if:**
- You want global commands (accessible from any directory)
- You need faster startup (4ms vs 115ms+)
- You want security (no eval)
- You need personal command management

**Use npm scripts if:**
- You're working in a Node.js project
- Your team needs package.json for collaboration
- You need npm-specific features (lifecycle scripts)

**Migration from npm scripts:**

```json
// package.json
{
  "scripts": {
    "dev": "webpack serve --mode development",
    "build": "webpack --mode production",
    "test": "jest"
  }
}
```

```toml
# commands.toml
[commands.dev]
cmd = "npm run dev"
# or directly: cmd = "webpack serve --mode development"

[commands.build]
cmd = "npm run build"

[commands.test]
cmd = "npm test"
```

**Bonus:** Add project-independent commands!
```toml
[commands.ssh-prod]
cmd = "ssh user@production.example.com"

[commands.db-backup]
cmd = "pg_dump mydb > backup_$(date +%Y%m%d).sql"
```

### cmdrun vs Make

**Use cmdrun if:**
- You want modern, readable configuration (TOML)
- You need cross-platform compatibility
- You want better error messages
- You don't need incremental compilation

**Use Make if:**
- You need incremental builds (only rebuild changed files)
- You're working with C/C++ projects (Make is standard)
- Your team is familiar with Makefiles

**Migration from Make:**

```makefile
# Makefile
.PHONY: build test clean

build:
	cargo build --release

test: build
	cargo test

clean:
	cargo clean
```

```toml
# commands.toml
[commands.build]
description = "Build release binary"
cmd = "cargo build --release"

[commands.test]
description = "Run tests"
cmd = "cargo test"
deps = ["build"]  # Automatic dependency

[commands.clean]
description = "Clean build artifacts"
cmd = "cargo clean"
```

### cmdrun vs just

**Similarities:**
- Modern alternatives to Make
- TOML-like configuration
- Cross-platform
- Fast startup

**cmdrun advantages:**
- Global command management (not just project-local)
- Custom config file support (`-c` flag)
- Built-in watch mode
- Internationalization (i18n)
- Variable expansion with `${VAR}`

**just advantages:**
- More Make-like (recipes, working directory per command)
- Recipe parameters
- String interpolation with `{{}}`

**Choose cmdrun for:**
- Personal global command management
- Multi-project command organization
- Security-focused use cases

**Choose just for:**
- Project-specific task running
- Make replacement in a single project

---

## Migration

### Migrating from npm scripts

**Step 1: Identify commands**
```bash
# In your project
cat package.json | grep "scripts" -A 10
```

**Step 2: Convert to TOML**
```bash
# Create project-specific config
cmdrun -c ./commands.toml add dev "npm run dev"
cmdrun -c ./commands.toml add build "npm run build"
cmdrun -c ./commands.toml add test "npm test"
```

**Step 3: Use cmdrun**
```bash
cmdrun -c ./commands.toml run dev
```

**Optional: Create alias**
```bash
# ~/.bashrc or ~/.zshrc
alias cr='cmdrun -c ./commands.toml'

# Usage
cr run dev
```

### Migrating from Make

**Step 1: List targets**
```bash
make -qp | grep -E '^[a-zA-Z0-9_-]+:' | sed 's/:.*$//'
```

**Step 2: Convert each target**
```makefile
# Makefile
build:
	cargo build --release

test: build
	cargo test
```

```toml
# commands.toml
[commands.build]
cmd = "cargo build --release"

[commands.test]
cmd = "cargo test"
deps = ["build"]
```

**Step 3: Handle variables**
```makefile
# Makefile
DEPLOY_HOST ?= production.example.com

deploy:
	scp app user@$(DEPLOY_HOST):/app
```

```toml
# commands.toml
[commands.deploy]
cmd = "scp app user@${DEPLOY_HOST:-production.example.com}:/app"
```

### Migrating from shell scripts

**Before: Multiple shell scripts**
```bash
# scripts/dev.sh
#!/bin/bash
npm run dev

# scripts/deploy.sh
#!/bin/bash
npm run build
scp dist/ user@server:/var/www
```

**After: Single TOML config**
```toml
[commands.dev]
cmd = "npm run dev"

[commands.deploy]
cmd = [
    "npm run build",
    "scp dist/ user@server:/var/www"
]
```

**Benefits:**
- No need to manage file permissions (+x)
- No need to add scripts/ to PATH
- Centralized configuration
- Better discoverability (`cmdrun list`)

---

## Troubleshooting

### Command not found after installation

**Issue:** `cmdrun: command not found`

**Solution 1: Check PATH**
```bash
# Verify cargo bin directory is in PATH
echo $PATH | grep -o ~/.cargo/bin

# If not, add to ~/.bashrc or ~/.zshrc
echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc
```

**Solution 2: Verify installation**
```bash
# Check if binary exists
ls -la ~/.cargo/bin/cmdrun

# If not, reinstall
cargo install --path . --force
```

### Config file not found

**Issue:** `Error: Configuration file not found`

**Solution:**
```bash
# Check default location
# Linux/macOS
ls -la ~/.config/cmdrun/commands.toml

# Windows (PowerShell)
ls "$env:APPDATA\cmdrun\commands.toml"

# Create if missing
mkdir -p ~/.config/cmdrun
touch ~/.config/cmdrun/commands.toml

# Or let cmdrun create it
cmdrun add test "echo test" "Test command"
```

### Variable expansion doesn't work

**Issue:** `${VAR}` appears literally in output

**Solution 1: Set environment variable**
```bash
export VAR="value"
cmdrun run your-command
```

**Solution 2: Use default value**
```toml
[commands.example]
cmd = "echo ${VAR:-default-value}"
```

**Solution 3: Check strict mode**
```toml
[config]
strict_mode = false  # Allow undefined variables (empty string)
```

### Permission denied errors

**Issue:** `Permission denied` when running commands

**Solution 1: Check file permissions**
```bash
# Make script executable
chmod +x script.sh
```

**Solution 2: Use full path to shell**
```toml
[commands.example]
cmd = "/bin/bash script.sh"
```

**Solution 3: Check working directory**
```toml
[commands.example]
cmd = "./script.sh"
working_dir = "/path/to/scripts"
```

### Watch mode not detecting changes

**Issue:** File changes don't trigger command execution

**Solution 1: Check pattern**
```bash
# Use correct glob pattern
cmdrun watch build --pattern "**/*.rs"  # Recursive
cmdrun watch build --pattern "*.rs"     # Current dir only
```

**Solution 2: Check gitignore**
```bash
# Watch respects .gitignore by default
# To watch ignored files:
cmdrun watch build --pattern "**/*.rs" --no-gitignore
```

**Solution 3: Increase debounce**
```bash
# Some editors create multiple events
cmdrun watch build --pattern "**/*.rs" --debounce 1000
```

See [TROUBLESHOOTING.md](TROUBLESHOOTING.md) for more solutions.

---

## Advanced Topics

### Can I use cmdrun in CI/CD?

**Yes!** cmdrun works great in CI/CD:

```yaml
# .github/workflows/ci.yml
name: CI
on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3

      - name: Install Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable

      - name: Install cmdrun
        run: cargo install --path .

      - name: Run tests
        run: cmdrun run test

      - name: Build
        run: cmdrun run build
```

**Benefits:**
- Consistent commands between local and CI
- Centralized configuration
- Easy to replicate CI locally

### How do I debug command execution?

**Enable verbose mode:**
```bash
cmdrun -v run your-command
cmdrun --verbose run your-command
```

**Use RUST_BACKTRACE for errors:**
```bash
RUST_BACKTRACE=1 cmdrun run your-command
RUST_BACKTRACE=full cmdrun run your-command
```

**Check execution logs:**
```bash
# Enable trace logging
RUST_LOG=trace cmdrun run your-command

# JSON output for parsing
RUST_LOG=info cmdrun run your-command 2>&1 | jq
```

### Can I extend cmdrun with plugins?

**Not yet.** Plugin system is planned for future releases.

**Current workarounds:**
```toml
# Call external scripts
[commands.custom]
cmd = "/path/to/my-script.sh"

# Use hooks for extensions
[hooks.commands.deploy]
pre_run = "/path/to/pre-deploy-check.sh"
post_run = "/path/to/post-deploy-notify.sh"
```

### How do I contribute to cmdrun?

See [CONTRIBUTING.md](../../CONTRIBUTING.md) for:
- Development setup
- Code style guidelines
- Pull request process
- Testing requirements

**Quick start:**
```bash
# Clone repository
git clone git@github.com:sanae-abe/cmdrun.git
cd cmdrun

# Install development tools
rustup component add rustfmt clippy

# Run tests
cargo test

# Submit PR
# (See CONTRIBUTING.md for details)
```

---

## Still Have Questions?

- **Documentation:** Check [docs/](../) directory
- **Issues:** [GitHub Issues](https://github.com/sanae-abe/cmdrun/issues)
- **Discussions:** [GitHub Discussions](https://github.com/sanae-abe/cmdrun/discussions)
- **Email:** sanae.a.sunny@gmail.com

---

**Last Updated:** 2025-11-07
**Version:** 1.0.0
