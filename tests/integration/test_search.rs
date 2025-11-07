//! Integration tests for the search command
//!
//! Tests searching commands by keywords across different fields

use anyhow::Result;
use std::fs;
use tempfile::tempdir;

/// Test searching by command ID
#[tokio::test]
async fn test_search_by_id() -> Result<()> {
    let temp_dir = tempdir()?;
    let config_path = temp_dir.path().join("commands.toml");
    let config_content = r#"
[commands]
test = { description = "Run tests", cmd = "cargo test" }
build = { description = "Build project", cmd = "cargo build" }
"#;
    fs::write(&config_path, config_content)?;

    cmdrun::commands::handle_search("test".to_string(), Some(config_path)).await?;

    Ok(())
}

/// Test searching by description
#[tokio::test]
async fn test_search_by_description() -> Result<()> {
    let temp_dir = tempdir()?;
    let config_path = temp_dir.path().join("commands.toml");
    let config_content = r#"
[commands]
test = { description = "Run unit tests", cmd = "cargo test" }
integration = { description = "Run integration tests", cmd = "cargo test --test" }
build = { description = "Build project", cmd = "cargo build" }
"#;
    fs::write(&config_path, config_content)?;

    cmdrun::commands::handle_search("tests".to_string(), Some(config_path)).await?;

    Ok(())
}

/// Test searching by command text
#[tokio::test]
async fn test_search_by_command() -> Result<()> {
    let temp_dir = tempdir()?;
    let config_path = temp_dir.path().join("commands.toml");
    let config_content = r#"
[commands]
test = { description = "Test", cmd = "cargo test" }
build = { description = "Build", cmd = "cargo build --release" }
fmt = { description = "Format", cmd = "cargo fmt" }
"#;
    fs::write(&config_path, config_content)?;

    cmdrun::commands::handle_search("cargo".to_string(), Some(config_path)).await?;

    Ok(())
}

/// Test searching by tags
#[tokio::test]
async fn test_search_by_tags() -> Result<()> {
    let temp_dir = tempdir()?;
    let config_path = temp_dir.path().join("commands.toml");
    let config_content = r#"
[commands]
test = { description = "Test", cmd = "cargo test", tags = ["ci", "quality"] }
build = { description = "Build", cmd = "cargo build", tags = ["build"] }
deploy = { description = "Deploy", cmd = "deploy.sh", tags = ["production", "deploy"] }
"#;
    fs::write(&config_path, config_content)?;

    cmdrun::commands::handle_search("production".to_string(), Some(config_path)).await?;

    Ok(())
}

/// Test case-insensitive search
#[tokio::test]
async fn test_search_case_insensitive() -> Result<()> {
    let temp_dir = tempdir()?;
    let config_path = temp_dir.path().join("commands.toml");
    let config_content = r#"
[commands]
test = { description = "Run TESTS", cmd = "cargo test" }
build = { description = "Build Project", cmd = "cargo build" }
"#;
    fs::write(&config_path, config_content)?;

    // Search with lowercase keyword
    cmdrun::commands::handle_search("tests".to_string(), Some(config_path.clone())).await?;

    // Search with uppercase keyword
    cmdrun::commands::handle_search("BUILD".to_string(), Some(config_path)).await?;

    Ok(())
}

/// Test search with no matches
#[tokio::test]
async fn test_search_no_matches() -> Result<()> {
    let temp_dir = tempdir()?;
    let config_path = temp_dir.path().join("commands.toml");
    let config_content = r#"
[commands]
test = { description = "Test", cmd = "cargo test" }
build = { description = "Build", cmd = "cargo build" }
"#;
    fs::write(&config_path, config_content)?;

    cmdrun::commands::handle_search("nonexistent".to_string(), Some(config_path)).await?;

    Ok(())
}

/// Test search with empty config
#[tokio::test]
async fn test_search_empty_config() -> Result<()> {
    let temp_dir = tempdir()?;
    let config_path = temp_dir.path().join("commands.toml");
    fs::write(&config_path, "[commands]\n")?;

    cmdrun::commands::handle_search("test".to_string(), Some(config_path)).await?;

    Ok(())
}

/// Test search with multiple matches
#[tokio::test]
async fn test_search_multiple_matches() -> Result<()> {
    let temp_dir = tempdir()?;
    let config_path = temp_dir.path().join("commands.toml");
    let config_content = r#"
[commands]
test-unit = { description = "Unit tests", cmd = "cargo test --lib" }
test-integration = { description = "Integration tests", cmd = "cargo test --test" }
test-all = { description = "All tests", cmd = "cargo test" }
build = { description = "Build", cmd = "cargo build" }
"#;
    fs::write(&config_path, config_content)?;

    cmdrun::commands::handle_search("test".to_string(), Some(config_path)).await?;

    Ok(())
}

/// Test search with partial keyword match
#[tokio::test]
async fn test_search_partial_match() -> Result<()> {
    let temp_dir = tempdir()?;
    let config_path = temp_dir.path().join("commands.toml");
    let config_content = r#"
[commands]
test = { description = "Testing", cmd = "cargo test" }
testing = { description = "Run tests", cmd = "cargo test" }
"#;
    fs::write(&config_path, config_content)?;

    cmdrun::commands::handle_search("test".to_string(), Some(config_path)).await?;

    Ok(())
}

/// Test search with multiple command specs
#[tokio::test]
async fn test_search_multiple_commands() -> Result<()> {
    let temp_dir = tempdir()?;
    let config_path = temp_dir.path().join("commands.toml");
    let config_content = r#"
[commands]
test = { description = "Test", cmd = ["cargo test", "cargo bench"] }
"#;
    fs::write(&config_path, config_content)?;

    cmdrun::commands::handle_search("bench".to_string(), Some(config_path)).await?;

    Ok(())
}

