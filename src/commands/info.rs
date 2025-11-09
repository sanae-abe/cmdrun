//! Info command - Show detailed command information

use crate::config::loader::ConfigLoader;
use crate::config::schema::{CommandSpec, CommandsConfig};
use crate::i18n::{get_message, MessageKey};
use anyhow::Result;
use colored::*;
use std::io::{self, Write};
use std::path::PathBuf;

/// Show detailed information about a command
pub async fn handle_info(command_id: Option<String>, config_path: Option<PathBuf>) -> Result<()> {
    let config_loader = if let Some(path) = config_path {
        ConfigLoader::with_path(path)?
    } else {
        ConfigLoader::new()
    };
    let loaded = config_loader.load_with_paths().await?;
    let config = &loaded.config;
    let lang = config.config.language;

    // Load history for statistics
    let history_storage = crate::history::HistoryStorage::new().ok();

    // Get command ID (from argument or interactive selection)
    let id = if let Some(id) = command_id {
        id
    } else {
        select_command_interactive(config, lang)?
    };

    // Find command
    let command = config.commands.get(&id).ok_or_else(|| {
        anyhow::anyhow!("{}", get_message(MessageKey::ErrorCommandNotFound, lang))
    })?;

    // Display detailed information
    println!(
        "{} {}",
        get_message(MessageKey::LabelCommandDetails, lang)
            .cyan()
            .bold(),
        id.white().bold()
    );
    println!("{}", "━".repeat(50).cyan());
    println!();

    // Basic information
    println!(
        "{} {}",
        format!("{}:", get_message(MessageKey::LabelDescription, lang))
            .white()
            .bold(),
        command.description
    );
    println!();

    // Command specification
    println!(
        "{}",
        format!("{}:", get_message(MessageKey::LabelCommand, lang))
            .white()
            .bold()
    );
    match &command.cmd {
        CommandSpec::Single(cmd) => {
            println!("  {}", cmd.bright_white());
        }
        CommandSpec::Multiple(cmds) => {
            for (idx, cmd) in cmds.iter().enumerate() {
                println!("  {}. {}", idx + 1, cmd.bright_white());
            }
        }
        CommandSpec::Platform(platform_cmds) => {
            if let Some(unix) = &platform_cmds.unix {
                println!("  {} {}", "Unix:".dimmed(), unix.bright_white());
            }
            if let Some(linux) = &platform_cmds.linux {
                println!("  {} {}", "Linux:".dimmed(), linux.bright_white());
            }
            if let Some(macos) = &platform_cmds.macos {
                println!("  {} {}", "macOS:".dimmed(), macos.bright_white());
            }
            if let Some(windows) = &platform_cmds.windows {
                println!("  {} {}", "Windows:".dimmed(), windows.bright_white());
            }
        }
    }
    println!();

    // Dependencies
    if !command.deps.is_empty() {
        println!(
            "{}",
            format!("{}:", get_message(MessageKey::LabelDependencies, lang))
                .white()
                .bold()
        );
        for dep in &command.deps {
            println!("  {} {}", "→".blue(), dep.bright_white());
        }
        println!();
    }

    // Tags
    if !command.tags.is_empty() {
        println!(
            "{} {}",
            format!("{}:", get_message(MessageKey::LabelTags, lang))
                .white()
                .bold(),
            command.tags.join(", ")
        );
        println!();
    }

    // Working directory
    if let Some(working_dir) = &command.working_dir {
        println!(
            "{} {}",
            format!("{}:", get_message(MessageKey::LabelWorkingDirectory, lang))
                .white()
                .bold(),
            working_dir.display()
        );
        println!();
    }

    // Environment variables
    if !command.env.is_empty() {
        println!(
            "{}",
            format!(
                "{}:",
                get_message(MessageKey::LabelEnvironmentVariables, lang)
            )
            .white()
            .bold()
        );
        for (key, value) in &command.env {
            println!("  {} = {}", key.yellow(), value.bright_white());
        }
        println!();
    }

    // Execution settings
    println!(
        "{}",
        format!("{}:", get_message(MessageKey::LabelExecutionSettings, lang))
            .white()
            .bold()
    );
    println!(
        "  {} {}",
        format!("{}:", get_message(MessageKey::LabelParallel, lang)).dimmed(),
        format_bool(command.parallel, lang)
    );
    println!(
        "  {} {}",
        format!("{}:", get_message(MessageKey::LabelConfirm, lang)).dimmed(),
        format_bool(command.confirm, lang)
    );
    if let Some(timeout) = command.timeout {
        println!(
            "  {} {}s",
            format!("{}:", get_message(MessageKey::LabelTimeout, lang)).dimmed(),
            timeout
        );
    }
    println!();

    // Platform support
    if !command.platform.is_empty() {
        println!(
            "{} {}",
            format!("{}:", get_message(MessageKey::LabelPlatforms, lang))
                .white()
                .bold(),
            command
                .platform
                .iter()
                .map(|p| format!("{:?}", p))
                .collect::<Vec<_>>()
                .join(", ")
        );
        println!();
    }

    // Configuration paths
    println!(
        "{}",
        format!("{}:", get_message(MessageKey::InfoConfigurationPaths, lang))
            .white()
            .bold()
    );
    if let Some(global_path) = &loaded.global_path {
        println!(
            "  {} {}",
            format!("{}:", get_message(MessageKey::InfoGlobalConfigPath, lang)).dimmed(),
            global_path.display().to_string().bright_white()
        );
    }
    if let Some(local_path) = &loaded.local_path {
        println!(
            "  {} {}",
            format!("{}:", get_message(MessageKey::InfoLocalConfigPath, lang)).dimmed(),
            local_path.display().to_string().bright_white()
        );
    }

    // Actual working directory
    let actual_working_dir = if let Some(cmd_working_dir) = &command.working_dir {
        cmd_working_dir.clone()
    } else {
        config.config.working_dir.clone()
    };

    // Resolve to absolute path if needed
    let absolute_working_dir = if actual_working_dir.is_absolute() {
        actual_working_dir
    } else {
        std::env::current_dir()?.join(&actual_working_dir)
    };

    println!(
        "  {} {}",
        format!(
            "{}:",
            get_message(MessageKey::InfoActualWorkingDirectory, lang)
        )
        .dimmed(),
        absolute_working_dir.display().to_string().bright_white()
    );
    println!();

    // Execution statistics from history
    if let Some(storage) = &history_storage {
        if let Ok(stats) = get_command_statistics(storage, &id).await {
            println!(
                "{}",
                format!("{}:", get_message(MessageKey::InfoExecutionStatistics, lang))
                    .white()
                    .bold()
            );
            println!(
                "  {} {}",
                format!("{}:", get_message(MessageKey::InfoTotalExecutions, lang)).dimmed(),
                stats.total_count.to_string().bright_white()
            );
            println!(
                "  {} {}",
                format!("{}:", get_message(MessageKey::InfoSuccessfulRuns, lang)).dimmed(),
                format!("{} ({}%)", stats.success_count, stats.success_rate).green()
            );
            println!(
                "  {} {}",
                format!("{}:", get_message(MessageKey::InfoFailedRuns, lang)).dimmed(),
                format!("{} ({}%)", stats.failed_count, stats.failure_rate).red()
            );
            if let Some(last_run) = &stats.last_run_time {
                println!(
                    "  {} {}",
                    format!("{}:", get_message(MessageKey::InfoLastRun, lang)).dimmed(),
                    last_run.bright_white()
                );
            }
            if let Some(avg_duration) = stats.avg_duration {
                println!(
                    "  {} {:.2}s",
                    format!("{}:", get_message(MessageKey::InfoAverageDuration, lang)).dimmed(),
                    avg_duration
                );
            }
            println!();
        }
    }

    Ok(())
}

