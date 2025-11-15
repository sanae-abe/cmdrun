//! cmdrun - Fast, secure, and cross-platform command runner
//!
//! A modern replacement for package.json scripts and Makefiles.

use anyhow::Result;
use clap::Parser;
#[cfg(feature = "plugin-system")]
use cmdrun::cli::PluginAction;
use cmdrun::cli::{
    Cli, ColorChoice, Commands, ConfigAction, EnvAction, GraphFormat, HistoryAction, TemplateAction,
};
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

    // Configure color output (must be done before any colored output)
    configure_color_output(cli.color);

    // Initialize logging (skip for CompletionList to avoid polluting shell completion)
    if !matches!(cli.command, Commands::CompletionList) {
        init_logging(cli.verbose, cli.color);
    }

    // Run command
    if let Err(e) = run(cli).await {
        eprintln!("{} {}", "Error:".red().bold(), e);
        process::exit(1);
    }
}

/// Configure colored output based on CLI arguments and environment
fn configure_color_output(color_choice: ColorChoice) {
    use colored::control;

    match color_choice {
        ColorChoice::Never => {
            // Force disable colored output
            control::set_override(false);
        }
        ColorChoice::Always => {
            // Force enable colored output (even when piping)
            control::set_override(true);
        }
        ColorChoice::Auto => {
            // Respect NO_COLOR environment variable
            if std::env::var("NO_COLOR").is_ok() {
                control::set_override(false);
            }
            // Otherwise, let colored crate auto-detect (TTY/pipe)
        }
    }
}

