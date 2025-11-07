//! Integration tests for environment management
//!
//! 環境管理機能の統合テスト

use cmdrun::config::environment::EnvironmentManager;
use tempfile::TempDir;
use tokio::fs;

#[tokio::test]
async fn test_environment_lifecycle() {
    // Setup test directory
    let temp_dir = TempDir::new().unwrap();
    let config_dir = temp_dir.path().join(".cmdrun");
    fs::create_dir_all(&config_dir).await.unwrap();

    let manager = EnvironmentManager::new(config_dir.clone());

    // Test 1: Default environment
    let current = manager.get_current_environment().await.unwrap();
    assert_eq!(current, "default");

    // Test 2: Create new environment
    manager
        .create_environment("dev".to_string(), "Development environment".to_string())
        .await
        .unwrap();

    // Test 3: List environments
    let envs = manager.list_environments().await.unwrap();
    assert_eq!(envs.len(), 2); // default + dev
    assert!(envs.iter().any(|(name, _)| name == "dev"));

    // Test 4: Switch environment
    manager.switch_environment("dev").await.unwrap();
    let current = manager.get_current_environment().await.unwrap();
    assert_eq!(current, "dev");

    // Test 5: Set environment variable
    manager
        .set_variable("dev", "API_URL".to_string(), "http://localhost:3000".to_string())
        .await
        .unwrap();

    // Test 6: Verify variable was set
    let config = manager.load_environment_config().await.unwrap();
    let dev_env = config.environments.get("dev").unwrap();
    assert_eq!(
        dev_env.variables.get("API_URL").unwrap(),
        "http://localhost:3000"
    );
}

#[tokio::test]
async fn test_environment_config_merge() {
    let temp_dir = TempDir::new().unwrap();
    let config_dir = temp_dir.path().join(".cmdrun");
    fs::create_dir_all(&config_dir).await.unwrap();

    // Create base config
    let base_config = r#"
[config]
shell = "bash"
timeout = 300

[config.env]
BASE_VAR = "base_value"

[commands.test]
description = "Test command"
cmd = "echo test"
"#;

    fs::write(config_dir.join("config.toml"), base_config)
        .await
        .unwrap();

    // Create dev environment config
    let dev_config = r#"
[config]
timeout = 600

[config.env]
DEV_VAR = "dev_value"
BASE_VAR = "dev_override"

[commands.dev_test]
description = "Dev test command"
cmd = "echo dev"
"#;

    fs::write(config_dir.join("config.dev.toml"), dev_config)
        .await
        .unwrap();

    let manager = EnvironmentManager::new(config_dir.clone());

    // Load config for dev environment
    let merged = manager.load_config_for_environment("dev").await.unwrap();

    // Verify merge
    assert_eq!(merged.config.timeout, 600); // dev overrides
    assert_eq!(merged.commands.len(), 2); // base + dev commands
    assert!(merged.commands.contains_key("test"));
    assert!(merged.commands.contains_key("dev_test"));
    assert_eq!(merged.config.env.get("BASE_VAR").unwrap(), "dev_override");
    assert_eq!(merged.config.env.get("DEV_VAR").unwrap(), "dev_value");
}

#[tokio::test]
async fn test_multiple_environments() {
    let temp_dir = TempDir::new().unwrap();
    let config_dir = temp_dir.path().join(".cmdrun");
    fs::create_dir_all(&config_dir).await.unwrap();

    let manager = EnvironmentManager::new(config_dir);

    // Create multiple environments
    let environments = vec![
        ("dev", "Development environment"),
        ("staging", "Staging environment"),
        ("prod", "Production environment"),
    ];

    for (name, desc) in &environments {
        manager
            .create_environment(name.to_string(), desc.to_string())
            .await
            .unwrap();
    }

    // List all environments
    let envs = manager.list_environments().await.unwrap();
    assert_eq!(envs.len(), 4); // default + 3 created

    // Verify all environments exist
    for (name, _) in &environments {
        assert!(envs.iter().any(|(env_name, _)| env_name == name));
    }
}

#[tokio::test]
async fn test_environment_not_found_error() {
    let temp_dir = TempDir::new().unwrap();
    let config_dir = temp_dir.path().join(".cmdrun");
    fs::create_dir_all(&config_dir).await.unwrap();

    let manager = EnvironmentManager::new(config_dir);

    // Try to switch to non-existent environment
    let result = manager.switch_environment("nonexistent").await;
    assert!(result.is_err());
    assert!(result
        .unwrap_err()
        .to_string()
        .contains("not found"));
}

#[tokio::test]
async fn test_duplicate_environment_error() {
    let temp_dir = TempDir::new().unwrap();
    let config_dir = temp_dir.path().join(".cmdrun");
    fs::create_dir_all(&config_dir).await.unwrap();

    let manager = EnvironmentManager::new(config_dir);

    // Create environment
    manager
        .create_environment("test".to_string(), "Test".to_string())
        .await
        .unwrap();

    // Try to create duplicate
    let result = manager
        .create_environment("test".to_string(), "Duplicate".to_string())
        .await;
    assert!(result.is_err());
    assert!(result
        .unwrap_err()
        .to_string()
        .contains("already exists"));
}

#[tokio::test]
async fn test_environment_variables_isolation() {
    let temp_dir = TempDir::new().unwrap();
    let config_dir = temp_dir.path().join(".cmdrun");
    fs::create_dir_all(&config_dir).await.unwrap();

    let manager = EnvironmentManager::new(config_dir);

    // Create two environments
    manager
        .create_environment("env1".to_string(), "Environment 1".to_string())
        .await
        .unwrap();
    manager
        .create_environment("env2".to_string(), "Environment 2".to_string())
        .await
        .unwrap();

    // Set different variables in each
    manager
        .set_variable("env1", "VAR".to_string(), "value1".to_string())
        .await
        .unwrap();
    manager
        .set_variable("env2", "VAR".to_string(), "value2".to_string())
        .await
        .unwrap();

    // Verify isolation
    let config = manager.load_environment_config().await.unwrap();
    assert_eq!(
        config
            .environments
            .get("env1")
            .unwrap()
            .variables
            .get("VAR")
            .unwrap(),
        "value1"
    );
    assert_eq!(
        config
            .environments
            .get("env2")
            .unwrap()
            .variables
            .get("VAR")
            .unwrap(),
        "value2"
    );
}
