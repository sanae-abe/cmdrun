# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.0.0] - 2025-11-06

### Added

- **Core Functionality**
  - TOML-based configuration with `commands.toml`
  - Command execution with dependency resolution
  - Platform-specific command support (Unix, Windows, Linux, macOS)
  - Variable interpolation with multiple syntax options:
    - `${VAR}` - Basic expansion
    - `${VAR:-default}` - Default value
    - `${VAR:?error}` - Required variable
    - `${VAR:+value}` - Conditional substitution
  - Command dependencies with topological sort
  - Parallel command execution support
  - Pre/post execution hooks
  - Command-specific environment variables

- **CLI Features**
  - `cmdrun run <command>` - Execute commands
    - `--parallel` flag for parallel dependency execution
  - `cmdrun list` - List available commands
  - `cmdrun init` - Initialize commands.toml with templates
    - `--template <type>` - Choose from web, rust, node, python templates
    - `--interactive` - Interactive template selection
    - `--output <path>` - Custom output path
  - `cmdrun graph` - Visualize dependency graph
    - `--format <tree|dot|mermaid>` - Multiple output formats
    - `--show-groups` - Display execution groups
    - `--output <path>` - Export to file
  - `cmdrun validate` - Validate configuration
  - `cmdrun completion-list` - Internal command for shell completion
  - Shell completion support (bash, zsh, fish, powershell)
  - Colored output for better readability
  - Verbose and quiet modes

- **Environment Management**
  - `cmdrun env use <env>` - Switch between environments (dev/staging/prod)
  - `cmdrun env current` - Show current active environment
  - `cmdrun env list` - List all available environments
  - `cmdrun env set <key> <value>` - Set environment-specific variables
  - `cmdrun env create <name>` - Create new environment
  - `cmdrun env info <name>` - Show environment details
  - Configuration merging (base + environment-specific settings)
  - Environment variable profiles per environment
  - Isolated command execution per environment

- **History & Logging**
  - `cmdrun history list` - Display command execution history
  - `cmdrun history search <query>` - Search command history
  - `cmdrun history clear` - Clear command history
  - `cmdrun history export` - Export history to JSON/CSV
  - `cmdrun history stats` - Show execution statistics
  - `cmdrun retry` - Re-execute last failed command
  - SQLite-based persistent storage (max 1000 entries)
  - Execution time tracking and exit code recording
  - Sensitive information filtering (passwords, tokens)
  - Statistical analysis (success rate, average execution time)

- **Template System**
  - `cmdrun template list` - List available templates
  - `cmdrun template use <name>` - Apply template to current project
  - `cmdrun template add <name>` - Create custom template
  - `cmdrun template remove <name>` - Remove template
  - `cmdrun template export <name>` - Export template to file
  - `cmdrun template import <path>` - Import template from file
  - Built-in templates:
    - `rust-cli` - Rust CLI development (cargo build/test/clippy/fmt)
    - `nodejs-web` - Node.js web development (npm dev/build/test)
    - `python-data` - Python data science (pytest/jupyter)
    - `react-app` - React application (dev/build/storybook)
  - Template validation and sharing capabilities
  - Custom template creation from existing configurations

- **Plugin System**
  - `cmdrun plugin list` - List all plugins
  - `cmdrun plugin info <name>` - Show plugin details
  - `cmdrun plugin enable <name>` - Enable plugin
  - `cmdrun plugin disable <name>` - Disable plugin
  - Dynamic plugin loading with `libloading`
  - Plugin API with hooks (pre_run, post_run, on_error)
  - Custom command registration from plugins
  - Plugin configuration in commands.toml
  - Sample plugins:
    - `hello_plugin` - Basic example plugin
    - `logger_plugin` - Command execution logging
  - Comprehensive plugin development documentation (850+ lines)

- **Watch Mode**
  - `cmdrun watch <command>` - Watch files and auto-execute commands
  - File pattern matching with glob support (`**/*.rs`, `**/*.ts`, etc.)
  - Exclude patterns to ignore specific files/directories
  - Configurable debounce delay (default 500ms)
  - `.gitignore` integration (automatic exclusion of common directories)
  - Recursive and non-recursive watching modes
  - Support for multiple watch paths
  - Real-time file change detection
  - Graceful shutdown with Ctrl+C

- **Graph Visualization**
  - Tree format with colorful output and Unicode box drawing
  - DOT format for Graphviz (export as PNG/SVG)
  - Mermaid diagram format (embed in documentation)
  - Execution group visualization showing parallel execution plan
  - Support for full graph or specific command subgraphs