/// Main execution flow
async fn run(cli: Cli) -> Result<()> {
    // Extract config path and global flag before matching on command
    let config_path = cli.config.clone();
    let global_only = cli.global;

    match cli.command {
        Commands::Run {
            name,
            args,
            parallel,
        } => {
            run_command(&name, args, parallel, global_only, config_path).await?;
        }
        Commands::List { verbose } => {
            list_commands(verbose, global_only, config_path).await?;
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
            show_dependency_graph(
                command,
                format,
                output,
                show_groups,
                global_only,
                config_path,
            )
            .await?;
        }
        Commands::Completion { shell } => {
            use cmdrun::config::Language;
            cmdrun::commands::handle_completion(shell, Language::English);
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
            cmdrun::commands::handle_info(id, global_only, config_path).await?;
        }
        Commands::Search { keyword } => {
            cmdrun::commands::handle_search(keyword, global_only, config_path).await?;
        }
        Commands::CompletionList => {
            list_completion(global_only, config_path).await?;
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
                use cmdrun::config::Language;
                cmdrun::commands::handle_current(Language::English).await?;
            }
            EnvAction::List => {
                use cmdrun::config::Language;
                cmdrun::commands::handle_env_list(Language::English).await?;
            }
            EnvAction::Set { key, value, env } => {
                cmdrun::commands::handle_env_set(key, value, env).await?;
            }
            EnvAction::Create { name, description } => {
                cmdrun::commands::handle_create(name, description).await?;
            }
            EnvAction::Info { name } => {
                use cmdrun::config::Language;
                cmdrun::commands::handle_env_info(name, Language::English).await?;
            }
        },
        Commands::History { action } => match action {
            HistoryAction::List {
                limit,
                offset,
                failed,
                stats,
            } => {
                use cmdrun::config::Language;
                cmdrun::commands::handle_history(
                    Some(limit),
                    offset,
                    failed,
                    stats,
                    Language::English,
                )
                .await?;
            }
            HistoryAction::Search { query, limit } => {
                use cmdrun::config::Language;
                cmdrun::commands::handle_history_search(&query, limit, Language::English).await?;
            }
            HistoryAction::Clear { force } => {
                cmdrun::commands::handle_history_clear(force).await?;
            }
            HistoryAction::Export {
                format,
                output,
                limit,
            } => {
                let export_format = match format {
                    cmdrun::cli::ExportFormat::Json => cmdrun::commands::ExportFormat::Json,
                    cmdrun::cli::ExportFormat::Csv => cmdrun::commands::ExportFormat::Csv,
                };
                cmdrun::commands::handle_history_export(export_format, output, limit).await?;
            }
            HistoryAction::Stats => {
                use cmdrun::config::Language;
                cmdrun::commands::handle_history(None, None, false, true, Language::English)
                    .await?;
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
                use cmdrun::config::Language;
                cmdrun::commands::handle_template_list(verbose, Language::English).await?;
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
        #[cfg(feature = "plugin-system")]
        Commands::Plugin { action } => match action {
            PluginAction::List { enabled, verbose } => {
                use cmdrun::config::Language;
                cmdrun::commands::handle_plugin_list(
                    enabled,
                    verbose,
                    config_path,
                    Language::English,
                )
                .await?;
            }
            PluginAction::Info { name } => {
                use cmdrun::config::Language;
                cmdrun::commands::handle_plugin_info(&name, config_path, Language::English).await?;
            }
            PluginAction::Enable { name } => {
                cmdrun::commands::handle_plugin_enable(&name, config_path).await?;
            }
            PluginAction::Disable { name } => {
                cmdrun::commands::handle_plugin_disable(&name, config_path).await?;
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
    global_only: bool,
    config_path: Option<std::path::PathBuf>,
) -> Result<()> {
    // Initialize history recorder
    let storage = cmdrun::history::HistoryStorage::new()?;
    let mut recorder = cmdrun::history::HistoryRecorder::with_storage(storage);

    // Load configuration (with environment support)
    let config_loader = if let Some(path) = config_path {
        ConfigLoader::with_path(path)?
    } else if global_only {
        ConfigLoader::global_only()
    } else {
        ConfigLoader::new()
    };

    // Try to load config (with environment), fallback to global-only if no local config
    let config = match config_loader.load_with_environment().await {
        Ok(cfg) => cfg,
        Err(_) => {
            // If local config not found, try loading from global config
            if let Some(global_dir) = dirs::config_dir() {
                let global_path = global_dir.join("cmdrun").join("commands.toml");
                if global_path.exists() {
                    ConfigLoader::with_path(global_path)?.load().await?
                } else {
                    // Use default language (English) for error message when config not found
                    use cmdrun::config::Language;
                    use cmdrun::i18n::{get_message, MessageKey};
                    anyhow::bail!(
                        "{}",
                        get_message(MessageKey::ErrorNoConfigFileFound, Language::English)
                    );
                }
            } else {
                use cmdrun::config::Language;
                use cmdrun::i18n::{get_message, MessageKey};
                anyhow::bail!(
                    "{}",
                    get_message(MessageKey::ErrorCannotDetermineConfigDir, Language::English)
                );
            }
        }
    };

    // Find command
    let command = match config.commands.get(name) {
        Some(cmd) => cmd,
        None => {
            // Typo detection if command not found
            if config.config.typo_detection {
                use cmdrun::i18n::{get_message, MessageKey};
                use cmdrun::utils::typo_detector::{TypoDetector, TypoDetectorConfig};

                let detector = TypoDetector::with_config(TypoDetectorConfig {
                    threshold: config.config.typo_threshold,
                    max_suggestions: 5,
                });

                let available_commands: Vec<&str> =
                    config.commands.keys().map(|s| s.as_str()).collect();
                let suggestions = detector.suggest(name, &available_commands);

                if !suggestions.is_empty() {
                    let language = config.config.language;
                    eprintln!(
                        "{} '{}'",
                        get_message(MessageKey::TypoUnknownCommand, language)
                            .red()
                            .bold(),
                        name.bright_white()
                    );
                    eprintln!();
                    eprintln!(
                        "{} {}",
                        "💡".bright_white(),
                        get_message(MessageKey::TypoDidYouMean, language)
                    );
                    for (suggestion, distance) in suggestions {
                        eprintln!(
                            "  {} {} {}",
                            "→".cyan(),
                            suggestion.green().bold(),
                            format!("(distance: {})", distance).dimmed()
                        );
                    }
                    eprintln!();
                    eprintln!(
                        "{} {}",
                        "ℹ".bright_white(),
                        get_message(MessageKey::TypoRunHelp, language).dimmed()
                    );
                    anyhow::bail!(
                        "{}",
                        get_message(MessageKey::ErrorCommandNotFound, language)
                    );
                }
            }

            use cmdrun::config::Language;
            use cmdrun::i18n::{get_message, MessageKey};
            return Err(anyhow::anyhow!(
                "{}",
                get_message(MessageKey::ErrorCommandNotFound, Language::English)
            ));
        }
    };

    // Create execution context with positional arguments
    let mut env = config.config.env.clone();

    // Add positional arguments as environment variables: 1, 2, 3, ...
    for (idx, arg) in args.iter().enumerate() {
        env.insert((idx + 1).to_string(), arg.clone());
    }

    let ctx = ExecutionContext {
        working_dir: config.config.working_dir.clone(),
        env: env.clone(),
        shell: detect_shell()
            .map(|s| s.name)
            .unwrap_or_else(|_| config.config.shell.clone()),
        timeout: command.timeout.or(Some(config.config.timeout)),
        strict: config.config.strict_mode,
        echo: true,
        color: true,
        language: config.config.language,
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

            // 結果チェックと履歴記録
            for (cmd_idx, result) in results.iter().enumerate() {
                let cmd_name = group.commands[cmd_idx];
                let duration_ms = result.duration.as_millis() as i64;

                // 各コマンドの履歴を記録
                if let Err(e) = recorder.record(
                    cmd_name,
                    &args,
                    &env,
                    duration_ms,
                    result.exit_code,
                    result.success,
                ) {
                    eprintln!("Warning: Failed to record command history: {}", e);
                }

                if !result.success {
                    // Record failure state before bailing
                    let _id = recorder.record(
                        name,
                        &args,
                        &env,
                        result.duration.as_millis() as i64,
                        result.exit_code,
                        false,
                    );
                    use cmdrun::i18n::{get_message, MessageKey};
                    anyhow::bail!(
                        "{} {}",
                        get_message(
                            MessageKey::ErrorCommandExecutionFailed,
                            config.config.language
                        ),
                        result.exit_code
                    );
                }
            }
        }

        let total_duration = start.elapsed();
        let duration_ms = total_duration.as_millis() as i64;

        // メインコマンドの履歴を記録（すべて成功した場合）
        if let Err(e) = recorder.record(name, &args, &env, duration_ms, 0, true) {
            eprintln!("Warning: Failed to record main command history: {}", e);
        }

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

        // Execute and always record history (even on failure)
        let result = match executor.execute(command).await {
            Ok(r) => r,
            Err(e) => {
                // Record failed execution in history before returning error
                let _ = recorder.record(name, &args, &env, 0, 1, false);
                return Err(e.into());
            }
        };

        let duration_ms = result.duration.as_millis() as i64;

        // 履歴を記録
        if let Err(e) = recorder.record(
            name,
            &args,
            &env,
            duration_ms,
            result.exit_code,
            result.success,
        ) {
            eprintln!("Warning: Failed to record command history: {}", e);
        }

        if result.success {
            println!(
                "{} Completed in {:.2}s",
                "✓".green().bold(),
                result.duration.as_secs_f64()
            );
        } else {
            use cmdrun::i18n::{get_message, MessageKey};
            anyhow::bail!(
                "{} {}",
                get_message(
                    MessageKey::ErrorCommandExecutionFailed,
                    config.config.language
                ),
                result.exit_code
            );
        }
    }

    Ok(())
}

/// List available commands
async fn list_commands(
    verbose: bool,
    global: bool,
    config_path: Option<std::path::PathBuf>,
) -> Result<()> {
    let config_loader = if let Some(path) = config_path {
        ConfigLoader::with_path(path)?
    } else if global {
        ConfigLoader::global_only()
    } else {
        ConfigLoader::new()
    };
    let config = config_loader.load_with_environment().await?;
    let lang = config.config.language;

    if config.commands.is_empty() {
        println!(
            "{}",
            cmdrun::i18n::get_message(cmdrun::i18n::MessageKey::ListNoCommandsDefined, lang)
                .yellow()
        );
        return Ok(());
    }

    println!(
        "{}",
        cmdrun::i18n::get_message(cmdrun::i18n::MessageKey::ListAvailableCommands, lang)
            .cyan()
            .bold()
    );
    println!();

    let mut commands: Vec<_> = config.commands.iter().collect();
    commands.sort_by_key(|(name, _)| *name);

    for (name, cmd) in commands {
        if verbose {
            println!("  {} - {}", name.green().bold(), cmd.description);
            println!(
                "    {}",
                cmdrun::i18n::get_message(cmdrun::i18n::MessageKey::LabelCommand, lang).dimmed()
            );
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
                println!(
                    "    {} {:?}",
                    cmdrun::i18n::get_message(cmdrun::i18n::MessageKey::LabelDependencies, lang)
                        .dimmed(),
                    cmd.deps
                );
            }
            println!();
        } else {
            println!("  {} - {}", name.green().bold(), cmd.description);
        }
    }

    Ok(())
}

/// List command names for shell completion (with descriptions)
async fn list_completion(global_only: bool, config_path: Option<std::path::PathBuf>) -> Result<()> {
    let config_loader = if let Some(path) = config_path {
        ConfigLoader::with_path(path)?
    } else if global_only {
        ConfigLoader::global_only()
    } else {
        ConfigLoader::new()
    };

    // Try to load config (with environment), fallback to global-only if no local config
    let config = match config_loader.load_with_environment().await {
        Ok(cfg) => cfg,
        Err(_) => {
            // If local config not found, try loading from global config
            if let Some(global_dir) = dirs::config_dir() {
                let global_path = global_dir.join("cmdrun").join("commands.toml");
                if global_path.exists() {
                    match ConfigLoader::with_path(global_path)?.load().await {
                        Ok(cfg) => cfg,
                        Err(_) => return Ok(()), // Global config invalid, return empty
                    }
                } else {
                    return Ok(()); // No global config, return empty
                }
            } else {
                return Ok(()); // Can't determine config directory, return empty
            }
        }
    };

    // Output command names with descriptions (format: "name:description")
    // This format is compatible with bash/zsh completion systems
    for (name, cmd) in config.commands.iter() {
        // Escape colons in description to avoid parsing issues
        let desc = cmd.description.replace(':', "\\:");
        println!("{}:{}", name, desc);
    }

    Ok(())
}

/// Show dependency graph
async fn show_dependency_graph(
    command: Option<String>,
    format: GraphFormat,
    output_path: Option<std::path::PathBuf>,
    show_groups: bool,
    global_only: bool,
    config_path: Option<std::path::PathBuf>,
) -> Result<()> {
    let config_loader = if let Some(path) = config_path {
        ConfigLoader::with_path(path)?
    } else if global_only {
        ConfigLoader::global_only()
    } else {
        ConfigLoader::new()
    };
    let config = config_loader.load_with_environment().await?;

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
fn init_logging(verbose: u8, color_choice: ColorChoice) {
    use std::io::IsTerminal;
    use tracing_subscriber::fmt::format::FmtSpan;

    let level = match verbose {
        0 => "info",
        1 => "debug",
        _ => "trace",
    };

    // Determine if colors should be used
    let use_colors = match color_choice {
        ColorChoice::Never => false,
        ColorChoice::Always => true,
        ColorChoice::Auto => {
            // Auto mode: use colors if NO_COLOR is not set and stderr is a TTY
            std::env::var("NO_COLOR").is_err() && std::io::stderr().is_terminal()
        }
    };

    let result = tracing_subscriber::fmt()
        .with_env_filter(level)
        .with_span_events(FmtSpan::CLOSE)
        .with_ansi(use_colors)
        .try_init();

    if let Err(e) = result {
        eprintln!("Warning: Failed to initialize logging: {}", e);
    }
}
