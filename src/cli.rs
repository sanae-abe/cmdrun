//! CLI argument definitions

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(
    name = "cmdrun",
    version,
    about = "Fast, secure, and cross-platform command runner",
    long_about = "A modern replacement for package.json scripts and Makefiles"
)]
pub struct Cli {
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
    },

    /// List available commands
    List {
        /// Show detailed information
        #[arg(short, long)]
        verbose: bool,
    },

    /// Initialize a new commands.toml file
    Init,

    /// Validate configuration file
    Validate,

    /// Show dependency graph
    Graph {
        /// Specific command to show dependencies for
        command: Option<String>,
    },

    /// Generate shell completion scripts
    Completion {
        /// Shell to generate completion for
        #[arg(value_enum)]
        shell: clap_complete::Shell,
    },
}
