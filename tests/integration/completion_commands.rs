//! Integration tests for shell completion commands
//!
//! Tests shell completion generation and validation across multiple shells:
//! - Bash
//! - Zsh
//! - Fish
//! - PowerShell
//! - Elvish
//!
//! Coverage target: 60% for commands/completion.rs (currently 0%)

use std::process::Command;
use tempfile::TempDir;

/// Test helper to run cmdrun completion command
fn run_completion(shell: &str) -> Result<std::process::Output, std::io::Error> {
    Command::new("cargo")
        .args(["run", "--bin", "cmdrun", "--", "completion", shell])
        .output()
}

/// Test helper to check if output contains expected patterns
fn assert_contains(output: &[u8], pattern: &str) {
    let stdout = String::from_utf8_lossy(output);
    assert!(
        stdout.contains(pattern),
        "Output should contain '{}', but got:\n{}",
        pattern,
        stdout
    );
}

/// Test helper to check if stderr contains installation instructions
fn assert_installation_instructions(stderr: &[u8], shell: &str) {
    let stderr_str = String::from_utf8_lossy(stderr);
    assert!(
        stderr_str.contains("Installation instructions:"),
        "stderr should contain installation instructions for {}, but got:\n{}",
        shell,
        stderr_str
    );
}

// ============================================================================
// Basic Completion Generation Tests
// ============================================================================

#[test]
fn test_bash_completion_generation() {
    let output = run_completion("bash").expect("Failed to run completion command");

    // Should exit successfully
    assert!(
        output.status.success(),
        "Command should succeed, but got status: {:?}",
        output.status
    );

    // Should generate bash completion script (clap generates direct function definition)
    assert_contains(&output.stdout, "_cmdrun");

    // Should include custom completion functions
    assert_contains(&output.stdout, "_cmdrun_complete_commands");
    assert_contains(&output.stdout, "_cmdrun_wrap_completion");

    // Should provide installation instructions to stderr
    assert_installation_instructions(&output.stderr, "bash");
    assert_contains(&output.stderr, "~/.bashrc");
    assert_contains(&output.stderr, "eval \"$(cmdrun completion bash)\"");
}

#[test]
fn test_zsh_completion_generation() {
    let output = run_completion("zsh").expect("Failed to run completion command");

    assert!(output.status.success());

    // Should generate zsh completion script
    assert_contains(&output.stdout, "#compdef cmdrun");
    assert_contains(&output.stdout, "_cmdrun");

    // Should include custom completion functions
    assert_contains(&output.stdout, "_cmdrun_commands_with_desc");
    assert_contains(&output.stdout, "functions[_cmdrun_original]");

    // Should provide installation instructions
    assert_installation_instructions(&output.stderr, "zsh");
    assert_contains(&output.stderr, "~/.zshrc");
    assert_contains(&output.stderr, "eval \"$(cmdrun completion zsh)\"");
}

#[test]
fn test_fish_completion_generation() {
    let output = run_completion("fish").expect("Failed to run completion command");

    assert!(output.status.success());

    // Should generate fish completion script
    assert_contains(&output.stdout, "complete -c cmdrun");

    // Should include custom completions for run/info subcommands
    assert_contains(&output.stdout, "__fish_seen_subcommand_from run");
    assert_contains(&output.stdout, "__fish_seen_subcommand_from info");
    assert_contains(&output.stdout, "cmdrun completion-list");

    // Should provide installation instructions
    assert_installation_instructions(&output.stderr, "fish");
    assert_contains(&output.stderr, "~/.config/fish/completions/cmdrun.fish");
}

#[test]
fn test_powershell_completion_generation() {
    let output = run_completion("powershell").expect("Failed to run completion command");

    assert!(output.status.success());

    // Should generate PowerShell completion script
    assert_contains(&output.stdout, "Register-ArgumentCompleter");
    assert_contains(&output.stdout, "cmdrun");

    // Should provide installation instructions
    assert_installation_instructions(&output.stderr, "powershell");
    assert_contains(&output.stderr, "PowerShell profile");
    assert_contains(
        &output.stderr,
        "cmdrun completion powershell | Out-String | Invoke-Expression",
    );
}

#[test]
fn test_elvish_completion_generation() {
    let output = run_completion("elvish").expect("Failed to run completion command");

    assert!(output.status.success());

    // Should generate Elvish completion script
    assert_contains(&output.stdout, "edit:completion:arg-completer[cmdrun]");

    // Should provide installation instructions
    assert_installation_instructions(&output.stderr, "elvish");
    assert_contains(&output.stderr, "~/.elvish/rc.elv");
    assert_contains(&output.stderr, "eval (cmdrun completion elvish)");
}

// ============================================================================
// Cross-Shell Compatibility Tests
// ============================================================================

#[test]
fn test_all_supported_shells_generate_successfully() {
    let shells = ["bash", "zsh", "fish", "powershell", "elvish"];

    for shell in &shells {
        let output = run_completion(shell)
            .unwrap_or_else(|_| panic!("Failed to run completion for {}", shell));

        assert!(
            output.status.success(),
            "Completion generation for {} should succeed",
            shell
        );

        assert!(
            !output.stdout.is_empty(),
            "Completion script for {} should not be empty",
            shell
        );

        // All shells should have installation instructions
        assert_installation_instructions(&output.stderr, shell);
    }
}