- **Project Initialization**
  - Template-based initialization with 5 templates:
    - Default - Generic command runner
    - Web - Web development (HTML/CSS/JS)
    - Rust - Rust project with cargo commands
    - Node - Node.js with npm/yarn commands
    - Python - Python with common tools
  - Interactive template selection with preview
  - Automatic validation of generated files

- **Security Features**
  - Zero `eval()` - No dynamic code execution
  - Safe variable expansion (no shell injection)
  - Required variable validation
  - Strict mode for secure deployments

- **Configuration Features**
  - Hierarchical config loading (global, user, project)
  - Command aliases
  - Config inheritance and merging
  - Platform-specific configuration
  - Custom shell selection

- **Developer Experience**
  - Detailed error messages with context
  - Configuration validation with helpful errors
  - Circular dependency detection
  - Type-safe TOML parsing
  - Comprehensive test coverage (29 unit tests)

### Changed

- **Environment Variable Expansion**
  - Fixed expansion to properly handle all syntax forms
  - Improved error messages for missing required variables
  - Better handling of default values and conditional substitution

### Performance

- **Startup Time**: 4ms average (29x faster than npm)
- **Memory Footprint**: ~10MB (vs 200MB+ for Node.js)
- **Binary Size**: Optimized with LTO and strip
- **Build Optimizations**:
  - LTO (Link-Time Optimization) enabled
  - Code stripping for smaller binaries
  - Single codegen unit for maximum optimization

### Testing

- 303 passing tests (171 unit + 132 integration) covering:
  - Command execution and shell detection
  - Variable interpolation (basic, nested, conditional)
  - Configuration loading and merging
  - Platform-specific command resolution
  - Dependency validation and topological sorting
  - Circular dependency detection
  - Error handling and error chains
  - Graph visualization (tree, dot, mermaid)
  - Template initialization
  - Watch mode configuration and pattern matching
  - Environment management (6 integration tests)
  - Command history and logging (7 integration tests)
  - Template system (45 tests)
  - Plugin system (sample plugin tests)
  - Property-based testing with proptest (20 tests)
  - Security testing (fuzzing with cargo-fuzz, 4 targets)

### Documentation

- Comprehensive README with examples (Japanese and English)
- Installation instructions for multiple platforms
- Configuration reference
- Usage examples for common scenarios
- Performance benchmarks vs npm/make
- Migration guides from npm scripts and Makefiles
- Updated CLI reference with new commands
- **User Guides**:
  - Watch Mode guide with detailed examples
  - History & Logging guide (HISTORY.md)
  - Environment Management guide (ENVIRONMENT_MANAGEMENT.md)
  - FAQ (19KB)
  - Recipes (23KB)
  - Troubleshooting (17KB)
- **Technical Documentation**:
  - Architecture guide (40KB)
  - Performance guide (15KB)
  - Profiling guide (830 lines)
  - Security guide (SECURITY.md)
- **Plugin Development**:
  - Plugin API specification (API.md)
  - Plugin development guide (DEVELOPMENT_GUIDE.md, 850+ lines)
  - Sample plugin documentation
- **Feature Reports**:
  - Template Feature Report (TEMPLATE_FEATURE_REPORT.md)
  - Plugin System Implementation Report (PLUGIN_SYSTEM_IMPLEMENTATION_REPORT.md)
  - Environment Implementation Summary (ENVIRONMENT_IMPLEMENTATION_SUMMARY.md)
  - History Implementation Summary (HISTORY_IMPLEMENTATION_SUMMARY.md)

### Technical Stack

- **Language**: Rust 1.75+ (2021 edition)
- **Core Dependencies**:
  - clap 4.5 (CLI argument parsing with derive macros)
  - tokio 1.39 (async runtime)
  - toml 0.8 (configuration parsing)
  - serde 1.0 (serialization/deserialization)
  - colored 2.1 (colorful terminal output)
  - dialoguer 0.11 (interactive prompts)
  - notify 6.1 (file system watching)
  - globset 0.4 (glob pattern matching)
  - rusqlite 0.32 (SQLite database for history)
  - libloading 0.8 (dynamic plugin loading)
  - shell-words 1.1 (safe shell argument parsing)
  - chrono 0.4 (date/time handling)
  - anyhow 1.0 (error handling)
- **Testing Dependencies**:
  - proptest 1.5 (property-based testing)
  - cargo-fuzz (fuzzing framework)
  - cargo-tarpaulin 0.31 (code coverage)
- **Platform Support**: Linux, macOS, Windows, FreeBSD
- **Shell Support**: bash, zsh, fish, pwsh

### Project Structure

