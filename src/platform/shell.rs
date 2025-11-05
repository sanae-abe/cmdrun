//! Platform-specific shell detection and command execution
//!
//! Unix（Linux/macOS）とWindows両対応のシェル検出・実行機能

use anyhow::{Context, Result};
use std::env;
use std::path::PathBuf;
use tracing::{debug, warn};

/// シェル情報
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShellInfo {
    /// シェル名（bash, zsh, fish, pwsh等）
    pub name: String,

    /// シェルの実行可能パス
    pub path: PathBuf,

    /// プラットフォーム種別
    pub platform: Platform,
}

/// プラットフォーム種別
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Platform {
    Unix,
    Windows,
}

impl ShellInfo {
    /// シェル情報を作成
    pub fn new(name: String, path: PathBuf, platform: Platform) -> Self {
        Self {
            name,
            path,
            platform,
        }
    }

    /// コマンドライン引数を取得（シェル実行用）
    pub fn get_command_args(&self, script: &str) -> Vec<String> {
        match self.platform {
            Platform::Unix => vec!["-c".to_string(), script.to_string()],
            Platform::Windows => {
                // PowerShell の場合
                if self.name.contains("pwsh") || self.name.contains("powershell") {
                    vec![
                        "-NoProfile".to_string(),
                        "-NonInteractive".to_string(),
                        "-Command".to_string(),
                        script.to_string(),
                    ]
                } else {
                    // cmd.exe の場合
                    vec!["/C".to_string(), script.to_string()]
                }
            }
        }
    }
}

/// 現在のプラットフォームのデフォルトシェルを検出
pub fn detect_shell() -> Result<ShellInfo> {
    if cfg!(target_os = "windows") {
        detect_windows_shell()
    } else {
        detect_unix_shell()
    }
}

/// 指定されたシェル名でシェル情報を取得
pub fn get_shell_by_name(name: &str) -> Result<ShellInfo> {
    let platform = if cfg!(target_os = "windows") {
        Platform::Windows
    } else {
        Platform::Unix
    };

    // シェルのパスを検索
    let path = which::which(name).with_context(|| format!("Shell '{}' not found in PATH", name))?;

    debug!("Found shell '{}' at {}", name, path.display());

    Ok(ShellInfo::new(name.to_string(), path, platform))
}

/// Unix系のシェルを検出
fn detect_unix_shell() -> Result<ShellInfo> {
    // 優先順位:
    // 1. SHELL環境変数
    // 2. よく使われるシェルの順（bash, zsh, fish, sh）

    // 環境変数 SHELL をチェック
    if let Ok(shell_path) = env::var("SHELL") {
        let path = PathBuf::from(&shell_path);
        if path.exists() {
            let name = path
                .file_name()
                .and_then(|s| s.to_str())
                .unwrap_or("sh")
                .to_string();

            debug!("Using shell from SHELL env: {}", shell_path);
            return Ok(ShellInfo::new(name, path, Platform::Unix));
        }
    }

    // よく使われるシェルを順番に試す
    let common_shells = ["bash", "zsh", "fish", "sh"];

    for shell in &common_shells {
        if let Ok(path) = which::which(shell) {
            debug!("Found shell: {} at {}", shell, path.display());
            return Ok(ShellInfo::new(shell.to_string(), path, Platform::Unix));
        }
    }

    anyhow::bail!("No suitable Unix shell found")
}

/// Windowsのシェルを検出
fn detect_windows_shell() -> Result<ShellInfo> {
    // 優先順位:
    // 1. PowerShell Core (pwsh)
    // 2. Windows PowerShell (powershell)
    // 3. cmd.exe

    let common_shells = ["pwsh", "powershell", "cmd"];

    for shell in &common_shells {
        if let Ok(path) = which::which(shell) {
            debug!("Found Windows shell: {} at {}", shell, path.display());
            return Ok(ShellInfo::new(shell.to_string(), path, Platform::Windows));
        }
    }

    // Fallback: cmd.exe は通常 System32 にある
    let cmd_path = PathBuf::from(r"C:\Windows\System32\cmd.exe");
    if cmd_path.exists() {
        debug!("Using fallback cmd.exe");
        return Ok(ShellInfo::new(
            "cmd".to_string(),
            cmd_path,
            Platform::Windows,
        ));
    }

    anyhow::bail!("No suitable Windows shell found")
}

