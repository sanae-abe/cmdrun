//! Integration tests for plugin system commands
//!
//! Tests plugin management via CLI commands:
//! - Plugin listing (list, list --verbose, list --only-enabled)
//! - Plugin information display
//! - Plugin enable/disable operations
//! - Error handling for missing plugins
//!
//! Coverage target: 60% for commands/plugin.rs (currently 0%)
//!
//! Note: These tests require the `plugin-system` feature to be enabled.
//! Some tests are conditional based on feature availability.

use std::process::Command;
use tempfile::TempDir;

/// Test helper to run cmdrun plugin command with config file
fn run_plugin_command_with_config(
    args: &[&str],
    config_path: &std::path::Path,
) -> Result<std::process::Output, std::io::Error> {
    let mut cmd_args = vec![
        "run",
        "--bin",
        "cmdrun",
        "--features",
        "plugin-system",
        "--",
        "--config",
        config_path.to_str().unwrap(),
    ];
    cmd_args.extend_from_slice(args);

    Command::new("cargo").args(&cmd_args).output()
}

/// Test helper to run cmdrun plugin command (backward compatibility)
fn run_plugin_command(args: &[&str]) -> Result<std::process::Output, std::io::Error> {
    // Create temporary config for backward compatibility
    // Leak temp_dir to keep it alive for the duration of the command
    let temp_dir = Box::leak(Box::new(TempDir::new().expect("Failed to create temp dir")));
    let config_path = create_test_config(temp_dir);
    run_plugin_command_with_config(args, &config_path)
}

/// Test helper to check if output contains expected pattern
#[allow(dead_code)]
fn assert_contains(output: &[u8], pattern: &str) {
    let stdout = String::from_utf8_lossy(output);
    assert!(
        stdout.contains(pattern),
        "Output should contain '{}', but got:\n{}",
        pattern,
        stdout
    );
}

/// Test helper to check if stderr contains expected pattern
#[allow(dead_code)]
fn assert_stderr_contains(stderr: &[u8], pattern: &str) {
    let stderr_str = String::from_utf8_lossy(stderr);
    assert!(
        stderr_str.contains(pattern),
        "Stderr should contain '{}', but got:\n{}",
        pattern,
        stderr_str
    );
}

/// Create a test configuration with plugins
fn create_test_config(temp_dir: &TempDir) -> std::path::PathBuf {
    let config_path = temp_dir.path().join("commands.toml");

    let config_content = r#"
[plugins]
# Plugin configuration would go here
# For now, we test with no plugins configured

[commands]
# Empty commands section
"#;

    std::fs::write(&config_path, config_content).expect("Failed to write config");
    config_path
}

// ============================================================================
// Basic Plugin List Tests
// ============================================================================

#[test]
#[cfg(feature = "plugin-system")]
fn test_plugin_list_command() {
    let output =
        run_plugin_command(&["plugin", "list"]).expect("Failed to run plugin list command");

    // Should succeed even with no plugins
    assert!(
        output.status.success(),
        "Plugin list command should succeed, got status: {:?}",
        output.status
    );

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Should either show "No plugins installed" or list plugins
    assert!(
        stdout.contains("No plugins installed") || stdout.contains("Installed Plugins"),
        "Output should show plugin status, got:\n{}",
        stdout
    );
}

#[test]
#[cfg(feature = "plugin-system")]
fn test_plugin_list_verbose() {
    let output = run_plugin_command(&["plugin", "list", "--verbose"])
        .expect("Failed to run plugin list verbose command");

    assert!(output.status.success());

    // Verbose mode should work (even with no plugins)
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("No plugins installed") || stdout.contains("Installed Plugins"),
        "Verbose output should show plugin status"
    );
}

#[test]
#[cfg(feature = "plugin-system")]
fn test_plugin_list_only_enabled() {
    let output = run_plugin_command(&["plugin", "list", "--enabled"])
        .expect("Failed to run plugin list command");

    assert!(output.status.success());
}

#[test]
#[cfg(feature = "plugin-system")]
fn test_plugin_list_with_custom_config() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let config_path = create_test_config(&temp_dir);

    let output = Command::new("cargo")
        .args([
            "run",
            "--bin",
            "cmdrun",
            "--features",
            "plugin-system",
            "--",
            "--config",
            config_path.to_str().unwrap(),
            "plugin",
            "list",
        ])
        .output()
        .expect("Failed to run command");

    assert!(output.status.success());
}

// ============================================================================
// Plugin Info Tests
// ============================================================================

#[test]
#[cfg(feature = "plugin-system")]
fn test_plugin_info_nonexistent() {
    let output = run_plugin_command(&["plugin", "info", "nonexistent-plugin"])
        .expect("Failed to run plugin info command");

    // Should fail for nonexistent plugin
    assert!(
        !output.status.success(),
        "Plugin info should fail for nonexistent plugin"
    );

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("not found") || stderr.contains("Plugin not found"),
        "Should indicate plugin was not found, got:\n{}",
        stderr
    );
}

