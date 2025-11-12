//! CLI argument definitions

use clap::{Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

/// Color output control
#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum ColorChoice {
    /// Never use colored output
    Never,
    /// Automatically detect (TTY/pipe detection)
    Auto,
    /// Always use colored output
    Always,
}

#[derive(Parser, Debug)]
#[command(
    name = "cmdrun",
    version,
    about = "Fast, secure, and cross-platform command runner",
    long_about = "A modern replacement for package.json scripts and Makefiles.\n\n\
                  cmdrun is a high-performance command runner built in Rust, optimized for:\n\
                  - Speed: 4ms startup time, 10MB memory footprint\n\
                  - Security: Shell injection protection, input validation\n\
                  - Developer Experience: Intuitive CLI, interactive prompts, multi-language support\n\n\
                  Features:\n\
                  - Parallel command execution with dependency resolution\n\
                  - File watching with automatic command re-execution\n\
                  - Environment management (dev/staging/prod)\n\
                  - Command history tracking and retry functionality\n\
                  - Reusable templates for common project types\n\
                  - Plugin system for extensibility"
)]
pub struct Cli {
    /// Path to configuration file (default: ~/.config/cmdrun/commands.toml)
    ///
    /// Use this option to specify which configuration file to use.
    /// This allows you to maintain multiple command sets for different
    /// purposes (work, personal, projects, environments, etc.)
    #[arg(
        short,
        long,
        value_name = "FILE",
        global = true,
        conflicts_with = "global"
    )]
    pub config: Option<PathBuf>,

    /// Use only global configuration (skip local config search)
    ///
    /// When this flag is set, cmdrun will only look for the global
    /// configuration file and will not search for or merge local
    /// configuration files in the current directory.
    #[arg(short, long, global = true)]
    pub global: bool,

    /// Control colored output
    ///
    /// Choose when to use colored output. The default is 'auto', which
    /// automatically enables color when outputting to a terminal and disables
    /// it when piping to another command. Use 'never' to disable colors
    /// completely, or 'always' to force colors even when piping.
    ///
    /// Respects the NO_COLOR environment variable (https://no-color.org/).
    #[arg(long, value_enum, default_value = "auto", global = true)]
    pub color: ColorChoice,

    /// Subcommand to execute
    #[command(subcommand)]
    pub command: Commands,

    /// Verbose output (-v, -vv, -vvv for more verbosity)
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub verbose: u8,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Run a command defined in commands.toml
    ///
    /// Execute a command with automatic dependency resolution. Dependencies
    /// are executed in the correct order before the main command runs.
    /// Use --parallel to execute independent dependencies concurrently.
    ///
    /// Examples:
    ///   cmdrun run build
    ///   cmdrun run test --parallel
    ///   cmdrun run deploy -- --env prod
    #[command(visible_alias = "r")]
    Run {
        /// Command name
        name: String,

        /// Additional arguments to pass to the command
        #[arg(trailing_var_arg = true)]
        args: Vec<String>,

        /// Execute dependencies in parallel when possible
        #[arg(short, long)]
        parallel: bool,
    },

    /// List available commands from configuration
    ///
    /// Display all defined commands with their descriptions. Use --verbose
    /// to see additional details including dependencies, tags, and categories.
    ///
    /// Examples:
    ///   cmdrun list
    ///   cmdrun list --verbose
    ///   cmdrun list --global
    #[command(visible_alias = "ls")]
    List {
        /// Show detailed information
        #[arg(short, long)]
        verbose: bool,
    },

    /// Initialize a new commands.toml file in the current directory
    ///
    /// Create a new configuration file with optional template support.
    /// Available templates: rust-cli, nodejs-web, python-data, react-app.
    /// Use --interactive to walk through setup with prompts.
    ///
    /// Examples:
    ///   cmdrun init
    ///   cmdrun init --template rust-cli
    ///   cmdrun init --interactive
    Init {
        /// Template to use (rust-cli, nodejs-web, python-data, react-app)
        #[arg(short, long)]
        template: Option<String>,

        /// Use interactive mode with step-by-step prompts
        #[arg(short, long)]
        interactive: bool,

        /// Output path (default: commands.toml)
        #[arg(short, long)]
        output: Option<PathBuf>,
    },

    /// Validate configuration file syntax and semantics
    ///
    /// Check your commands.toml file for errors including:
    /// - TOML syntax errors
    /// - Invalid command definitions
    /// - Circular dependencies (with --check-cycles)
    /// - Missing dependencies
    ///
    /// Examples:
    ///   cmdrun validate
    ///   cmdrun validate --verbose
    ///   cmdrun validate --check-cycles
    Validate {
        /// Path to configuration file
        #[arg(short, long)]
        path: Option<PathBuf>,

        /// Show detailed validation report
        #[arg(short, long)]
        verbose: bool,

        /// Check for circular dependencies
        #[arg(long)]
        check_cycles: bool,
    },

    /// Show dependency graph for commands
    ///
    /// Visualize command dependencies in various formats. The graph shows
    /// execution order and parallel execution opportunities.
    ///
    /// Examples:
    ///   cmdrun graph              # Show all dependencies as tree
    ///   cmdrun graph build        # Show dependencies for 'build' command
    ///   cmdrun graph --format dot # Generate Graphviz DOT format
    ///   cmdrun graph --format mermaid -o diagram.md
    Graph {
        /// Specific command to show dependencies for
        command: Option<String>,

        /// Output format (tree, dot, mermaid)
        #[arg(short, long, value_enum, default_value = "tree")]
        format: GraphFormat,

        /// Output file path (prints to stdout if not specified)
        #[arg(short, long)]
        output: Option<PathBuf>,

        /// Show execution groups (parallel execution plan)
        #[arg(short = 'g', long)]
        show_groups: bool,
    },

    /// Generate shell completion scripts
    ///
    /// Generate completion scripts for your shell. This enables tab completion
    /// for cmdrun commands and options.
    ///
    /// Installation:
    ///   bash:   cmdrun completion bash > ~/.local/share/bash-completion/completions/cmdrun
    ///   zsh:    cmdrun completion zsh > ~/.zsh/completions/_cmdrun
    ///   fish:   cmdrun completion fish > ~/.config/fish/completions/cmdrun.fish
    ///   pwsh:   cmdrun completion powershell > cmdrun.ps1
    Completion {
        /// Shell to generate completion for
        #[arg(value_enum)]
        shell: clap_complete::Shell,
    },

    /// Remove a command from the configuration
    ///
    /// Delete a command definition from commands.toml. Use --force to skip
    /// the confirmation prompt.
    ///
    /// Examples:
    ///   cmdrun remove old-build
    ///   cmdrun remove deprecated-test --force
    Remove {
        /// Command ID to remove
        id: String,

        /// Skip confirmation prompt
        #[arg(short, long)]
        force: bool,
    },

    /// Add a new command to the configuration
    ///
    /// Add a command definition to commands.toml. If arguments are omitted,
    /// an interactive prompt will guide you through the process.
    ///
    /// Examples:
    ///   cmdrun add                    # Interactive mode
    ///   cmdrun add build "cargo build --release" --category rust
    ///   cmdrun add test "npm test" --tags ci,test
    Add {
        /// Command ID (unique identifier)
        id: Option<String>,

        /// Command to execute
        command: Option<String>,

        /// Description of the command
        description: Option<String>,

        /// Category for the command
        #[arg(short = 'C', long)]
        category: Option<String>,

        /// Tags (comma-separated)
        #[arg(short, long, value_delimiter = ',')]
        tags: Option<Vec<String>>,
    },

    /// Open commands.toml in the default editor
    ///
    /// Opens your configuration file in the editor specified by $EDITOR
    /// environment variable. Falls back to sensible defaults if not set.
    ///
    /// Example:
    ///   cmdrun open
    Open,

    /// Edit an existing command interactively
    ///
    /// Modify a command's properties using an interactive prompt. If no ID
    /// is provided, a list of commands will be presented for selection.
    ///
    /// Examples:
    ///   cmdrun edit
    ///   cmdrun edit build
    Edit {
        /// Command ID to edit (optional - will prompt if not provided)
        id: Option<String>,
    },

    /// Show detailed information about a command
    ///
    /// Display comprehensive information about a command including its
    /// definition, dependencies, environment variables, and metadata.
    ///
    /// Examples:
    ///   cmdrun info
    ///   cmdrun info build
    Info {
        /// Command ID to show info for (optional - will prompt if not provided)
        id: Option<String>,
    },

    /// Search commands by keyword
    ///
    /// Search for commands matching a keyword in their ID, description,
    /// tags, or category.
    ///
    /// Examples:
    ///   cmdrun search test
    ///   cmdrun search docker
    Search {
        /// Keyword to search for
        keyword: String,
    },

    /// List command names for completion (internal use)
    #[command(hide = true)]
    CompletionList,

    /// Manage configuration settings (get/set/show configuration values)
    ///
    /// This subcommand allows you to view and modify settings within
    /// your configuration file, such as language, shell, or timeout.
    /// Note: This is different from --config option which specifies
    /// which configuration file to use.
    Config {
        #[command(subcommand)]
        action: ConfigAction,
    },

    /// Watch files and automatically execute commands on changes
    ///
    /// Monitor specified files or directories for changes and automatically
    /// re-execute commands when changes are detected. Useful for development
    /// workflows with automatic recompilation, testing, or reloading.
    ///
    /// The watcher uses efficient file system events and includes debouncing
    /// to prevent excessive re-execution. Respects .gitignore by default.
    ///
    /// Examples:
    ///   cmdrun watch build
    ///   cmdrun watch test -w "src/**/*.rs"
    ///   cmdrun watch dev -e "**/target/**" --debounce 1000
    Watch {
        /// Command name to execute on file changes
        command: String,

        /// Additional arguments to pass to the command
        #[arg(trailing_var_arg = true)]
        args: Vec<String>,

        /// Paths to watch (default: current directory)
        #[arg(short = 'p', long = "path", value_name = "PATH")]
        paths: Vec<PathBuf>,

        /// File patterns to watch (glob patterns, e.g., "**/*.rs")
        #[arg(short = 'w', long = "pattern", value_name = "PATTERN")]
        patterns: Vec<String>,

        /// Patterns to exclude (glob patterns)
        #[arg(short = 'e', long = "exclude", value_name = "PATTERN")]
        exclude: Vec<String>,

        /// Debounce delay in milliseconds (default: 500ms)
        #[arg(short, long, value_name = "MS", default_value = "500")]
        debounce: u64,

        /// Ignore .gitignore files
        #[arg(long)]
        ignore_gitignore: bool,

        /// Non-recursive watching
        #[arg(long)]
        no_recursive: bool,
    },

    /// Manage environments (switch between dev, staging, prod, etc.)
    ///
    /// Environment management allows you to maintain different configurations
    /// for different deployment targets or development stages. Each environment
    /// can have its own configuration file and environment variables.
    ///
    /// Examples:
    ///   cmdrun env use staging
    ///   cmdrun env list
    ///   cmdrun env create qa --description "QA environment"
    ///   cmdrun env set API_URL https://api.staging.com --env staging
    Env {
        #[command(subcommand)]
        action: EnvAction,
    },

    /// View and manage command execution history
    ///
    /// Track command executions with timestamps, exit codes, and execution
    /// duration. Search, filter, and export history. Use statistics to identify
    /// frequently failing commands.
    ///
    /// Examples:
    ///   cmdrun history list
    ///   cmdrun history search build
    ///   cmdrun history stats
    ///   cmdrun history export --format json -o history.json
    History {
        #[command(subcommand)]
        action: HistoryAction,
    },

    /// Retry the last failed command or a specific command by ID
    ///
    /// Re-execute a previously failed command with the same arguments.
    /// If no ID is provided, retries the most recently failed command.
    /// Useful for transient failures or after fixing environment issues.
    ///
    /// Examples:
    ///   cmdrun retry          # Retry last failed command
    ///   cmdrun retry 42       # Retry specific history entry
    Retry {
        /// History entry ID to retry (optional)
        id: Option<i64>,
    },

    /// Manage command templates
    ///
    /// Create, use, and manage reusable command configuration templates.
    /// Templates allow you to quickly set up common project types with
    /// predefined commands. Built-in templates are available for popular
    /// frameworks and languages.
    ///
    /// Available built-in templates:
    ///   - rust-cli:     Rust CLI application with cargo commands
    ///   - nodejs-web:   Node.js web project with npm scripts
    ///   - python-data:  Python data science with pytest and jupyter
    ///   - react-app:    React application with build, test, and dev server
    ///
    /// Examples:
    ///   cmdrun template list
    ///   cmdrun template use rust-cli
    ///   cmdrun template add my-template
    Template {
        #[command(subcommand)]
        action: TemplateAction,
    },

    /// Manage plugins
    ///
    /// List, enable, disable, and get information about cmdrun plugins.
    /// Plugins extend cmdrun's functionality with custom hooks, commands,
    /// and integrations. Plugins can execute code at various lifecycle
    /// points (pre-run, post-run, on-error).
    ///
    /// Examples:
    ///   cmdrun plugin list
    ///   cmdrun plugin info my-plugin
    ///   cmdrun plugin enable notifications
    ///   cmdrun plugin disable analytics
    #[cfg(feature = "plugin-system")]
    Plugin {
        #[command(subcommand)]
        action: PluginAction,
    },
}