```
cmdrun/
├── src/
│   ├── main.rs              # CLI entry point
│   ├── lib.rs               # Library exports
│   ├── cli.rs               # CLI argument definitions
│   ├── commands/            # CLI commands
│   │   ├── init.rs          # Project initialization
│   │   ├── add.rs           # Add commands
│   │   ├── remove.rs        # Remove commands
│   │   ├── edit.rs          # Edit commands
│   │   ├── info.rs          # Command information
│   │   ├── search.rs        # Search commands
│   │   ├── open.rs          # Open config file
│   │   ├── completion.rs    # Shell completion
│   │   ├── validate.rs      # Config validation
│   │   ├── watch.rs         # Watch mode
│   │   ├── env.rs           # Environment management (NEW)
│   │   ├── history.rs       # Command history (NEW)
│   │   ├── template.rs      # Template system (NEW)
│   │   └── plugin.rs        # Plugin management (NEW)
│   ├── command/             # Command execution
│   │   ├── executor.rs      # Command runner
│   │   ├── interpolation.rs # Variable expansion
│   │   ├── dependency.rs    # Dependency resolution
│   │   └── graph_visualizer.rs # Graph visualization
│   ├── config/              # Configuration
│   │   ├── loader.rs        # Config loading
│   │   ├── schema.rs        # TOML schema
│   │   ├── validation.rs    # Config validation
│   │   └── environment.rs   # Environment config (NEW)
│   ├── watch/               # Watch mode implementation
│   │   ├── mod.rs           # Watch module
│   │   ├── config.rs        # Watch configuration
│   │   └── runner.rs        # Watch runner
│   ├── history/             # History & logging (NEW)
│   │   ├── mod.rs           # History module
│   │   ├── storage.rs       # SQLite storage
│   │   └── stats.rs         # Statistics
│   ├── template/            # Template system (NEW)
│   │   ├── mod.rs           # Template module
│   │   ├── manager.rs       # Template manager
│   │   └── builtin.rs       # Built-in templates
│   ├── plugin/              # Plugin system (NEW)
│   │   ├── mod.rs           # Plugin module
│   │   ├── loader.rs        # Dynamic loader
│   │   ├── registry.rs      # Plugin registry
│   │   └── api.rs           # Plugin API
│   ├── error.rs             # Error types
│   └── i18n.rs              # Internationalization
├── templates/               # Project templates
│   ├── commands.toml        # Default template
│   ├── web.toml            # Web development
│   ├── rust.toml           # Rust project
│   ├── node.toml           # Node.js project
│   └── python.toml         # Python project
├── templates/builtin/       # Built-in templates (NEW)
│   ├── rust-cli.toml       # Rust CLI template
│   ├── nodejs-web.toml     # Node.js web template
│   ├── python-data.toml    # Python data science
│   └── react-app.toml      # React app template
├── examples/plugins/        # Sample plugins (NEW)
│   ├── hello_plugin/       # Basic example
│   └── logger_plugin/      # Logging plugin
├── tests/                   # Integration tests
├── docs/                    # Documentation
│   ├── user-guide/
│   │   ├── CLI.md
│   │   ├── CONFIGURATION.md
│   │   ├── I18N.md
│   │   ├── WATCH_MODE.md
│   │   └── HISTORY.md      # History guide (NEW)
│   ├── technical/
│   ├── plugins/            # Plugin documentation (NEW)
│   │   ├── API.md          # Plugin API spec
│   │   └── DEVELOPMENT_GUIDE.md # Plugin dev guide
│   └── ENVIRONMENT_MANAGEMENT.md # Environment guide (NEW)
└── Cargo.toml              # Project manifest
```

### Breaking Changes

- Initial release (no breaking changes from previous versions)

### Known Limitations

- Test coverage at 46% (target: 60%)
  - Main entry point (main.rs) requires subprocess testing
  - Interactive UI commands need automated testing solutions
- Some commands not fully internationalized (90% coverage, 9/10 commands)
- Plugin system is foundational (ecosystem to be built by community)

### Future Roadmap

- [x] Watch mode for automatic command re-execution (Completed in v1.0.0)
- [x] Built-in file watching capabilities (Completed in v1.0.0)
- [x] Environment management for multi-environment workflows (Completed in v1.0.0)
- [x] Command history and logging (Completed in v1.0.0)
- [x] Template system for project initialization (Completed in v1.0.0)
- [x] Plugin system for extensibility (Completed in v1.0.0)
- [ ] Remote command execution
- [ ] Interactive mode for command selection (fuzzy finder)
- [ ] Performance profiling and optimization tools
- [ ] Advanced debugging features

---

## Release Notes

### Installation

**Cargo (Recommended)**
```bash
cargo install cmdrun
```

