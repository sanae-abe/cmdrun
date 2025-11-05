# cmdrun - Fast, Secure, and Cross-platform Command Runner

[English](README.md) | [日本語](README.ja.md)

> A modern replacement for `package.json` scripts and Makefiles, written in Rust.

[![Crates.io](https://img.shields.io/crates/v/cmdrun.svg)](https://crates.io/crates/cmdrun)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE)
[![Build Status](https://github.com/sanae-abe/cmdrun/workflows/CI/badge.svg)](https://github.com/sanae-abe/cmdrun/actions)

## Table of Contents

- [Why cmdrun?](#why-cmdrun)
- [Quick Start](#quick-start)
- [Features](#features)
- [Documentation](#documentation)
- [Comparison](#comparison)
- [Performance Benchmarks](#performance-benchmarks)
- [Examples](#examples)
- [Contributing](#contributing)
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
- **Works everywhere**: Linux, macOS, Windows, FreeBSD
- **Shell detection**: Auto-detects bash/zsh/fish/pwsh
- **Native binaries**: No runtime dependencies

### 💎 Developer Experience
- **TOML configuration** - Type-safe, easy to read
- **Powerful features** - Dependencies, parallel execution, hooks
- **Great errors** - Detailed error messages with context

## Quick Start

### System Requirements

- **Operating System**: Linux, macOS, Windows, FreeBSD
- **For building from source**: Rust 1.70+ (MSRV)

### Installation

#### Install Rust Toolchain (if not already installed)

```bash
# 1. Download and run Rustup (Rust installer)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 2. Load environment variables
source ~/.cargo/env

# Or open a new terminal, or run:
# For bash
source ~/.bashrc

# For zsh (macOS default)
source ~/.zshrc

# 3. Verify installation
rustc --version
cargo --version
```

#### Install cmdrun

**Option 1: From Source (Recommended for Development)**

```bash
# 1. Clone the repository
git clone ssh://git@rendezvous.m3.com:3789/sanae-abe/cmdrun.git
cd cmdrun

# 2. Build and install
cargo install --path .

# 3. Verify installation
cmdrun --version
cmdrun --help
```

**Option 2: From crates.io**

```bash
cargo install cmdrun
```

#### Update

```bash
# If installed from source
cd cmdrun  # Navigate to project directory
git pull

# Rebuild and install
cargo install --path . --force
```

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

### Basic Usage

#### 1. Initialize a new project

```bash
# Create commands.toml with a template
cmdrun init

# Or use a specific template
cmdrun init --template rust  # For Rust projects
cmdrun init --template node  # For Node.js projects
cmdrun init --template web   # For web development
cmdrun init --template python # For Python projects

# Interactive template selection
cmdrun init --interactive
```

#### 2. Or create manually

Create a `commands.toml` in your project:

```toml
[config]
language = "english"  # Optional: "english" (default) or "japanese"

[commands.dev]
description = "Start development server"
cmd = "npm run dev"

[commands.build]
description = "Build for production"
cmd = [
    "npm run type-check",
    "npm run lint",
    "npm run build",
]

[commands.test]
description = "Run tests"
cmd = "cargo test --all-features"
```

#### 3. Run commands

```bash
# Run a command
cmdrun run dev

# List available commands
cmdrun list

# Visualize dependency graph
cmdrun graph

# Manage configuration
cmdrun config show              # Show all settings
cmdrun config get language      # Get a specific setting
cmdrun config set language japanese  # Change language to Japanese

# Show help
cmdrun --help
```

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

Run dependencies in parallel:
```bash
cmdrun run build --parallel
```

### Dependency Graph Visualization

Visualize command dependencies in multiple formats:

```bash
# Tree format (default, colorful)
cmdrun graph build

# Show execution groups for parallel execution
cmdrun graph build --show-groups

# Export as DOT format (Graphviz)
cmdrun graph build --format dot --output deps.dot
dot -Tpng deps.dot -o deps.png

# Export as Mermaid diagram
cmdrun graph build --format mermaid --output deps.mmd
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

## Documentation

### User Guide
- [Installation Guide](docs/user-guide/INSTALLATION.md)
- [CLI Reference](docs/user-guide/CLI.md)
- [Configuration Reference](docs/user-guide/CONFIGURATION.md)
- [Internationalization (i18n)](docs/user-guide/I18N.md)

### Technical Documentation
- [Performance](docs/technical/PERFORMANCE.md)
- [Security](docs/technical/SECURITY.md)
- [Cross-platform Support](docs/technical/CROSS_PLATFORM.md)
- [Distribution](docs/technical/DISTRIBUTION.md)

### Development
- [Contributing](CONTRIBUTING.md)
- [Roadmap](docs/development/ROADMAP.md)

## Comparison

### vs npm scripts

```json
// package.json (Node.js)
{
  "scripts": {
    "build": "tsc && webpack",
    "test": "jest",
    "deploy": "npm run build && scp -r dist/ user@host:/path"
  }
}
```

vs

```toml
# commands.toml (cmdrun)
[commands.build]
cmd = ["tsc", "webpack"]

[commands.test]
cmd = "jest"

[commands.deploy]
cmd = "scp -r dist/ ${DEPLOY_USER}@${DEPLOY_HOST}:${DEPLOY_PATH}"
deps = ["build"]
```

**Benefits**:
- ✅ ~29x faster startup
- ✅ Type-safe configuration
- ✅ Dependency management
- ✅ Variable expansion
- ✅ Platform-specific commands

### vs Makefile

```makefile
# Makefile
.PHONY: build test

build:
	cargo build --release

test: build
	cargo test
```

vs

```toml
# commands.toml
[commands.build]
cmd = "cargo build --release"

[commands.test]
cmd = "cargo test"
deps = ["build"]
```

**Benefits**:
- ✅ Easier syntax (TOML vs Make's tab-sensitivity)
- ✅ Cross-platform (no GNU Make required)
- ✅ Better error messages
- ✅ Variable expansion
- ✅ Parallel execution

## Performance Benchmarks

```bash
# Startup time comparison (measured with hyperfine)
$ hyperfine --shell=none './target/release/cmdrun --version' 'npm --version' --warmup 5

Benchmark 1: ./target/release/cmdrun --version
  Time (mean ± σ):       4.0 ms ±   0.3 ms    [User: 1.3 ms, System: 1.3 ms]
  Range (min … max):     3.5 ms …   4.6 ms    30 runs

Benchmark 2: npm --version
  Time (mean ± σ):     115.4 ms ±  13.0 ms    [User: 59.7 ms, System: 18.9 ms]
  Range (min … max):   104.5 ms … 158.4 ms    30 runs

Summary
  ./target/release/cmdrun --version ran
    28.88 ± 3.79 times faster than npm --version
```

**Key Performance Metrics:**
- **Startup time**: 4ms average (well below 100ms target)
- **Speed improvement**: ~29x faster than npm (28.88 ± 3.79x measured)
- **Memory footprint**: ~10MB vs 200MB+ for Node.js
- **Binary size**: Optimized with LTO and strip

## Examples

<details>
<summary>📱 Web Development</summary>

```toml
[config]
shell = "bash"

[commands.dev]
description = "Start development server"
cmd = "npm run dev"
env = { PORT = "3000", NODE_ENV = "development" }

[commands.build]
description = "Production build"
cmd = [
    "npm run type-check",
    "npm run lint",
    "npm run build",
]

[commands.deploy]
description = "Deploy to production"
cmd = "npm run build && firebase deploy"
deps = ["build"]
confirm = true
```

**Usage:**
```bash
# Start development server
cmdrun run dev

# Build for production (runs type-check, lint, build in sequence)
cmdrun run build

# Deploy (asks for confirmation, runs build first)
cmdrun run deploy

# Visualize build dependencies
cmdrun graph build --show-groups
```
</details>

<details>
<summary>🦀 Rust Project</summary>

```toml
[commands.dev]
cmd = "cargo watch -x run"

[commands.test]
cmd = "cargo test --all-features"

[commands.bench]
cmd = "cargo bench"

[commands.release]
cmd = [
    "cargo test --all-features",
    "cargo build --release",
    "cargo package",
]
confirm = true
```

**Usage:**
```bash
# Watch mode for development
cmdrun run dev

# Run all tests
cmdrun run test

# Create release (with confirmation)
cmdrun run release

# Show dependency graph
cmdrun graph release
```
</details>

<details>
<summary>⚡ Advanced Features</summary>

#### Dependency Management
```toml
[commands.e2e]
description = "Run end-to-end tests"
cmd = "playwright test"
deps = ["build"]  # Automatically runs 'build' before 'e2e'

[commands.ci]
description = "Full CI pipeline"
deps = ["test", "lint", "build"]  # Runs all checks
```

#### Platform-Specific Commands
```toml
[commands.open-browser]
description = "Open browser"
cmd.unix = "open http://localhost:3000"
cmd.windows = "start http://localhost:3000"
cmd.linux = "xdg-open http://localhost:3000"
```

#### Parallel Execution
```toml
[commands.lint-all]
description = "Run all linters in parallel"
parallel = true
cmd = [
    "eslint src/",
    "stylelint src/**/*.css",
    "tsc --noEmit",
]
```

#### Visualize Execution Plan
```bash
# See how commands will be executed
cmdrun graph ci --show-groups

# Output:
# Execution Plan: 3 groups
#
# ▶ Group 1 / 3
#   • lint
#   • test
#   ⚡ Can run in parallel
#
# ▶ Group 2 / 3
#   • build
#
# ▶ Group 3 / 3
#   • ci
```
</details>

## Contributing

We welcome contributions! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for details.

### Development Setup

```bash
# Clone repository
git clone https://github.com/sanae-abe/cmdrun
cd cmdrun

# Build
cargo build

# Run tests
cargo test

# Run benchmarks
cargo bench

# Format code
cargo fmt

# Lint
cargo clippy
```

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Acknowledgments

- Inspired by [npm scripts](https://docs.npmjs.com/cli/v9/using-npm/scripts), [make](https://www.gnu.org/software/make/), and [just](https://github.com/casey/just)
- Built with amazing Rust crates: [clap](https://github.com/clap-rs/clap), [tokio](https://github.com/tokio-rs/tokio), [serde](https://github.com/serde-rs/serde)
- Thanks to all [contributors](https://github.com/sanae-abe/cmdrun/graphs/contributors)

## Support

- 📖 [Documentation](https://sanae-abe.github.io/cmdrun)
- 💬 [Discussions](https://github.com/sanae-abe/cmdrun/discussions)
- 🐛 [Issue Tracker](https://github.com/sanae-abe/cmdrun/issues)

<!-- Future support channels (commented out until available)
- 🐦 [Twitter](https://twitter.com/sanae_abe)
-->

---

<p align="center">
  Made with ❤️ in Rust
</p>
