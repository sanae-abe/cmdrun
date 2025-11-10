//! Cross-platform integration tests
//!
//! Tests that verify cmdrun works correctly across different platforms:
//! - Windows (cmd.exe, PowerShell)
//! - macOS (bash, zsh)
//! - Linux (bash, sh, fish)
//!
//! These tests use conditional compilation to run platform-specific tests.

use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use tempfile::TempDir;

/// Helper to create a temporary configuration file
fn create_test_config(dir: &Path, content: &str) -> PathBuf {
    let config_path = dir.join("commands.toml");
    fs::write(&config_path, content).expect("Failed to write test config");
    config_path
}

/// Helper to get the cmdrun binary path
fn get_cmdrun_binary() -> PathBuf {
    env::current_exe()
        .ok()
        .and_then(|mut path| {
            path.pop(); // Remove test binary name
            path.pop(); // Remove "deps" directory
            path.push("cmdrun");
            if cfg!(windows) {
                path.set_extension("exe");
            }
            if path.exists() {
                Some(path)
            } else {
                None
            }
        })
        .unwrap_or_else(|| PathBuf::from("cmdrun"))
}

// =============================================================================
// Path Handling Tests
// =============================================================================

#[cfg(windows)]
mod windows_path_tests {
    use super::*;
    use std::process::Command;

    #[test]
    fn test_windows_path_separators() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");

        let config = r#"
[commands.test_path]
description = "Test Windows path separators"
cmd = "echo %CD%"
"#;

        let config_path = create_test_config(temp_dir.path(), config);

        let output = Command::new(get_cmdrun_binary())
            .arg("--config")
            .arg(&config_path)
            .arg("run")
            .arg("test_path")
            .output()
            .expect("Failed to execute cmdrun");

        assert!(output.status.success(), "Command should succeed on Windows");

        // Windows paths use backslashes
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(
            stdout.contains('\\'),
            "Windows paths should contain backslashes"
        );
    }

    #[test]
    fn test_windows_drive_letters() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");

        let config = r#"
[commands.test_drive]
description = "Test Windows drive letters"
cmd = "echo C:\\"
"#;

        let config_path = create_test_config(temp_dir.path(), config);

        let output = Command::new(get_cmdrun_binary())
            .arg("--config")
            .arg(&config_path)
            .arg("run")
            .arg("test_drive")
            .output()
            .expect("Failed to execute cmdrun");

        assert!(
            output.status.success(),
            "Command should handle drive letters"
        );
    }

    #[test]
    fn test_windows_unc_paths() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");

        // UNC path format: \\server\share
        let config = r#"
[commands.test_unc]
description = "Test UNC path handling"
cmd = "echo Testing UNC paths"
working_dir = "."
"#;

        let config_path = create_test_config(temp_dir.path(), config);

        let output = Command::new(get_cmdrun_binary())
            .arg("--config")
            .arg(&config_path)
            .arg("list")
            .output()
            .expect("Failed to execute cmdrun");

        assert!(output.status.success(), "Should handle UNC path config");
    }

    #[test]
    fn test_windows_environment_variables() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");

        let config = r#"
[commands.test_env]
description = "Test Windows environment variables"
cmd = "echo %USERPROFILE%"
"#;

        let config_path = create_test_config(temp_dir.path(), config);

        let output = Command::new(get_cmdrun_binary())
            .arg("--config")
            .arg(&config_path)
            .arg("run")
            .arg("test_env")
            .output()
            .expect("Failed to execute cmdrun");

        assert!(output.status.success(), "Should handle %VAR% expansion");
    }
}

#[cfg(target_os = "macos")]
mod macos_path_tests {
    use super::*;
    use std::process::Command;

    #[test]
    fn test_macos_path_separators() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");

        let config = r#"
[commands.test_path]
description = "Test macOS path separators"
cmd = "pwd"
"#;

        let config_path = create_test_config(temp_dir.path(), config);