/// Configuration management actions
#[derive(Subcommand, Debug)]
pub enum ConfigAction {
    /// Get a specific configuration value
    ///
    /// Examples:
    ///   cmdrun config get language
    ///   cmdrun config get shell
    Get {
        /// Configuration key (e.g., language, shell, timeout)
        key: String,
    },

    /// Set a configuration value
    ///
    /// Examples:
    ///   cmdrun config set language japanese
    ///   cmdrun config set shell zsh
    Set {
        /// Configuration key (e.g., language, shell, timeout)
        key: String,

        /// Value to set
        value: String,
    },

    /// Show all current configuration settings
    ///
    /// Displays all configuration values from the active configuration file
    Show,
}

/// Environment management actions
#[derive(Subcommand, Debug)]
pub enum EnvAction {
    /// Switch to a different environment
    ///
    /// Examples:
    ///   cmdrun env use dev
    ///   cmdrun env use staging
    Use {
        /// Environment name to switch to
        name: String,
    },

    /// Show the current active environment
    Current,

    /// List all available environments
    List,

    /// Set an environment variable for an environment
    ///
    /// Examples:
    ///   cmdrun env set NODE_ENV development
    ///   cmdrun env set API_URL https://api.example.com --env prod
    Set {
        /// Variable name
        key: String,

        /// Variable value
        value: String,

        /// Target environment (defaults to current)
        #[arg(short, long)]
        env: Option<String>,
    },

