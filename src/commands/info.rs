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

    Ok(())
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
