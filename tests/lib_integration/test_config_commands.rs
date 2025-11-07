//! Direct library tests for config command module

use cmdrun::commands::{handle_get, handle_set, handle_show};
use std::fs;
use tempfile::tempdir;

#[tokio::test]
async fn test_config_get_directly() {
    let temp_dir = tempdir().unwrap();
    let config_path = temp_dir.path().join("commands.toml");
    let config_content = r#"
[config]
language = "english"

[commands]
"#;
    fs::write(&config_path, config_content).unwrap();

    let result = handle_get("language", Some(config_path)).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_config_set_directly() {
    let temp_dir = tempdir().unwrap();
    let config_path = temp_dir.path().join("commands.toml");
    let config_content = r#"
[config]
timeout = 30

[commands]
"#;
    fs::write(&config_path, config_content).unwrap();

    let result = handle_set("timeout", "60", Some(config_path.clone())).await;
    assert!(result.is_ok());

    let updated_content = fs::read_to_string(&config_path).unwrap();
    assert!(updated_content.contains("60"));
}

#[tokio::test]
async fn test_config_show_directly() {
    let temp_dir = tempdir().unwrap();
    let config_path = temp_dir.path().join("commands.toml");
    let config_content = r#"
[config]
language = "english"

[commands]
"#;
    fs::write(&config_path, config_content).unwrap();

    let result = handle_show(Some(config_path)).await;
    assert!(result.is_ok());
}
