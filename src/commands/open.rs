//! Open command - Opens commands.toml in the default editor

use anyhow::{Context, Result};
use colored::*;
use std::path::PathBuf;
use std::process::Command;
use tracing::{debug, info};

use crate::config::Language;
use crate::i18n::{get_message, MessageKey};

/// Open the configuration file in the default editor
pub async fn handle_open(config_file_path: Option<PathBuf>) -> Result<()> {
    let config_path = if let Some(path) = config_file_path {
        path
    } else {
        find_config_file().await?
    };

    info!("Opening configuration file: {}", config_path.display());
    println!(
        "{} {}",
        "Opening:".cyan().bold(),
        config_path.display().to_string().bright_white()
    );

    open_in_editor(&config_path)?;

    Ok(())
}

/// Find the configuration file (TOML preferred, JSON fallback)
async fn find_config_file() -> Result<PathBuf> {
    const CONFIG_FILENAMES: &[&str] = &["commands.toml", ".cmdrun.toml", "cmdrun.toml"];

    // Search in current directory and upwards
    let current_dir = std::env::current_dir().context("Failed to get current directory")?;

    if let Some(path) = search_upwards(&current_dir, CONFIG_FILENAMES).await? {
        return Ok(path);
    }

    // Search in home directory
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
        "{}",
        get_message(MessageKey::ErrorNoConfigFileFound, Language::English)
    )
}

/// Search upwards from a directory for config files
async fn search_upwards(
    start_dir: &std::path::Path,
    filenames: &[&str],
) -> Result<Option<PathBuf>> {
    let mut current = start_dir.to_path_buf();

    loop {
        for filename in filenames {
            let path = current.join(filename);
            if path.exists() && path.is_file() {
                debug!("Found config file: {}", path.display());
                return Ok(Some(path));
            }
        }

        // Move to parent directory
        if !current.pop() {
            break;
        }
    }

    Ok(None)
}

/// Open file in the appropriate editor
fn open_in_editor(path: &std::path::Path) -> Result<()> {
    // Try different editor commands in order of preference
    let editors = if cfg!(target_os = "macos") {
        vec![
            ("open", vec![path.to_str().unwrap()]),
            ("code", vec![path.to_str().unwrap()]),
            ("vim", vec![path.to_str().unwrap()]),
        ]
    } else if cfg!(target_os = "windows") {
        vec![
            ("code", vec![path.to_str().unwrap()]),
            ("notepad", vec![path.to_str().unwrap()]),
        ]
    } else {
        vec![
            ("xdg-open", vec![path.to_str().unwrap()]),
            ("code", vec![path.to_str().unwrap()]),
            ("vim", vec![path.to_str().unwrap()]),
            ("nano", vec![path.to_str().unwrap()]),
        ]
    };

    for (cmd, args) in editors {
        if let Ok(which_result) = which::which(cmd) {
            debug!("Using editor: {} at {:?}", cmd, which_result);

            let status = Command::new(cmd)
                .args(&args)
                .status()
                .with_context(|| format!("Failed to execute editor: {}", cmd))?;

            if status.success() {
                println!("{} Opened in {}", "✓".green().bold(), cmd.bright_white());
                return Ok(());
            }
        }
    }

    // Fallback: just print the path
    println!(
        "{} Please manually open: {}",
        "⚠".yellow().bold(),
        path.display().to_string().bright_white()
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use tokio::fs::File;
    use tokio::io::AsyncWriteExt;

    #[tokio::test]
    async fn test_find_config_in_current_dir() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("commands.toml");

        let mut file = File::create(&config_path).await.unwrap();
        file.write_all(b"[config]\nshell = \"bash\"").await.unwrap();

        // Change to temp directory
        std::env::set_current_dir(temp_dir.path()).unwrap();

        let found = find_config_file().await;
        assert!(found.is_ok());
    }

    #[tokio::test]
    async fn test_search_upwards() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("commands.toml");

        let mut file = File::create(&config_path).await.unwrap();
        file.write_all(b"[config]\nshell = \"bash\"").await.unwrap();

        let subdir = temp_dir.path().join("subdir");
        tokio::fs::create_dir(&subdir).await.unwrap();

        let result = search_upwards(&subdir, &["commands.toml"]).await.unwrap();
        assert!(result.is_some());
        assert_eq!(result.unwrap(), config_path);
    }
}
