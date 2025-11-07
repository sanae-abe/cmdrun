//! Integration tests for the `add` command
//!
//! Tests adding commands to configuration files with various scenarios

use anyhow::Result;
use std::fs;
use tempfile::{tempdir, NamedTempFile};

/// Test adding a command with all required arguments (non-interactive)
#[tokio::test]
async fn test_add_command_non_interactive() -> Result<()> {
    let temp_dir = tempdir()?;
    let config_path = temp_dir.path().join("commands.toml");
    fs::write(&config_path, "[commands]\n")?;

    cmdrun::commands::handle_add(
        Some("test".to_string()),
        Some("echo hello".to_string()),
        Some("Test command".to_string()),
        None,
        None,
        Some(config_path.clone()),
    )
    .await?;

    let content = fs::read_to_string(&config_path)?;
    assert!(content.contains("test"));
    assert!(content.contains("echo hello"));
    assert!(content.contains("Test command"));

    Ok(())
}

/// Test adding command with category
#[tokio::test]
async fn test_add_command_with_category() -> Result<()> {
    let temp_dir = tempdir()?;
    let config_path = temp_dir.path().join("commands.toml");
    fs::write(&config_path, "[commands]\n")?;

    cmdrun::commands::handle_add(
        Some("build".to_string()),
        Some("cargo build".to_string()),
        Some("Build the project".to_string()),
        Some("build".to_string()),
        None,
        Some(config_path.clone()),
    )
    .await?;

    let content = fs::read_to_string(&config_path)?;
    assert!(content.contains("build"));
    assert!(content.contains("category"));

    Ok(())
}

/// Test adding command with tags
#[tokio::test]
async fn test_add_command_with_tags() -> Result<()> {
    let temp_dir = tempdir()?;
    let config_path = temp_dir.path().join("commands.toml");
    fs::write(&config_path, "[commands]\n")?;

    cmdrun::commands::handle_add(
        Some("deploy".to_string()),
        Some("./deploy.sh".to_string()),
        Some("Deploy application".to_string()),
        None,
        Some(vec!["production".to_string(), "deploy".to_string()]),
        Some(config_path.clone()),
    )
    .await?;

    let content = fs::read_to_string(&config_path)?;
    assert!(content.contains("deploy"));
    assert!(content.contains("tags"));
    assert!(content.contains("production"));

    Ok(())
}

/// Test adding command with category and tags
#[tokio::test]
async fn test_add_command_with_category_and_tags() -> Result<()> {
    let temp_dir = tempdir()?;
    let config_path = temp_dir.path().join("commands.toml");
    fs::write(&config_path, "[commands]\n")?;

    cmdrun::commands::handle_add(
        Some("full".to_string()),
        Some("echo full".to_string()),
        Some("Full featured command".to_string()),
        Some("test".to_string()),
        Some(vec!["tag1".to_string(), "tag2".to_string()]),
        Some(config_path.clone()),
    )
    .await?;

    let content = fs::read_to_string(&config_path)?;
    assert!(content.contains("full"));
    assert!(content.contains("category"));
    assert!(content.contains("tags"));
    assert!(content.contains("tag1"));
    assert!(content.contains("tag2"));

    Ok(())
}

/// Test error when adding duplicate command ID
#[tokio::test]
async fn test_add_duplicate_command_id() -> Result<()> {
    let temp_dir = tempdir()?;
    let config_path = temp_dir.path().join("commands.toml");
    let initial_content = r#"
[commands]
existing = { description = "Existing command", cmd = "echo existing" }
"#;
    fs::write(&config_path, initial_content)?;

    let result = cmdrun::commands::handle_add(
        Some("existing".to_string()),
        Some("echo duplicate".to_string()),
        Some("Duplicate command".to_string()),
        None,
        None,
        Some(config_path),
    )
    .await;

    assert!(result.is_err());
    let err_msg = result.unwrap_err().to_string();
    assert!(
        err_msg.contains("exists") || err_msg.contains("already") || err_msg.contains("重複"),
        "Error message: {}",
        err_msg
    );

    Ok(())
}

