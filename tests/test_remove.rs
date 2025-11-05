//! Integration test for the remove command

use cmdrun::commands::handle_remove;
use cmdrun::config::loader::ConfigLoader;
use std::path::PathBuf;
use tempfile::TempDir;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

#[tokio::test]
async fn test_remove_command_with_force() {
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

[commands.deploy]
description = "Deploy application"
cmd = "deploy.sh"
deps = ["build", "test"]
"#;

    let mut file = File::create(&config_path).await.unwrap();
    file.write_all(toml_content.as_bytes()).await.unwrap();

    // Remove the "test" command with force flag
    let result = handle_remove(
        "test".to_string(),
        true,
        Some(config_path.clone())
    ).await;

    assert!(result.is_ok(), "Failed to remove command: {:?}", result.err());

    // Verify the command was removed
    let loader = ConfigLoader::with_path(&config_path);
    let config = loader.load().await.unwrap();

    assert!(!config.commands.contains_key("test"), "Command 'test' should be removed");
    assert!(config.commands.contains_key("build"), "Command 'build' should still exist");
    assert!(config.commands.contains_key("deploy"), "Command 'deploy' should still exist");
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

    let mut file = File::create(&config_path).await.unwrap();
    file.write_all(toml_content.as_bytes()).await.unwrap();

    // Try to remove a command that doesn't exist
    let result = handle_remove(
        "nonexistent".to_string(),
        true,
        Some(config_path)
    ).await;

    assert!(result.is_err(), "Should fail when removing nonexistent command");
    let err_msg = result.unwrap_err().to_string();
    assert!(err_msg.contains("not found"), "Error should mention command not found, got: {}", err_msg);
}

#[tokio::test]
async fn test_backup_is_created() {
    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir.path().join("commands.toml");

    let toml_content = r#"
[config]
shell = "bash"

[commands.test]
description = "Run tests"
cmd = "cargo test"
tags = ["testing"]
"#;

    let mut file = File::create(&config_path).await.unwrap();
    file.write_all(toml_content.as_bytes()).await.unwrap();

    // Remove command with force
    let result = handle_remove(
        "test".to_string(),
        true,
        Some(config_path.clone())
    ).await;

    assert!(result.is_ok(), "Failed to remove command: {:?}", result.err());

    // Check that a backup file was created
    let backup_files: Vec<_> = std::fs::read_dir(temp_dir.path())
        .unwrap()
        .filter_map(Result::ok)
        .filter(|entry| {
            entry.file_name()
                .to_str()
                .map(|s| s.contains("backup"))
                .unwrap_or(false)
        })
        .collect();

    assert_eq!(backup_files.len(), 1, "Should have exactly one backup file");

    // Verify backup content matches original
    let backup_content = tokio::fs::read_to_string(backup_files[0].path()).await.unwrap();
    assert!(backup_content.contains("Run tests"), "Backup should contain original command");
    assert!(backup_content.contains("cargo test"), "Backup should contain original cmd");
}

#[tokio::test]
async fn test_remove_command_with_dependencies() {
    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir.path().join("commands.toml");

    let toml_content = r#"
[config]
shell = "bash"

[commands.lint]
description = "Lint code"
cmd = "cargo clippy"

[commands.test]
description = "Run tests"
cmd = "cargo test"
deps = ["lint"]

[commands.build]
description = "Build project"
cmd = "cargo build"
deps = ["test"]
"#;

    let mut file = File::create(&config_path).await.unwrap();
    file.write_all(toml_content.as_bytes()).await.unwrap();

    // Remove the "lint" command (which is a dependency of test)
    let result = handle_remove(
        "lint".to_string(),
        true,
        Some(config_path.clone())
    ).await;

    assert!(result.is_ok(), "Should be able to remove command even if it's a dependency");

    // Verify the command was removed
    let loader = ConfigLoader::with_path(&config_path);
    let config = loader.load().await.unwrap();

    assert!(!config.commands.contains_key("lint"), "Command 'lint' should be removed");
    assert!(config.commands.contains_key("test"), "Command 'test' should still exist");
    assert!(config.commands.contains_key("build"), "Command 'build' should still exist");
}
