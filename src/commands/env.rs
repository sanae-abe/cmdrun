//! Environment management commands
//!
//! 環境切り替えと環境変数管理コマンド

use crate::config::environment::EnvironmentManager;
use anyhow::{Context, Result};
use colored::Colorize;

/// `cmdrun env use <env>` - 環境切り替え
pub async fn handle_use(env_name: String) -> Result<()> {
    let manager = EnvironmentManager::default_instance()
        .context("Failed to initialize environment manager")?;

    manager
        .switch_environment(&env_name)
        .await
        .with_context(|| format!("Failed to switch to environment '{}'", env_name))?;

    println!(
        "{} Switched to environment: {}",
        "✓".green().bold(),
        env_name.cyan().bold()
    );

    Ok(())
}

/// `cmdrun env current` - 現在の環境表示
pub async fn handle_current() -> Result<()> {
    let manager = EnvironmentManager::default_instance()
        .context("Failed to initialize environment manager")?;

    let current = manager
        .get_current_environment()
        .await
        .context("Failed to get current environment")?;

    println!("{}", "Current environment:".bold());
    println!("  {}", current.cyan().bold());

    Ok(())
}

/// `cmdrun env list` - 利用可能な環境一覧
pub async fn handle_list() -> Result<()> {
    let manager = EnvironmentManager::default_instance()
        .context("Failed to initialize environment manager")?;

    let envs = manager
        .list_environments()
        .await
        .context("Failed to list environments")?;

    let current = manager.get_current_environment().await?;

    println!("{}", "Available environments:".bold());
    println!();

    for (name, description) in envs {
        let is_current = name == current;
        let marker = if is_current { "→" } else { " " };
        let name_display = if is_current {
            name.green().bold()
        } else {
            name.cyan()
        };

        println!(
            "  {} {} - {}",
            marker.green().bold(),
            name_display,
            description.dimmed()
        );
    }

    Ok(())
}

/// `cmdrun env set <key> <value>` - 環境変数設定
pub async fn handle_set(key: String, value: String, env_name: Option<String>) -> Result<()> {
    let manager = EnvironmentManager::default_instance()
        .context("Failed to initialize environment manager")?;

    let target_env = if let Some(env) = env_name {
        env
    } else {
        manager.get_current_environment().await?
    };

    manager
        .set_variable(&target_env, key.clone(), value.clone())
        .await
        .with_context(|| format!("Failed to set variable {} in environment '{}'", key, target_env))?;

    println!(
        "{} Set {}={} in environment '{}'",
        "✓".green().bold(),
        key.cyan(),
        value.yellow(),
        target_env.cyan().bold()
    );

    Ok(())
}

/// `cmdrun env create <name>` - 新しい環境を作成
pub async fn handle_create(name: String, description: Option<String>) -> Result<()> {
    let manager = EnvironmentManager::default_instance()
        .context("Failed to initialize environment manager")?;

    let desc = description.unwrap_or_else(|| format!("{} environment", name));

    manager
        .create_environment(name.clone(), desc.clone())
        .await
        .with_context(|| format!("Failed to create environment '{}'", name))?;

    println!(
        "{} Created environment: {} - {}",
        "✓".green().bold(),
        name.cyan().bold(),
        desc.dimmed()
    );

    Ok(())
}

/// `cmdrun env info <name>` - 環境の詳細情報表示
pub async fn handle_info(env_name: Option<String>) -> Result<()> {
    let manager = EnvironmentManager::default_instance()
        .context("Failed to initialize environment manager")?;

    let target_env = if let Some(env) = env_name {
        env
    } else {
        manager.get_current_environment().await?
    };

    let config = manager.load_environment_config().await?;

    println!("{}", format!("Environment: {}", target_env).bold());
    println!();

    if target_env == "default" {
        println!("  {}: Default environment", "Description".dimmed());
        println!("  {}: .cmdrun/config.toml", "Config file".dimmed());
    } else if let Some(env) = config.environments.get(&target_env) {
        println!("  {}: {}", "Description".dimmed(), env.description);

        if !env.variables.is_empty() {
            println!();
            println!("  {}:", "Environment variables".bold());
            for (key, value) in &env.variables {
                println!("    {} = {}", key.cyan(), value.yellow());
            }
        }

        let config_path = manager.get_env_config_path(&target_env);
        println!();
        println!("  {}: {}", "Config file".dimmed(), config_path.display());
    } else {
        anyhow::bail!("Environment '{}' not found", target_env);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use tokio::fs;

    async fn setup_test_env() -> (TempDir, EnvironmentManager) {
        let temp_dir = TempDir::new().unwrap();
        let config_dir = temp_dir.path().join(".cmdrun");
        fs::create_dir_all(&config_dir).await.unwrap();

        // Set current directory for testing
        std::env::set_current_dir(temp_dir.path()).unwrap();

        let manager = EnvironmentManager::new(config_dir);
        (temp_dir, manager)
    }

    #[tokio::test]
    async fn test_create_environment() {
        let (_temp_dir, manager) = setup_test_env().await;

        let result = manager
            .create_environment("test".to_string(), "Test environment".to_string())
            .await;

        assert!(result.is_ok());

        let envs = manager.list_environments().await.unwrap();
        assert!(envs.iter().any(|(name, _)| name == "test"));
    }

    #[tokio::test]
    async fn test_switch_environment() {
        let (_temp_dir, manager) = setup_test_env().await;

        manager
            .create_environment("dev".to_string(), "Development".to_string())
            .await
            .unwrap();

        manager.switch_environment("dev").await.unwrap();

        let current = manager.get_current_environment().await.unwrap();
        assert_eq!(current, "dev");
    }
}
