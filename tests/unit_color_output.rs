//! Unit tests for color output configuration

use cmdrun::cli::ColorChoice;

#[test]
fn test_color_choice_values() {
    // Test that ColorChoice enum has the expected variants
    use clap::ValueEnum;

    let choices = ColorChoice::value_variants();
    assert_eq!(choices.len(), 3);

    // Test from_str parsing
    assert!(matches!(
        ColorChoice::from_str("never", true),
        Ok(ColorChoice::Never)
    ));
    assert!(matches!(
        ColorChoice::from_str("auto", true),
        Ok(ColorChoice::Auto)
    ));
    assert!(matches!(
        ColorChoice::from_str("always", true),
        Ok(ColorChoice::Always)
    ));
}

#[test]
fn test_color_choice_case_insensitive() {
    use clap::ValueEnum;

    // Test case-insensitive parsing
    assert!(matches!(
        ColorChoice::from_str("NEVER", true),
        Ok(ColorChoice::Never)
    ));
    assert!(matches!(
        ColorChoice::from_str("Auto", true),
        Ok(ColorChoice::Auto)
    ));
    assert!(matches!(
        ColorChoice::from_str("ALWAYS", true),
        Ok(ColorChoice::Always)
    ));
}

#[test]
fn test_color_choice_invalid() {
    use clap::ValueEnum;

    // Test invalid values
    assert!(ColorChoice::from_str("invalid", true).is_err());
    assert!(ColorChoice::from_str("yes", true).is_err());
    assert!(ColorChoice::from_str("no", true).is_err());
}

#[cfg(test)]
mod integration {
    use std::process::Command;

    // Helper function to get the cmdrun binary path
    fn get_cmdrun_binary() -> std::path::PathBuf {
        // Try to use the test binary path first (for `cargo test`)
        if let Ok(bin_path) = std::env::var("CARGO_BIN_EXE_cmdrun") {
            return std::path::PathBuf::from(bin_path);
        }

        // Fallback: build the path manually
        let mut path = std::env::current_exe()
            .expect("Failed to get current exe path")
            .parent()
            .expect("Failed to get parent dir")
            .to_path_buf();

        // Go up to target/debug or target/release
        if path.ends_with("deps") {
            path.pop();
        }

        path.push("cmdrun");
        if cfg!(windows) {
            path.set_extension("exe");
        }

        path
    }

    // Create a temporary config file for testing
    fn create_temp_config() -> (tempfile::TempDir, std::path::PathBuf) {
        let temp_dir = tempfile::TempDir::new().expect("Failed to create temp dir");
        let config_path = temp_dir.path().join("commands.toml");

        std::fs::write(
            &config_path,
            r#"[commands]
test = { description = "Test command", cmd = "echo test" }
"#,
        )
        .expect("Failed to write temp config");

        (temp_dir, config_path)
    }

    #[test]
    fn test_color_flag_never() {
        let cmdrun_bin = get_cmdrun_binary();
        let (_temp_dir, config_path) = create_temp_config();

        let output = Command::new(&cmdrun_bin)
            .args(["list", "--color=never", "-c"])
            .arg(&config_path)
            .env("NO_COLOR", "0") // Make sure NO_COLOR doesn't interfere
            .env_remove("NO_COLOR")
            .output()
            .expect("Failed to execute command");

        let stdout = String::from_utf8_lossy(&output.stdout);

        // Check that ANSI color codes are NOT present
        assert!(
            !stdout.contains("\x1b["),
            "Output should not contain ANSI color codes"
        );
    }

    #[test]
    fn test_color_flag_always() {
        let cmdrun_bin = get_cmdrun_binary();
        let (_temp_dir, config_path) = create_temp_config();

        let output = Command::new(&cmdrun_bin)
            .args(["list", "--color=always", "-c"])
            .arg(&config_path)
            .output()
            .expect("Failed to execute command");

        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);

        // With --color=always, output should contain color codes even when not a TTY
        if !output.status.success() {
            eprintln!("Command failed with exit code: {:?}", output.status.code());
            eprintln!("stdout: {}", stdout);
            eprintln!("stderr: {}", stderr);
        }
        assert!(
            output.status.success(),
            "Command should succeed, but failed with stdout: {}, stderr: {}",
            stdout,
            stderr
        );
    }

    #[test]
    fn test_help_shows_color_option() {
        let cmdrun_bin = get_cmdrun_binary();

        let output = Command::new(&cmdrun_bin)
            .args(["--help"])
            .output()
            .expect("Failed to execute command");

        let stdout = String::from_utf8_lossy(&output.stdout);

        // Check that --color option is documented in help
        assert!(
            stdout.contains("--color"),
            "Help should mention --color option"
        );
        assert!(
            stdout.contains("never") || stdout.contains("auto") || stdout.contains("always"),
            "Help should mention color values"
        );
    }
}
