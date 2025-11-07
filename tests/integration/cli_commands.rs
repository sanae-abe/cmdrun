//! Integration tests for CLI commands
//!
//! Tests the main CLI entry points through command execution

use anyhow::Result;
use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::{tempdir, TempDir};

/// Helper to create a test config
fn create_test_config(dir: &TempDir, content: &str) -> std::path::PathBuf {
    let config_path = dir.path().join("commands.toml");
    fs::write(&config_path, content).unwrap();
    config_path
}

#[test]
fn test_cli_list_empty_config() -> Result<()> {
    let temp_dir = tempdir()?;
    let config = create_test_config(&temp_dir, "[commands]\n");

    let mut cmd = Command::cargo_bin("cmdrun")?;
    cmd.arg("--config")
        .arg(config)
        .arg("list")
        .assert()
        .success()
        .stdout(predicate::str::contains("No commands defined"));

    Ok(())
}

#[test]
fn test_cli_list_with_commands() -> Result<()> {
    let temp_dir = tempdir()?;
    let config_content = r#"
[commands]
test = { description = "Test command", cmd = "echo test" }
build = { description = "Build project", cmd = "cargo build" }
"#;
    let config = create_test_config(&temp_dir, config_content);

    let mut cmd = Command::cargo_bin("cmdrun")?;
    cmd.arg("--config")
        .arg(config)
        .arg("list")
        .assert()
        .success()
        .stdout(predicate::str::contains("Available commands"))
        .stdout(predicate::str::contains("test"))
        .stdout(predicate::str::contains("build"));

    Ok(())
}

#[test]
fn test_cli_list_verbose() -> Result<()> {
    let temp_dir = tempdir()?;
    let config_content = r#"
[commands]
test = { description = "Test command", cmd = "echo test", deps = ["dep1"] }
"#;
    let config = create_test_config(&temp_dir, config_content);

    let mut cmd = Command::cargo_bin("cmdrun")?;
    cmd.arg("--config")
        .arg(config)
        .arg("list")
        .arg("--verbose")
        .assert()
        .success()
        .stdout(predicate::str::contains("Command:"))
        .stdout(predicate::str::contains("Dependencies:"));

    Ok(())
}

#[test]
fn test_cli_run_simple_command() -> Result<()> {
    let temp_dir = tempdir()?;
    let config_content = r#"
[commands]
hello = { description = "Say hello", cmd = "echo 'Hello, World!'" }
"#;
    let config = create_test_config(&temp_dir, config_content);

    let mut cmd = Command::cargo_bin("cmdrun")?;
    cmd.arg("--config")
        .arg(config)
        .arg("run")
        .arg("hello")
        .assert()
        .success()
        .stdout(predicate::str::contains("Running:"));

    Ok(())
}

#[test]
fn test_cli_run_nonexistent_command() -> Result<()> {
    let temp_dir = tempdir()?;
    let config = create_test_config(&temp_dir, "[commands]\n");

    let mut cmd = Command::cargo_bin("cmdrun")?;
    cmd.arg("--config")
        .arg(config)
        .arg("run")
        .arg("nonexistent")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Command not found"));

    Ok(())
}

#[test]
fn test_cli_run_with_positional_args() -> Result<()> {
    let temp_dir = tempdir()?;
    let config_content = r#"
[commands]
greet = { description = "Greet user", cmd = "echo Hello ${1}" }
"#;
    let config = create_test_config(&temp_dir, config_content);

    let mut cmd = Command::cargo_bin("cmdrun")?;
    cmd.arg("--config")
        .arg(config)
        .arg("run")
        .arg("greet")
        .arg("--")
        .arg("Alice")
        .assert()
        .success();

    Ok(())
}

#[test]
fn test_cli_completion_bash() -> Result<()> {
    let mut cmd = Command::cargo_bin("cmdrun")?;
    cmd.arg("completion")
        .arg("bash")
        .assert()
        .success()
        .stdout(predicate::str::contains("complete"));

    Ok(())
}

#[test]
fn test_cli_completion_zsh() -> Result<()> {
    let mut cmd = Command::cargo_bin("cmdrun")?;
    cmd.arg("completion")
        .arg("zsh")
        .assert()
        .success()
        .stdout(predicate::str::contains("compdef"));

    Ok(())
}