    /// Create a new environment
    ///
    /// Examples:
    ///   cmdrun env create qa --description "QA environment"
    Create {
        /// Environment name
        name: String,

        /// Environment description
        #[arg(short, long)]
        description: Option<String>,
    },

    /// Show detailed information about an environment
    ///
    /// Examples:
    ///   cmdrun env info
    ///   cmdrun env info prod
    Info {
        /// Environment name (defaults to current)
        name: Option<String>,
    },
}

/// History management actions
#[derive(Subcommand, Debug)]
pub enum HistoryAction {
    /// List command execution history
    ///
    /// Examples:
    ///   cmdrun history list
    ///   cmdrun history list --limit 20
    ///   cmdrun history list --failed
    List {
        /// Maximum number of entries to display
        #[arg(short, long, default_value = "50")]
        limit: usize,

        /// Offset for pagination
        #[arg(short = 'o', long)]
        offset: Option<usize>,

        /// Show only failed commands
        #[arg(short, long)]
        failed: bool,

        /// Show statistics summary
        #[arg(short, long)]
        stats: bool,
    },

    /// Search command history
    ///
    /// Examples:
    ///   cmdrun history search build
    ///   cmdrun history search test --limit 10
    Search {
        /// Search query (matches command name or arguments)
        query: String,

        /// Maximum number of results to display
        #[arg(short, long)]
        limit: Option<usize>,
    },

