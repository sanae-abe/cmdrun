//! cmdrun - Fast, secure, and cross-platform command runner
//!
//! A modern replacement for package.json scripts and Makefiles.

use anyhow::Result;
use clap::Parser;
use cmdrun::cli::{Cli, Commands, ConfigAction, EnvAction, GraphFormat, HistoryAction, TemplateAction};
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
    // Extract config path before matching on command
    let config_path = cli.config.clone();

    match cli.command {
        Commands::Run {
            name,
            args,
            parallel,
        } => {
            run_command(&name, args, parallel, config_path).await?;
        }
        Commands::List { verbose } => {
            list_commands(verbose, config_path).await?;
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
        Commands::Graph {
            command,
            format,
            output,
            show_groups,
        } => {
            show_dependency_graph(command, format, output, show_groups, config_path).await?;
        }
        Commands::Completion { shell } => {
            cmdrun::commands::handle_completion(shell);
        }
        Commands::Remove { id, force } => {
            cmdrun::commands::handle_remove(id, force, config_path).await?;
        }
        Commands::Add {
            id,
            command,
            description,
            category,
            tags,
        } => {
            cmdrun::commands::handle_add(id, command, description, category, tags, config_path)
                .await?;
        }
        Commands::Open => {
            cmdrun::commands::handle_open(config_path).await?;
        }
        Commands::Edit { id } => {
            cmdrun::commands::handle_edit(id, config_path).await?;
        }
        Commands::Info { id } => {
            cmdrun::commands::handle_info(id, config_path).await?;
        }
        Commands::Search { keyword } => {
            cmdrun::commands::handle_search(keyword, config_path).await?;
        }
        Commands::CompletionList => {
            list_completion(config_path).await?;
        }
        Commands::Config { action } => match action {
            ConfigAction::Get { key } => {
                cmdrun::commands::handle_get(&key, config_path).await?;
            }
            ConfigAction::Set { key, value } => {
                cmdrun::commands::handle_set(&key, &value, config_path).await?;
            }
            ConfigAction::Show => {
                cmdrun::commands::handle_show(config_path).await?;
            }
        },
        Commands::Watch {
            command,
            args,
            paths,
            patterns,
            exclude,
            debounce,
            ignore_gitignore,
            no_recursive,
        } => {
            cmdrun::commands::handle_watch(
                command,
                args,
                paths,
                patterns,
                exclude,
                debounce,
                ignore_gitignore,
                no_recursive,
            )
            .await?;
        }
        Commands::Env { action } => match action {
            EnvAction::Use { name } => {
                cmdrun::commands::handle_use(name).await?;
            }
            EnvAction::Current => {
                cmdrun::commands::handle_current().await?;
            }
            EnvAction::List => {
                cmdrun::commands::handle_env_list().await?;
            }
            EnvAction::Set { key, value, env } => {
                cmdrun::commands::handle_env_set(key, value, env).await?;
            }
            EnvAction::Create { name, description } => {
                cmdrun::commands::handle_create(name, description).await?;
            }
            EnvAction::Info { name } => {
                cmdrun::commands::handle_env_info(name).await?;
            }
        },
        Commands::History { action } => match action {
            HistoryAction::List { limit, offset, failed, stats } => {
                cmdrun::commands::handle_history(Some(limit), offset, failed, stats).await?;
            }
            HistoryAction::Search { query, limit } => {
                cmdrun::commands::handle_history_search(&query, limit).await?;
            }
            HistoryAction::Clear { force } => {
                cmdrun::commands::handle_history_clear(force).await?;
            }
            HistoryAction::Export { format, output, limit } => {
                let export_format = match format {
                    cmdrun::cli::ExportFormat::Json => cmdrun::commands::ExportFormat::Json,
                    cmdrun::cli::ExportFormat::Csv => cmdrun::commands::ExportFormat::Csv,
                };
                cmdrun::commands::handle_history_export(export_format, output, limit).await?;
            }
            HistoryAction::Stats => {
                cmdrun::commands::handle_history(None, None, false, true).await?;
            }
        },
        Commands::Retry { id } => {
            cmdrun::commands::handle_retry(id).await?;
        }
        Commands::Template { action } => match action {
            TemplateAction::Add { name } => {
                cmdrun::commands::handle_template_add(name, config_path).await?;
            }
            TemplateAction::Use { name, output } => {
                cmdrun::commands::handle_template_use(name, output).await?;
            }
            TemplateAction::List { verbose } => {
                cmdrun::commands::handle_template_list(verbose).await?;
            }
            TemplateAction::Remove { name, force } => {
                cmdrun::commands::handle_template_remove(name, force).await?;
            }
            TemplateAction::Export { name, output } => {
                cmdrun::commands::handle_template_export(name, output).await?;
            }
            TemplateAction::Import { file } => {
                cmdrun::commands::handle_template_import(file).await?;
            }
        },
    }

    Ok(())
}

/// Run a command
async fn run_command(
    name: &str,
    args: Vec<String>,
    parallel: bool,
    config_path: Option<std::path::PathBuf>,
) -> Result<()> {
    // Load configuration
    let config_loader = if let Some(path) = config_path {
        ConfigLoader::with_path(path)
    } else {
        ConfigLoader::new()
    };
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
async fn list_commands(verbose: bool, config_path: Option<std::path::PathBuf>) -> Result<()> {
    let config_loader = if let Some(path) = config_path {
        ConfigLoader::with_path(path)
    } else {
        ConfigLoader::new()
    };
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
async fn list_completion(config_path: Option<std::path::PathBuf>) -> Result<()> {
    let config_loader = if let Some(path) = config_path {
        ConfigLoader::with_path(path)
    } else {
        ConfigLoader::new()
    };
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
    config_path: Option<std::path::PathBuf>,
) -> Result<()> {
    let config_loader = if let Some(path) = config_path {
        ConfigLoader::with_path(path)
    } else {
        ConfigLoader::new()
    };
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
