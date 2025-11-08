//! History command implementation
//!
//! Provides command history display, search, clear, and export functionality.

use crate::history::{HistoryEntry, HistoryStorage};
use anyhow::{Context, Result};
use colored::*;
use std::path::PathBuf;

/// Handle the history list command
pub async fn handle_history(
    limit: Option<usize>,
    offset: Option<usize>,
    show_failed_only: bool,
    show_stats: bool,
) -> Result<()> {
    let storage = HistoryStorage::new().context("Failed to open history database")?;

    if show_stats {
        display_stats(&storage)?;
        return Ok(());
    }

    let entries = if show_failed_only {
        // Get all entries and filter failed ones
        storage
            .list(None, None)?
            .into_iter()
            .filter(|e| !e.success)
            .take(limit.unwrap_or(50))
            .collect::<Vec<_>>()
    } else {
        storage.list(limit, offset)?
    };

    if entries.is_empty() {
        println!("{}", "No history entries found".yellow());
        return Ok(());
    }

    println!("{}", "Command Execution History".cyan().bold());
    println!();

    let count = entries.len();
    for entry in entries {
        display_entry(&entry);
    }

    println!();
    println!("{} Showing {} entries", "ℹ".blue(), count);

    Ok(())
}

/// Handle the history search command
pub async fn handle_history_search(query: &str, limit: Option<usize>) -> Result<()> {
    let storage = HistoryStorage::new().context("Failed to open history database")?;

    println!(
        "{} Searching for: {}",
        "🔍".bright_white(),
        query.bright_white()
    );

    let results = storage.search(query, limit)?;

    if results.is_empty() {
        println!("{}", format!("No commands matching '{}'", query).yellow());
        return Ok(());
    }

    println!();
    println!("{} Found {} matching entries", "✓".green(), results.len());
    println!();

    for entry in results {
        display_entry(&entry);
    }

    Ok(())
}

/// Handle the history clear command
pub async fn handle_history_clear(force: bool) -> Result<()> {
    use dialoguer::Confirm;

    if !force {
        let confirm = Confirm::new()
            .with_prompt("Are you sure you want to clear all history?")
            .default(false)
            .interact()?;

        if !confirm {
            println!("{}", "Cancelled".yellow());
            return Ok(());
        }
    }

    let mut storage = HistoryStorage::new().context("Failed to open history database")?;

    let count = storage.clear()?;

    println!("{} Cleared {} history entries", "✓".green().bold(), count);

    Ok(())
}

/// Handle the history export command
pub async fn handle_history_export(
    format: ExportFormat,
    output: Option<PathBuf>,
    limit: Option<usize>,
) -> Result<()> {
    let storage = HistoryStorage::new().context("Failed to open history database")?;

    let data = match format {
        ExportFormat::Json => storage.export_json(limit)?,
        ExportFormat::Csv => storage.export_csv(limit)?,
    };

    if let Some(path) = output {
        std::fs::write(&path, data)
            .with_context(|| format!("Failed to write to {}", path.display()))?;

        println!(
            "{} Exported history to: {}",
            "✓".green().bold(),
            path.display().to_string().bright_white()
        );
    } else {
        print!("{}", data);
    }

    Ok(())
}

/// Handle the retry command (re-execute last failed command)
pub async fn handle_retry(id: Option<i64>) -> Result<()> {
    let storage = HistoryStorage::new().context("Failed to open history database")?;

    let entry = if let Some(id) = id {
        storage
            .get_by_id(id)?
            .ok_or_else(|| anyhow::anyhow!("No history entry found with ID {}", id))?
    } else {
        storage
            .get_last_failed()?
            .ok_or_else(|| anyhow::anyhow!("No failed commands in history"))?
    };

    println!(
        "{} Retrying command: {}",
        "🔄".bright_white(),
        entry.command.bright_white()
    );
    println!("  {} {}", "ID:".dimmed(), entry.id);
    println!(
        "  {} {}",
        "Original run:".dimmed(),
        entry.start_time_as_datetime()
    );
    println!();

    // Import the necessary modules for re-execution
    use crate::command::executor::{CommandExecutor, ExecutionContext};
    use crate::config::loader::ConfigLoader;
    use crate::platform::shell::detect_shell;

    // Load configuration
    let config_loader = ConfigLoader::new();
    let config = config_loader.load().await?;

    // Find command
    let command = config.commands.get(&entry.command).ok_or_else(|| {
        anyhow::anyhow!(
            "Command not found in current configuration: {}",
            entry.command
        )
    })?;

    // Parse arguments from history
    let args: Vec<String> = if let Some(args_json) = &entry.args {
        serde_json::from_str(args_json).unwrap_or_default()
    } else {
        Vec::new()
    };

    // Create execution context
    let mut env = config.config.env.clone();
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
        language: config.config.language,
    };

    let executor = CommandExecutor::new(ctx);
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

/// Export format enum
#[derive(Debug, Clone, Copy)]
pub enum ExportFormat {
    Json,
    Csv,
}

/// Display a single history entry
fn display_entry(entry: &HistoryEntry) {
    let status_icon = if entry.success {
        "✓".green()
    } else {
        "✗".red()
    };

    let status_text = if entry.success {
        "success".green()
    } else {
        "failed".red()
    };

    println!(
        "{} {} {} {}",
        status_icon,
        format!("#{}", entry.id).dimmed(),
        entry.command.bright_white().bold(),
        status_text
    );

    println!("  {} {}", "Time:".dimmed(), entry.start_time_as_datetime());
    println!("  {} {}", "Duration:".dimmed(), entry.duration_string());

    if let Some(exit_code) = entry.exit_code {
        println!("  {} {}", "Exit code:".dimmed(), exit_code);
    }

    if let Some(args) = &entry.args {
        if let Ok(parsed_args) = serde_json::from_str::<Vec<String>>(args) {
            if !parsed_args.is_empty() {
                println!("  {} {:?}", "Args:".dimmed(), parsed_args);
            }
        }
    }

    if let Some(working_dir) = &entry.working_dir {
        println!("  {} {}", "Working dir:".dimmed(), working_dir);
    }

    println!();
}

/// Display history statistics
fn display_stats(storage: &HistoryStorage) -> Result<()> {
    let stats = storage.get_stats()?;

    println!("{}", "History Statistics".cyan().bold());
    println!();

    println!("  {} {}", "Total commands:".bright_white(), stats.total);
    println!("  {} {}", "Successful:".green(), stats.successful);
    println!("  {} {}", "Failed:".red(), stats.failed);
    println!(
        "  {} {:.1}%",
        "Success rate:".bright_white(),
        stats.success_rate()
    );
    println!(
        "  {} {}",
        "Avg duration:".bright_white(),
        stats.avg_duration_string()
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_export_format() {
        let format = ExportFormat::Json;
        assert!(matches!(format, ExportFormat::Json));
    }
}