    /// Clear command history
    ///
    /// Examples:
    ///   cmdrun history clear
    ///   cmdrun history clear --force
    Clear {
        /// Skip confirmation prompt
        #[arg(short, long)]
        force: bool,
    },

    /// Export history to file
    ///
    /// Examples:
    ///   cmdrun history export --format json -o history.json
    ///   cmdrun history export --format csv --limit 100
    Export {
        /// Export format
        #[arg(short, long, value_enum, default_value = "json")]
        format: ExportFormat,

        /// Output file path (prints to stdout if not specified)
        #[arg(short, long)]
        output: Option<PathBuf>,

        /// Maximum number of entries to export
        #[arg(short, long)]
        limit: Option<usize>,
    },

    /// Show history statistics
    ///
    /// Display aggregate statistics including:
    ///   - Total executions
    ///   - Success/failure rates
    ///   - Most frequently executed commands
    ///   - Average execution duration
    ///   - Most frequent failure reasons
    Stats,
}

/// Export format for history
#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum ExportFormat {
    /// JSON format (machine-readable, preserves all data)
    Json,
    /// CSV format (spreadsheet-compatible, tabular data)
    Csv,
}

/// Template management actions
#[derive(Subcommand, Debug)]
pub enum TemplateAction {
    /// Add a new template from current configuration
    ///
    /// Create a new template based on your current commands.toml file.
    /// The template will be saved to ~/.cmdrun/templates/ and can be
    /// reused in other projects.
    Add {
        /// Template name (optional, will prompt if not provided)
        name: Option<String>,
    },