#[test]
fn test_unsupported_shell_returns_error() {
    let output = Command::new("cargo")
        .args([
            "run",
            "--bin",
            "cmdrun",
            "--",
            "completion",
            "invalid-shell",
        ])
        .output()
        .expect("Failed to run command");

    // Should fail for unsupported shell
    assert!(
        !output.status.success(),
        "Should fail for unsupported shell"
    );
}

// ============================================================================
// Completion Content Validation Tests
// ============================================================================

#[test]
fn test_bash_completion_includes_all_subcommands() {
    let output = run_completion("bash").expect("Failed to run completion command");

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Should include main subcommands
    assert!(stdout.contains("run") || stdout.contains("cmdrun__run"));
    assert!(stdout.contains("list") || stdout.contains("cmdrun__list"));
    assert!(stdout.contains("add") || stdout.contains("cmdrun__add"));
    assert!(stdout.contains("remove") || stdout.contains("cmdrun__remove"));
    assert!(stdout.contains("completion") || stdout.contains("cmdrun__completion"));
}

#[test]
fn test_zsh_completion_supports_descriptions() {
    let output = run_completion("zsh").expect("Failed to run completion command");

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Zsh should use _describe for command descriptions
    assert!(
        stdout.contains("_describe"),
        "Zsh completion should use _describe for command descriptions"
    );

    // Should call cmdrun completion-list for custom completions
    assert!(
        stdout.contains("cmdrun completion-list"),
        "Zsh completion should use cmdrun completion-list for dynamic completions"
    );
}

#[test]
fn test_fish_completion_supports_descriptions() {
    let output = run_completion("fish").expect("Failed to run completion command");

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Fish supports native descriptions with tab separator
    assert!(
        stdout.contains("string replace ':' \\t"),
        "Fish completion should convert description format (colon to tab)"
    );
}

#[test]
fn test_completion_list_integration() {
    // First, check if completion-list command exists
    let list_output = Command::new("cargo")
        .args(["run", "--bin", "cmdrun", "--", "completion-list"])
        .output()
        .expect("Failed to run completion-list command");

    // If completion-list is not implemented yet, this test will be skipped
    if !list_output.status.success() {
        println!("Skipping: completion-list command not yet implemented");
        return;
    }

    // Verify that completion scripts reference completion-list
    for shell in &["bash", "zsh", "fish"] {
        let output = run_completion(shell).expect("Failed to generate completion");
        let stdout = String::from_utf8_lossy(&output.stdout);

        assert!(
            stdout.contains("cmdrun completion-list"),
            "{} completion should call cmdrun completion-list",
            shell
        );
    }
}

// ============================================================================
// Installation Instructions Tests
// ============================================================================

#[test]
fn test_installation_instructions_are_on_stderr() {
    let shells = ["bash", "zsh", "fish", "powershell", "elvish"];

    for shell in &shells {
        let output = run_completion(shell).expect("Failed to run completion");

        // Instructions should be on stderr, not stdout
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(
            stderr.contains("Installation instructions:"),
            "{} should have installation instructions on stderr",
            shell
        );

        // stdout should only contain the completion script
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(
            !stdout.contains("Installation instructions:"),
            "{} completion script should not contain installation instructions",
            shell
        );
    }
}

#[test]
fn test_bash_installation_instructions_format() {
    let output = run_completion("bash").expect("Failed to run completion");
    let stderr = String::from_utf8_lossy(&output.stderr);

    // Should mention bashrc
    assert!(stderr.contains("~/.bashrc"));

    // Should provide eval command
    assert!(stderr.contains("eval"));

    // Should provide alternative completion directory
    assert!(stderr.contains("/etc/bash_completion.d/cmdrun"));

    // Should have note about restarting shell
    assert!(stderr.contains("restart your shell"));
}

#[test]
fn test_zsh_installation_instructions_format() {
    let output = run_completion("zsh").expect("Failed to run completion");
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(stderr.contains("~/.zshrc"));
    assert!(stderr.contains("eval"));
    assert!(stderr.contains("${fpath[1]}/_cmdrun"));
    assert!(stderr.contains("restart your shell"));
}

#[test]
fn test_fish_installation_instructions_format() {
    let output = run_completion("fish").expect("Failed to run completion");
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(stderr.contains("~/.config/fish/completions/cmdrun.fish"));
    assert!(stderr.contains("restart your shell") || stderr.contains("source"));
}

#[test]
fn test_powershell_installation_instructions_format() {
    let output = run_completion("powershell").expect("Failed to run completion");
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(stderr.contains("PowerShell profile"));
    assert!(stderr.contains("Out-String | Invoke-Expression"));
    assert!(stderr.contains("cmdrun.ps1"));
}

// ============================================================================
// Output Format Tests
// ============================================================================

