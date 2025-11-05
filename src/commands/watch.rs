//! Watch command implementation

use anyhow::{Context, Result};
use colored::Colorize;
use std::env;
use std::path::PathBuf;
use tracing::{info, warn};

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
    // Display watch configuration
    display_watch_info(&command, &args, &paths, &patterns, &exclude, debounce_ms);

    // Build the full command with arguments
    let full_command = build_full_command(&command, &args);

    // Get base path (current directory if no paths specified)
    let base_path = get_base_path(&paths)?;

    // Build watch configuration
    let config = build_watch_config(
        paths,
        patterns,
        exclude,
        debounce_ms,
        ignore_gitignore,
        no_recursive,
    )?;

    // Create and run the watch runner
    let mut runner = WatchRunner::new(config, full_command, &base_path)
        .context("Failed to create watch runner")?;

    // Set up Ctrl+C handler
    let running = setup_ctrl_c_handler()?;

    // Run the watcher
    info!(
        "Watch mode started. Press {} to stop.",
        "Ctrl+C".bright_cyan()
    );
    println!();

    let result = runner.run().await;

    // Check if we were interrupted
    if !running.load(std::sync::atomic::Ordering::SeqCst) {
        info!("Watch mode stopped by user");
        return Ok(());
    }

    result
}

/// Display watch configuration information
fn display_watch_info(
    command: &str,
    args: &[String],
    paths: &[PathBuf],
    patterns: &[String],
    exclude: &[String],
    debounce_ms: u64,
) {
    println!("{}", "Watch Configuration".bright_green().bold());
    println!("{}", "═".repeat(60).bright_black());

    // Command
    print!("  {} ", "Command:".bright_cyan());
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
    print!("  {} ", "Watching:".bright_cyan());
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
        print!("  {} ", "Patterns:".bright_cyan());
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
        print!("  {} ", "Exclude:".bright_cyan());
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
        "Debounce:".bright_cyan(),
        debounce_ms.to_string().bright_white()
    );

    println!("{}", "═".repeat(60).bright_black());
}

/// Build the full command string with arguments
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
fn setup_ctrl_c_handler() -> Result<std::sync::Arc<std::sync::atomic::AtomicBool>> {
    use std::sync::atomic::AtomicBool;
    use std::sync::Arc;

    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        warn!("Received Ctrl+C, stopping watch mode...");
        r.store(false, std::sync::atomic::Ordering::SeqCst);
    })
    .context("Failed to set Ctrl+C handler")?;

    Ok(running)
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