/// Statistics for a command's execution history
struct CommandStatistics {
    total_count: i64,
    success_count: i64,
    failed_count: i64,
    success_rate: i64,
    failure_rate: i64,
    last_run_time: Option<String>,
    avg_duration: Option<f64>,
}

/// Get execution statistics for a command from history
async fn get_command_statistics(
    storage: &crate::history::HistoryStorage,
    command_name: &str,
) -> Result<CommandStatistics> {
    let entries = storage.search(command_name, None)?;

    if entries.is_empty() {
        return Ok(CommandStatistics {
            total_count: 0,
            success_count: 0,
            failed_count: 0,
            success_rate: 0,
            failure_rate: 0,
            last_run_time: None,
            avg_duration: None,
        });
    }

    let total_count = entries.len() as i64;
    let success_count = entries.iter().filter(|e| e.success).count() as i64;
    let failed_count = total_count - success_count;
    let success_rate = if total_count > 0 {
        (success_count * 100) / total_count
    } else {
        0
    };
    let failure_rate = 100 - success_rate;

    // Convert Unix timestamp to human-readable format
    let last_run_time = entries.first().map(|e| {
        let datetime = chrono::DateTime::from_timestamp(e.start_time / 1000, 0)
            .unwrap_or_else(|| chrono::Utc::now());
        datetime.format("%Y-%m-%d %H:%M:%S").to_string()
    });

    let avg_duration = if !entries.is_empty() {
        let total_duration: i64 = entries
            .iter()
            .filter_map(|e| e.duration_ms)
            .sum();
        let count = entries.iter().filter(|e| e.duration_ms.is_some()).count();
        if count > 0 {
            Some(total_duration as f64 / count as f64 / 1000.0) // Convert ms to seconds
        } else {
            None
        }
    } else {
        None
    };

    Ok(CommandStatistics {
        total_count,
        success_count,
        failed_count,
        success_rate,
        failure_rate,
        last_run_time,
        avg_duration,
    })
}

