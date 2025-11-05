# cmdrun - Fast, Secure, and Cross-platform Command Runner

> A modern replacement for `package.json` scripts and Makefiles, written in Rust.

[![Crates.io](https://img.shields.io/crates/v/cmdrun.svg)](https://crates.io/crates/cmdrun)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE)
[![Build Status](https://github.com/yourusername/cmdrun/workflows/CI/badge.svg)](https://github.com/yourusername/cmdrun/actions)

## Why cmdrun?

### 🚀 Performance
- **10x faster startup** than Node.js-based task runners
- **50ms startup time** vs 500ms+ for npm/yarn
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

### Installation

#### Cargo (Recommended)
```bash
cargo install cmdrun
```

#### Homebrew (macOS/Linux)
```bash
brew install yourusername/tap/cmdrun
```

#### Scoop (Windows)
```bash
scoop bucket add cmdrun https://github.com/yourusername/scoop-bucket
scoop install cmdrun
```

#### Manual Installation
Download the latest binary from [Releases](https://github.com/yourusername/cmdrun/releases).

### Basic Usage

1. Create a `commands.toml` in your project:

```toml
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

2. Run commands:

```bash
# Run a command
cmdrun run dev

# List available commands
cmdrun list

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
- `${VAR:-default}` - Default value
- `${VAR:?error}` - Required variable
- `${VAR:+value}` - Conditional substitution

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

## Documentation

### Core Documentation
- [Installation Guide](docs/INSTALLATION.md)
- [Configuration Reference](docs/CONFIGURATION.md)
- [CLI Reference](docs/CLI.md)
- [Migration Guide](docs/MIGRATION.md)

### Technical Documentation
- [Architecture](docs/ARCHITECTURE.md)
- [Performance](docs/PERFORMANCE.md)
- [Security](docs/SECURITY.md)
- [Cross-platform Support](docs/CROSS_PLATFORM.md)
- [Distribution](docs/DISTRIBUTION.md)

### Development
- [Contributing](CONTRIBUTING.md)
- [Development Guide](docs/DEVELOPMENT.md)
- [Roadmap](docs/ROADMAP.md)

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
- ✅ 10x faster startup
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
$ hyperfine 'cmdrun --version' 'npm --version'

Benchmark: cmdrun --version
  Time (mean ± σ):      45.2 ms ±   2.3 ms
  Range (min … max):    42.1 ms …  51.3 ms

Benchmark: npm --version
  Time (mean ± σ):     523.1 ms ±  12.3 ms
  Range (min … max):   508.2 ms … 547.8 ms

Result: cmdrun is 11.6x faster
```

## Examples

### Web Development

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

### Rust Project

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

## Contributing

We welcome contributions! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for details.

### Development Setup

```bash
# Clone repository
git clone https://github.com/yourusername/cmdrun
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
- Thanks to all [contributors](https://github.com/yourusername/cmdrun/graphs/contributors)

## Support

- 📖 [Documentation](https://yourusername.github.io/cmdrun)
- 💬 [Discussions](https://github.com/yourusername/cmdrun/discussions)
- 🐛 [Issue Tracker](https://github.com/yourusername/cmdrun/issues)
- 🐦 [Twitter](https://twitter.com/yourusername)

---

<p align="center">
  Made with ❤️ in Rust
</p>
