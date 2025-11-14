//! Direct library tests for search command module

use cmdrun::commands::handle_search;
use std::fs;
use tempfile::tempdir;

#[tokio::test]
async fn test_search_directly() {
    let temp_dir = tempdir().unwrap();
    let config_path = temp_dir.path().join("commands.toml");
    let config_content = r#"
[commands]
test = { description = "Run tests", cmd = "cargo test" }
build = { description = "Build project", cmd = "cargo build" }
"#;
    fs::write(&config_path, config_content).unwrap();

    let result = handle_search("test".to_string(), false, Some(config_path)).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_search_by_description() {
    let temp_dir = tempdir().unwrap();
    let config_path = temp_dir.path().join("commands.toml");
    let config_content = r#"
[commands]
test = { description = "Run unit tests", cmd = "cargo test" }
"#;
    fs::write(&config_path, config_content).unwrap();

    let result = handle_search("unit".to_string(), false, Some(config_path)).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_search_no_results() {
    let temp_dir = tempdir().unwrap();
    let config_path = temp_dir.path().join("commands.toml");
    fs::write(&config_path, "[commands]\n").unwrap();

    let result = handle_search("nonexistent".to_string(), false, Some(config_path)).await;
    assert!(result.is_ok());
}