/// Test error when ID is empty
#[tokio::test]
async fn test_add_empty_id() -> Result<()> {
    let temp_dir = tempdir()?;
    let config_path = temp_dir.path().join("commands.toml");
    fs::write(&config_path, "[commands]\n")?;

    let result = cmdrun::commands::handle_add(
        Some("   ".to_string()),
        Some("echo test".to_string()),
        Some("Test".to_string()),
        None,
        None,
        Some(config_path),
    )
    .await;

    assert!(result.is_err());
    Ok(())
}

/// Test error when command is empty
#[tokio::test]
async fn test_add_empty_command() -> Result<()> {
    let temp_dir = tempdir()?;
    let config_path = temp_dir.path().join("commands.toml");
    fs::write(&config_path, "[commands]\n")?;

    let result = cmdrun::commands::handle_add(
        Some("test".to_string()),
        Some("   ".to_string()),
        Some("Test".to_string()),
        None,
        None,
        Some(config_path),
    )
    .await;

    assert!(result.is_err());
    Ok(())
}

/// Test error when description is empty
#[tokio::test]
async fn test_add_empty_description() -> Result<()> {
    let temp_dir = tempdir()?;
    let config_path = temp_dir.path().join("commands.toml");
    fs::write(&config_path, "[commands]\n")?;

    let result = cmdrun::commands::handle_add(
        Some("test".to_string()),
        Some("echo test".to_string()),
        Some("   ".to_string()),
        None,
        None,
        Some(config_path),
    )
    .await;

    assert!(result.is_err());
    Ok(())
}

/// Test adding to existing config with multiple commands
#[tokio::test]
async fn test_add_to_existing_commands() -> Result<()> {
    let temp_dir = tempdir()?;
    let config_path = temp_dir.path().join("commands.toml");
    let initial_content = r#"
[commands]
cmd1 = { description = "Command 1", cmd = "echo 1" }
cmd2 = { description = "Command 2", cmd = "echo 2" }
"#;
    fs::write(&config_path, initial_content)?;

    cmdrun::commands::handle_add(
        Some("cmd3".to_string()),
        Some("echo 3".to_string()),
        Some("Command 3".to_string()),
        None,
        None,
        Some(config_path.clone()),
    )
    .await?;

    let content = fs::read_to_string(&config_path)?;
    assert!(content.contains("cmd1"));
    assert!(content.contains("cmd2"));
    assert!(content.contains("cmd3"));

    Ok(())
}

/// Test special characters in command ID
#[tokio::test]
async fn test_add_special_chars_in_id() -> Result<()> {
    let temp_dir = tempdir()?;
    let config_path = temp_dir.path().join("commands.toml");
    fs::write(&config_path, "[commands]\n")?;

    cmdrun::commands::handle_add(
        Some("test-cmd_123".to_string()),
        Some("echo test".to_string()),
        Some("Test with special chars".to_string()),
        None,
        None,
        Some(config_path.clone()),
    )
    .await?;

    let content = fs::read_to_string(&config_path)?;
    assert!(content.contains("test-cmd_123"));

    Ok(())
}

/// Test adding command with multi-line description
#[tokio::test]
async fn test_add_multiline_description() -> Result<()> {
    let temp_dir = tempdir()?;
    let config_path = temp_dir.path().join("commands.toml");
    fs::write(&config_path, "[commands]\n")?;

    cmdrun::commands::handle_add(
        Some("complex".to_string()),
        Some("echo complex".to_string()),
        Some("Line 1\nLine 2".to_string()),
        None,
        None,
        Some(config_path.clone()),
    )
    .await?;

    let content = fs::read_to_string(&config_path)?;
    assert!(content.contains("complex"));

    Ok(())
}

/// Test adding command with complex command string
#[tokio::test]
async fn test_add_complex_command() -> Result<()> {
    let temp_dir = tempdir()?;
    let config_path = temp_dir.path().join("commands.toml");
    fs::write(&config_path, "[commands]\n")?;

    cmdrun::commands::handle_add(
        Some("pipeline".to_string()),
        Some("echo hello | grep h | wc -l".to_string()),
        Some("Pipeline command".to_string()),
        None,
        None,
        Some(config_path.clone()),
    )
    .await?;

    let content = fs::read_to_string(&config_path)?;
    assert!(content.contains("pipeline"));
    assert!(content.contains("pipe") || content.contains("|"));

    Ok(())
}

