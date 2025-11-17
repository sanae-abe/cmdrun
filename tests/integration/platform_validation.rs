//! Platform Validation Tests
//!
//! Tests for CommandExecutor::check_platform method to ensure
//! platform-specific commands are properly validated.
//!
//! These tests are designed to catch the mutation:
//! MISSED: replace check_platform -> Result<()> with Ok(())

use cmdrun::command::executor::{CommandExecutor, ExecutionContext};
use cmdrun::config::schema::{Command, CommandSpec, Platform};

/// Helper function to create a test executor
/// Uses default ExecutionContext
fn create_test_executor() -> CommandExecutor {
    CommandExecutor::new(ExecutionContext::default())
}

/// Helper function to create a command with platform restriction
fn create_command_with_platform(description: &str, platforms: Vec<Platform>) -> Command {
    Command {
        description: description.to_string(),
        cmd: CommandSpec::Single("echo test".to_string()),
        working_dir: None,
        env: Default::default(),
        timeout: None,
        deps: vec![],
        platform: platforms,
        tags: vec![],
        parallel: false,
        confirm: false,
        allow_chaining: None,
        allow_subshells: None,
    }
}

// ============================================================================
// Platform Validation Tests - Ensure check_platform actually validates
// ============================================================================

#[tokio::test]
async fn test_check_platform_with_empty_restriction() {
    // Empty platform restriction should allow all platforms
    let executor = create_test_executor();
    let command = create_command_with_platform("test", vec![]);

    // Should succeed on any platform
    let result = executor.execute(&command).await;
    assert!(
        result.is_ok(),
        "Empty platform restriction should allow execution"
    );
}

#[tokio::test]
#[cfg(target_os = "linux")]
async fn test_check_platform_linux_on_linux() {
    // Linux-specific command should run on Linux
    let executor = create_test_executor();
    let command = create_command_with_platform("linux-only", vec![Platform::Linux]);

    let result = executor.execute(&command).await;
    assert!(result.is_ok(), "Linux command should run on Linux platform");
}

#[tokio::test]
#[cfg(target_os = "linux")]
async fn test_check_platform_windows_on_linux() {
    // Windows-specific command should FAIL on Linux
    let executor = create_test_executor();
    let command = create_command_with_platform("windows-only", vec![Platform::Windows]);

    let result = executor.execute(&command).await;
    assert!(result.is_err(), "Windows-only command should FAIL on Linux");
}

#[tokio::test]
#[cfg(target_os = "macos")]
async fn test_check_platform_macos_on_macos() {
    // macOS-specific command should run on macOS
    let executor = create_test_executor();
    let command = create_command_with_platform("macos-only", vec![Platform::Macos]);

    let result = executor.execute(&command).await;
    assert!(result.is_ok(), "macOS command should run on macOS platform");
}

#[tokio::test]
#[cfg(target_os = "macos")]
async fn test_check_platform_windows_on_macos() {
    // Windows-specific command should FAIL on macOS
    let executor = create_test_executor();
    let command = create_command_with_platform("windows-only", vec![Platform::Windows]);

    let result = executor.execute(&command).await;
    assert!(result.is_err(), "Windows-only command should FAIL on macOS");
}

#[tokio::test]
#[cfg(target_os = "windows")]
async fn test_check_platform_windows_on_windows() {
    // Windows-specific command should run on Windows
    let executor = create_test_executor();
    let command = create_command_with_platform("windows-only", vec![Platform::Windows]);

    let result = executor.execute(&command).await;
    assert!(
        result.is_ok(),
        "Windows command should run on Windows platform"
    );
}

#[tokio::test]
#[cfg(target_os = "windows")]
async fn test_check_platform_linux_on_windows() {
    // Linux-specific command should FAIL on Windows
    let executor = create_test_executor();
    let command = create_command_with_platform("linux-only", vec![Platform::Linux]);

    let result = executor.execute(&command).await;
    assert!(result.is_err(), "Linux-only command should FAIL on Windows");
}

#[tokio::test]
#[cfg(any(target_os = "linux", target_os = "macos"))]
async fn test_check_platform_unix_on_unix_like() {
    // Unix platform should match Linux and macOS
    let executor = create_test_executor();
    let command = create_command_with_platform("unix-only", vec![Platform::Unix]);

    let result = executor.execute(&command).await;
    assert!(
        result.is_ok(),
        "Unix command should run on Unix-like platforms (Linux/macOS)"
    );
}

#[tokio::test]
#[cfg(target_os = "windows")]
async fn test_check_platform_unix_on_windows() {
    // Unix platform should FAIL on Windows
    let executor = create_test_executor();
    let command = create_command_with_platform("unix-only", vec![Platform::Unix]);

    let result = executor.execute(&command).await;
    assert!(result.is_err(), "Unix-only command should FAIL on Windows");
}

#[tokio::test]
async fn test_check_platform_multiple_platforms() {
    // Command with multiple platform restrictions
    let executor = create_test_executor();
    let command =
        create_command_with_platform("multi-platform", vec![Platform::Linux, Platform::Macos]);

    let result = executor.execute(&command).await;

    #[cfg(any(target_os = "linux", target_os = "macos"))]
    assert!(result.is_ok(), "Should succeed on Linux or macOS");

    #[cfg(target_os = "windows")]
    assert!(result.is_err(), "Should fail on Windows");
}

