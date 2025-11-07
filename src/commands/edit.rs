//! Edit command - Edit existing command interactively

use crate::config::loader::ConfigLoader;
use crate::config::schema::{Command, CommandSpec, CommandsConfig};
use crate::i18n::{get_message, MessageKey};
use anyhow::{Context, Result};
use colored::*;
use dialoguer::{theme::ColorfulTheme, Confirm, Input, Select};
use std::path::PathBuf;
use tracing::info;

/// Edit an existing command interactively
pub async fn handle_edit(command_id: Option<String>, config_path: Option<PathBuf>) -> Result<()> {
    let config_loader = if let Some(ref path) = config_path {
        ConfigLoader::with_path(path)
    } else {
        ConfigLoader::new()
    };
    let config = config_loader.load().await?;
    let lang = config.config.language;

    // Get command ID (from argument or interactive selection)
    let id = if let Some(id) = command_id {
        id
    } else {
        select_command_interactive(&config, lang)?
    };

    // Check if command exists
    let command = config.commands.get(&id).ok_or_else(|| {
        anyhow::anyhow!("{}", get_message(MessageKey::ErrorCommandNotFound, lang))
    })?;

    println!(
        "{}",
        get_message(MessageKey::LabelCurrentSettings, lang)
            .cyan()
            .bold()
    );
    println!(
        "  {} {}",
        format!("{}:", get_message(MessageKey::LabelId, lang))
            .white()
            .bold(),
        id
    );
    println!(
        "  {} {}",
        format!("{}:", get_message(MessageKey::LabelDescription, lang))
            .white()
            .bold(),
        command.description
    );
    println!(
        "  {} {}",
        format!("{}:", get_message(MessageKey::LabelCommand, lang))
            .white()
            .bold(),
        format_command_spec(&command.cmd)
    );
    println!(
        "  {} {:?}",
        format!("{}:", get_message(MessageKey::LabelTags, lang))
            .white()
            .bold(),
        command.tags
    );
    println!(
        "  {} {}",
        format!("{}:", get_message(MessageKey::EditParallelExecution, lang))
            .white()
            .bold(),
        command.parallel
    );
    println!(
        "  {} {}",
        format!("{}:", get_message(MessageKey::EditConfirmBeforeExecution, lang))
            .white()
            .bold(),
        command.confirm
    );
    println!();

    // Interactive editing
    let new_description = prompt_with_default(
        get_message(MessageKey::PromptDescription, lang),
        &command.description,
    )?;
    let new_command_str = prompt_with_default(
        get_message(MessageKey::PromptCommand, lang),
        &format_command_spec(&command.cmd),
    )?;
    let new_tags = prompt_with_default(
        get_message(MessageKey::PromptTags, lang),
        &command.tags.join(","),
    )?;
    let new_parallel = prompt_bool(
        get_message(MessageKey::EditParallelExecution, lang),
        command.parallel,
    )?;
    let new_confirm = prompt_bool(
        get_message(MessageKey::EditConfirmBeforeExecution, lang),
        command.confirm,
    )?;

    // Create updated command
    let updated_command = Command {
        description: new_description,
        cmd: CommandSpec::Single(new_command_str),
        tags: new_tags
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect(),
        parallel: new_parallel,
        confirm: new_confirm,
        ..command.clone()
    };

    // Save to configuration file
    save_edited_command(&id, updated_command, config_path).await?;

    println!(
        "{} {} '{}'",
        "✓".green().bold(),
        get_message(MessageKey::CommandUpdated, lang),
        id.bright_white()
    );

    Ok(())
}

/// Select a command interactively from the list
fn select_command_interactive(
    config: &CommandsConfig,
    lang: crate::config::schema::Language,
) -> Result<String> {
    if config.commands.is_empty() {
        anyhow::bail!("{}", get_message(MessageKey::NoCommandsFound, lang));
    }

    let mut commands: Vec<_> = config.commands.iter().collect();
    commands.sort_by_key(|(name, _)| *name);

    let items: Vec<String> = commands
        .iter()
        .map(|(name, cmd)| format!("{} - {}", name.green().bold(), cmd.description))
        .collect();

    let theme = ColorfulTheme::default();
    let selection = Select::with_theme(&theme)
        .with_prompt(get_message(MessageKey::PromptSelectCommand, lang))
        .items(&items)
        .default(0)
        .interact()?;

    Ok(commands[selection].0.clone())
}

/// Prompt for input with a default value
fn prompt_with_default(prompt: &str, default: &str) -> Result<String> {
    let theme = ColorfulTheme::default();
    Input::with_theme(&theme)
        .with_prompt(prompt)
        .default(default.to_string())
        .interact_text()
        .map_err(Into::into)
}

/// Prompt for boolean value
fn prompt_bool(prompt: &str, default: bool) -> Result<bool> {
    let theme = ColorfulTheme::default();
    Confirm::with_theme(&theme)
        .with_prompt(prompt)
        .default(default)
        .interact()
        .map_err(Into::into)
}

/// Format CommandSpec for display
fn format_command_spec(spec: &CommandSpec) -> String {
    match spec {
        CommandSpec::Single(cmd) => cmd.clone(),
        CommandSpec::Multiple(cmds) => cmds.join(" && "),
        CommandSpec::Platform(_) => "[Platform-specific]".to_string(),
    }
}

/// Save edited command to configuration file
async fn save_edited_command(
    id: &str,
    command: Command,
    config_path: Option<PathBuf>,
) -> Result<()> {
    let config_loader = if let Some(ref path) = config_path {
        ConfigLoader::with_path(path)
    } else {
        ConfigLoader::new()
    };
    let mut config = config_loader.load().await?;

    // Update the command
    config.commands.insert(id.to_string(), command);

    // Find and write to config file
    let config_file_path = if let Some(path) = config_path {
        path
    } else {
        find_config_file().await.context(get_message(
            MessageKey::ErrorConfigNotFound,
            crate::config::schema::Language::default(),
        ))?
    };
    let toml_content =
        toml::to_string_pretty(&config).context("Failed to serialize configuration")?;

    tokio::fs::write(&config_file_path, toml_content)
        .await
        .context("Failed to write configuration file")?;

    info!("Updated command '{}' in {}", id, config_file_path.display());

    Ok(())
}

/// Find the configuration file
async fn find_config_file() -> Result<PathBuf> {
    const CONFIG_FILENAMES: &[&str] = &["commands.toml", ".cmdrun.toml", "cmdrun.toml"];

    let current_dir = std::env::current_dir().context("Failed to get current directory")?;

    let mut current = current_dir;
    loop {
        for filename in CONFIG_FILENAMES {
            let path = current.join(filename);
            if path.exists() && path.is_file() {
                return Ok(path);
            }
        }

        if !current.pop() {
            break;
        }
    }

    // Check home directory
    if let Some(home_dir) = dirs::home_dir() {
        let cmdrun_dir = home_dir.join(".cmdrun");
        for filename in CONFIG_FILENAMES {
            let path = cmdrun_dir.join(filename);
            if path.exists() && path.is_file() {
                return Ok(path);
            }
        }
    }

    anyhow::bail!("Configuration file not found")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_command_spec() {
        let single = CommandSpec::Single("echo hello".to_string());
        assert_eq!(format_command_spec(&single), "echo hello");

        let multiple =
            CommandSpec::Multiple(vec!["echo hello".to_string(), "echo world".to_string()]);
        assert_eq!(format_command_spec(&multiple), "echo hello && echo world");
    }
}
