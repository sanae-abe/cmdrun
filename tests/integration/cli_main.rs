//! Integration tests for main.rs CLI functionality
//!
//! Tests main entry point and primary command flows:
//! - Exit code validation
//! - Help messages
//! - Error handling
//! - Color output configuration
//! - Logging initialization
//!
//! Coverage target: 50% for main.rs (currently 12.8%)

use std::process::Command;
use tempfile::TempDir;

/// Test helper to run cmdrun binary
fn run_cmdrun(args: &[&str]) -> Result<std::process::Output, std::io::Error> {
    let mut cmd_args = vec!["run", "--bin", "cmdrun", "--"];
    cmd_args.extend_from_slice(args);

    Command::new("cargo").args(&cmd_args).output()
}

/// Test helper to check exit code
fn assert_exit_code(output: &std::process::Output, expected: i32) {
    let actual = output.status.code().unwrap_or(-1);
    assert_eq!(
        actual,
        expected,
        "Expected exit code {}, but got {}. stderr: {}",
        expected,
        actual,
        String::from_utf8_lossy(&output.stderr)
    );
}

// ============================================================================
// Help and Version Tests
// ============================================================================

#[test]
fn test_help_message() {
    let output = run_cmdrun(&["--help"]).expect("Failed to run cmdrun --help");

    assert_exit_code(&output, 0);

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("cmdrun"));
    assert!(stdout.contains("modern replacement") || stdout.contains("high-performance"));
    assert!(stdout.contains("Usage:") || stdout.contains("USAGE:"));
}

#[test]
fn test_version_flag() {
    let output = run_cmdrun(&["--version"]).expect("Failed to run cmdrun --version");

    assert_exit_code(&output, 0);

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("cmdrun"));
    // Version number should be present
    assert!(stdout.contains(char::is_numeric));
}

#[test]
fn test_help_for_subcommands() {
    let subcommands = vec!["run", "list", "add", "remove", "init", "search"];

    for subcmd in subcommands {
        let output = run_cmdrun(&[subcmd, "--help"])
            .unwrap_or_else(|_| panic!("Failed to run cmdrun {} --help", subcmd));

        assert_exit_code(&output, 0);

        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(
            stdout.len() > 50,
            "Help message for {} should be descriptive",
            subcmd
        );
    }
}

// ============================================================================
// Exit Code Tests
// ============================================================================

#[test]
fn test_exit_code_on_invalid_subcommand() {
    let output = run_cmdrun(&["invalid-subcommand-12345"])
        .expect("Failed to run cmdrun with invalid subcommand");

    // Should exit with error
    assert!(!output.status.success());
}

#[test]
fn test_exit_code_on_missing_config() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");

    let output = Command::new("cargo")
        .args([
            "run",
            "--bin",
            "cmdrun",
            "--",
            "--config",
            temp_dir.path().join("nonexistent.toml").to_str().unwrap(),
            "list",
        ])
        .output()
        .expect("Failed to run command");

    // Should exit with error when config file doesn't exist
    assert!(!output.status.success());
}

#[test]
fn test_exit_code_on_successful_list() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let config_path = temp_dir.path().join("commands.toml");

    // Create minimal config
    std::fs::write(
        &config_path,
        r#"
[commands.test]
description = "Test command"
cmd = "echo test"
"#,
    )
    .expect("Failed to write config");

    let output = Command::new("cargo")
        .args([
            "run",
            "--bin",
            "cmdrun",
            "--",
            "--config",
            config_path.to_str().unwrap(),
            "list",
        ])
        .output()
        .expect("Failed to run command");

    assert_exit_code(&output, 0);
}

// ============================================================================
// Color Output Tests
// ============================================================================

#[test]
fn test_color_never_flag() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let config_path = temp_dir.path().join("commands.toml");

    std::fs::write(
        &config_path,
        r#"
[commands.test]
description = "Test"
cmd = "echo test"
"#,
    )
    .expect("Failed to write config");

    let output = Command::new("cargo")
        .args([
            "run",
            "--bin",
            "cmdrun",
            "--",
            "--color",
            "never",
            "--config",
            config_path.to_str().unwrap(),
            "list",
        ])
        .output()
        .expect("Failed to run command");

    assert_exit_code(&output, 0);

    // Output should not contain ANSI escape codes
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        !stdout.contains("\x1b["),
        "Output with --color never should not contain ANSI codes"
    );
}