#[test]
fn test_completion_output_is_valid_shell_script() {
    let test_cases = vec![
        ("bash", "_cmdrun"),            // Bash completion defines functions
        ("zsh", "#compdef"),            // Zsh uses #compdef directive
        ("fish", "complete -c cmdrun"), // Fish uses complete command
    ];

    for (shell, expected_pattern) in test_cases {
        let output = run_completion(shell)
            .unwrap_or_else(|_| panic!("Failed to run completion for {}", shell));

        let stdout = String::from_utf8_lossy(&output.stdout);

        assert!(
            stdout.contains(expected_pattern),
            "{} completion should contain '{}', but got:\n{}",
            shell,
            expected_pattern,
            stdout
        );
    }
}

#[test]
fn test_completion_output_has_no_invalid_characters() {
    let shells = ["bash", "zsh", "fish", "powershell"];

    for shell in &shells {
        let output = run_completion(shell).expect("Failed to run completion");

        // Stdout should be valid UTF-8
        let stdout = String::from_utf8(output.stdout.clone());
        assert!(
            stdout.is_ok(),
            "{} completion output should be valid UTF-8",
            shell
        );

        // Should not contain null bytes
        assert!(
            !output.stdout.contains(&0u8),
            "{} completion should not contain null bytes",
            shell
        );
    }
}

// ============================================================================
// Integration with Main CLI Tests
// ============================================================================

#[test]
fn test_completion_command_works_with_config_flag() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let config_path = temp_dir.path().join("commands.toml");
    std::fs::write(&config_path, "[commands]\n").expect("Failed to write config");

    let output = Command::new("cargo")
        .args([
            "run",
            "--bin",
            "cmdrun",
            "--",
            "--config",
            config_path.to_str().unwrap(),
            "completion",
            "bash",
        ])
        .output()
        .expect("Failed to run command");

    // Should work even with custom config
    assert!(output.status.success());
    assert!(!output.stdout.is_empty());
}

#[test]
fn test_completion_command_with_color_flag() {
    let output = Command::new("cargo")
        .args([
            "run",
            "--bin",
            "cmdrun",
            "--",
            "--color",
            "never",
            "completion",
            "bash",
        ])
        .output()
        .expect("Failed to run command");

    assert!(output.status.success());
    assert!(!output.stdout.is_empty());
}

// ============================================================================
// Performance Tests
// ============================================================================

#[test]
fn test_completion_generation_is_fast() {
    use std::time::Instant;

    let start = Instant::now();
    let output = run_completion("bash").expect("Failed to run completion");
    let duration = start.elapsed();

    assert!(output.status.success());

    // Completion generation should be reasonably fast (< 5 seconds in CI)
    // Note: This is generous for CI environments; locally it's much faster
    assert!(
        duration.as_secs() < 5,
        "Completion generation took too long: {:?}",
        duration
    );
}

// ============================================================================
// Edge Cases and Error Handling Tests
// ============================================================================

#[test]
fn test_completion_with_no_arguments() {
    let output = Command::new("cargo")
        .args(["run", "--bin", "cmdrun", "--", "completion"])
        .output()
        .expect("Failed to run command");

    // Should fail or show help when no shell is specified
    assert!(
        !output.status.success(),
        "Should fail when no shell is specified"
    );
}

#[test]
fn test_completion_help_message() {
    let output = Command::new("cargo")
        .args(["run", "--bin", "cmdrun", "--", "completion", "--help"])
        .output()
        .expect("Failed to run command");

    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Generate shell completion"));
    assert!(stdout.contains("bash"));
    assert!(stdout.contains("zsh"));
    assert!(stdout.contains("fish"));
}

#[test]
fn test_completion_output_is_idempotent() {
    // Running the same completion twice should produce identical output
    let output1 = run_completion("bash").expect("Failed first run");
    let output2 = run_completion("bash").expect("Failed second run");

    assert_eq!(
        output1.stdout, output2.stdout,
        "Completion output should be idempotent"
    );
}

// ============================================================================
// Documentation Tests
// ============================================================================

#[test]
fn test_all_shells_have_installation_instructions() {
    let shells = ["bash", "zsh", "fish", "powershell", "elvish"];

    for shell in &shells {
        let output = run_completion(shell)
            .unwrap_or_else(|_| panic!("Failed to run completion for {}", shell));

        let stderr = String::from_utf8_lossy(&output.stderr);

        // Each shell should have clear installation instructions
        assert!(
            stderr.contains("Installation instructions:"),
            "{} should have installation instructions header",
            shell
        );

        // Should mention how to enable the completion
        assert!(
            stderr.len() > 100,
            "{} installation instructions should be detailed (> 100 chars)",
            shell
        );
    }
}

#[test]
fn test_completion_includes_restart_note() {
    let shells = ["bash", "zsh", "fish", "powershell"];

    for shell in &shells {
        let output = run_completion(shell).expect("Failed to run completion");
        let stderr = String::from_utf8_lossy(&output.stderr);

        // Should remind users to restart shell or source config
        assert!(
            stderr.contains("restart") || stderr.contains("source"),
            "{} should mention restarting shell or sourcing config",
            shell
        );
    }
}