/// Select a command interactively from the list
fn select_command_interactive(
    config: &CommandsConfig,
    lang: crate::config::schema::Language,
) -> Result<String> {
    if config.commands.is_empty() {
        anyhow::bail!(
            "{}",
            get_message(MessageKey::ErrorNoCommandsAvailable, lang)
        );
    }

    println!(
        "{}",
        get_message(MessageKey::InfoSelectCommandToView, lang)
            .cyan()
            .bold()
    );
    println!();

    let mut commands: Vec<_> = config.commands.iter().collect();
    commands.sort_by_key(|(name, _)| *name);

    for (idx, (name, cmd)) in commands.iter().enumerate() {
        println!(
            "  {}. {} - {}",
            (idx + 1).to_string().yellow(),
            name.green().bold(),
            cmd.description
        );
    }

    println!();
    print!(
        "{} ",
        format!("{}:", get_message(MessageKey::PromptEnterNumber, lang)).bright_white()
    );
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let selection: usize = input
        .trim()
        .parse()
        .map_err(|_| anyhow::anyhow!("{}", get_message(MessageKey::ErrorInvalidSelection, lang)))?;

    if selection < 1 || selection > commands.len() {
        anyhow::bail!(
            "{}",
            get_message(MessageKey::ErrorSelectionOutOfRange, lang)
        );
    }

    Ok(commands[selection - 1].0.clone())
}

/// Format boolean value with colored output
fn format_bool(value: bool, lang: crate::config::schema::Language) -> ColoredString {
    if value {
        get_message(MessageKey::LabelYes, lang).green()
    } else {
        get_message(MessageKey::LabelNo, lang).red()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_bool() {
        use crate::config::schema::Language;

        let yes = format_bool(true, Language::English);
        assert!(yes.to_string().contains("yes"));

        let no = format_bool(false, Language::English);
        assert!(no.to_string().contains("no"));

        // Test Japanese
        let yes_ja = format_bool(true, Language::Japanese);
        assert!(yes_ja.to_string().contains("はい"));

        let no_ja = format_bool(false, Language::Japanese);
        assert!(no_ja.to_string().contains("いいえ"));
    }
}