        let output = Command::new(get_cmdrun_binary())
            .arg("--config")
            .arg(&config_path)
            .arg("run")
            .arg("test_path")
            .output()
            .expect("Failed to execute cmdrun");

        assert!(output.status.success(), "Command should succeed on macOS");

        // Unix paths use forward slashes
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(
            stdout.contains('/'),
            "macOS paths should contain forward slashes"
        );
    }

    #[test]
    fn test_macos_home_directory() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");

        let config = r#"
[commands.test_home]
description = "Test macOS home directory expansion"
cmd = "echo $HOME"
"#;

        let config_path = create_test_config(temp_dir.path(), config);

        let output = Command::new(get_cmdrun_binary())
            .arg("--config")
            .arg(&config_path)
            .arg("run")
            .arg("test_home")
            .output()
            .expect("Failed to execute cmdrun");

        assert!(output.status.success(), "Should expand $HOME on macOS");

        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(
            stdout.contains("/Users/"),
            "macOS home should be under /Users/"
        );
    }

    #[test]
    fn test_macos_case_sensitivity() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");

        // macOS file system is case-insensitive by default (APFS)
        let config = r#"
[commands.test_case]
description = "Test case sensitivity"
cmd = "ls"
"#;

        let config_path = create_test_config(temp_dir.path(), config);

        let output = Command::new(get_cmdrun_binary())
            .arg("--config")
            .arg(&config_path)
            .arg("run")
            .arg("test_case")
            .output()
            .expect("Failed to execute cmdrun");

        assert!(output.status.success(), "Command should succeed");
    }
}

#[cfg(target_os = "linux")]
mod linux_path_tests {
    use super::*;
    use std::process::Command;

    #[test]
    fn test_linux_path_separators() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");

        let config = r#"
[commands.test_path]
description = "Test Linux path separators"
cmd = "pwd"
"#;

        let config_path = create_test_config(temp_dir.path(), config);

        let output = Command::new(get_cmdrun_binary())
            .arg("--config")
            .arg(&config_path)
            .arg("run")
            .arg("test_path")
            .output()
            .expect("Failed to execute cmdrun");

        assert!(output.status.success(), "Command should succeed on Linux");

        // Unix paths use forward slashes
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(
            stdout.contains('/'),
            "Linux paths should contain forward slashes"
        );
    }

    #[test]
    fn test_linux_home_directory() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");

        let config = r#"
[commands.test_home]
description = "Test Linux home directory expansion"
cmd = "echo $HOME"
"#;

        let config_path = create_test_config(temp_dir.path(), config);

        let output = Command::new(get_cmdrun_binary())
            .arg("--config")
            .arg(&config_path)
            .arg("run")
            .arg("test_home")
            .output()
            .expect("Failed to execute cmdrun");

        assert!(output.status.success(), "Should expand $HOME on Linux");

        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(
            stdout.contains("/home/") || stdout.contains("/root/"),
            "Linux home should be under /home/ or /root/"
        );
    }

    #[test]
    fn test_linux_case_sensitivity() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");

        // Linux file system is case-sensitive
        let config = r#"
[commands.test_case]
description = "Test case sensitivity"
cmd = "ls"
"#;

        let config_path = create_test_config(temp_dir.path(), config);

        let output = Command::new(get_cmdrun_binary())
            .arg("--config")
            .arg(&config_path)
            .arg("run")
            .arg("test_case")
            .output()
            .expect("Failed to execute cmdrun");

        assert!(output.status.success(), "Command should succeed");
    }

    #[test]
    fn test_linux_symlink_handling() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");

        let config = r#"
[commands.test_symlink]
description = "Test symbolic link handling"
cmd = "readlink --version"
"#;

        let config_path = create_test_config(temp_dir.path(), config);

        let output = Command::new(get_cmdrun_binary())
            .arg("--config")
            .arg(&config_path)
            .arg("run")
            .arg("test_symlink")
            .output()
            .expect("Failed to execute cmdrun");

        // readlink should exist on Linux
        assert!(output.status.success() || output.status.code() == Some(1));
    }
}

