//! Integration tests for config management commands
//!
//! Tests get, set, and show configuration operations

use anyhow::Result;
use std::fs;
use tempfile::tempdir;

/// Test getting a configuration value
#[tokio::test]
async fn test_config_get_language() -> Result<()> {
    let temp_dir = tempdir()?;
    let config_path = temp_dir.path().join("commands.toml");
    let config_content = r#"
[config]
language = "english"

[commands]
"#;
    fs::write(&config_path, config_content)?;

    // Test by calling the function (we can't easily capture stdout in unit tests,
    // but we can verify it doesn't error)
    cmdrun::commands::handle_get("language", Some(config_path)).await?;

    Ok(())
}

/// Test getting shell configuration
#[tokio::test]
async fn test_config_get_shell() -> Result<()> {
    let temp_dir = tempdir()?;
    let config_path = temp_dir.path().join("commands.toml");
    let config_content = r#"
[config]
shell = "bash"

[commands]
"#;
    fs::write(&config_path, config_content)?;

    cmdrun::commands::handle_get("shell", Some(config_path)).await?;

    Ok(())
}

/// Test getting timeout configuration
#[tokio::test]
async fn test_config_get_timeout() -> Result<()> {
    let temp_dir = tempdir()?;
    let config_path = temp_dir.path().join("commands.toml");
    let config_content = r#"
[config]
timeout = 60

[commands]
"#;
    fs::write(&config_path, config_content)?;

    cmdrun::commands::handle_get("timeout", Some(config_path)).await?;

    Ok(())
}

/// Test getting strict_mode configuration
#[tokio::test]
async fn test_config_get_strict_mode() -> Result<()> {
    let temp_dir = tempdir()?;
    let config_path = temp_dir.path().join("commands.toml");
    let config_content = r#"
[config]
strict_mode = true

[commands]
"#;
    fs::write(&config_path, config_content)?;

    cmdrun::commands::handle_get("strict_mode", Some(config_path)).await?;

    Ok(())
}

/// Test getting parallel configuration
#[tokio::test]
async fn test_config_get_parallel() -> Result<()> {
    let temp_dir = tempdir()?;
    let config_path = temp_dir.path().join("commands.toml");
    let config_content = r#"
[config]
parallel = false

[commands]
"#;
    fs::write(&config_path, config_content)?;

    cmdrun::commands::handle_get("parallel", Some(config_path)).await?;

    Ok(())
}

/// Test getting working_dir configuration
#[tokio::test]
async fn test_config_get_working_dir() -> Result<()> {
    let temp_dir = tempdir()?;
    let config_path = temp_dir.path().join("commands.toml");
    let config_content = r#"
[config]
working_dir = "/tmp"

[commands]
"#;
    fs::write(&config_path, config_content)?;

    cmdrun::commands::handle_get("working_dir", Some(config_path)).await?;

    Ok(())
}

/// Test error on unknown config key
#[tokio::test]
async fn test_config_get_unknown_key() -> Result<()> {
    let temp_dir = tempdir()?;
    let config_path = temp_dir.path().join("commands.toml");
    let config_content = r#"
[config]
language = "english"

[commands]
"#;
    fs::write(&config_path, config_content)?;

    let result = cmdrun::commands::handle_get("unknown_key", Some(config_path)).await;

    assert!(result.is_err());
    let err_msg = result.unwrap_err().to_string();
    assert!(
        err_msg.contains("unknown") || err_msg.contains("Unknown") || err_msg.contains("不明"),
        "Error message: {}",
        err_msg
    );

    Ok(())
}

/// Test setting language configuration
#[tokio::test]
async fn test_config_set_language() -> Result<()> {
    let temp_dir = tempdir()?;
    let config_path = temp_dir.path().join("commands.toml");
    let config_content = r#"
[config]
language = "english"

[commands]
"#;
    fs::write(&config_path, config_content)?;

    cmdrun::commands::handle_set("language", "japanese", Some(config_path.clone())).await?;

    let updated_content = fs::read_to_string(&config_path)?;
    assert!(updated_content.contains("japanese"));

    Ok(())
}