#[test]
fn test_cli_completion_fish() -> Result<()> {
    let mut cmd = Command::cargo_bin("cmdrun")?;
    cmd.arg("completion")
        .arg("fish")
        .assert()
        .success()
        .stdout(predicate::str::contains("complete"));

    Ok(())
}

#[test]
fn test_cli_completion_list() -> Result<()> {
    let temp_dir = tempdir()?;
    let config_content = r#"
[commands]
test = { description = "Test", cmd = "echo test" }
build = { description = "Build", cmd = "cargo build" }
"#;
    let config = create_test_config(&temp_dir, config_content);

    let mut cmd = Command::cargo_bin("cmdrun")?;
    cmd.arg("--config")
        .arg(config)
        .arg("completion-list")
        .assert()
        .success()
        .stdout(predicate::str::contains("test"))
        .stdout(predicate::str::contains("build"));

    Ok(())
}

#[test]
fn test_cli_graph_tree_format() -> Result<()> {
    let temp_dir = tempdir()?;
    let config_content = r#"
[commands]
test = { description = "Test", cmd = "echo test" }
build = { description = "Build", cmd = "cargo build", deps = ["test"] }
"#;
    let config = create_test_config(&temp_dir, config_content);

    let mut cmd = Command::cargo_bin("cmdrun")?;
    cmd.arg("--config")
        .arg(config)
        .arg("graph")
        .arg("--format")
        .arg("tree")
        .assert()
        .success()
        .stdout(predicate::str::contains("test"))
        .stdout(predicate::str::contains("build"));

    Ok(())
}

#[test]
fn test_cli_graph_specific_command() -> Result<()> {
    let temp_dir = tempdir()?;
    let config_content = r#"
[commands]
test = { description = "Test", cmd = "echo test" }
build = { description = "Build", cmd = "cargo build", deps = ["test"] }
deploy = { description = "Deploy", cmd = "echo deploy", deps = ["build"] }
"#;
    let config = create_test_config(&temp_dir, config_content);

    let mut cmd = Command::cargo_bin("cmdrun")?;
    cmd.arg("--config")
        .arg(config)
        .arg("graph")
        .arg("--command")
        .arg("deploy")
        .arg("--format")
        .arg("tree")
        .assert()
        .success()
        .stdout(predicate::str::contains("deploy"));

    Ok(())
}

#[test]
fn test_cli_init_default() -> Result<()> {
    let temp_dir = tempdir()?;
    std::env::set_current_dir(temp_dir.path())?;

    let mut cmd = Command::cargo_bin("cmdrun")?;
    cmd.arg("init")
        .arg("--output")
        .arg(temp_dir.path().join("commands.toml"))
        .assert()
        .success();

    assert!(temp_dir.path().join("commands.toml").exists());
    Ok(())
}

#[test]
fn test_cli_init_with_template() -> Result<()> {
    let temp_dir = tempdir()?;
    std::env::set_current_dir(temp_dir.path())?;

    let mut cmd = Command::cargo_bin("cmdrun")?;
    cmd.arg("init")
        .arg("--template")
        .arg("rust")
        .arg("--output")
        .arg(temp_dir.path().join("commands.toml"))
        .assert()
        .success();

    let content = fs::read_to_string(temp_dir.path().join("commands.toml"))?;
    assert!(content.contains("test"));
    assert!(content.contains("build"));
    Ok(())
}

#[test]
fn test_cli_validate_valid_config() -> Result<()> {
    let temp_dir = tempdir()?;
    let config_content = r#"
[commands]
test = { description = "Test", cmd = "echo test" }
build = { description = "Build", cmd = "cargo build", deps = ["test"] }
"#;
    let config = create_test_config(&temp_dir, config_content);

    let mut cmd = Command::cargo_bin("cmdrun")?;
    cmd.arg("validate")
        .arg("--path")
        .arg(config)
        .assert()
        .success()
        .stdout(predicate::str::contains("valid"));

    Ok(())
}