#[test]
#[cfg(feature = "plugin-system")]
fn test_plugin_info_help() {
    let output =
        run_plugin_command(&["plugin", "info", "--help"]).expect("Failed to run plugin info help");

    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("Show detailed") || stdout.contains("plugin information"),
        "Help text should describe plugin info command, got:\n{}",
        stdout
    );
}

// ============================================================================
// Plugin Enable/Disable Tests
// ============================================================================

#[test]
#[cfg(feature = "plugin-system")]
fn test_plugin_enable_nonexistent() {
    let output = run_plugin_command(&["plugin", "enable", "nonexistent-plugin"])
        .expect("Failed to run plugin enable command");

    // Should fail for nonexistent plugin
    assert!(!output.status.success());
}

#[test]
#[cfg(feature = "plugin-system")]
fn test_plugin_disable_nonexistent() {
    let output = run_plugin_command(&["plugin", "disable", "nonexistent-plugin"])
        .expect("Failed to run plugin disable command");

    // Should fail for nonexistent plugin
    assert!(!output.status.success());
}

#[test]
#[cfg(feature = "plugin-system")]
fn test_plugin_enable_help() {
    let output = run_plugin_command(&["plugin", "enable", "--help"])
        .expect("Failed to run plugin enable help");

    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Enable a plugin"));
}

#[test]
#[cfg(feature = "plugin-system")]
fn test_plugin_disable_help() {
    let output = run_plugin_command(&["plugin", "disable", "--help"])
        .expect("Failed to run plugin disable help");

    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Disable a plugin"));
}

// ============================================================================
// Plugin Command Help Tests
// ============================================================================

#[test]
#[cfg(feature = "plugin-system")]
fn test_plugin_help_command() {
    let output = run_plugin_command(&["plugin", "--help"]).expect("Failed to run plugin help");

    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Should list all plugin subcommands
    assert!(stdout.contains("list"));
    assert!(stdout.contains("info"));
    assert!(stdout.contains("enable"));
    assert!(stdout.contains("disable"));
}

#[test]
#[cfg(feature = "plugin-system")]
fn test_plugin_subcommands_listed() {
    let output = run_plugin_command(&["plugin", "--help"]).expect("Failed to run plugin help");

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Verify all main subcommands are documented
    assert!(
        stdout.contains("list") && stdout.contains("List"),
        "Should document 'list' subcommand"
    );
    assert!(
        stdout.contains("info") && stdout.contains("Show"),
        "Should document 'info' subcommand"
    );
    assert!(
        stdout.contains("enable"),
        "Should document 'enable' subcommand"
    );
    assert!(
        stdout.contains("disable"),
        "Should document 'disable' subcommand"
    );
}

// ============================================================================
// Error Handling Tests
// ============================================================================

#[test]
#[cfg(feature = "plugin-system")]
fn test_plugin_command_without_subcommand() {
    let output = run_plugin_command(&["plugin"]).expect("Failed to run plugin command");

    // clap returns exit code 2 when subcommand is missing, but shows help
    // Help is shown on stdout (some commands) or stderr (some clap versions)
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    let combined = format!("{}{}", stdout, stderr);

    assert!(
        combined.contains("Usage") || combined.contains("Commands"),
        "Should show usage or commands list, got stdout:\n{}\nstderr:\n{}",
        stdout,
        stderr
    );
}

#[test]
#[cfg(feature = "plugin-system")]
fn test_plugin_invalid_subcommand() {
    let output = run_plugin_command(&["plugin", "invalid-subcommand"])
        .expect("Failed to run plugin command");

    assert!(!output.status.success());
}

// ============================================================================
// Feature Flag Tests
// ============================================================================

#[test]
#[cfg(not(feature = "plugin-system"))]
fn test_plugin_disabled_without_feature() {
    // This test verifies behavior when plugin-system feature is disabled
    let output = Command::new("cargo")
        .args(["run", "--bin", "cmdrun", "--", "plugin", "list"])
        .output()
        .expect("Failed to run command");

    // Behavior when plugin-system is disabled may vary:
    // - Could show an error
    // - Could show empty plugin list
    // - Could indicate feature is not available

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(
        stdout.contains("No plugins")
            || stderr.contains("not available")
            || stderr.contains("feature")
            || output.status.success(), // Some implementations may succeed with empty list
        "Should handle plugin command gracefully when feature is disabled"
    );
}

// ============================================================================
// Output Format Tests
// ============================================================================

#[test]
#[cfg(feature = "plugin-system")]
fn test_plugin_list_output_format() {
    let output = run_plugin_command(&["plugin", "list"]).expect("Failed to run plugin list");

    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Output should be well-formatted
    if !stdout.contains("No plugins installed") {
        // If there are plugins, check formatting
        assert!(
            stdout.contains("Installed Plugins"),
            "Should have proper header"
        );
        assert!(stdout.contains("Summary"), "Should have summary section");
    }
}

