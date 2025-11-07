//! cmdrun エラー型定義
//!
//! 包括的で情報豊富なエラーハンドリングを提供

use std::path::PathBuf;
use thiserror::Error;

/// cmdrun の全エラー型
#[derive(Error, Debug)]
pub enum CmdrunError {
    /// 設定ファイル関連エラー
    #[error("Configuration error: {0}")]
    Config(#[from] ConfigError),

    /// コマンド実行エラー
    #[error("Command execution error: {0}")]
    Execution(#[from] ExecutionError),

    /// 変数展開エラー
    #[error("Variable interpolation error: {0}")]
    Interpolation(#[from] InterpolationError),

    /// プラグイン関連エラー
    #[error("Plugin error in '{plugin}': {message}")]
    PluginError { plugin: String, message: String },

    /// プラグインロードエラー
    #[error("Failed to load plugin: {0}")]
    PluginLoad(String),

    /// IO エラー
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// その他のエラー
    #[error("Unexpected error: {0}")]
    Other(#[from] anyhow::Error),
}

/// 設定ファイル関連エラー
#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Config file not found: {0}")]
    FileNotFound(PathBuf),

    #[error("Failed to parse TOML: {source}\nFile: {file}")]
    ParseError {
        file: PathBuf,
        #[source]
        source: toml::de::Error,
    },

    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),

    #[error("Command not found: {0}")]
    CommandNotFound(String),

    #[error("Circular dependency detected: {0}")]
    CircularDependency(String),

    #[error("Missing required field: {field} in {context}")]
    MissingField { field: String, context: String },

    #[error("Invalid shell: {0}")]
    InvalidShell(String),

    #[error("Platform not supported: {platform} for command {command}")]
    UnsupportedPlatform { platform: String, command: String },
}

/// コマンド実行エラー
#[derive(Error, Debug)]
pub enum ExecutionError {
    #[error("Command failed with exit code {code}: {command}")]
    CommandFailed { command: String, code: i32 },

    #[error("Command timed out after {timeout}s: {command}")]
    Timeout { command: String, timeout: u64 },

    #[error("Shell not found: {0}")]
    ShellNotFound(String),

    #[error("Failed to spawn process: {command}\nReason: {source}")]
    SpawnFailed {
        command: String,
        #[source]
        source: std::io::Error,
    },

    #[error("User cancelled execution")]
    Cancelled,

    #[error("Dependency failed: {dependency} required by {command}")]
    DependencyFailed { dependency: String, command: String },
}

/// 変数展開エラー
#[derive(Error, Debug)]
pub enum InterpolationError {
    #[error("Undefined variable: {0}")]
    UndefinedVariable(String),

    #[error("Required variable not set: {0}")]
    RequiredVariableNotSet(String),

    #[error("Invalid variable syntax: {0}")]
    InvalidSyntax(String),

    #[error("Recursive variable expansion detected: {0}")]
    RecursiveExpansion(String),
}

/// Result型のエイリアス
pub type Result<T> = std::result::Result<T, CmdrunError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = ConfigError::CommandNotFound("test".to_string());
        assert_eq!(err.to_string(), "Command not found: test");
    }

    #[test]
    fn test_error_chain() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
        let exec_err = ExecutionError::SpawnFailed {
            command: "test".to_string(),
            source: io_err,
        };
        assert!(exec_err.to_string().contains("Failed to spawn process"));
    }
}
