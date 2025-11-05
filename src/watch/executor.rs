//! Command execution for watch mode

use anyhow::{Context, Result};
use std::path::{Path, PathBuf};
use std::process::Stdio;
use tokio::process::Command;
use tracing::{debug, error, info};

/// Command executor for watch mode
#[derive(Clone, Debug)]
pub struct CommandExecutor {
    /// Working directory for command execution
    pub(crate) working_dir: PathBuf,

    /// Shell to use for command execution
    pub(crate) shell: String,
}

impl CommandExecutor {
    /// Create a new command executor
    pub fn new(working_dir: PathBuf) -> Self {
        let shell = Self::detect_shell();
        Self { working_dir, shell }
    }

    /// Detect the current shell
    fn detect_shell() -> String {
        #[cfg(unix)]
        {
            std::env::var("SHELL").unwrap_or_else(|_| "/bin/sh".to_string())
        }
        #[cfg(windows)]
        {
            std::env::var("COMSPEC").unwrap_or_else(|_| "cmd.exe".to_string())
        }
    }

    /// Execute a command asynchronously
    pub async fn execute(&self, command: &str, changed_path: &Path) -> Result<()> {
        info!(
            command = %command,
            path = %changed_path.display(),
            "Executing command due to file change"
        );

        let mut cmd = Command::new(&self.shell);

        #[cfg(unix)]
        {
            cmd.arg("-c");
        }
        #[cfg(windows)]
        {
            cmd.arg("/C");
        }

        cmd.arg(command)
            .current_dir(&self.working_dir)
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .stdin(Stdio::null());

        // Set environment variable with the changed file path
        cmd.env(
            "CMDRUN_CHANGED_FILE",
            changed_path.to_string_lossy().as_ref(),
        );

        debug!(shell = %self.shell, "Spawning command");

        let status = cmd
            .status()
            .await
            .with_context(|| format!("Failed to execute command: {}", command))?;

        if status.success() {
            info!(exit_code = %status, "Command completed successfully");
        } else {
            error!(exit_code = %status, "Command failed");
        }

        Ok(())
    }

    /// Execute a command with custom environment variables
    pub async fn execute_with_env(
        &self,
        command: &str,
        changed_path: &Path,
        env: &[(String, String)],
    ) -> Result<()> {
        info!(
            command = %command,
            path = %changed_path.display(),
            env_count = env.len(),
            "Executing command with custom environment"
        );

        let mut cmd = Command::new(&self.shell);

        #[cfg(unix)]
        {
            cmd.arg("-c");
        }
        #[cfg(windows)]
        {
            cmd.arg("/C");
        }

        cmd.arg(command)
            .current_dir(&self.working_dir)
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .stdin(Stdio::null());

        // Set environment variables
        cmd.env(
            "CMDRUN_CHANGED_FILE",
            changed_path.to_string_lossy().as_ref(),
        );
        for (key, value) in env {
            cmd.env(key, value);
        }

        let status = cmd
            .status()
            .await
            .with_context(|| format!("Failed to execute command: {}", command))?;

        if status.success() {
            info!(exit_code = %status, "Command completed successfully");
        } else {
            error!(exit_code = %status, "Command failed");
        }

        Ok(())
    }

    /// Set working directory
    pub fn with_working_dir(mut self, dir: PathBuf) -> Self {
        self.working_dir = dir;
        self
    }

    /// Set shell
    pub fn with_shell(mut self, shell: String) -> Self {
        self.shell = shell;
        self
    }

    /// Get working directory
    pub fn working_dir(&self) -> &PathBuf {
        &self.working_dir
    }

    /// Get shell
    pub fn shell(&self) -> &str {
        &self.shell
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_detect_shell() {
        let shell = CommandExecutor::detect_shell();
        #[cfg(unix)]
        assert!(shell.contains("sh") || shell.contains("bash") || shell.contains("zsh"));
        #[cfg(windows)]
        assert!(shell.contains("cmd") || shell.contains("powershell"));
    }

    #[test]
    fn test_new_executor() {
        let temp_dir = TempDir::new().unwrap();
        let executor = CommandExecutor::new(temp_dir.path().to_path_buf());
        assert_eq!(executor.working_dir, temp_dir.path());
    }

    #[test]
    fn test_builder_pattern() {
        let temp_dir = TempDir::new().unwrap();
        let executor = CommandExecutor::new(PathBuf::from("/tmp"))
            .with_working_dir(temp_dir.path().to_path_buf())
            .with_shell("/bin/bash".to_string());

        assert_eq!(executor.working_dir, temp_dir.path());
        assert_eq!(executor.shell, "/bin/bash");
    }

    #[tokio::test]
    async fn test_execute_simple_command() {
        let temp_dir = TempDir::new().unwrap();
        let executor = CommandExecutor::new(temp_dir.path().to_path_buf());
        let changed_file = PathBuf::from("test.txt");

        #[cfg(unix)]
        let result = executor.execute("echo 'test'", &changed_file).await;
        #[cfg(windows)]
        let result = executor.execute("echo test", &changed_file).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_execute_with_env() {
        let temp_dir = TempDir::new().unwrap();
        let executor = CommandExecutor::new(temp_dir.path().to_path_buf());
        let changed_file = PathBuf::from("test.txt");
        let env = vec![("TEST_VAR".to_string(), "test_value".to_string())];

        #[cfg(unix)]
        let result = executor
            .execute_with_env("echo $TEST_VAR", &changed_file, &env)
            .await;
        #[cfg(windows)]
        let result = executor
            .execute_with_env("echo %TEST_VAR%", &changed_file, &env)
            .await;

        assert!(result.is_ok());
    }
}