    /// Use a template to create/update commands.toml
    ///
    /// Apply a template to create a new commands.toml file or update
    /// an existing one. Available templates include built-in templates
    /// (rust-cli, nodejs-web, python-data, react-app) and your custom
    /// user templates.
    Use {
        /// Template name
        name: String,

        /// Output path (default: commands.toml)
        #[arg(short, long)]
        output: Option<PathBuf>,
    },

    /// List all available templates
    ///
    /// Show all available templates including built-in templates and
    /// your custom user templates.
    List {
        /// Show detailed information
        #[arg(short, long)]
        verbose: bool,
    },

    /// Remove a user template
    ///
    /// Delete a custom user template. Built-in templates cannot be removed.
    Remove {
        /// Template name
        name: String,

        /// Skip confirmation prompt
        #[arg(short, long)]
        force: bool,
    },

    /// Export a template to a file
    ///
    /// Export a template (built-in or user) to a TOML file for sharing
    /// or backup purposes.
    Export {
        /// Template name
        name: String,

        /// Output file path
        output: PathBuf,
    },

    /// Import a template from a file
    ///
    /// Import a template from a TOML file. The template will be validated
    /// and saved to ~/.cmdrun/templates/.
    Import {
        /// Template file path
        file: PathBuf,
    },
}

/// Graph output format
#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum GraphFormat {
    /// Tree-like text output (default, human-readable)
    Tree,
    /// DOT format for Graphviz (generate diagrams with 'dot' tool)
    Dot,
    /// Mermaid diagram format (for documentation, GitHub, GitLab)
    Mermaid,
}

/// Plugin management actions
#[cfg(feature = "plugin-system")]
#[derive(Subcommand, Debug)]
pub enum PluginAction {
    /// List all installed plugins
    ///
    /// Display all plugins discovered in the plugins directory.
    /// Use --enabled to show only active plugins.
    ///
    /// Examples:
    ///   cmdrun plugin list
    ///   cmdrun plugin list --enabled
    ///   cmdrun plugin list --verbose
    List {
        /// Show only enabled plugins
        #[arg(short, long)]
        enabled: bool,

        /// Show detailed information including hooks and metadata
        #[arg(short, long)]
        verbose: bool,
    },

    /// Show detailed plugin information
    ///
    /// Display comprehensive information about a plugin including:
    ///   - Plugin metadata (name, version, author)
    ///   - Enabled/disabled status
    ///   - Registered hooks (pre-run, post-run, on-error)
    ///   - Configuration options
    ///
    /// Examples:
    ///   cmdrun plugin info notifications
    Info {
        /// Plugin name
        name: String,
    },

    /// Enable a plugin
    ///
    /// Activate a previously disabled plugin. The plugin will start
    /// executing its hooks on subsequent command runs.
    ///
    /// Examples:
    ///   cmdrun plugin enable notifications
    Enable {
        /// Plugin name
        name: String,
    },

    /// Disable a plugin
    ///
    /// Deactivate a plugin temporarily without removing it. The plugin
    /// will not execute its hooks until re-enabled.
    ///
    /// Examples:
    ///   cmdrun plugin disable analytics
    Disable {
        /// Plugin name
        name: String,
    },
}