/// Test setting shell configuration
#[tokio::test]
async fn test_config_set_shell() -> Result<()> {
    let temp_dir = tempdir()?;
    let config_path = temp_dir.path().join("commands.toml");
    let config_content = r#"
[config]
shell = "bash"

[commands]
"#;
    fs::write(&config_path, config_content)?;

    cmdrun::commands::handle_set("shell", "zsh", Some(config_path.clone())).await?;

    let updated_content = fs::read_to_string(&config_path)?;
    assert!(updated_content.contains("zsh"));

    Ok(())
}

/// Test setting timeout configuration
#[tokio::test]
async fn test_config_set_timeout() -> Result<()> {
    let temp_dir = tempdir()?;
    let config_path = temp_dir.path().join("commands.toml");
    let config_content = r#"
[config]
timeout = 30

[commands]
"#;
    fs::write(&config_path, config_content)?;

    cmdrun::commands::handle_set("timeout", "120", Some(config_path.clone())).await?;

    let updated_content = fs::read_to_string(&config_path)?;
    assert!(updated_content.contains("120"));

    Ok(())
}

/// Test setting strict_mode configuration
#[tokio::test]
async fn test_config_set_strict_mode() -> Result<()> {
    let temp_dir = tempdir()?;
    let config_path = temp_dir.path().join("commands.toml");
    let config_content = r#"
[config]
strict_mode = false

[commands]
"#;
    fs::write(&config_path, config_content)?;

    cmdrun::commands::handle_set("strict_mode", "true", Some(config_path.clone())).await?;

    let updated_content = fs::read_to_string(&config_path)?;
    assert!(updated_content.contains("true"));

    Ok(())
}

/// Test setting parallel configuration
#[tokio::test]
async fn test_config_set_parallel() -> Result<()> {
    let temp_dir = tempdir()?;
    let config_path = temp_dir.path().join("commands.toml");
    let config_content = r#"
[config]
parallel = false

[commands]
"#;
    fs::write(&config_path, config_content)?;

    cmdrun::commands::handle_set("parallel", "true", Some(config_path.clone())).await?;

    let updated_content = fs::read_to_string(&config_path)?;
    assert!(updated_content.contains("true"));

    Ok(())
}

/// Test setting working_dir configuration
#[tokio::test]
async fn test_config_set_working_dir() -> Result<()> {
    let temp_dir = tempdir()?;
    let config_path = temp_dir.path().join("commands.toml");
    let config_content = r#"
[config]
working_dir = "."

[commands]
"#;
    fs::write(&config_path, config_content)?;

    cmdrun::commands::handle_set("working_dir", "/home/user", Some(config_path.clone())).await?;

    let updated_content = fs::read_to_string(&config_path)?;
    assert!(updated_content.contains("/home/user"));

    Ok(())
}

/// Test error on setting unknown config key
#[tokio::test]
async fn test_config_set_unknown_key() -> Result<()> {
    let temp_dir = tempdir()?;
    let config_path = temp_dir.path().join("commands.toml");
    let config_content = r#"
[config]
language = "english"

[commands]
"#;
    fs::write(&config_path, config_content)?;

    let result =
        cmdrun::commands::handle_set("unknown_key", "value", Some(config_path.clone())).await;

    assert!(result.is_err());

    Ok(())
}

/// Test error on invalid timeout value
#[tokio::test]
async fn test_config_set_invalid_timeout() -> Result<()> {
    let temp_dir = tempdir()?;
    let config_path = temp_dir.path().join("commands.toml");
    let config_content = r#"
[config]
timeout = 30

[commands]
"#;
    fs::write(&config_path, config_content)?;

    let result = cmdrun::commands::handle_set("timeout", "not_a_number", Some(config_path)).await;

    assert!(result.is_err());

    Ok(())
}

/// Test error on invalid boolean value for strict_mode
#[tokio::test]
async fn test_config_set_invalid_strict_mode() -> Result<()> {
    let temp_dir = tempdir()?;
    let config_path = temp_dir.path().join("commands.toml");
    let config_content = r#"
[config]
strict_mode = false

[commands]
"#;
    fs::write(&config_path, config_content)?;

    let result =
        cmdrun::commands::handle_set("strict_mode", "not_a_bool", Some(config_path)).await;

    assert!(result.is_err());

    Ok(())
}

/// Test error on invalid boolean value for parallel
#[tokio::test]
async fn test_config_set_invalid_parallel() -> Result<()> {
    let temp_dir = tempdir()?;
    let config_path = temp_dir.path().join("commands.toml");
    let config_content = r#"
[config]
parallel = false

[commands]
"#;
    fs::write(&config_path, config_content)?;

    let result = cmdrun::commands::handle_set("parallel", "maybe", Some(config_path)).await;

    assert!(result.is_err());

    Ok(())
}

