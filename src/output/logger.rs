//! Logging utilities
//!
//! tracing ベースの構造化ロギング機能

use anyhow::Result;
use tracing::{debug, error, info, Level};
use tracing_subscriber::{
    fmt::{self, format::FmtSpan},
    layer::SubscriberExt,
    util::SubscriberInitExt,
    EnvFilter,
};

/// ロガー設定
#[derive(Debug, Clone)]
pub struct LoggerConfig {
    /// ログレベル
    pub level: LogLevel,

    /// JSON形式で出力
    pub json_output: bool,

    /// タイムスタンプを表示
    pub show_timestamps: bool,

    /// ターゲットモジュールを表示
    pub show_target: bool,

    /// ファイル出力先
    pub log_file: Option<String>,
}

/// ログレベル
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

impl From<LogLevel> for Level {
    fn from(level: LogLevel) -> Self {
        match level {
            LogLevel::Trace => Level::TRACE,
            LogLevel::Debug => Level::DEBUG,
            LogLevel::Info => Level::INFO,
            LogLevel::Warn => Level::WARN,
            LogLevel::Error => Level::ERROR,
        }
    }
}

impl LoggerConfig {
    /// 新しい設定を作成
    pub fn new() -> Self {
        Self {
            level: LogLevel::Info,
            json_output: false,
            show_timestamps: true,
            show_target: false,
            log_file: None,
        }
    }

    /// ログレベルを設定
    pub fn with_level(mut self, level: LogLevel) -> Self {
        self.level = level;
        self
    }

    /// JSON出力を設定
    pub fn with_json_output(mut self, enabled: bool) -> Self {
        self.json_output = enabled;
        self
    }

    /// タイムスタンプ表示を設定
    pub fn with_timestamps(mut self, enabled: bool) -> Self {
        self.show_timestamps = enabled;
        self
    }

    /// ターゲット表示を設定
    pub fn with_target(mut self, enabled: bool) -> Self {
        self.show_target = enabled;
        self
    }

    /// ファイル出力を設定
    pub fn with_log_file(mut self, path: String) -> Self {
        self.log_file = Some(path);
        self
    }
}

impl Default for LoggerConfig {
    fn default() -> Self {
        Self::new()
    }
}

/// ロガーを初期化
pub fn init_logger() -> Result<()> {
    init_logger_with_config(LoggerConfig::default())
}

/// 設定を指定してロガーを初期化
pub fn init_logger_with_config(config: LoggerConfig) -> Result<()> {
    // 環境変数 RUST_LOG があればそれを優先
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| {
        let level_str = match config.level {
            LogLevel::Trace => "trace",
            LogLevel::Debug => "debug",
            LogLevel::Info => "info",
            LogLevel::Warn => "warn",
            LogLevel::Error => "error",
        };

        // cmdrun のログのみを設定レベルで出力、他のクレートは warn 以上
        EnvFilter::new(format!("{}={}:warn", env!("CARGO_PKG_NAME"), level_str))
    });

    if config.json_output {
        // JSON形式で出力
        tracing_subscriber::registry()
            .with(env_filter)
            .with(
                fmt::layer()
                    .json()
                    .with_span_events(FmtSpan::CLOSE)
                    .with_current_span(false),
            )
            .try_init()
            .map_err(|e| anyhow::anyhow!("Failed to initialize logger: {}", e))?;
    } else {
        // 人間が読みやすい形式で出力
        if config.show_timestamps {
            tracing_subscriber::registry()
                .with(env_filter)
                .with(
                    fmt::layer()
                        .with_target(config.show_target)
                        .with_span_events(FmtSpan::NONE)
                        .with_timer(fmt::time::uptime()),
                )
                .try_init()
                .map_err(|e| anyhow::anyhow!("Failed to initialize logger: {}", e))?;
        } else {
            tracing_subscriber::registry()
                .with(env_filter)
                .with(
                    fmt::layer()
                        .with_target(config.show_target)
                        .with_span_events(FmtSpan::NONE)
                        .without_time(),
                )
                .try_init()
                .map_err(|e| anyhow::anyhow!("Failed to initialize logger: {}", e))?;
        }
    }

    debug!("Logger initialized with level: {:?}", config.level);
    Ok(())
}

/// 詳細モード用のロガーを初期化
pub fn init_verbose_logger() -> Result<()> {
    init_logger_with_config(
        LoggerConfig::new()
            .with_level(LogLevel::Debug)
            .with_target(true),
    )
}

/// 静かモード用のロガーを初期化
pub fn init_quiet_logger() -> Result<()> {
    init_logger_with_config(LoggerConfig::new().with_level(LogLevel::Warn))
}

/// コマンド実行をログに記録
pub fn log_command(command_name: &str, command: &str) {
    info!(
        command_name = command_name,
        command = command,
        "Executing command"
    );
}

/// コマンド完了をログに記録
pub fn log_command_success(command_name: &str, duration_ms: u64) {
    info!(
        command_name = command_name,
        duration_ms = duration_ms,
        "Command completed successfully"
    );
}

/// コマンド失敗をログに記録
pub fn log_command_failure(command_name: &str, exit_code: i32, error: &str) {
    error!(
        command_name = command_name,
        exit_code = exit_code,
        error = error,
        "Command failed"
    );
}

/// 依存関係の解決をログに記録
pub fn log_dependency_resolution(command_name: &str, dependencies: &[String]) {
    debug!(
        command_name = command_name,
        dependencies = ?dependencies,
        "Resolving dependencies"
    );
}

/// 設定読み込みをログに記録
pub fn log_config_loaded(config_path: &str, command_count: usize) {
    info!(
        config_path = config_path,
        command_count = command_count,
        "Configuration loaded"
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_logger_config() {
        let config = LoggerConfig::new()
            .with_level(LogLevel::Debug)
            .with_json_output(true)
            .with_timestamps(false)
            .with_target(true);

        assert_eq!(config.level, LogLevel::Debug);
        assert!(config.json_output);
        assert!(!config.show_timestamps);
        assert!(config.show_target);
    }

    #[test]
    fn test_log_level_conversion() {
        assert_eq!(Level::from(LogLevel::Trace), Level::TRACE);
        assert_eq!(Level::from(LogLevel::Debug), Level::DEBUG);
        assert_eq!(Level::from(LogLevel::Info), Level::INFO);
        assert_eq!(Level::from(LogLevel::Warn), Level::WARN);
        assert_eq!(Level::from(LogLevel::Error), Level::ERROR);
    }

    // 注意: 実際のロガー初期化テストは統合テストで行う
    // （一度しか初期化できないため、ユニットテストでは難しい）
}
