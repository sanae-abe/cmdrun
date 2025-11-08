//! Watch command implementation

use anyhow::{Context, Result};
use colored::Colorize;
use std::env;
use std::path::PathBuf;
use tracing::{info, warn};

use crate::command::executor::ExecutionContext;
use crate::config::loader::ConfigLoader;
use crate::i18n::{get_message, MessageKey};
use crate::platform::shell::detect_shell;
use crate::watch::{WatchConfig, WatchPattern, WatchRunner};

/// Handle the watch command
///
/// Monitors files and automatically executes commands when changes are detected.
/// Supports glob patterns, debouncing, and gitignore integration.
#[allow(clippy::too_many_arguments)]
pub async fn handle_watch(
    command: String,
    args: Vec<String>,
    paths: Vec<PathBuf>,
    patterns: Vec<String>,
    exclude: Vec<String>,
    debounce_ms: u64,
    ignore_gitignore: bool,
    no_recursive: bool,
) -> Result<()> {
    // Load cmdrun configuration
    let config_loader = ConfigLoader::new();
    let cmdrun_config = config_loader.load().await?;
    let lang = cmdrun_config.config.language;

    // Validate that the command exists in the configuration
    let cmd_def = cmdrun_config.commands.get(&command).ok_or_else(|| {
        anyhow::anyhow!(
            "Unknown command: {}\n\nAvailable commands:\n{}",
            command,
            cmdrun_config
                .commands
                .keys()
                .map(|k| format!("  - {}", k))
                .collect::<Vec<_>>()
                .join("\n")
        )
    })?;

    // Display watch configuration
    display_watch_info(
        &command,
        &args,
        &paths,
        &patterns,
        &exclude,
        debounce_ms,
        lang,
    );

    // Get base path (current directory if no paths specified)
    let base_path = get_base_path(&paths)?;

    // Build watch configuration
    let watch_config = build_watch_config(
        paths,
        patterns,
        exclude,
        debounce_ms,
        ignore_gitignore,
        no_recursive,
    )?;

    // Create execution context for the command
    let mut env = cmdrun_config.config.env.clone();

    // Add positional arguments as environment variables
    for (idx, arg) in args.iter().enumerate() {
        env.insert((idx + 1).to_string(), arg.clone());
    }

    let exec_ctx = ExecutionContext {
        working_dir: cmdrun_config.config.working_dir.clone(),
        env,
        shell: detect_shell()
            .map(|s| s.name)
            .unwrap_or_else(|_| cmdrun_config.config.shell.clone()),
        timeout: cmd_def.timeout.or(Some(cmdrun_config.config.timeout)),
        strict: cmdrun_config.config.strict_mode,
        echo: false, // Don't echo in watch mode to reduce noise
        color: true,
        language: lang,
    };

    // Create and run the watch runner with cmdrun integration
    let mut runner = WatchRunner::new_with_cmdrun(
        watch_config,
        command.clone(),
        cmd_def.clone(),
        exec_ctx,
        &base_path,
    )
    .context("Failed to create watch runner")?;

    // Set up signal handler
    let mut shutdown_rx = setup_signal_handler().await?;

    // Run the watcher
    info!("{}", get_message(MessageKey::WatchModeStarted, lang));
    println!();

    // Run with shutdown signal
    tokio::select! {
        result = runner.run() => {
            result
        }
        _ = shutdown_rx.recv() => {
            info!("{}", get_message(MessageKey::WatchModeStoppedByUser, lang));
            Ok(())
        }
    }
}