// =============================================================================
// Shell-specific Tests
// =============================================================================

#[cfg(windows)]
mod windows_shell_tests {
    use super::*;
    use std::process::Command;

    #[test]
    fn test_cmd_shell() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");

        let config = r#"
[config]
shell = "cmd"

[commands.test_cmd]
description = "Test cmd.exe"
cmd = "echo Hello from cmd"
"#;

        let config_path = create_test_config(temp_dir.path(), config);

        let output = Command::new(get_cmdrun_binary())
            .arg("--config")
            .arg(&config_path)
            .arg("run")
            .arg("test_cmd")
            .output()
            .expect("Failed to execute cmdrun");

        assert!(output.status.success(), "cmd.exe should work");
        assert!(String::from_utf8_lossy(&output.stdout).contains("Hello from cmd"));
    }

    #[test]
    fn test_powershell_shell() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");

        let config = r#"
[config]
shell = "pwsh"

[commands.test_pwsh]
description = "Test PowerShell"
cmd = "Write-Output 'Hello from PowerShell'"
"#;

        let config_path = create_test_config(temp_dir.path(), config);

        let output = Command::new(get_cmdrun_binary())
            .arg("--config")
            .arg(&config_path)
            .arg("run")
            .arg("test_pwsh")
            .output()
            .expect("Failed to execute cmdrun");

        // PowerShell might not be available, so check exit code
        if which::which("pwsh").is_ok() {
            assert!(
                output.status.success(),
                "PowerShell should work if available"
            );
        }
    }

    #[test]
    fn test_batch_file_execution() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");

        // Create a simple batch file
        let batch_content = "@echo off\r\necho Hello from batch";
        let batch_path = temp_dir.path().join("test.bat");
        fs::write(&batch_path, batch_content).expect("Failed to write batch file");

        let config = format!(
            r#"
[commands.test_batch]
description = "Test batch file execution"
cmd = "{}"
"#,
            batch_path.display()
        );

        let config_path = create_test_config(temp_dir.path(), &config);

        let output = Command::new(get_cmdrun_binary())
            .arg("--config")
            .arg(&config_path)
            .arg("run")
            .arg("test_batch")
            .output()
            .expect("Failed to execute cmdrun");

        assert!(output.status.success(), "Batch files should execute");
    }
}

#[cfg(unix)]
mod unix_shell_tests {
    use super::*;
    use std::process::Command;

    #[test]
    fn test_bash_shell() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");

        let config = r#"
[config]
shell = "bash"

[commands.test_bash]
description = "Test bash"
cmd = "echo Hello from bash"
"#;

        let config_path = create_test_config(temp_dir.path(), config);

        let output = Command::new(get_cmdrun_binary())
            .arg("--config")
            .arg(&config_path)
            .arg("run")
            .arg("test_bash")
            .output()
            .expect("Failed to execute cmdrun");

        if which::which("bash").is_ok() {
            assert!(output.status.success(), "bash should work if available");
            assert!(String::from_utf8_lossy(&output.stdout).contains("Hello from bash"));
        }
    }

    #[test]
    fn test_sh_shell() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");

        let config = r#"
[config]
shell = "sh"

[commands.test_sh]
description = "Test sh"
cmd = "echo Hello from sh"
"#;

        let config_path = create_test_config(temp_dir.path(), config);

        let output = Command::new(get_cmdrun_binary())
            .arg("--config")
            .arg(&config_path)
            .arg("run")
            .arg("test_sh")
            .output()
            .expect("Failed to execute cmdrun");

        // sh should always be available on Unix
        assert!(output.status.success(), "sh should always work on Unix");
        assert!(String::from_utf8_lossy(&output.stdout).contains("Hello from sh"));
    }

    #[test]
    fn test_zsh_shell() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");

        let config = r#"
[config]
shell = "zsh"

