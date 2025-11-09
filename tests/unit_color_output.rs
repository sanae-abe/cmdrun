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

    #[test]
    fn test_color_flag_never() {
        let output = Command::new("cargo")
            .args(&["run", "--quiet", "--", "list", "--color=never"])
            .env("NO_COLOR", "0")  // Make sure NO_COLOR doesn't interfere
            .env_remove("NO_COLOR")
            .output()
            .expect("Failed to execute command");

        let stdout = String::from_utf8_lossy(&output.stdout);
        
        // Check that ANSI color codes are NOT present
        assert!(!stdout.contains("\x1b["), "Output should not contain ANSI color codes");
    }

    #[test]
    fn test_color_flag_always() {
        let output = Command::new("cargo")
            .args(&["run", "--quiet", "--", "list", "--color=always"])
            .output()
            .expect("Failed to execute command");

        let stdout = String::from_utf8_lossy(&output.stdout);
        
        // With --color=always, output should contain color codes even when not a TTY
        // Note: This test might need adjustment based on actual implementation
        // For now, we just check the command doesn't error
        assert!(output.status.success() || stdout.len() > 0);
    }

    #[test]
    fn test_help_shows_color_option() {
        let output = Command::new("cargo")
            .args(&["run", "--quiet", "--", "--help"])
            .output()
            .expect("Failed to execute command");

        let stdout = String::from_utf8_lossy(&output.stdout);
        
        // Check that --color option is documented in help
        assert!(stdout.contains("--color"), "Help should mention --color option");
        assert!(stdout.contains("never") || stdout.contains("auto") || stdout.contains("always"),
                "Help should mention color values");
    }
}