/// シェルの機能を検出
pub fn detect_shell_features(shell: &ShellInfo) -> ShellFeatures {
    let mut features = ShellFeatures::default();

    match shell.name.as_str() {
        "bash" | "zsh" | "fish" => {
            features.supports_pipes = true;
            features.supports_redirects = true;
            features.supports_background = true;
            features.supports_job_control = true;
        }
        "sh" => {
            features.supports_pipes = true;
            features.supports_redirects = true;
            features.supports_background = true;
        }
        "pwsh" | "powershell" => {
            features.supports_pipes = true;
            features.supports_redirects = true;
            features.supports_background = true;
        }
        "cmd" => {
            features.supports_pipes = true;
            features.supports_redirects = true;
        }
        _ => {
            warn!("Unknown shell: {}, assuming basic features", shell.name);
        }
    }

    features
}

/// シェルの機能情報
#[derive(Debug, Clone, Default)]
pub struct ShellFeatures {
    /// パイプ対応
    pub supports_pipes: bool,

    /// リダイレクト対応
    pub supports_redirects: bool,

    /// バックグラウンド実行対応
    pub supports_background: bool,

    /// ジョブコントロール対応
    pub supports_job_control: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_shell() {
        let shell = detect_shell();
        assert!(shell.is_ok(), "Should detect a shell");

        let shell = shell.unwrap();
        assert!(!shell.name.is_empty());
        assert!(shell.path.exists());
    }

    #[test]
    fn test_shell_command_args_unix() {
        let shell = ShellInfo::new(
            "bash".to_string(),
            PathBuf::from("/bin/bash"),
            Platform::Unix,
        );

        let args = shell.get_command_args("echo hello");
        assert_eq!(args, vec!["-c", "echo hello"]);
    }

    #[test]
    fn test_shell_command_args_pwsh() {
        let shell = ShellInfo::new("pwsh".to_string(), PathBuf::from("pwsh"), Platform::Windows);

        let args = shell.get_command_args("echo hello");
        assert_eq!(args.len(), 4);
        assert_eq!(args[0], "-NoProfile");
        assert_eq!(args[1], "-NonInteractive");
        assert_eq!(args[2], "-Command");
        assert_eq!(args[3], "echo hello");
    }

    #[test]
    fn test_shell_command_args_cmd() {
        let shell = ShellInfo::new(
            "cmd".to_string(),
            PathBuf::from("cmd.exe"),
            Platform::Windows,
        );

        let args = shell.get_command_args("echo hello");
        assert_eq!(args, vec!["/C", "echo hello"]);
    }

    #[test]
    fn test_detect_shell_features() {
        let bash_shell = ShellInfo::new(
            "bash".to_string(),
            PathBuf::from("/bin/bash"),
            Platform::Unix,
        );

        let features = detect_shell_features(&bash_shell);
        assert!(features.supports_pipes);
        assert!(features.supports_redirects);
        assert!(features.supports_background);
        assert!(features.supports_job_control);
    }

    #[test]
    fn test_get_shell_by_name() {
        // システムにあるシェルを検索
        #[cfg(unix)]
        {
            let result = get_shell_by_name("sh");
            assert!(result.is_ok(), "sh should be available on Unix systems");
        }

        #[cfg(windows)]
        {
            let result = get_shell_by_name("cmd");
            assert!(result.is_ok(), "cmd should be available on Windows");
        }
    }
}