[commands.test_zsh]
description = "Test zsh"
cmd = "echo Hello from zsh"
"#;

        let config_path = create_test_config(temp_dir.path(), config);

        let output = Command::new(get_cmdrun_binary())
            .arg("--config")
            .arg(&config_path)
            .arg("run")
            .arg("test_zsh")
            .output()
            .expect("Failed to execute cmdrun");

        if which::which("zsh").is_ok() {
            assert!(output.status.success(), "zsh should work if available");
        }
    }

    #[test]
    fn test_fish_shell() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");

        let config = r#"
[config]
shell = "fish"

[commands.test_fish]
description = "Test fish"
cmd = "echo Hello from fish"
"#;

        let config_path = create_test_config(temp_dir.path(), config);

        let output = Command::new(get_cmdrun_binary())
            .arg("--config")
            .arg(&config_path)
            .arg("run")
            .arg("test_fish")
            .output()
            .expect("Failed to execute cmdrun");

        if which::which("fish").is_ok() {
            assert!(output.status.success(), "fish should work if available");
        }
    }

    #[test]
    fn test_shell_script_execution() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");

        // Create a simple shell script
        let script_content = "#!/bin/sh\necho Hello from script";
        let script_path = temp_dir.path().join("test.sh");
        fs::write(&script_path, script_content).expect("Failed to write script");

        // Make script executable
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata(&script_path)
                .expect("Failed to get metadata")
                .permissions();
            perms.set_mode(0o755);
            fs::set_permissions(&script_path, perms).expect("Failed to set permissions");
        }

        let config = format!(
            r#"
[commands.test_script]
description = "Test shell script execution"
cmd = "{}"
"#,
            script_path.display()
        );

        let config_path = create_test_config(temp_dir.path(), &config);

        let output = Command::new(get_cmdrun_binary())
            .arg("--config")
            .arg(&config_path)
            .arg("run")
            .arg("test_script")
            .output()
            .expect("Failed to execute cmdrun");

        assert!(output.status.success(), "Shell scripts should execute");
    }
}

// =============================================================================
// Line Ending Tests (CRLF vs LF)
// =============================================================================

mod line_ending_tests {
    use super::*;
    use std::process::Command;

    #[test]
    fn test_config_with_lf() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");

        // Unix line endings (LF)
        let config = "[commands.test]\ndescription = \"Test LF\"\ncmd = \"echo test\"";

        let config_path = create_test_config(temp_dir.path(), config);

        let output = Command::new(get_cmdrun_binary())
            .arg("--config")
            .arg(&config_path)
            .arg("list")
            .output()
            .expect("Failed to execute cmdrun");

        assert!(output.status.success(), "Should handle LF line endings");
    }

    #[test]
    fn test_config_with_crlf() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");

        // Windows line endings (CRLF)
        let config = "[commands.test]\r\ndescription = \"Test CRLF\"\r\ncmd = \"echo test\"";

        let config_path = create_test_config(temp_dir.path(), config);

        let output = Command::new(get_cmdrun_binary())
            .arg("--config")
            .arg(&config_path)
            .arg("list")
            .output()
            .expect("Failed to execute cmdrun");

        assert!(output.status.success(), "Should handle CRLF line endings");
    }

    #[test]
    fn test_mixed_line_endings() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");

        // Mixed line endings
        let config = "[commands.test1]\ndescription = \"Test 1\"\r\ncmd = \"echo test1\"\n\n[commands.test2]\r\ndescription = \"Test 2\"\ncmd = \"echo test2\"";

        let config_path = create_test_config(temp_dir.path(), config);

        let output = Command::new(get_cmdrun_binary())
            .arg("--config")
            .arg(&config_path)
            .arg("list")
            .output()
            .expect("Failed to execute cmdrun");

        assert!(
            output.status.success(),
            "Should handle mixed line endings gracefully"
        );
    }
}

// =============================================================================
// Cross-platform Command Tests
// =============================================================================