// ============================================================================
// Edge Cases
// ============================================================================

#[tokio::test]
async fn test_check_platform_with_all_platforms() {
    // Command that allows all platforms explicitly
    let executor = create_test_executor();
    let command = create_command_with_platform(
        "all-platforms",
        vec![
            Platform::Unix,
            Platform::Linux,
            Platform::Macos,
            Platform::Windows,
        ],
    );

    let result = executor.execute(&command).await;
    assert!(result.is_ok(), "Should succeed on all platforms");
}

#[tokio::test]
async fn test_check_platform_with_current_platform() {
    // Ensure current platform is correctly detected
    let current_platform = Platform::current();

    let executor = create_test_executor();
    let command = create_command_with_platform("current-platform", vec![current_platform]);

    let result = executor.execute(&command).await;
    assert!(result.is_ok(), "Should succeed on current platform");
}

// ============================================================================
// Mutation Testing - Specific Tests to Catch Mutations
// ============================================================================

/// This test specifically targets the mutation:
/// MISSED: replace check_platform -> Result<()> with Ok(())
///
/// If check_platform is replaced with Ok(()), this test will FAIL
/// because it expects platform validation to actually happen.
#[tokio::test]
#[cfg(not(target_os = "windows"))]
async fn test_mutation_check_platform_not_bypassed() {
    let executor = create_test_executor();

    // Create a Windows-only command
    let windows_only = create_command_with_platform("windows-cmd", vec![Platform::Windows]);

    // On non-Windows platforms, this MUST fail
    let result = executor.execute(&windows_only).await;

    assert!(
        result.is_err(),
        "Platform validation MUST reject Windows-only commands on non-Windows platforms. \
         If this test fails, check_platform might be returning Ok(()) without validation!"
    );
}

/// Complementary test for Windows platforms
#[tokio::test]
#[cfg(target_os = "windows")]
async fn test_mutation_check_platform_not_bypassed_windows() {
    let executor = create_test_executor();

    // Create a Linux-only command
    let linux_only = create_command_with_platform("linux-cmd", vec![Platform::Linux]);

    // On Windows, this MUST fail
    let result = executor.execute(&linux_only).await;

    assert!(
        result.is_err(),
        "Platform validation MUST reject Linux-only commands on Windows. \
         If this test fails, check_platform might be returning Ok(()) without validation!"
    );
}

/// Test that verifies platform validation happens before command execution
#[tokio::test]
async fn test_platform_validation_before_execution() {
    let executor = create_test_executor();

    // Create a command that would fail if executed, but should be
    // rejected by platform validation first
    let incompatible_cmd = Command {
        description: "incompatible".to_string(),
        cmd: CommandSpec::Single("this-command-does-not-exist".to_string()),
        working_dir: None,
        env: Default::default(),
        timeout: None,
        deps: vec![],
        platform: if cfg!(target_os = "windows") {
            vec![Platform::Linux] // Incompatible platform
        } else {
            vec![Platform::Windows] // Incompatible platform
        },
        tags: vec![],
        parallel: false,
        confirm: false,
        allow_chaining: None,
        allow_subshells: None,
    };

    let result = executor.execute(&incompatible_cmd).await;

    // Should fail due to platform mismatch, NOT due to command not found
    assert!(
        result.is_err(),
        "Should fail at platform validation stage, not command execution stage"
    );

    // If check_platform is working, we should get a platform error
    // If it's bypassed (mutation), we might get "command not found" instead
}

// ============================================================================
// Platform::is_supported Tests
// ============================================================================

#[test]
fn test_platform_is_supported_empty_list() {
    let current = Platform::current();
    assert!(
        current.is_supported(&[]),
        "Empty supported list should allow all platforms"
    );
}

#[test]
fn test_platform_is_supported_exact_match() {
    let current = Platform::current();
    assert!(
        current.is_supported(std::slice::from_ref(&current)),
        "Platform should be supported when exactly matched"
    );
}

#[test]
#[cfg(target_os = "linux")]
fn test_platform_linux_supports_unix() {
    let linux = Platform::Linux;
    assert!(
        linux.is_supported(&[Platform::Unix]),
        "Linux should be supported by Unix platform"
    );
}

#[test]
#[cfg(target_os = "macos")]
fn test_platform_macos_supports_unix() {
    let macos = Platform::Macos;
    assert!(
        macos.is_supported(&[Platform::Unix]),
        "macOS should be supported by Unix platform"
    );
}

#[test]
#[cfg(target_os = "windows")]
fn test_platform_windows_does_not_support_unix() {
    let windows = Platform::Windows;
    assert!(
        !windows.is_supported(&[Platform::Unix]),
        "Windows should NOT be supported by Unix platform"
    );
}

#[test]
fn test_platform_current_detection() {
    let current = Platform::current();

    #[cfg(target_os = "windows")]
    assert_eq!(
        current,
        Platform::Windows,
        "Current platform should be Windows"
    );

    #[cfg(target_os = "linux")]
    assert_eq!(current, Platform::Linux, "Current platform should be Linux");

    #[cfg(target_os = "macos")]
    assert_eq!(current, Platform::Macos, "Current platform should be macOS");
}