/// Test showing all configuration
#[tokio::test]
async fn test_config_show_all() -> Result<()> {
    let temp_dir = tempdir()?;
    let config_path = temp_dir.path().join("commands.toml");
    let config_content = r#"
[config]
language = "english"
shell = "bash"
timeout = 30
strict_mode = true
parallel = false
working_dir = "."

[commands]
"#;
    fs::write(&config_path, config_content)?;

    cmdrun::commands::handle_show(Some(config_path)).await?;

    Ok(())
}

/// Test showing configuration with environment variables
#[tokio::test]
async fn test_config_show_with_env() -> Result<()> {
    let temp_dir = tempdir()?;
    let config_path = temp_dir.path().join("commands.toml");
    let config_content = r#"
[config]
language = "english"

[config.env]
FOO = "bar"
BAZ = "qux"

[commands]
"#;
    fs::write(&config_path, config_content)?;

    cmdrun::commands::handle_show(Some(config_path)).await?;

    Ok(())
}

/// Test showing minimal configuration
#[tokio::test]
async fn test_config_show_minimal() -> Result<()> {
    let temp_dir = tempdir()?;
    let config_path = temp_dir.path().join("commands.toml");
    let config_content = r#"
[commands]
"#;
    fs::write(&config_path, config_content)?;

    cmdrun::commands::handle_show(Some(config_path)).await?;

    Ok(())
}

/// Test setting preserves other config values
#[tokio::test]
async fn test_config_set_preserves_other_values() -> Result<()> {
    let temp_dir = tempdir()?;
    let config_path = temp_dir.path().join("commands.toml");
    let config_content = r#"
[config]
language = "english"
shell = "bash"
timeout = 30

[commands]
"#;
    fs::write(&config_path, config_content)?;

    cmdrun::commands::handle_set("timeout", "60", Some(config_path.clone())).await?;

    let updated_content = fs::read_to_string(&config_path)?;
    assert!(updated_content.contains("english"));
    assert!(updated_content.contains("bash"));
    assert!(updated_content.contains("60"));

    Ok(())
}

/// Test setting multiple values sequentially
#[tokio::test]
async fn test_config_set_multiple_values() -> Result<()> {
    let temp_dir = tempdir()?;
    let config_path = temp_dir.path().join("commands.toml");
    let config_content = r#"
[config]
language = "english"
shell = "bash"
timeout = 30

[commands]
"#;
    fs::write(&config_path, config_content)?;

    cmdrun::commands::handle_set("language", "japanese", Some(config_path.clone())).await?;
    cmdrun::commands::handle_set("shell", "zsh", Some(config_path.clone())).await?;
    cmdrun::commands::handle_set("timeout", "120", Some(config_path.clone())).await?;

    let updated_content = fs::read_to_string(&config_path)?;
    assert!(updated_content.contains("japanese"));
    assert!(updated_content.contains("zsh"));
    assert!(updated_content.contains("120"));

    Ok(())
}

/// Test setting value with special characters
#[tokio::test]
async fn test_config_set_special_characters() -> Result<()> {
    let temp_dir = tempdir()?;
    let config_path = temp_dir.path().join("commands.toml");
    let config_content = r#"
[config]
working_dir = "."

[commands]
"#;
    fs::write(&config_path, config_content)?;

    cmdrun::commands::handle_set(
        "working_dir",
        "/path/with spaces/and-dashes",
        Some(config_path.clone()),
    )
    .await?;

    let updated_content = fs::read_to_string(&config_path)?;
    assert!(updated_content.contains("with spaces"));

    Ok(())
}

/// Test config operations with default config (no path specified)
#[tokio::test]
async fn test_config_get_default_path() -> Result<()> {
    // This test may fail if no default config exists, which is expected
    let result = cmdrun::commands::handle_get("language", None).await;

    // Either succeeds or fails with config not found - both are valid
    match result {
        Ok(_) => Ok(()),
        Err(e) => {
            let err_msg = e.to_string();
            assert!(
                err_msg.contains("not found") || err_msg.contains("見つかりません"),
                "Unexpected error: {}",
                err_msg
            );
            Ok(())
        }
    }
}