/// Display watch configuration information
fn display_watch_info(
    command: &str,
    args: &[String],
    paths: &[PathBuf],
    patterns: &[String],
    exclude: &[String],
    debounce_ms: u64,
    lang: crate::config::schema::Language,
) {
    println!(
        "{}",
        get_message(MessageKey::WatchConfiguration, lang)
            .bright_green()
            .bold()
    );
    println!("{}", "═".repeat(60).bright_black());

    // Command
    print!(
        "  {} ",
        format!("{}:", get_message(MessageKey::WatchCommand, lang)).bright_cyan()
    );
    if args.is_empty() {
        println!("{}", command.bright_white());
    } else {
        println!(
            "{} {}",
            command.bright_white(),
            args.join(" ").bright_white()
        );
    }

    // Paths
    print!(
        "  {} ",
        format!("{}:", get_message(MessageKey::WatchWatching, lang)).bright_cyan()
    );
    if paths.is_empty() {
        println!("{}", ".".bright_white());
    } else {
        for (i, path) in paths.iter().enumerate() {
            if i == 0 {
                println!("{}", path.display().to_string().bright_white());
            } else {
                println!("            {}", path.display().to_string().bright_white());
            }
        }
    }

    // Patterns
    if !patterns.is_empty() {
        print!(
            "  {} ",
            format!("{}:", get_message(MessageKey::WatchPatterns, lang)).bright_cyan()
        );
        for (i, pattern) in patterns.iter().enumerate() {
            if i == 0 {
                println!("{}", pattern.bright_white());
            } else {
                println!("            {}", pattern.bright_white());
            }
        }
    }

    // Exclude patterns
    if !exclude.is_empty() {
        print!(
            "  {} ",
            format!("{}:", get_message(MessageKey::WatchExclude, lang)).bright_cyan()
        );
        for (i, pattern) in exclude.iter().enumerate() {
            if i == 0 {
                println!("{}", pattern.bright_yellow());
            } else {
                println!("            {}", pattern.bright_yellow());
            }
        }
    }

    // Debounce
    println!(
        "  {} {}ms",
        format!("{}:", get_message(MessageKey::WatchDebounce, lang)).bright_cyan(),
        debounce_ms.to_string().bright_white()
    );

    println!("{}", "═".repeat(60).bright_black());
}

/// Build the full command string with arguments (kept for backward compatibility)
#[allow(dead_code)]
fn build_full_command(command: &str, args: &[String]) -> String {
    if args.is_empty() {
        command.to_string()
    } else {
        format!("{} {}", command, args.join(" "))
    }
}

/// Get the base path for watching
fn get_base_path(paths: &[PathBuf]) -> Result<PathBuf> {
    if paths.is_empty() {
        env::current_dir().context("Failed to get current directory")
    } else {
        Ok(paths[0].clone())
    }
}

/// Build watch configuration from CLI arguments
fn build_watch_config(
    paths: Vec<PathBuf>,
    patterns: Vec<String>,
    exclude: Vec<String>,
    debounce_ms: u64,
    ignore_gitignore: bool,
    no_recursive: bool,
) -> Result<WatchConfig> {
    let mut config = WatchConfig::new();

    // Set paths (use current directory if none specified)
    if paths.is_empty() {
        config.paths = vec![env::current_dir().context("Failed to get current directory")?];
    } else {
        config.paths = paths;
    }

    // Set patterns (use default if none specified)
    if patterns.is_empty() {
        config.patterns = vec![WatchPattern {
            pattern: "**/*".to_string(),
            case_insensitive: false,
        }];
    } else {
        config.patterns = patterns
            .into_iter()
            .map(|p| WatchPattern {
                pattern: p,
                case_insensitive: false,
            })
            .collect();
    }

    // Set exclude patterns
    config.exclude = exclude;

    // Add common exclusions if not ignoring gitignore
    if !ignore_gitignore {
        let common_excludes = vec![
            "**/node_modules/**",
            "**/target/**",
            "**/.git/**",
            "**/dist/**",
            "**/build/**",
            "**/__pycache__/**",
            "**/.cache/**",
        ];
        for pattern in common_excludes {
            if !config.exclude.contains(&pattern.to_string()) {
                config.exclude.push(pattern.to_string());
            }
        }
    }

    // Set debounce
    config.debounce_ms = debounce_ms;

    // Set gitignore flag
    config.ignore_gitignore = ignore_gitignore;

    // Set recursive flag
    config.recursive = !no_recursive;

    Ok(config)
}

