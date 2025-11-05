//! Configuration management commands

use anyhow::Result;
use colored::*;
use std::fs;
use std::path::PathBuf;
use toml_edit::DocumentMut;

use crate::config::loader::ConfigLoader;

/// Get a configuration value
pub async fn handle_get(key: &str, config_path: Option<PathBuf>) -> Result<()> {
    let config_loader = if let Some(path) = config_path {
        ConfigLoader::with_path(path)
    } else {
        ConfigLoader::new()
    };
    let config = config_loader.load().await?;

    match key {
        "language" => {
            let lang = match config.config.language {
                crate::config::schema::Language::English => "english",
                crate::config::schema::Language::Japanese => "japanese",
            };
            println!("{}", lang);
        }
        "shell" => println!("{}", config.config.shell),
        "timeout" => println!("{}", config.config.timeout),
        "strict_mode" => println!("{}", config.config.strict_mode),
        "parallel" => println!("{}", config.config.parallel),
        "working_dir" => println!("{}", config.config.working_dir.display()),
        _ => anyhow::bail!("Unknown configuration key: {}", key),
    }

    Ok(())
}

/// Set a configuration value
pub async fn handle_set(key: &str, value: &str, config_file_path: Option<PathBuf>) -> Result<()> {
    let config_path = if let Some(path) = config_file_path {
        path
    } else {
        get_config_path()?
    };
    let content = fs::read_to_string(&config_path)?;
    let mut doc = content.parse::<DocumentMut>()?;

    match key {
        "language" => {
            doc["config"]["language"] = toml_edit::value(value);
        }
        "shell" => {
            doc["config"]["shell"] = toml_edit::value(value);
        }
        "timeout" => {
            let timeout: u64 = value.parse()?;
            doc["config"]["timeout"] = toml_edit::value(timeout as i64);
        }
        "strict_mode" => {
            let strict: bool = value.parse()?;
            doc["config"]["strict_mode"] = toml_edit::value(strict);
        }
        "parallel" => {
            let parallel: bool = value.parse()?;
            doc["config"]["parallel"] = toml_edit::value(parallel);
        }
        "working_dir" => {
            doc["config"]["working_dir"] = toml_edit::value(value);
        }
        _ => anyhow::bail!("Unknown configuration key: {}", key),
    }

    fs::write(&config_path, doc.to_string())?;

    println!(
        "{} Set {} = {}",
        "✓".green().bold(),
        key.cyan(),
        value.bright_white()
    );

    Ok(())
}

/// Show all configuration settings
pub async fn handle_show(config_path: Option<PathBuf>) -> Result<()> {
    let config_loader = if let Some(path) = config_path {
        ConfigLoader::with_path(path)
    } else {
        ConfigLoader::new()
    };
    let config = config_loader.load().await?;

    let lang = match config.config.language {
        crate::config::schema::Language::English => "english",
        crate::config::schema::Language::Japanese => "japanese",
    };

    println!("{}", "Configuration:".cyan().bold());
    println!();
    println!("  {} {}", "language:".dimmed(), lang);
    println!("  {} {}", "shell:".dimmed(), config.config.shell);
    println!("  {} {}", "timeout:".dimmed(), config.config.timeout);
    println!(
        "  {} {}",
        "strict_mode:".dimmed(),
        config.config.strict_mode
    );
    println!("  {} {}", "parallel:".dimmed(), config.config.parallel);
    println!(
        "  {} {}",
        "working_dir:".dimmed(),
        config.config.working_dir.display()
    );

    if !config.config.env.is_empty() {
        println!();
        println!("  {}", "Environment variables:".dimmed());
        for (key, value) in &config.config.env {
            println!("    {} = {}", key.cyan(), value);
        }
    }

    Ok(())
}

/// Get configuration file path
fn get_config_path() -> Result<PathBuf> {
    // カレントディレクトリから上位へ探索
    let current_dir = std::env::current_dir()?;
    let mut current = current_dir.clone();

    loop {
        for filename in &["commands.toml", ".cmdrun.toml", "cmdrun.toml"] {
            let path = current.join(filename);
            if path.exists() && path.is_file() {
                return Ok(path);
            }
        }

        // 親ディレクトリへ移動
        if !current.pop() {
            break;
        }
    }

    // ホームディレクトリを探索
    if let Some(home_dir) = dirs::home_dir() {
        let cmdrun_dir = home_dir.join(".cmdrun");
        for filename in &["commands.toml", ".cmdrun.toml", "cmdrun.toml"] {
            let path = cmdrun_dir.join(filename);
            if path.exists() && path.is_file() {
                return Ok(path);
            }
        }
    }

    anyhow::bail!("Configuration file not found")
}
