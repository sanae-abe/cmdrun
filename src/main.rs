//! cmdrun - Fast, secure, and cross-platform command runner
//!
//! A modern replacement for package.json scripts and Makefiles.

use anyhow::Result;
use clap::Parser;
use cmdrun::cli::{Cli, Commands, GraphFormat};
use cmdrun::command::dependency::DependencyGraph;
use cmdrun::command::executor::{CommandExecutor, ExecutionContext};
use cmdrun::command::graph_visualizer::GraphVisualizer;
use cmdrun::config::loader::ConfigLoader;
use cmdrun::platform::shell::detect_shell;
use colored::*;
use std::fs;
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
        Commands::Run {
            name,
            args,
            parallel,
        } => {
            run_command(&name, args, parallel).await?;
        }
        Commands::List { verbose } => {
            list_commands(verbose).await?;
        }
        Commands::Init {
            template,
            interactive,
            output,
        } => {
            cmdrun::commands::handle_init(template, interactive, output).await?;
        }
        Commands::Validate {
            path,
            verbose,
            check_cycles,
        } => {
            cmdrun::commands::handle_validate(path, verbose, check_cycles).await?;
        }
        Commands::Graph { command, format, output, show_groups } => {
            show_dependency_graph(command, format, output, show_groups).await?;
        }
        Commands::Completion { shell } => {
            cmdrun::commands::handle_completion(shell);
        }
        Commands::Remove { id, force, config } => {
            cmdrun::commands::handle_remove(id, force, config).await?;
        }
        Commands::Add {
            id,
            command,
            description,
            category,
            tags,
        } => {
            cmdrun::commands::handle_add(id, command, description, category, tags).await?;
        }
        Commands::Open => {
            cmdrun::commands::handle_open().await?;
        }
        Commands::Edit { id } => {
            cmdrun::commands::handle_edit(id).await?;
        }
        Commands::Info { id } => {
            cmdrun::commands::handle_info(id).await?;
        }
        Commands::Search { keyword } => {
            cmdrun::commands::handle_search(keyword).await?;
        }
        Commands::CompletionList => {
            list_completion().await?;
        }
    }

    Ok(())
}

/// Run a command
async fn run_command(name: &str, args: Vec<String>, parallel: bool) -> Result<()> {
    // Load configuration
    let config_loader = ConfigLoader::new();
    let config = config_loader.load().await?;

    // Find command
    let command = config
        .commands
        .get(name)
        .ok_or_else(|| anyhow::anyhow!("Command not found: {}", name))?;

    // Create execution context with positional arguments
    let mut env = config.config.env.clone();

    // Add positional arguments as environment variables: 1, 2, 3, ...
    for (idx, arg) in args.iter().enumerate() {
        env.insert((idx + 1).to_string(), arg.clone());
    }

    let ctx = ExecutionContext {
        working_dir: config.config.working_dir.clone(),
        env,
        shell: detect_shell()
            .map(|s| s.name)
            .unwrap_or_else(|_| config.config.shell.clone()),
        timeout: command.timeout.or(Some(config.config.timeout)),
        strict: config.config.strict_mode,
        echo: true,
        color: true,
    };

    let executor = CommandExecutor::new(ctx);

    // 並列実行が指定されている場合、依存関係を解決して並列実行
    if parallel || command.parallel {
        println!(
            "{} {} (with parallel dependencies)",
            "Running:".cyan().bold(),
            command.description.bright_white()
        );

        let start = std::time::Instant::now();

        // 依存関係グラフを構築
        let dep_graph = DependencyGraph::new(&config);

        // 循環依存チェック
        dep_graph.check_cycles()?;

        // 実行グループを解決
        let groups = dep_graph.resolve(name)?;

        println!(
            "{} Execution plan: {} groups",
            "📋".bright_white(),
            groups.len()
        );

        // 各グループを順次実行（グループ内は並列）
        for (idx, group) in groups.iter().enumerate() {
            println!(
                "{} Group {}/{} ({} commands)",
                "▶".blue().bold(),
                idx + 1,
                groups.len(),
                group.commands.len()
            );

            // グループ内のコマンドを取得
            let commands: Vec<_> = group
                .commands
                .iter()
                .filter_map(|cmd_name| config.commands.get(*cmd_name))
                .collect();

            // 並列実行
            let results = executor.execute_parallel(&commands).await?;

            // 結果チェック
            for result in results {
                if !result.success {
                    anyhow::bail!("Command failed with exit code {}", result.exit_code);
                }
            }
        }

        let total_duration = start.elapsed();
        println!(
            "{} All commands completed in {:.2}s",
            "✓".green().bold(),
            total_duration.as_secs_f64()
        );
    } else {
        // 逐次実行（従来の動作）
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

/// List command names for shell completion
async fn list_completion() -> Result<()> {
    let config_loader = ConfigLoader::new();
    let config = config_loader.load().await?;

    // Output command names one per line for shell completion
    for name in config.commands.keys() {
        println!("{}", name);
    }

    Ok(())
}

/// Show dependency graph
async fn show_dependency_graph(
    command: Option<String>,
    format: GraphFormat,
    output_path: Option<std::path::PathBuf>,
    show_groups: bool,
) -> Result<()> {
    let config_loader = ConfigLoader::new();
    let config = config_loader.load().await?;

    // グラフ視覚化
    let visualizer = GraphVisualizer::new(&config);
    let graph_output = visualizer.visualize(command.as_deref(), format, show_groups)?;

    // 出力
    if let Some(path) = output_path {
        fs::write(&path, &graph_output)?;
        println!(
            "{} Graph saved to: {}",
            "✓".green().bold(),
            path.display().to_string().bright_white()
        );

        // ファイル形式のヒント
        match format {
            GraphFormat::Dot => {
                println!(
                    "{} Render with: {}",
                    "💡".bright_white(),
                    format!("dot -Tpng {} -o graph.png", path.display()).dimmed()
                );
            }
            GraphFormat::Mermaid => {
                println!(
                    "{} View at: {}",
                    "💡".bright_white(),
                    "https://mermaid.live".dimmed()
                );
            }
            _ => {}
        }
    } else {
        // 標準出力
        print!("{}", graph_output);
    }

    Ok(())
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
