# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.0.0] - 2025-11-05

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

- **Watch Mode** (New Feature)
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

- 29 passing unit tests covering:
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

### Documentation

- Comprehensive README with examples (Japanese and English)
- Installation instructions for multiple platforms
- Configuration reference
- Usage examples for common scenarios
- Performance benchmarks vs npm/make
- Migration guides from npm scripts and Makefiles
- Updated CLI reference with new commands
- **New**: Watch Mode user guide with detailed examples

### Technical Stack

- **Language**: Rust 1.75+ (2021 edition)
- **Core Dependencies**:
  - clap 4.5 (CLI argument parsing)
  - tokio 1.39 (async runtime)
  - toml 0.8 (configuration parsing)
  - serde 1.0 (serialization)
  - colored 2.1 (colorful output)
  - dialoguer 0.11 (interactive prompts)
  - notify 6.1 (file system watching)
  - globset 0.4 (glob pattern matching)
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
│   │   └── watch.rs         # Watch mode (NEW)
│   ├── command/             # Command execution
│   │   ├── executor.rs      # Command runner
│   │   ├── interpolation.rs # Variable expansion
│   │   ├── dependency.rs    # Dependency resolution
│   │   └── graph_visualizer.rs # Graph visualization
│   ├── config/              # Configuration
│   │   ├── loader.rs        # Config loading
│   │   ├── schema.rs        # TOML schema
│   │   └── validation.rs    # Config validation
│   ├── watch/               # Watch mode implementation (NEW)
│   │   ├── mod.rs           # Watch module
│   │   ├── config.rs        # Watch configuration
│   │   └── runner.rs        # Watch runner
│   ├── error.rs             # Error types
│   └── i18n.rs              # Internationalization
├── templates/               # Project templates
│   ├── commands.toml        # Default template
│   ├── web.toml            # Web development
│   ├── rust.toml           # Rust project
│   ├── node.toml           # Node.js project
│   └── python.toml         # Python project
├── tests/                   # Integration tests
├── docs/                    # Documentation
│   ├── user-guide/
│   │   ├── CLI.md
│   │   ├── CONFIGURATION.md
│   │   ├── I18N.md
│   │   └── WATCH_MODE.md   # Watch mode guide (NEW)
│   └── technical/
└── Cargo.toml              # Project manifest
```

### Breaking Changes

- Initial release (no breaking changes from previous versions)

### Known Limitations

- Integration tests pending (unit tests fully implemented)
- Documentation improvements in progress
- Additional platform-specific features planned

### Future Roadmap

- [x] Watch mode for automatic command re-execution (Completed in v1.0.0)
- [x] Built-in file watching capabilities (Completed in v1.0.0)
- [ ] Remote command execution
- [ ] Plugin system for extensibility
- [ ] Interactive mode for command selection
- [ ] Performance profiling and optimization tools
- [ ] Advanced logging and debugging features

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

#### 1. Watch Mode (NEW in v1.0.0)
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

#### 2. Project Initialization
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
