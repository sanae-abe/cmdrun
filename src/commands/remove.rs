//! Command removal functionality

use crate::config::loader::ConfigLoader;
use crate::i18n::{get_message, MessageKey};
use anyhow::{Context, Result};
use colored::*;
use std::io::{self, Write};
use std::path::PathBuf;
use tokio::fs;

/// Handle the `remove` subcommand
pub async fn handle_remove(id: String, force: bool, config_path: Option<PathBuf>) -> Result<()> {
    // Load configuration
    let loader = if let Some(path) = &config_path {
        ConfigLoader::with_path(path)
    } else {
        ConfigLoader::new()
    };

    let config = loader.load().await?;
    let lang = config.config.language;

    // Check if command exists
    if !config.commands.contains_key(&id) {
        anyhow::bail!("{}", get_message(MessageKey::ErrorCommandNotFound, lang));
    }

    // Get command details for display
    let command = config.commands.get(&id).unwrap();

    // Display command information
    println!("{}", "Removal target:".cyan().bold());
    println!(
        "  {} {}",
        format!("{}:", get_message(MessageKey::LabelId, lang)).dimmed(),
        id.green().bold()
    );
    println!(
        "  {} {}",
        format!("{}:", get_message(MessageKey::LabelDescription, lang)).dimmed(),
        command.description
    );

    // Show command content
    match &command.cmd {
        crate::config::schema::CommandSpec::Single(cmd) => {
            println!(
                "  {} {}",
                format!("{}:", get_message(MessageKey::LabelCommand, lang)).dimmed(),
                cmd
            );
        }
        crate::config::schema::CommandSpec::Multiple(cmds) => {
            println!(
                "  {} ({} commands)",
                format!("{}:", get_message(MessageKey::LabelCommand, lang)).dimmed(),
                cmds.len()
            );
            for (i, cmd) in cmds.iter().enumerate() {
                println!("    {}. {}", i + 1, cmd);
            }
        }
        crate::config::schema::CommandSpec::Platform(_) => {
            println!("  {} Platform-specific", "Type:".dimmed());
        }
    }

    // Show dependencies if any
    if !command.deps.is_empty() {
        println!("  {} {:?}", "Dependencies:".dimmed(), command.deps);
    }

    // Show tags if any
    if !command.tags.is_empty() {
        println!(
            "  {} {:?}",
            format!("{}:", get_message(MessageKey::LabelTags, lang)).dimmed(),
            command.tags
        );
    }

    println!();

    // Confirmation prompt (unless --force)
    if !force {
        print!(
            "{}",
            format!("{} (y/N): ", get_message(MessageKey::PromptConfirm, lang)).yellow()
        );
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim().to_lowercase();

        if input != "y" && input != "yes" {
            println!("{}", get_message(MessageKey::Cancelled, lang).green());
            return Ok(());
        }
    }

    // Determine config file path
    let config_file_path = if let Some(path) = config_path {
        path
    } else {
        find_config_file().await?
    };

    // Create backup
    let backup_path = create_backup(&config_file_path).await?;
    println!(
        "{} Backup created: {}",
        "✓".green().bold(),
        backup_path.display()
    );

    // Remove command from config
    let mut updated_config = config;
    updated_config.commands.remove(&id);

    // Serialize back to TOML
    let toml_string = toml::to_string_pretty(&updated_config)
        .context("Failed to serialize configuration to TOML")?;

    // Write updated config
    fs::write(&config_file_path, toml_string)
        .await
        .context("Failed to write configuration file")?;

    println!(
        "{} {} '{}'",
        "✓".green().bold(),
        get_message(MessageKey::CommandRemoved, lang),
        id.green()
    );

    // Log the action
    tracing::info!("Removed command: {}", id);

    Ok(())
}

/// Find the configuration file path
async fn find_config_file() -> Result<PathBuf> {
    const CONFIG_FILENAMES: &[&str] = &["commands.toml", ".cmdrun.toml", "cmdrun.toml"];

    // Search in current directory first
    let current_dir = std::env::current_dir().context("Failed to get current directory")?;

    for filename in CONFIG_FILENAMES {
        let path = current_dir.join(filename);
        if path.exists() && path.is_file() {
            return Ok(path);
        }
    }

    // Search upwards
    let mut current = current_dir.clone();
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

    anyhow::bail!(
        "Configuration file not found. Searched for: {}",
        CONFIG_FILENAMES.join(", ")
    )
}

/// Create a backup of the configuration file
async fn create_backup(config_path: &PathBuf) -> Result<PathBuf> {
    let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S");
    let backup_filename = format!(
        "{}.backup.{}",
        config_path
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("commands.toml"),
        timestamp
    );

    let backup_path = if let Some(parent) = config_path.parent() {
        parent.join(backup_filename)
    } else {
        PathBuf::from(backup_filename)
    };

    fs::copy(config_path, &backup_path)
        .await
        .context("Failed to create backup")?;

    Ok(backup_path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use tokio::io::AsyncWriteExt;

    #[tokio::test]
    async fn test_remove_command() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("commands.toml");

        let toml_content = r#"
[config]
shell = "bash"

[commands.test]
description = "Run tests"
cmd = "cargo test"

[commands.build]
description = "Build project"
cmd = "cargo build"
"#;

        let mut file = tokio::fs::File::create(&config_path).await.unwrap();
        file.write_all(toml_content.as_bytes()).await.unwrap();
        file.sync_all().await.unwrap(); // Ensure data is written to disk

        // Remove the "test" command with force flag
        let result = handle_remove("test".to_string(), true, Some(config_path.clone())).await;

        assert!(result.is_ok());

        // Verify the command was removed
        let loader = ConfigLoader::with_path(&config_path);
        let config = loader.load().await.unwrap();

        assert!(!config.commands.contains_key("test"));
        assert!(config.commands.contains_key("build"));
    }

    #[tokio::test]
    async fn test_remove_nonexistent_command() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("commands.toml");

        let toml_content = r#"
[config]
shell = "bash"

[commands.test]
description = "Run tests"
cmd = "cargo test"
"#;

        let mut file = tokio::fs::File::create(&config_path).await.unwrap();
        file.write_all(toml_content.as_bytes()).await.unwrap();
        file.sync_all().await.unwrap(); // Ensure data is written to disk

        // Try to remove a command that doesn't exist
        let result = handle_remove("nonexistent".to_string(), true, Some(config_path)).await;

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("not found"));
    }

    #[tokio::test]
    async fn test_backup_creation() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("commands.toml");

        let toml_content = r#"
[config]
shell = "bash"

[commands.test]
description = "Run tests"
cmd = "cargo test"
"#;

        let mut file = tokio::fs::File::create(&config_path).await.unwrap();
        file.write_all(toml_content.as_bytes()).await.unwrap();
        file.sync_all().await.unwrap(); // Ensure data is written to disk

        // Create backup
        let backup_path = create_backup(&config_path).await.unwrap();

        // Verify backup exists
        assert!(backup_path.exists());
        assert!(backup_path
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .contains("backup"));

        // Verify backup content matches original
        let original_content = tokio::fs::read_to_string(&config_path).await.unwrap();
        let backup_content = tokio::fs::read_to_string(&backup_path).await.unwrap();
        assert_eq!(original_content, backup_content);
    }
}