**From Source**
```bash
git clone https://github.com/sanae-abe/cmdrun
cd cmdrun
cargo build --release
```

### Quick Start

1. Initialize with template:
```bash
cmdrun init --template rust
```

2. Or create `commands.toml` manually:
```toml
[commands.dev]
description = "Start development server"
cmd = "npm run dev"

[commands.build]
cmd = ["npm run type-check", "npm run lint", "npm run build"]
```

3. Run commands:
```bash
cmdrun run dev
cmdrun list
cmdrun graph build --show-groups

# Use Watch Mode (NEW)
cmdrun watch dev --pattern "**/*.rs"
```

### Performance Comparison

**Startup Time (measured with hyperfine)**
```
cmdrun --version:  4.0ms ± 0.3ms
npm --version:    115.4ms ± 13.0ms

Result: cmdrun is 28.88x faster than npm
```

### Migration from npm scripts

**Before (package.json)**
```json
{
  "scripts": {
    "dev": "npm run clean && npm run build && npm run serve",
    "build": "tsc && webpack"
  }
}
```

**After (commands.toml)**
```toml
[commands.dev]
cmd = "npm run serve"
deps = ["clean", "build"]

[commands.build]
cmd = ["tsc", "webpack"]
```

### New Features Highlights

#### 1. Environment Management (NEW in v1.0.0)
```bash
# Create and switch between environments
cmdrun env create dev --description "Development environment"
cmdrun env create prod --description "Production environment"

# Switch environments
cmdrun env use dev
cmdrun run start  # Runs with dev environment settings

# Set environment-specific variables
cmdrun env set API_URL https://api.dev.com --env dev
cmdrun env set API_URL https://api.prod.com --env prod

# View environment info
cmdrun env current
cmdrun env list
```

**Environment Management Features:**
- Multi-environment support (dev/staging/prod)
- Configuration merging (base + environment-specific)
- Environment variable profiles
- Isolated command execution

#### 2. History & Logging (NEW in v1.0.0)
```bash
# View command history
cmdrun history list
cmdrun history search build

# Show statistics
cmdrun history stats

# Retry last failed command
cmdrun retry

# Export history
cmdrun history export --format json -o history.json
cmdrun history export --format csv -o history.csv
```

**History Features:**
- SQLite-based persistent storage
- Execution time tracking
- Success/failure statistics
- Sensitive information filtering
- JSON/CSV export

#### 3. Template System (NEW in v1.0.0)
```bash
# Use built-in templates
cmdrun template list
cmdrun template use rust-cli

# Create custom templates
cmdrun template add my-template
cmdrun template export my-template ./my-template.toml

# Share templates
cmdrun template import ./shared-template.toml
```

**Template System Features:**
- 4 built-in templates (rust-cli, nodejs-web, python-data, react-app)
- Custom template creation
- Template validation
- Import/export functionality

#### 4. Plugin System (NEW in v1.0.0)
```bash
# Manage plugins
cmdrun plugin list
cmdrun plugin info logger
cmdrun plugin enable logger
cmdrun plugin disable logger
```

**Plugin System Features:**
- Dynamic plugin loading
- Plugin hooks (pre_run, post_run, on_error)
- Custom command registration
- Sample plugins included

#### 5. Watch Mode (NEW in v1.0.0)
```bash
# Watch for file changes and auto-execute
cmdrun watch dev

# Watch specific patterns
cmdrun watch build --pattern "**/*.rs" --pattern "**/*.toml"

# Exclude directories
cmdrun watch dev --exclude "**/test/**"

# Adjust debounce time
cmdrun watch dev --debounce 1000
```

**Watch Mode Features:**
- Glob pattern matching (`**/*.rs`, `**/*.ts`, etc.)
- Configurable debounce delay (default 500ms)
- `.gitignore` integration
- Exclude patterns
- Multiple watch paths
- Graceful shutdown

#### 6. Project Initialization
```bash
# Quick start with templates
cmdrun init --template rust
cmdrun init --interactive
```

#### 3. Dependency Graph Visualization
```bash
# Colorful tree format
cmdrun graph build

# Export as Graphviz
cmdrun graph build --format dot --output deps.dot

# Mermaid diagram for documentation
cmdrun graph build --format mermaid --output deps.mmd

# Show execution plan
cmdrun graph build --show-groups
```

#### 4. Parallel Execution
```bash
# Run dependencies in parallel
cmdrun run build --parallel
```

### Acknowledgments

Special thanks to:
- The Rust community for excellent tooling and libraries
- Inspiration from npm scripts, make, and just
- All early testers and contributors

---

[1.0.0]: https://github.com/sanae-abe/cmdrun/releases/tag/v1.0.0
