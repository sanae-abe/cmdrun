//! Direct library tests for add command module

use cmdrun::commands::handle_add;
use std::fs;
use tempfile::tempdir;

#[tokio::test]
async fn test_add_command_directly() {
    let temp_dir = tempdir().unwrap();
    let config_path = temp_dir.path().join("commands.toml");
    fs::write(&config_path, "[commands]\n").unwrap();

    let result = handle_add(
        Some("test".to_string()),
        Some("echo hello".to_string()),
        Some("Test command".to_string()),
        None,
        None,
        Some(config_path.clone()),
    )
    .await;

    assert!(result.is_ok());
    let content = fs::read_to_string(&config_path).unwrap();
    assert!(content.contains("test"));
}

#[tokio::test]
async fn test_add_with_tags() {
    let temp_dir = tempdir().unwrap();
    let config_path = temp_dir.path().join("commands.toml");
    fs::write(&config_path, "[commands]\n").unwrap();

    let result = handle_add(
        Some("tagged".to_string()),
        Some("echo tagged".to_string()),
        Some("Tagged command".to_string()),
        None,
        Some(vec!["tag1".to_string(), "tag2".to_string()]),
        Some(config_path.clone()),
    )
    .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_add_with_category() {
    let temp_dir = tempdir().unwrap();
    let config_path = temp_dir.path().join("commands.toml");
    fs::write(&config_path, "[commands]\n").unwrap();

    let result = handle_add(
        Some("build".to_string()),
        Some("cargo build".to_string()),
        Some("Build project".to_string()),
        Some("build".to_string()),
        None,
        Some(config_path.clone()),
    )
    .await;

    assert!(result.is_ok());
}