/// Set up Ctrl+C handler for graceful shutdown
async fn setup_signal_handler() -> Result<tokio::sync::mpsc::Receiver<()>> {
    use tokio::signal;
    use tokio::sync::mpsc;

    let (tx, rx) = mpsc::channel(1);

    tokio::spawn(async move {
        #[cfg(unix)]
        {
            use signal::unix::{signal, SignalKind};
            let mut sigint = signal(SignalKind::interrupt()).expect("Failed to setup SIGINT");
            let mut sigterm = signal(SignalKind::terminate()).expect("Failed to setup SIGTERM");

            tokio::select! {
                _ = sigint.recv() => {
                    warn!("Received SIGINT (Ctrl+C), stopping watch mode...");
                }
                _ = sigterm.recv() => {
                    warn!("Received SIGTERM, stopping watch mode...");
                }
            }
        }

        #[cfg(windows)]
        {
            signal::ctrl_c().await.expect("Failed to listen for Ctrl+C");
            warn!("Received Ctrl+C, stopping watch mode...");
        }

        let _ = tx.send(()).await;
    });

    Ok(rx)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_full_command_no_args() {
        let command = "cargo build";
        let args = vec![];
        let result = build_full_command(command, &args);
        assert_eq!(result, "cargo build");
    }

    #[test]
    fn test_build_full_command_with_args() {
        let command = "cargo";
        let args = vec!["test".to_string(), "--all".to_string()];
        let result = build_full_command(command, &args);
        assert_eq!(result, "cargo test --all");
    }

    #[test]
    fn test_build_watch_config_defaults() {
        let config =
            build_watch_config(vec![PathBuf::from(".")], vec![], vec![], 500, false, false);
        assert!(config.is_ok());
        let config = config.unwrap();
        assert_eq!(config.debounce_ms, 500);
        assert!(config.recursive);
        assert!(!config.ignore_gitignore);
        assert!(!config.exclude.is_empty()); // Should have common excludes
    }

    #[test]
    fn test_build_watch_config_with_patterns() {
        let patterns = vec!["**/*.rs".to_string(), "**/*.toml".to_string()];
        let config = build_watch_config(
            vec![PathBuf::from(".")],
            patterns.clone(),
            vec![],
            500,
            false,
            false,
        );
        assert!(config.is_ok());
        let config = config.unwrap();
        assert_eq!(config.patterns.len(), 2);
        assert_eq!(config.patterns[0].pattern, "**/*.rs");
        assert_eq!(config.patterns[1].pattern, "**/*.toml");
    }

    #[test]
    fn test_build_watch_config_with_exclude() {
        let exclude = vec!["**/test/**".to_string()];
        let config = build_watch_config(
            vec![PathBuf::from(".")],
            vec![],
            exclude.clone(),
            500,
            false,
            false,
        );
        assert!(config.is_ok());
        let config = config.unwrap();
        assert!(config.exclude.contains(&"**/test/**".to_string()));
        // Should also have common excludes
        assert!(config.exclude.contains(&"**/node_modules/**".to_string()));
    }

    #[test]
    fn test_build_watch_config_ignore_gitignore() {
        let config = build_watch_config(vec![PathBuf::from(".")], vec![], vec![], 500, true, false);
        assert!(config.is_ok());
        let config = config.unwrap();
        assert!(config.ignore_gitignore);
        // Should NOT have common excludes when ignoring gitignore
        assert_eq!(config.exclude.len(), 0);
    }

    #[test]
    fn test_build_watch_config_no_recursive() {
        let config = build_watch_config(vec![PathBuf::from(".")], vec![], vec![], 500, false, true);
        assert!(config.is_ok());
        let config = config.unwrap();
        assert!(!config.recursive);
    }

    #[test]
    fn test_build_watch_config_custom_debounce() {
        let config =
            build_watch_config(vec![PathBuf::from(".")], vec![], vec![], 1000, false, false);
        assert!(config.is_ok());
        let config = config.unwrap();
        assert_eq!(config.debounce_ms, 1000);
    }
}