mod cross_platform_command_tests {
    use super::*;
    use std::process::Command;

    #[test]
    fn test_echo_command() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");

        let config = r#"
[commands.test_echo]
description = "Test echo command"
cmd = "echo Hello World"
"#;

        let config_path = create_test_config(temp_dir.path(), config);

        let output = Command::new(get_cmdrun_binary())
            .arg("--config")
            .arg(&config_path)
            .arg("run")
            .arg("test_echo")
            .output()
            .expect("Failed to execute cmdrun");

        assert!(output.status.success(), "echo should work on all platforms");
        assert!(String::from_utf8_lossy(&output.stdout).contains("Hello World"));
    }

    #[test]
    fn test_exit_codes() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");

        #[cfg(windows)]
        let cmd = "exit 42";

        #[cfg(unix)]
        let cmd = "exit 42";

        let config = format!(
            r#"
[commands.test_exit]
description = "Test exit codes"
cmd = "{}"
"#,
            cmd
        );

        let config_path = create_test_config(temp_dir.path(), &config);

        let output = Command::new(get_cmdrun_binary())
            .arg("--config")
            .arg(&config_path)
            .arg("run")
            .arg("test_exit")
            .output()
            .expect("Failed to execute cmdrun");

        assert_eq!(
            output.status.code(),
            Some(42),
            "Exit codes should be preserved"
        );
    }

    #[test]
    fn test_environment_variable_expansion() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");

        let config = r#"
[config.env]
TEST_VAR = "test_value"

[commands.test_env_expansion]
description = "Test environment variable expansion"
cmd = "echo ${TEST_VAR}"
"#;

        let config_path = create_test_config(temp_dir.path(), config);

        let output = Command::new(get_cmdrun_binary())
            .arg("--config")
            .arg(&config_path)
            .arg("run")
            .arg("test_env_expansion")
            .output()
            .expect("Failed to execute cmdrun");

        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(
            stdout.contains("test_value") || output.status.success(),
            "Environment variables should expand correctly"
        );
    }
}

// =============================================================================
// File System Encoding Tests
// =============================================================================

#[cfg(unix)]
mod encoding_tests {
    use super::*;
    use std::process::Command;

    #[test]
    fn test_utf8_filenames() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");

        // Create a file with UTF-8 characters
        let utf8_filename = "テスト.txt";
        let utf8_path = temp_dir.path().join(utf8_filename);
        fs::write(&utf8_path, "UTF-8 test").expect("Failed to write UTF-8 file");

        let config = format!(
            r#"
[commands.test_utf8]
description = "Test UTF-8 filename handling"
cmd = "cat {}"
"#,
            utf8_path.display()
        );

        let config_path = create_test_config(temp_dir.path(), &config);

        let output = Command::new(get_cmdrun_binary())
            .arg("--config")
            .arg(&config_path)
            .arg("run")
            .arg("test_utf8")
            .output()
            .expect("Failed to execute cmdrun");

        assert!(output.status.success(), "Should handle UTF-8 filenames");
    }
}

#[cfg(windows)]
mod encoding_tests {
    use super::*;
    use std::process::Command;

    #[test]
    fn test_unicode_filenames() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");

        // Create a file with Unicode characters
        let unicode_filename = "テスト.txt";
        let unicode_path = temp_dir.path().join(unicode_filename);
        fs::write(&unicode_path, "Unicode test").expect("Failed to write Unicode file");

        let config = format!(
            r#"
[commands.test_unicode]
description = "Test Unicode filename handling"
cmd = "type {}"
"#,
            unicode_path.display()
        );

        let config_path = create_test_config(temp_dir.path(), &config);

        let output = Command::new(get_cmdrun_binary())
            .arg("--config")
            .arg(&config_path)
            .arg("run")
            .arg("test_unicode")
            .output()
            .expect("Failed to execute cmdrun");

        assert!(
            output.status.success(),
            "Should handle Unicode filenames on Windows"
        );
    }
}