#[test]
fn test_color_always_flag() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let config_path = temp_dir.path().join("commands.toml");

    std::fs::write(
        &config_path,
        r#"
[commands.test]
description = "Test"
cmd = "echo test"
"#,
    )
    .expect("Failed to write config");

    let output = Command::new("cargo")
        .args([
            "run",
            "--bin",
            "cmdrun",
            "--",
            "--color",
            "always",
            "--config",
            config_path.to_str().unwrap(),
            "list",
        ])
        .output()
        .expect("Failed to run command");

    assert_exit_code(&output, 0);

    // Output should contain ANSI escape codes
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("\x1b["),
        "Output with --color always should contain ANSI codes"
    );
}

// ============================================================================
// List Command Tests
// ============================================================================

#[test]
fn test_list_with_empty_config() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let config_path = temp_dir.path().join("commands.toml");

    // Create empty commands section
    std::fs::write(&config_path, "[commands]\n").expect("Failed to write config");

    let output = Command::new("cargo")
        .args([
            "run",
            "--bin",
            "cmdrun",
            "--",
            "--config",
            config_path.to_str().unwrap(),
            "list",
        ])
        .output()
        .expect("Failed to run command");

    assert_exit_code(&output, 0);

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("No commands") || stdout.len() < 50);
}

#[test]
fn test_list_with_commands() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let config_path = temp_dir.path().join("commands.toml");

    std::fs::write(
        &config_path,
        r#"
[commands.build]
description = "Build the project"
cmd = "cargo build"

[commands.test]
description = "Run tests"
cmd = "cargo test"
"#,
    )
    .expect("Failed to write config");

    let output = Command::new("cargo")
        .args([
            "run",
            "--bin",
            "cmdrun",
            "--",
            "--config",
            config_path.to_str().unwrap(),
            "list",
        ])
        .output()
        .expect("Failed to run command");

    assert_exit_code(&output, 0);

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("build"));
    assert!(stdout.contains("test"));
    assert!(stdout.contains("Build the project"));
    assert!(stdout.contains("Run tests"));
}

#[test]
fn test_list_verbose() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let config_path = temp_dir.path().join("commands.toml");

    std::fs::write(
        &config_path,
        r#"
[commands.build]
description = "Build the project"
cmd = "cargo build"
deps = ["clean"]

[commands.clean]
description = "Clean build artifacts"
cmd = "cargo clean"
"#,
    )
    .expect("Failed to write config");

    let output = Command::new("cargo")
        .args([
            "run",
            "--bin",
            "cmdrun",
            "--",
            "--config",
            config_path.to_str().unwrap(),
            "list",
            "--verbose",
        ])
        .output()
        .expect("Failed to run command");

    assert_exit_code(&output, 0);

    let stdout = String::from_utf8_lossy(&output.stdout);
    // Verbose mode should show dependencies
    assert!(stdout.contains("clean") || stdout.contains("Dependencies"));
}

// ============================================================================
// Completion List Tests
// ============================================================================

#[test]
fn test_completion_list_with_commands() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let config_path = temp_dir.path().join("commands.toml");

    std::fs::write(
        &config_path,
        r#"
[commands.build]
description = "Build the project"
cmd = "cargo build"

[commands.test]
description = "Run tests"
cmd = "cargo test"
"#,
    )
    .expect("Failed to write config");

    let output = Command::new("cargo")
        .args([
            "run",
            "--bin",
            "cmdrun",
            "--",
            "--config",
            config_path.to_str().unwrap(),
            "completion-list",
        ])
        .output()
        .expect("Failed to run command");

    assert_exit_code(&output, 0);

    let stdout = String::from_utf8_lossy(&output.stdout);
    // Format: "name:description"
    assert!(stdout.contains("build:Build the project"));
    assert!(stdout.contains("test:Run tests"));
}

#[test]
fn test_completion_list_with_no_config() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let nonexistent = temp_dir.path().join("nonexistent.toml");

    let output = Command::new("cargo")
        .args([
            "run",
            "--bin",
            "cmdrun",
            "--",
            "--config",
            nonexistent.to_str().unwrap(),
            "completion-list",
        ])
        .output()
        .expect("Failed to run command");

    // Should fail when config doesn't exist
    assert!(!output.status.success());

    let stderr = String::from_utf8_lossy(&output.stderr);
    // Should show error about config
    assert!(stderr.contains("Error") || stderr.contains("error"));
}

// ============================================================================
// Error Message Tests
// ============================================================================

#[test]
fn test_command_not_found_error() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let config_path = temp_dir.path().join("commands.toml");

    std::fs::write(
        &config_path,
        r#"
[commands.build]
description = "Build"
cmd = "echo build"
"#,
    )
    .expect("Failed to write config");

    let output = Command::new("cargo")
        .args([
            "run",
            "--bin",
            "cmdrun",
            "--",
            "--config",
            config_path.to_str().unwrap(),
            "run",
            "nonexistent",
        ])
        .output()
        .expect("Failed to run command");

    assert!(!output.status.success());

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("not found") || stderr.contains("Command not found"));
}

