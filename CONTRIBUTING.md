# Contributing to cmdrun

Thank you for your interest in contributing to cmdrun! We welcome contributions from everyone, whether you're fixing a typo, adding a feature, or improving documentation.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Setup](#development-setup)
- [Building and Testing](#building-and-testing)
- [Code Style](#code-style)
- [Commit Message Format](#commit-message-format)
- [Pull Request Process](#pull-request-process)
- [Issue Reporting Guidelines](#issue-reporting-guidelines)
- [Community](#community)

## Code of Conduct

### Our Pledge

We are committed to providing a welcoming and inclusive environment for all contributors. We expect everyone to:

- Be respectful and considerate in all interactions
- Use welcoming and inclusive language
- Accept constructive criticism gracefully
- Focus on what's best for the community
- Show empathy towards other community members

### Unacceptable Behavior

- Harassment, discrimination, or offensive comments
- Trolling, insulting remarks, or personal attacks
- Publishing others' private information without permission
- Any conduct that would be inappropriate in a professional setting

### Enforcement

Instances of unacceptable behavior may be reported to the project maintainers. All complaints will be reviewed and investigated, and will result in a response deemed necessary and appropriate to the circumstances.

## Getting Started

### Prerequisites

Before contributing, ensure you have:

- **Rust toolchain** (1.75 or later)
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```
- **Git** for version control
- A GitHub account for submitting pull requests

### Finding Ways to Contribute

- Browse [open issues](https://github.com/sanae-abe/cmdrun/issues) labeled with `good first issue` or `help wanted`
- Check the [roadmap](docs/ROADMAP.md) for planned features
- Improve documentation or add examples
- Report bugs or suggest enhancements

## Development Setup

### 1. Fork and Clone

```bash
# Fork the repository on GitHub, then clone your fork
git clone https://github.com/YOUR_USERNAME/cmdrun.git
cd cmdrun

# Add upstream remote
git remote add upstream https://github.com/sanae-abe/cmdrun.git
```

### 2. Install Development Tools

```bash
# Install rustfmt and clippy (usually included with rustup)
rustup component add rustfmt clippy

# Optional: Install additional tools
cargo install cargo-audit      # Security auditing
cargo install cargo-outdated   # Dependency management
cargo install cargo-watch      # Auto-rebuild on changes
```

### 3. Verify Installation

```bash
# Build the project
cargo build

# Run tests
cargo test

# Verify formatting and linting
cargo fmt -- --check
cargo clippy -- -D warnings
```

## Building and Testing

### Building

```bash
# Debug build (fast compilation, slower runtime)
cargo build

# Release build (optimized)
cargo build --release

# Build with all features
cargo build --all-features

# Build specific binary
cargo build --bin cmdrun
```

### Running

```bash
# Run from source (debug mode)
cargo run -- --help

# Run with arguments
cargo run -- run dev

# Run release build
./target/release/cmdrun --help
```

### Testing

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_name

# Run integration tests only
cargo test --test basic

# Run tests with all features enabled
cargo test --all-features

# Run security tests
cargo test --test security_injection
```

### Code Coverage

```bash
# Install tarpaulin (if not already installed)
cargo install cargo-tarpaulin

# Generate coverage report
cargo tarpaulin --out Html --output-dir ./coverage
```

### Benchmarking

```bash
# Run benchmarks (when available)
cargo bench

# Run specific benchmark
cargo bench --bench performance
```

### Development Workflow

```bash
# Watch for changes and auto-rebuild
cargo watch -x check -x test

# Watch and run specific command
cargo watch -x 'run -- list'
```

## Code Style

### Formatting with rustfmt

We use `rustfmt` with default settings to ensure consistent code formatting.

```bash
# Check formatting (doesn't modify files)
cargo fmt -- --check

# Format all code
cargo fmt
```

**Important**: All pull requests must pass `cargo fmt -- --check`. Configure your editor to format on save:

- **VS Code**: Install `rust-analyzer` extension, enable "Format on Save"
- **Vim/Neovim**: Use `rust.vim` or `rustfmt` integration
- **IntelliJ IDEA**: Enable "Reformat with rustfmt" on save

### Linting with clippy

We use `clippy` to catch common mistakes and improve code quality.

```bash
# Run clippy
cargo clippy

# Run clippy with all features and deny warnings
cargo clippy --all-targets --all-features -- -D warnings
```

**Clippy Configuration**: We deny all clippy warnings in CI. Fix all warnings before submitting a PR:

```bash
# See what clippy finds
cargo clippy --all-targets --all-features

# Apply automatic fixes where possible
cargo clippy --fix
```

### Code Style Guidelines

1. **Use descriptive variable names**
   ```rust
   // Good
   let config_path = dirs::config_dir().unwrap();

   // Bad
   let p = dirs::config_dir().unwrap();
   ```

2. **Write self-documenting code with comments for complex logic**
   ```rust
   /// Parses command configuration from TOML and validates security constraints.
   ///
   /// # Arguments
   /// * `input` - Raw TOML string
   ///
   /// # Returns
   /// * `Ok(Config)` - Validated configuration
   /// * `Err(ConfigError)` - Parse or validation error
   fn parse_config(input: &str) -> Result<Config, ConfigError> {
       // Implementation
   }
   ```

3. **Prefer explicit error handling**
   ```rust
   // Good
   let file = std::fs::read_to_string(path)
       .context("Failed to read configuration file")?;

   // Avoid
   let file = std::fs::read_to_string(path).unwrap();
   ```

4. **Use type system for safety**
   ```rust
   // Good - explicit types prevent misuse
   struct CommandId(String);
   struct Description(String);

   fn add_command(id: CommandId, desc: Description) { /* ... */ }

   // Bad - easy to swap arguments
   fn add_command(id: String, desc: String) { /* ... */ }
   ```

5. **Keep functions focused and testable**
   ```rust
   // Good - single responsibility
   fn validate_command_id(id: &str) -> Result<()> { /* ... */ }
   fn sanitize_command(cmd: &str) -> String { /* ... */ }

   // Bad - doing too much
   fn validate_and_sanitize_and_execute(input: &str) -> Result<()> { /* ... */ }
   ```

## Commit Message Format

We follow [Conventional Commits](https://www.conventionalcommits.org/) for clear, structured commit history.

### Format

```
<type>(<scope>): <subject>

<body>

<footer>
```

### Type

- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes (formatting, no logic change)
- `refactor`: Code refactoring (no functional change)
- `perf`: Performance improvements
- `test`: Adding or updating tests
- `chore`: Maintenance tasks (dependencies, build, etc.)
- `ci`: CI/CD changes

### Scope (optional)

- `cli`: Command-line interface
- `config`: Configuration parsing
- `runner`: Command execution
- `security`: Security-related changes
- `i18n`: Internationalization
- `docs`: Documentation

### Examples

```bash
# Feature
git commit -m "feat(cli): add --version flag with build info"

# Bug fix
git commit -m "fix(security): prevent shell injection in variable expansion"

# Documentation
git commit -m "docs: update installation instructions for Windows"

# Refactoring
git commit -m "refactor(runner): simplify parallel execution logic"

# Breaking change
git commit -m "feat(config)!: change TOML structure for better clarity

BREAKING CHANGE: commands.toml now uses [commands.name] instead of [name]"
```

### Commit Message Guidelines

1. **Use imperative mood**: "add feature" not "added feature"
2. **First line under 72 characters**
3. **Provide context in the body** for complex changes
4. **Reference issues**: "Fixes #123" or "Relates to #456"
5. **Mark breaking changes**: Use `!` after type/scope or `BREAKING CHANGE:` in footer

## Pull Request Process

### Before Submitting

1. **Update your fork**
   ```bash
   git fetch upstream
   git rebase upstream/main
   ```

2. **Create a feature branch**
   ```bash
   git checkout -b feat/your-feature-name
   # or
   git checkout -b fix/bug-description
   ```

3. **Ensure all checks pass**
   ```bash
   # Format code
   cargo fmt

   # Run linter
   cargo clippy --all-targets --all-features -- -D warnings

   # Run tests
   cargo test --all-features

   # Run security audit
   cargo audit
   ```

4. **Update documentation**
   - Update relevant docs in `docs/` if behavior changes
   - Add examples to `examples/` for new features
   - Update CHANGELOG.md following [Keep a Changelog](https://keepachangelog.com/)

### Submitting the PR

1. **Push to your fork**
   ```bash
   git push origin feat/your-feature-name
   ```

2. **Create Pull Request on GitHub**
   - Use a clear, descriptive title
   - Fill out the PR template completely
   - Link related issues (e.g., "Closes #123")
   - Add screenshots for UI changes
   - Mark as draft if work-in-progress

3. **PR Title Format**
   ```
   feat(cli): add interactive mode for command selection
   fix(security): sanitize environment variables
   docs: improve configuration examples
   ```

### PR Review Process

1. **Automated Checks**: CI must pass
   - Tests on Linux, macOS, and Windows
   - Clippy and rustfmt checks
   - Security audit

2. **Code Review**: Maintainers will review
   - Provide constructive feedback
   - Request changes if needed
   - Approve when ready

3. **Addressing Feedback**
   ```bash
   # Make changes
   git add .
   git commit -m "fix: address review comments"
   git push origin feat/your-feature-name
   ```

4. **Merging**: Maintainers will merge when:
   - All checks pass
   - At least one approval
   - No unresolved conversations
   - Branch is up-to-date with main

### PR Checklist

- [ ] Code follows style guidelines (`cargo fmt`, `cargo clippy`)
- [ ] Tests added/updated for changes
- [ ] All tests pass (`cargo test`)
- [ ] Documentation updated (README, docs/, examples/)
- [ ] CHANGELOG.md updated
- [ ] Commit messages follow conventional format
- [ ] No breaking changes (or clearly documented)
- [ ] Security implications considered

## Issue Reporting Guidelines

### Before Creating an Issue

1. **Search existing issues** to avoid duplicates
2. **Check documentation** for answers
3. **Try latest version** to see if issue is resolved

### Bug Reports

Include the following information:

```markdown
## Description
Clear description of the bug

## Steps to Reproduce
1. Create commands.toml with...
2. Run `cmdrun run test`
3. Observe error...

## Expected Behavior
What should happen

## Actual Behavior
What actually happens (include error messages)

## Environment
- cmdrun version: `cmdrun --version`
- OS: macOS 13.5 / Ubuntu 22.04 / Windows 11
- Rust version: `rustc --version`
- Shell: bash 5.2 / zsh 5.9

## Additional Context
- Configuration file (if applicable)
- Logs or screenshots
- Possible solution (if you have one)
```

### Feature Requests

Include the following:

```markdown
## Problem Statement
What problem does this solve?

## Proposed Solution
How should it work?

## Alternatives Considered
Other approaches you've thought of

## Additional Context
Examples, mockups, or references
```

### Security Issues

**DO NOT** create public issues for security vulnerabilities. Instead:

1. Email security concerns to: [security@example.com]
2. Include detailed description and reproduction steps
3. We will respond within 48 hours
4. See [SECURITY.md](docs/SECURITY.md) for full policy

## Community

### Getting Help

- **Documentation**: Check [docs/](docs/) for guides
- **Discussions**: Use [GitHub Discussions](https://github.com/sanae-abe/cmdrun/discussions)
- **Chat**: Join our community chat (if available)
- **Stack Overflow**: Tag questions with `cmdrun`

### Recognition

Contributors will be:

- Listed in [CHANGELOG.md](CHANGELOG.md) for their contributions
- Mentioned in release notes
- Added to the contributors list

### Additional Resources

- [Architecture Documentation](docs/ARCHITECTURE.md)
- [Development Guide](docs/DEVELOPMENT.md)
- [Performance Guidelines](docs/PERFORMANCE.md)
- [Security Practices](docs/SECURITY.md)

## License

By contributing to cmdrun, you agree that your contributions will be licensed under either:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT License ([LICENSE-MIT](LICENSE-MIT))

at the option of the user.

---

Thank you for contributing to cmdrun! Your efforts help make this project better for everyone.
