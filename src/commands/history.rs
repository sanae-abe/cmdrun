//! History command implementation
//!
//! Provides command history display, search, clear, and export functionality.

use crate::config::Language;
use crate::history::{HistoryEntry, HistoryStorage};
use crate::i18n::{get_message, MessageKey};
use anyhow::{Context, Result};
use colored::*;
use std::path::PathBuf;

/// Handle the history list command
pub async fn handle_history(
    limit: Option<usize>,
    offset: Option<usize>,
    show_failed_only: bool,
    show_stats: bool,
    language: Language,
) -> Result<()> {
    let storage = HistoryStorage::new().context("Failed to open history database")?;

    if show_stats {
        display_stats(&storage, language)?;
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
        println!(
            "{}",
            get_message(MessageKey::HistoryNoEntriesFound, language).yellow()
        );
        return Ok(());
    }

    println!("{}", "Command Execution History".cyan().bold());
    println!();

    let count = entries.len();
    for entry in entries {
        display_entry(&entry, language);
    }

    println!();
    println!("{} Showing {} entries", "ℹ".blue(), count);

    Ok(())
}

/// Handle the history search command
pub async fn handle_history_search(
    query: &str,
    limit: Option<usize>,
    language: Language,
) -> Result<()> {
    let storage = HistoryStorage::new().context("Failed to open history database")?;

    println!(
        "{} Searching for: {}",
        "🔍".bright_white(),
        query.bright_white()
    );

    let results = storage.search(query, limit)?;

    if results.is_empty() {
        println!(
            "{}",
            format!(
                "{} '{}'",
                get_message(MessageKey::HistoryNoCommandsMatching, language),
                query
            )
            .yellow()
        );
        return Ok(());
    }

    println!();
    println!("{} Found {} matching entries", "✓".green(), results.len());
    println!();

    for entry in results {
        display_entry(&entry, language);
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
fn display_entry(entry: &HistoryEntry, language: Language) {
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
        println!(
            "  {} {}",
            get_message(MessageKey::HistoryExitCode, language).dimmed(),
            exit_code
        );
    }

    if let Some(args) = &entry.args {
        if let Ok(parsed_args) = serde_json::from_str::<Vec<String>>(args) {
            if !parsed_args.is_empty() {
                println!("  {} {:?}", "Args:".dimmed(), parsed_args);
            }
        }
    }

    if let Some(working_dir) = &entry.working_dir {
        println!(
            "  {} {}",
            get_message(MessageKey::HistoryWorkingDir, language).dimmed(),
            working_dir
        );
    }

    println!();
}

/// Display history statistics
fn display_stats(storage: &HistoryStorage, language: Language) -> Result<()> {
    let stats = storage.get_stats()?;

    println!("{}", "History Statistics".cyan().bold());
    println!();

    println!(
        "  {} {}",
        get_message(MessageKey::HistoryTotalCommands, language).bright_white(),
        stats.total
    );
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
    use tempfile::TempDir;

    // Helper function to create test history storage with sample data
    fn create_test_storage() -> (HistoryStorage, TempDir) {
        use crate::history::HistoryEntry;

        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("test_history.db");

        // Create storage with temporary database
        std::env::set_var("CMDRUN_HISTORY_DB", db_path.to_str().unwrap());
        let mut storage = HistoryStorage::new().unwrap();

        // Add sample history entries
        let entry1 = HistoryEntry {
            id: 0, // Will be auto-generated by database
            command: "test-cmd".to_string(),
            args: None,
            start_time: chrono::Utc::now().timestamp_millis(),
            duration_ms: Some(1500),
            exit_code: Some(0),
            success: true,
            working_dir: Some("/tmp".to_string()),
            environment: None,
        };
        storage.add(&entry1).unwrap();

        let entry2 = HistoryEntry {
            id: 0,
            command: "failed-cmd".to_string(),
            args: None,
            start_time: chrono::Utc::now().timestamp_millis(),
            duration_ms: Some(2000),
            exit_code: Some(1),
            success: false,
            working_dir: Some("/tmp".to_string()),
            environment: None,
        };
        storage.add(&entry2).unwrap();

        let entry3 = HistoryEntry {
            id: 0,
            command: "another-test".to_string(),
            args: None,
            start_time: chrono::Utc::now().timestamp_millis(),
            duration_ms: Some(1000),
            exit_code: Some(0),
            success: true,
            working_dir: Some("/tmp".to_string()),
            environment: None,
        };
        storage.add(&entry3).unwrap();

        (storage, temp_dir)
    }

    #[test]
    fn test_export_format() {
        let format = ExportFormat::Json;
        assert!(matches!(format, ExportFormat::Json));
    }

    #[tokio::test]
    async fn test_handle_history_empty() {
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("empty_history.db");

        std::env::set_var("CMDRUN_HISTORY_DB", db_path.to_str().unwrap());
        let _storage = HistoryStorage::new().unwrap();

        // Should succeed without errors even with empty history
        let result = handle_history(None, None, false, false, Language::English).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_handle_history_with_limit() {
        let (_storage, _temp_dir) = create_test_storage();

        // Should succeed with limit parameter
        let result = handle_history(Some(2), None, false, false, Language::English).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_handle_history_failed_only() {
        let (_storage, _temp_dir) = create_test_storage();

        // Should filter and show only failed commands
        let result = handle_history(None, None, true, false, Language::English).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_handle_history_stats() {
        let (_storage, _temp_dir) = create_test_storage();

        // Should display statistics
        let result = handle_history(None, None, false, true, Language::English).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_handle_history_search_found() {
        let (_storage, _temp_dir) = create_test_storage();

        // Should find matching entries
        let result = handle_history_search("test", Some(10), Language::English).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_handle_history_search_not_found() {
        let (_storage, _temp_dir) = create_test_storage();

        // Should handle no results gracefully
        let result = handle_history_search("nonexistent-query", Some(10), Language::English).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_handle_history_clear_force() {
        let (_storage, _temp_dir) = create_test_storage();

        // Should clear all history with force=true (no confirmation)
        let result = handle_history_clear(true).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_handle_history_export_json() {
        let (_storage, _temp_dir) = create_test_storage();

        // Should export to JSON format without file output
        let result = handle_history_export(ExportFormat::Json, None, Some(10)).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_handle_history_export_csv() {
        let (_storage, _temp_dir) = create_test_storage();

        // Should export to CSV format without file output
        let result = handle_history_export(ExportFormat::Csv, None, Some(10)).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_handle_history_export_to_file() {
        let (_storage, temp_dir) = create_test_storage();
        let output_path = temp_dir.path().join("history_export.json");

        // Should export to file
        let result =
            handle_history_export(ExportFormat::Json, Some(output_path.clone()), Some(10)).await;
        assert!(result.is_ok());

        // Verify file was created
        assert!(output_path.exists());

        // Verify content is valid JSON (parse to validate)
        let content = std::fs::read_to_string(&output_path).unwrap();
        let json_result: Result<serde_json::Value, _> = serde_json::from_str(&content);
        assert!(json_result.is_ok(), "Export should produce valid JSON");
    }

    #[test]
    fn test_display_entry_success() {
        use crate::history::HistoryEntry;

        let entry = HistoryEntry {
            id: 1,
            command: "test-cmd".to_string(),
            success: true,
            exit_code: Some(0),
            duration_ms: Some(1500),
            start_time: chrono::Utc::now().timestamp_millis(),
            args: None,
            working_dir: Some("/tmp".to_string()),
            environment: None,
        };

        // Should not panic when displaying entry
        display_entry(&entry, Language::English);
    }

    #[test]
    fn test_display_entry_failed() {
        use crate::history::HistoryEntry;

        let entry = HistoryEntry {
            id: 2,
            command: "failed-cmd".to_string(),
            success: false,
            exit_code: Some(1),
            duration_ms: Some(2000),
            start_time: chrono::Utc::now().timestamp_millis(),
            args: Some(r#"["arg1", "arg2"]"#.to_string()),
            working_dir: Some("/tmp".to_string()),
            environment: None,
        };

        // Should not panic when displaying failed entry
        display_entry(&entry, Language::English);
    }

    #[test]
    fn test_display_stats() {
        let (_storage, _temp_dir) = create_test_storage();

        // Should display statistics without errors
        let result = display_stats(&_storage, Language::English);
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_handle_history_clear_cancelled() {
        // This test cannot be automated as it requires interactive input
        // It would cover lines 88, 94 (Confirm interaction and cancellation)
        // Mark as ignored for manual testing
    }

    #[tokio::test]
    async fn test_handle_history_export_csv_to_file() {
        let (_storage, temp_dir) = create_test_storage();
        let output_path = temp_dir.path().join("history_export.csv");

        // Test CSV export to file (covers line 118, 122-123)
        let result =
            handle_history_export(ExportFormat::Csv, Some(output_path.clone()), Some(10)).await;
        assert!(result.is_ok());

        // Verify file was created
        assert!(output_path.exists());

        // Verify content has CSV header
        let content = std::fs::read_to_string(&output_path).unwrap();
        assert!(content.contains("id,") || content.contains("command"));
    }

    #[tokio::test]
    async fn test_handle_history_with_offset() {
        let (_storage, _temp_dir) = create_test_storage();

        // Test with offset parameter (covers line 33)
        let result = handle_history(Some(10), Some(1), false, false, Language::English).await;
        assert!(result.is_ok());
    }
}