#[test]
fn test_typo_detection_suggestions() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let config_path = temp_dir.path().join("commands.toml");

    std::fs::write(
        &config_path,
        r#"
[config]
typo_detection = true
typo_threshold = 0.7

[commands.build]
description = "Build"
cmd = "echo build"

[commands.test]
description = "Test"
cmd = "echo test"
"#,
    )
    .expect("Failed to write config");

    let output = Command::new("cargo")
        .args([
            "run",
            "--bin",
            "cmdrun",
            "--",
            "--config",
            config_path.to_str().unwrap(),
            "run",
            "buld", // Typo: should suggest "build"
        ])
        .output()
        .expect("Failed to run command");

    assert!(!output.status.success());

    let stderr = String::from_utf8_lossy(&output.stderr);
    // Should show suggestions
    assert!(stderr.contains("build") || stderr.contains("Did you mean"));
}

// ============================================================================
// Verbose Flag Tests
// ============================================================================

#[test]
fn test_verbose_flag() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let config_path = temp_dir.path().join("commands.toml");

    std::fs::write(
        &config_path,
        r#"
[commands.test]
description = "Test"
cmd = "echo test"
"#,
    )
    .expect("Failed to write config");

    let output = Command::new("cargo")
        .args([
            "run",
            "--bin",
            "cmdrun",
            "--",
            "-v",
            "--config",
            config_path.to_str().unwrap(),
            "list",
        ])
        .output()
        .expect("Failed to run command");

    assert_exit_code(&output, 0);
}

#[test]
fn test_very_verbose_flag() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let config_path = temp_dir.path().join("commands.toml");

    std::fs::write(
        &config_path,
        r#"
[commands.test]
description = "Test"
cmd = "echo test"
"#,
    )
    .expect("Failed to write config");

    let output = Command::new("cargo")
        .args([
            "run",
            "--bin",
            "cmdrun",
            "--",
            "-vv",
            "--config",
            config_path.to_str().unwrap(),
            "list",
        ])
        .output()
        .expect("Failed to run command");

    assert_exit_code(&output, 0);
}

// ============================================================================
// Config Path Tests
// ============================================================================

#[test]
fn test_custom_config_path() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let config_path = temp_dir.path().join("custom.toml");

    std::fs::write(
        &config_path,
        r#"
[commands.custom]
description = "Custom command"
cmd = "echo custom"
"#,
    )
    .expect("Failed to write config");

    let output = Command::new("cargo")
        .args([
            "run",
            "--bin",
            "cmdrun",
            "--",
            "--config",
            config_path.to_str().unwrap(),
            "list",
        ])
        .output()
        .expect("Failed to run command");

    assert_exit_code(&output, 0);

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("custom"));
}

#[test]
fn test_config_flag_with_invalid_path() {
    let output = Command::new("cargo")
        .args([
            "run",
            "--bin",
            "cmdrun",
            "--",
            "--config",
            "/this/path/definitely/does/not/exist.toml",
            "list",
        ])
        .output()
        .expect("Failed to run command");

    assert!(!output.status.success());
}

// ============================================================================
// Integration Tests (Multiple Flags)
// ============================================================================

#[test]
fn test_multiple_flags_combination() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let config_path = temp_dir.path().join("commands.toml");

    std::fs::write(
        &config_path,
        r#"
[commands.test]
description = "Test"
cmd = "echo test"
"#,
    )
    .expect("Failed to write config");

    let output = Command::new("cargo")
        .args([
            "run",
            "--bin",
            "cmdrun",
            "--",
            "-v",
            "--color",
            "never",
            "--config",
            config_path.to_str().unwrap(),
            "list",
        ])
        .output()
        .expect("Failed to run command");

    assert_exit_code(&output, 0);
}

// ============================================================================
// NO_COLOR Environment Variable Tests
// ============================================================================

#[test]
fn test_no_color_env_var() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let config_path = temp_dir.path().join("commands.toml");

    std::fs::write(
        &config_path,
        r#"
[commands.test]
description = "Test"
cmd = "echo test"
"#,
    )
    .expect("Failed to write config");

    let output = Command::new("cargo")
        .args([
            "run",
            "--bin",
            "cmdrun",
            "--",
            "--config",
            config_path.to_str().unwrap(),
            "list",
        ])
        .env("NO_COLOR", "1") // Set NO_COLOR environment variable
        .output()
        .expect("Failed to run command");

    assert_exit_code(&output, 0);

    // Should not contain colors when NO_COLOR is set
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        !stdout.contains("\x1b["),
        "Output should not contain ANSI codes when NO_COLOR is set"
    );
}