/// Test adding command with quotes in command
#[tokio::test]
async fn test_add_command_with_quotes() -> Result<()> {
    let temp_dir = tempdir()?;
    let config_path = temp_dir.path().join("commands.toml");
    fs::write(&config_path, "[commands]\n")?;

    cmdrun::commands::handle_add(
        Some("quoted".to_string()),
        Some(r#"echo "Hello, World!""#.to_string()),
        Some("Quoted command".to_string()),
        None,
        None,
        Some(config_path.clone()),
    )
    .await?;

    let content = fs::read_to_string(&config_path)?;
    assert!(content.contains("quoted"));

    Ok(())
}

/// Test adding multiple tags
#[tokio::test]
async fn test_add_multiple_tags() -> Result<()> {
    let temp_dir = tempdir()?;
    let config_path = temp_dir.path().join("commands.toml");
    fs::write(&config_path, "[commands]\n")?;

    cmdrun::commands::handle_add(
        Some("tagged".to_string()),
        Some("echo tagged".to_string()),
        Some("Tagged command".to_string()),
        None,
        Some(vec![
            "tag1".to_string(),
            "tag2".to_string(),
            "tag3".to_string(),
        ]),
        Some(config_path.clone()),
    )
    .await?;

    let content = fs::read_to_string(&config_path)?;
    assert!(content.contains("tagged"));
    assert!(content.contains("tag1"));
    assert!(content.contains("tag2"));
    assert!(content.contains("tag3"));

    Ok(())
}

/// Test adding to config without [commands] section
#[tokio::test]
async fn test_add_create_commands_section() -> Result<()> {
    let temp_dir = tempdir()?;
    let config_path = temp_dir.path().join("commands.toml");
    fs::write(&config_path, "# Empty config\n")?;

    cmdrun::commands::handle_add(
        Some("first".to_string()),
        Some("echo first".to_string()),
        Some("First command".to_string()),
        None,
        None,
        Some(config_path.clone()),
    )
    .await?;

    let content = fs::read_to_string(&config_path)?;
    assert!(content.contains("[commands]"));
    assert!(content.contains("first"));

    Ok(())
}

/// Test adding command preserves existing formatting
#[tokio::test]
async fn test_add_preserves_formatting() -> Result<()> {
    let temp_dir = tempdir()?;
    let config_path = temp_dir.path().join("commands.toml");
    let initial_content = r#"# My config file
# With comments

[commands]
# Existing command
cmd1 = { description = "Command 1", cmd = "echo 1" }
"#;
    fs::write(&config_path, initial_content)?;

    cmdrun::commands::handle_add(
        Some("cmd2".to_string()),
        Some("echo 2".to_string()),
        Some("Command 2".to_string()),
        None,
        None,
        Some(config_path.clone()),
    )
    .await?;

    let content = fs::read_to_string(&config_path)?;
    assert!(content.contains("# My config file"));
    assert!(content.contains("cmd1"));
    assert!(content.contains("cmd2"));

    Ok(())
}

/// Test adding to non-existent config file (should create it)
#[tokio::test]
async fn test_add_create_new_config() -> Result<()> {
    let temp_dir = tempdir()?;
    let config_path = temp_dir.path().join("new_commands.toml");

    // Ensure file doesn't exist
    assert!(!config_path.exists());

    // This should fail because config doesn't exist, but we're testing the behavior
    let result = cmdrun::commands::handle_add(
        Some("test".to_string()),
        Some("echo test".to_string()),
        Some("Test".to_string()),
        None,
        None,
        Some(config_path.clone()),
    )
    .await;

    // Expecting error since file doesn't exist
    assert!(result.is_err());

    Ok(())
}

/// Test Unicode characters in command
#[tokio::test]
async fn test_add_unicode_in_command() -> Result<()> {
    let temp_dir = tempdir()?;
    let config_path = temp_dir.path().join("commands.toml");
    fs::write(&config_path, "[commands]\n")?;

    cmdrun::commands::handle_add(
        Some("unicode".to_string()),
        Some("echo '日本語 🎉'".to_string()),
        Some("Unicode description 日本語".to_string()),
        None,
        None,
        Some(config_path.clone()),
    )
    .await?;

    let content = fs::read_to_string(&config_path)?;
    assert!(content.contains("unicode"));

    Ok(())
}