#[test]
fn test_cli_validate_circular_dependency() -> Result<()> {
    let temp_dir = tempdir()?;
    let config_content = r#"
[commands]
a = { description = "A", cmd = "echo a", deps = ["b"] }
b = { description = "B", cmd = "echo b", deps = ["a"] }
"#;
    let config = create_test_config(&temp_dir, config_content);

    let mut cmd = Command::cargo_bin("cmdrun")?;
    cmd.arg("validate")
        .arg("--path")
        .arg(config)
        .arg("--check-cycles")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Circular"));

    Ok(())
}

#[test]
fn test_cli_validate_verbose() -> Result<()> {
    let temp_dir = tempdir()?;
    let config_content = r#"
[commands]
test = { description = "Test", cmd = "echo test" }
"#;
    let config = create_test_config(&temp_dir, config_content);

    let mut cmd = Command::cargo_bin("cmdrun")?;
    cmd.arg("validate")
        .arg("--path")
        .arg(config)
        .arg("--verbose")
        .assert()
        .success()
        .stdout(predicate::str::contains("test"));

    Ok(())
}

#[test]
fn test_cli_run_failed_command() -> Result<()> {
    let temp_dir = tempdir()?;
    let config_content = r#"
[commands]
fail = { description = "Failing command", cmd = "false" }
"#;
    let config = create_test_config(&temp_dir, config_content);

    let mut cmd = Command::cargo_bin("cmdrun")?;
    cmd.arg("--config")
        .arg(config)
        .arg("run")
        .arg("fail")
        .assert()
        .failure()
        .stderr(predicate::str::contains("failed"));

    Ok(())
}

#[test]
fn test_cli_help() -> Result<()> {
    let mut cmd = Command::cargo_bin("cmdrun")?;
    cmd.arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("cmdrun"))
        .stdout(predicate::str::contains("USAGE"));

    Ok(())
}

#[test]
fn test_cli_version() -> Result<()> {
    let mut cmd = Command::cargo_bin("cmdrun")?;
    cmd.arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains(env!("CARGO_PKG_VERSION")));

    Ok(())
}

#[test]
fn test_cli_run_with_dependencies() -> Result<()> {
    let temp_dir = tempdir()?;
    let config_content = r#"
[commands]
clean = { description = "Clean", cmd = "echo cleaning" }
build = { description = "Build", cmd = "echo building", deps = ["clean"] }
"#;
    let config = create_test_config(&temp_dir, config_content);

    let mut cmd = Command::cargo_bin("cmdrun")?;
    cmd.arg("--config")
        .arg(config)
        .arg("run")
        .arg("build")
        .assert()
        .success();

    Ok(())
}

#[test]
fn test_cli_run_parallel() -> Result<()> {
    let temp_dir = tempdir()?;
    let config_content = r#"
[commands]
task1 = { description = "Task 1", cmd = "echo task1" }
task2 = { description = "Task 2", cmd = "echo task2" }
all = { description = "All tasks", cmd = "echo all", deps = ["task1", "task2"], parallel = true }
"#;
    let config = create_test_config(&temp_dir, config_content);

    let mut cmd = Command::cargo_bin("cmdrun")?;
    cmd.arg("--config")
        .arg(config)
        .arg("run")
        .arg("all")
        .arg("--parallel")
        .assert()
        .success()
        .stdout(predicate::str::contains("parallel dependencies"));

    Ok(())
}

#[test]
fn test_cli_verbose_flag() -> Result<()> {
    let temp_dir = tempdir()?;
    let config = create_test_config(&temp_dir, "[commands]\n");

    let mut cmd = Command::cargo_bin("cmdrun")?;
    cmd.arg("--config")
        .arg(config)
        .arg("-v")
        .arg("list")
        .assert()
        .success();

    Ok(())
}

#[test]
fn test_cli_multiple_verbose_flags() -> Result<()> {
    let temp_dir = tempdir()?;
    let config = create_test_config(&temp_dir, "[commands]\n");

    let mut cmd = Command::cargo_bin("cmdrun")?;
    cmd.arg("--config")
        .arg(config)
        .arg("-vv")
        .arg("list")
        .assert()
        .success();

    Ok(())
}