/// Test search with platform-specific commands
#[tokio::test]
async fn test_search_platform_commands() -> Result<()> {
    let temp_dir = tempdir()?;
    let config_path = temp_dir.path().join("commands.toml");
    let config_content = r#"
[commands.clean]
description = "Clean build artifacts"

  [commands.clean.cmd.platform]
  unix = "rm -rf target"
  windows = "rmdir /s /q target"
"#;
    fs::write(&config_path, config_content)?;

    cmdrun::commands::handle_search("clean".to_string(), Some(config_path.clone())).await?;
    cmdrun::commands::handle_search("target".to_string(), Some(config_path)).await?;

    Ok(())
}

/// Test search with special characters
#[tokio::test]
async fn test_search_special_characters() -> Result<()> {
    let temp_dir = tempdir()?;
    let config_path = temp_dir.path().join("commands.toml");
    let config_content = r#"
[commands]
test = { description = "Test (unit)", cmd = "cargo test" }
build = { description = "Build [release]", cmd = "cargo build --release" }
"#;
    fs::write(&config_path, config_content)?;

    cmdrun::commands::handle_search("unit".to_string(), Some(config_path.clone())).await?;
    cmdrun::commands::handle_search("release".to_string(), Some(config_path)).await?;

    Ok(())
}

/// Test search with Unicode characters
#[tokio::test]
async fn test_search_unicode() -> Result<()> {
    let temp_dir = tempdir()?;
    let config_path = temp_dir.path().join("commands.toml");
    let config_content = r#"
[commands]
test = { description = "テスト", cmd = "cargo test" }
build = { description = "ビルド", cmd = "cargo build" }
"#;
    fs::write(&config_path, config_content)?;

    cmdrun::commands::handle_search("テスト".to_string(), Some(config_path)).await?;

    Ok(())
}

/// Test search matching in ID and description
#[tokio::test]
async fn test_search_multiple_locations() -> Result<()> {
    let temp_dir = tempdir()?;
    let config_path = temp_dir.path().join("commands.toml");
    let config_content = r#"
[commands]
build = { description = "Build the project", cmd = "cargo build" }
"#;
    fs::write(&config_path, config_content)?;

    cmdrun::commands::handle_search("build".to_string(), Some(config_path)).await?;

    Ok(())
}

/// Test search with whitespace in keyword
#[tokio::test]
async fn test_search_whitespace_keyword() -> Result<()> {
    let temp_dir = tempdir()?;
    let config_path = temp_dir.path().join("commands.toml");
    let config_content = r#"
[commands]
test = { description = "Run unit tests", cmd = "cargo test" }
"#;
    fs::write(&config_path, config_content)?;

    cmdrun::commands::handle_search("unit tests".to_string(), Some(config_path)).await?;

    Ok(())
}

/// Test search with hyphenated keywords
#[tokio::test]
async fn test_search_hyphenated() -> Result<()> {
    let temp_dir = tempdir()?;
    let config_path = temp_dir.path().join("commands.toml");
    let config_content = r#"
[commands]
test-unit = { description = "Unit tests", cmd = "cargo test" }
test-integration = { description = "Integration tests", cmd = "cargo test --test" }
"#;
    fs::write(&config_path, config_content)?;

    cmdrun::commands::handle_search("test-unit".to_string(), Some(config_path)).await?;

    Ok(())
}

/// Test search with numbers
#[tokio::test]
async fn test_search_numbers() -> Result<()> {
    let temp_dir = tempdir()?;
    let config_path = temp_dir.path().join("commands.toml");
    let config_content = r#"
[commands]
test = { description = "Run test suite v2", cmd = "cargo test" }
"#;
    fs::write(&config_path, config_content)?;

    cmdrun::commands::handle_search("v2".to_string(), Some(config_path)).await?;

    Ok(())
}

/// Test search with single character
#[tokio::test]
async fn test_search_single_character() -> Result<()> {
    let temp_dir = tempdir()?;
    let config_path = temp_dir.path().join("commands.toml");
    let config_content = r#"
[commands]
a = { description = "Command A", cmd = "echo a" }
b = { description = "Command B", cmd = "echo b" }
"#;
    fs::write(&config_path, config_content)?;

    cmdrun::commands::handle_search("a".to_string(), Some(config_path)).await?;

    Ok(())
}

/// Test search with complex tags
#[tokio::test]
async fn test_search_complex_tags() -> Result<()> {
    let temp_dir = tempdir()?;
    let config_path = temp_dir.path().join("commands.toml");
    let config_content = r#"
[commands]
deploy-prod = {
    description = "Deploy to production",
    cmd = "deploy.sh",
    tags = ["deploy", "production", "critical", "ops"]
}
deploy-staging = {
    description = "Deploy to staging",
    cmd = "deploy.sh --staging",
    tags = ["deploy", "staging", "test"]
}
"#;
    fs::write(&config_path, config_content)?;

    cmdrun::commands::handle_search("critical".to_string(), Some(config_path)).await?;

    Ok(())
}

/// Test search with command containing pipes
#[tokio::test]
async fn test_search_piped_command() -> Result<()> {
    let temp_dir = tempdir()?;
    let config_path = temp_dir.path().join("commands.toml");
    let config_content = r#"
[commands]
count = { description = "Count lines", cmd = "find . -name '*.rs' | wc -l" }
"#;
    fs::write(&config_path, config_content)?;

    cmdrun::commands::handle_search("wc".to_string(), Some(config_path)).await?;

    Ok(())
}
