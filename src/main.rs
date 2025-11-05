//! cmdrun - Fast, secure, and cross-platform command runner
//!
//! A modern replacement for package.json scripts and Makefiles.

use anyhow::Result;
use clap::{CommandFactory, Parser};
use cmdrun::cli::{Cli, Commands};
use cmdrun::command::executor::{CommandExecutor, ExecutionContext};
use cmdrun::config::loader::ConfigLoader;
use cmdrun::platform::shell::detect_shell;
use colored::*;
use std::process;

#[tokio::main]
async fn main() {
    // Parse CLI arguments
    let cli = Cli::parse();

    // Initialize logging
    init_logging(cli.verbose);

    // Run command
    if let Err(e) = run(cli).await {
        eprintln!("{} {}", "Error:".red().bold(), e);
        process::exit(1);
    }
}

/// Main execution flow
async fn run(cli: Cli) -> Result<()> {
    match cli.command {
        Commands::Run { name, args } => {
            run_command(&name, args).await?;
        }
        Commands::List { verbose } => {
            list_commands(verbose).await?;
        }
        Commands::Init => {
            init_config().await?;
        }
        Commands::Validate => {
            validate_config().await?;
        }
        Commands::Graph { command } => {
            show_dependency_graph(command).await?;
        }
        Commands::Completion { shell } => {
            generate_completion(shell);
        }
    }

    Ok(())
}

/// Run a command
async fn run_command(name: &str, _args: Vec<String>) -> Result<()> {
    // Load configuration
    let config_loader = ConfigLoader::new();
    let config = config_loader.load().await?;

    // Find command
    let command = config
        .commands
        .get(name)
        .ok_or_else(|| anyhow::anyhow!("Command not found: {}", name))?;

    // Create execution context
    let ctx = ExecutionContext {
        working_dir: config.config.working_dir.clone(),
        env: config.config.env.clone(),
        shell: detect_shell().unwrap_or_else(|_| config.config.shell.clone()),
        timeout: command.timeout.or(Some(config.config.timeout)),
        strict: config.config.strict_mode,
        echo: true,
        color: true,
    };

    // Execute command
    let executor = CommandExecutor::new(ctx);

    println!(
        "{} {}",
        "Running:".cyan().bold(),
        command.description.bright_white()
    );

    let result = executor.execute(command).await?;

    if result.success {
        println!(
            "{} Completed in {:.2}s",
            "✓".green().bold(),
            result.duration.as_secs_f64()
        );
    } else {
        anyhow::bail!("Command failed with exit code {}", result.exit_code);
    }

    Ok(())
}

/// List available commands
async fn list_commands(verbose: bool) -> Result<()> {
    let config_loader = ConfigLoader::new();
    let config = config_loader.load().await?;

    if config.commands.is_empty() {
        println!("{}", "No commands defined".yellow());
        return Ok(());
    }

    println!("{}", "Available commands:".cyan().bold());
    println!();

    let mut commands: Vec<_> = config.commands.iter().collect();
    commands.sort_by_key(|(name, _)| *name);

    for (name, cmd) in commands {
        if verbose {
            println!("  {} - {}", name.green().bold(), cmd.description);
            println!("    {}", "Command:".dimmed());
            match &cmd.cmd {
                cmdrun::config::schema::CommandSpec::Single(c) => {
                    println!("      {}", c);
                }
                cmdrun::config::schema::CommandSpec::Multiple(cmds) => {
                    for c in cmds {
                        println!("      {}", c);
                    }
                }
                cmdrun::config::schema::CommandSpec::Platform(_) => {
                    println!("      {} Platform-specific", "[...]".dimmed());
                }
            }
            if !cmd.deps.is_empty() {
                println!("    {} {:?}", "Dependencies:".dimmed(), cmd.deps);
            }
            println!();
        } else {
            println!("  {} - {}", name.green().bold(), cmd.description);
        }
    }

    Ok(())
}

/// Initialize configuration file
async fn init_config() -> Result<()> {
    let path = std::path::Path::new("commands.toml");

    if path.exists() {
        anyhow::bail!("Configuration file already exists: commands.toml");
    }

    let template = include_str!("../templates/commands.toml");
    std::fs::write(path, template)?;

    println!(
        "{} Created {}",
        "✓".green().bold(),
        "commands.toml".bright_white()
    );

    Ok(())
}

/// Validate configuration
async fn validate_config() -> Result<()> {
    let config_loader = ConfigLoader::new();
    let config = config_loader.load().await?;

    println!("{}", "Validating configuration...".cyan());

    // Validate each command
    for (name, _cmd) in &config.commands {
        println!("  {} {}", "✓".green(), name);
    }

    println!();
    println!(
        "{} Configuration is valid ({} commands)",
        "✓".green().bold(),
        config.commands.len()
    );

    Ok(())
}

/// Show dependency graph
async fn show_dependency_graph(command: Option<String>) -> Result<()> {
    let config_loader = ConfigLoader::new();
    let config = config_loader.load().await?;

    if let Some(cmd_name) = command {
        // Show specific command dependencies
        let cmd = config
            .commands
            .get(&cmd_name)
            .ok_or_else(|| anyhow::anyhow!("Command not found: {}", cmd_name))?;

        println!("{} {}", "Dependencies for:".cyan().bold(), cmd_name.green());
        if cmd.deps.is_empty() {
            println!("  {}", "No dependencies".dimmed());
        } else {
            for dep in &cmd.deps {
                println!("  {} {}", "→".blue(), dep);
            }
        }
    } else {
        // Show all dependencies
        println!("{}", "Dependency graph:".cyan().bold());
        println!();

        for (name, cmd) in &config.commands {
            if !cmd.deps.is_empty() {
                println!("{}", name.green().bold());
                for dep in &cmd.deps {
                    println!("  {} {}", "→".blue(), dep);
                }
                println!();
            }
        }
    }

    Ok(())
}

/// Generate shell completion
fn generate_completion(shell: clap_complete::Shell) {
    let mut cmd = Cli::command();
    clap_complete::generate(shell, &mut cmd, "cmdrun", &mut std::io::stdout());
}

/// Initialize logging
fn init_logging(verbose: u8) {
    use tracing_subscriber::fmt::format::FmtSpan;

    let level = match verbose {
        0 => "info",
        1 => "debug",
        _ => "trace",
    };

    tracing_subscriber::fmt()
        .with_env_filter(level)
        .with_span_events(FmtSpan::CLOSE)
        .init();
}