#[test]
#[cfg(feature = "plugin-system")]
fn test_plugin_list_verbose_shows_details() {
    let output = run_plugin_command(&["plugin", "list", "--verbose"])
        .expect("Failed to run plugin list verbose");

    assert!(output.status.success());

    // Verbose mode should show additional details if plugins exist
    // This is mainly to ensure --verbose flag is recognized
}

// ============================================================================
// CLI Integration Tests
// ============================================================================

#[test]
#[cfg(feature = "plugin-system")]
fn test_plugin_with_global_config_flag() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let config_path = create_test_config(&temp_dir);

    let output = Command::new("cargo")
        .args([
            "run",
            "--bin",
            "cmdrun",
            "--features",
            "plugin-system",
            "--",
            "--config",
            config_path.to_str().unwrap(),
            "plugin",
            "list",
        ])
        .output()
        .expect("Failed to run command");

    assert!(
        output.status.success(),
        "Should work with global --config flag"
    );
}

#[test]
#[cfg(feature = "plugin-system")]
fn test_plugin_with_color_flag() {
    let output = Command::new("cargo")
        .args([
            "run",
            "--bin",
            "cmdrun",
            "--features",
            "plugin-system",
            "--",
            "--color",
            "never",
            "plugin",
            "list",
        ])
        .output()
        .expect("Failed to run command");

    assert!(
        output.status.success(),
        "Should work with global --color flag"
    );
}

#[test]
#[cfg(feature = "plugin-system")]
fn test_plugin_with_verbose_flag() {
    let output = Command::new("cargo")
        .args([
            "run",
            "--bin",
            "cmdrun",
            "--features",
            "plugin-system",
            "--",
            "-v",
            "plugin",
            "list",
        ])
        .output()
        .expect("Failed to run command");

    assert!(output.status.success(), "Should work with global -v flag");
}

// ============================================================================
// Edge Cases Tests
// ============================================================================

#[test]
#[cfg(feature = "plugin-system")]
fn test_plugin_list_multiple_flags() {
    let output = run_plugin_command(&["plugin", "list", "--verbose", "--enabled"])
        .expect("Failed to run plugin list");

    assert!(
        output.status.success(),
        "Should handle multiple flags correctly"
    );
}

#[test]
#[cfg(feature = "plugin-system")]
fn test_plugin_info_empty_name() {
    // Test with empty plugin name
    let output = run_plugin_command(&["plugin", "info", ""]).expect("Failed to run plugin info");

    // Should either fail or show error
    let stderr = String::from_utf8_lossy(&output.stderr);
    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(
        !output.status.success() || stderr.contains("required") || stdout.contains("required"),
        "Should handle empty plugin name"
    );
}

// ============================================================================
// Performance Tests
// ============================================================================

#[test]
#[cfg(feature = "plugin-system")]
fn test_plugin_list_is_fast() {
    use std::time::Instant;

    let start = Instant::now();
    let output = run_plugin_command(&["plugin", "list"]).expect("Failed to run plugin list");
    let duration = start.elapsed();

    assert!(output.status.success());

    // Plugin list should be reasonably fast (< 5 seconds in CI)
    assert!(
        duration.as_secs() < 5,
        "Plugin list took too long: {:?}",
        duration
    );
}

// ============================================================================
// Documentation Tests
// ============================================================================

#[test]
#[cfg(feature = "plugin-system")]
fn test_all_plugin_subcommands_have_help() {
    let subcommands = ["list", "info", "enable", "disable"];

    for subcommand in &subcommands {
        let output = run_plugin_command(&["plugin", subcommand, "--help"])
            .unwrap_or_else(|_| panic!("Failed to run plugin {} --help", subcommand));

        assert!(
            output.status.success(),
            "Help should be available for plugin {}",
            subcommand
        );

        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(
            stdout.len() > 50,
            "Help text for plugin {} should be descriptive",
            subcommand
        );
    }
}

// ============================================================================
// Compatibility Tests
// ============================================================================

#[test]
fn test_plugin_compiles_without_feature() {
    // This test ensures the code compiles even when plugin-system feature is disabled
    // The actual runtime behavior is tested in test_plugin_disabled_without_feature

    #[cfg(not(feature = "plugin-system"))]
    {
        // Create a PluginManager to ensure the API is available
        let mut manager = cmdrun::plugin::PluginManager::new();
        let plugins = ahash::AHashMap::new();

        // This should work (no-op) even without the feature
        assert!(manager.load_plugins(&plugins).is_ok());
    }

    #[cfg(feature = "plugin-system")]
    {
        // With feature enabled, same code should also work
        let mut manager = cmdrun::plugin::PluginManager::new();
        let plugins = ahash::AHashMap::new();
        assert!(manager.load_plugins(&plugins).is_ok());
    }
}
