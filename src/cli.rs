//! CLI argument definitions

use clap::{Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    name = "cmdrun",
    version,
    about = "Fast, secure, and cross-platform command runner",
    long_about = "A modern replacement for package.json scripts and Makefiles"
)]
pub struct Cli {
    /// Path to configuration file (default: ~/.config/cmdrun/commands.toml)
    ///
    /// Use this option to specify which configuration file to use.
    /// This allows you to maintain multiple command sets for different
    /// purposes (work, personal, projects, environments, etc.)
    #[arg(short, long, value_name = "FILE", global = true)]
    pub config: Option<PathBuf>,

    /// Subcommand to execute
    #[command(subcommand)]
    pub command: Commands,

    /// Verbose output (-v, -vv, -vvv for more verbosity)
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub verbose: u8,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Run a command
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

    /// List available commands
    List {
        /// Show detailed information
        #[arg(short, long)]
        verbose: bool,
    },

    /// Initialize a new commands.toml file
    Init {
        /// Template to use (web, rust, node, python)
        #[arg(short, long)]
        template: Option<String>,

        /// Use interactive mode
        #[arg(short, long)]
        interactive: bool,

        /// Output path (default: commands.toml)
        #[arg(short, long)]
        output: Option<PathBuf>,
    },

    /// Validate configuration file
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

    /// Show dependency graph
    Graph {
        /// Specific command to show dependencies for
        command: Option<String>,

        /// Output format
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
    Completion {
        /// Shell to generate completion for
        #[arg(value_enum)]
        shell: clap_complete::Shell,
    },

    /// Remove a command from the configuration
    Remove {
        /// Command ID to remove
        id: String,

        /// Skip confirmation prompt
        #[arg(short, long)]
        force: bool,
    },

    /// Add a new command to the configuration
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
    Open,

    /// Edit an existing command interactively
    Edit {
        /// Command ID to edit (optional - will prompt if not provided)
        id: Option<String>,
    },

    /// Show detailed information about a command
    Info {
        /// Command ID to show info for (optional - will prompt if not provided)
        id: Option<String>,
    },

    /// Search commands by keyword
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

/// Graph output format
#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum GraphFormat {
    /// Tree-like text output (default)
    Tree,
    /// DOT format (Graphviz)
    Dot,
    /// Mermaid diagram format
    Mermaid,
}
