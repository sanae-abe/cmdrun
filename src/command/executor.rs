//! コマンド実行エンジン
//!
//! 高性能・安全なコマンド実行機能を提供

use crate::command::interpolation::InterpolationContext;
use crate::config::schema::{Command, Platform};
use crate::error::{ExecutionError, Result};
use crate::security::{CommandValidator, SensitiveEnv};
use ahash::AHashMap;
use colored::*;
use std::path::PathBuf;
use std::process::Stdio;
use std::time::{Duration, Instant};
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command as TokioCommand;
use tokio::task::JoinSet;
use tokio::time::timeout;
use tracing::{debug, warn};

/// コマンド実行コンテキスト
#[derive(Debug, Clone)]
pub struct ExecutionContext {
    /// 作業ディレクトリ
    pub working_dir: PathBuf,
    /// 環境変数
    pub env: AHashMap<String, String>,
    /// シェル
    pub shell: String,
    /// タイムアウト（秒）
    pub timeout: Option<u64>,
    /// 厳格モード
    pub strict: bool,
    /// コマンドエコー
    pub echo: bool,
    /// カラー出力
    pub color: bool,
}

impl Default for ExecutionContext {
    fn default() -> Self {
        Self {
            working_dir: PathBuf::from("."),
            env: AHashMap::new(),
            shell: detect_shell(),
            timeout: Some(300),
            strict: true,
            echo: true,
            color: true,
        }
    }
}

/// コマンド実行結果
#[derive(Debug)]
pub struct ExecutionResult {
    /// 終了コード
    pub exit_code: i32,
    /// 実行時間
    pub duration: Duration,
    /// 標準出力
    pub stdout: String,
    /// 標準エラー出力
    pub stderr: String,
    /// 成功したか
    pub success: bool,
}

/// コマンドエグゼキューター
pub struct CommandExecutor {
    context: ExecutionContext,
    validator: CommandValidator,
    sensitive_env: SensitiveEnv,
}

impl CommandExecutor {
    /// 新規エグゼキューター作成
    pub fn new(context: ExecutionContext) -> Self {
        // バリデーターを設定（strictモードに応じて）
        let validator = if context.strict {
            // strictモードでも変数展開は許可する（安全な展開のみ）
            CommandValidator::new().allow_variable_expansion()
        } else {
            CommandValidator::new()
                .with_strict_mode(false)
                .allow_variable_expansion()
                .allow_pipe()
                .allow_redirect()
        };

        Self {
            context,
            validator,
            sensitive_env: SensitiveEnv::new(),
        }
    }

    /// コマンド実行
    pub async fn execute(&self, command: &Command) -> Result<ExecutionResult> {
        let start = Instant::now();

        // プラットフォームチェック
        self.check_platform(command)?;

        // コマンド文字列取得
        let commands = self.resolve_commands(command)?;

        // 環境変数マージ（コマンド固有の環境変数を追加）
        let mut merged_env = self.context.env.clone();
        merged_env.extend(command.env.clone());

        // 変数展開
        let interpolated_commands = self.interpolate_commands(&commands, command)?;

        // 実行
        let mut last_result = None;
        for cmd in interpolated_commands {
            let result = self.execute_single(&cmd, &merged_env).await?;
            if !result.success {
                return Err(ExecutionError::CommandFailed {
                    command: cmd,
                    code: result.exit_code,
                }
                .into());
            }
            last_result = Some(result);
        }

        let duration = start.elapsed();
        Ok(last_result.unwrap_or_else(|| ExecutionResult {
            exit_code: 0,
            duration,
            stdout: String::new(),
            stderr: String::new(),
            success: true,
        }))
    }

    /// プラットフォーム対応確認
    fn check_platform(&self, command: &Command) -> Result<()> {
        if command.platform.is_empty() {
            return Ok(());
        }

        let current = Platform::current();
        if !current.is_supported(&command.platform) {
            return Err(ExecutionError::CommandFailed {
                command: command.description.clone(),
                code: 1,
            }
            .into());
        }

        Ok(())
    }

    /// コマンド文字列解決
    fn resolve_commands(&self, command: &Command) -> Result<Vec<String>> {
        let platform = Platform::current();
        command.cmd.resolve_for_platform(&platform).ok_or_else(|| {
            ExecutionError::CommandFailed {
                command: command.description.clone(),
                code: 1,
            }
            .into()
        })
    }

    /// 変数展開
    fn interpolate_commands(&self, commands: &[String], command: &Command) -> Result<Vec<String>> {
        // 環境変数マージ
        let mut env = self.context.env.clone();
        env.extend(command.env.clone());

        // 展開コンテキスト作成
        let ctx = InterpolationContext::new(self.context.strict)
            .with_env_map(env)
            .merge_system_env();

        // 各コマンドを展開
        commands
            .iter()
            .map(|cmd| ctx.interpolate(cmd))
            .collect::<Result<Vec<_>>>()
    }

    /// 単一コマンド実行
    async fn execute_single(
        &self,
        command: &str,
        env: &AHashMap<String, String>,
    ) -> Result<ExecutionResult> {
        let start = Instant::now();

        // セキュリティ検証
        let validation_result = self.validator.validate(command);
        if !validation_result.is_safe() {
            if let Some(err) = validation_result.error() {
                warn!("Command validation failed: {}", err);
                return Err(ExecutionError::CommandFailed {
                    command: command.to_string(),
                    code: 1,
                }
                .into());
            }
        }

        // コマンドエコー
        if self.context.echo {
            self.print_command(command);
        }

        // シェルコマンド構築
        let (shell, args) = self.build_shell_command(command);

        // 環境変数のログ出力（機密情報マスキング）
        if self.context.echo && !env.is_empty() {
            let masked_env = self.sensitive_env.mask_ahash_map(env);
            debug!("Environment variables: {:?}", masked_env);
        }

        // プロセス起動
        let mut child = TokioCommand::new(&shell)
            .args(&args)
            .current_dir(&self.context.working_dir)
            .envs(env)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| ExecutionError::SpawnFailed {
                command: command.to_string(),
                source: e,
            })?;

        // 標準出力・エラー出力をキャプチャ
        let stdout = child.stdout.take().unwrap();
        let stderr = child.stderr.take().unwrap();

        let stdout_reader = BufReader::new(stdout);
        let stderr_reader = BufReader::new(stderr);

        // 非同期出力読み取り
        let stdout_handle = tokio::spawn(Self::read_output(stdout_reader, self.context.color));
        let stderr_handle = tokio::spawn(Self::read_output(stderr_reader, self.context.color));

        // タイムアウト付きプロセス待機
        let status = if let Some(timeout_secs) = self.context.timeout {
            match timeout(Duration::from_secs(timeout_secs), child.wait()).await {
                Ok(result) => result.map_err(|e| ExecutionError::SpawnFailed {
                    command: command.to_string(),
                    source: e,
                })?,
                Err(_) => {
                    // タイムアウト時はプロセスをキル
                    let _ = child.kill().await;
                    return Err(ExecutionError::Timeout {
                        command: command.to_string(),
                        timeout: timeout_secs,
                    }
                    .into());
                }
            }
        } else {
            child
                .wait()
                .await
                .map_err(|e| ExecutionError::SpawnFailed {
                    command: command.to_string(),
                    source: e,
                })?
        };

        // 出力取得
        let stdout_output = stdout_handle.await.unwrap();
        let stderr_output = stderr_handle.await.unwrap();

        let duration = start.elapsed();
        let exit_code = status.code().unwrap_or(-1);

        Ok(ExecutionResult {
            exit_code,
            duration,
            stdout: stdout_output,
            stderr: stderr_output,
            success: status.success(),
        })
    }

    /// シェルコマンド構築
    fn build_shell_command(&self, command: &str) -> (String, Vec<String>) {
        let shell = &self.context.shell;

        if cfg!(windows) {
            if shell.contains("pwsh") || shell.contains("powershell") {
                (
                    shell.clone(),
                    vec!["-Command".to_string(), command.to_string()],
                )
            } else {
                // cmd.exe
                (shell.clone(), vec!["/C".to_string(), command.to_string()])
            }
        } else {
            // Unix系
            (shell.clone(), vec!["-c".to_string(), command.to_string()])
        }
    }

    /// 出力読み取り（リアルタイム表示）
    async fn read_output<R>(reader: BufReader<R>, color: bool) -> String
    where
        R: tokio::io::AsyncRead + Unpin,
    {
        let mut output = String::new();
        let mut lines = reader.lines();

        while let Ok(Some(line)) = lines.next_line().await {
            if color {
                println!("{}", line);
            } else {
                println!("{}", line);
            }
            output.push_str(&line);
            output.push('\n');
        }

        output
    }

    /// コマンド表示
    fn print_command(&self, command: &str) {
        if self.context.color {
            eprintln!("{} {}", "→".cyan().bold(), command.bright_white());
        } else {
            eprintln!("→ {}", command);
        }
    }

    /// 複数コマンドを並列実行
    pub async fn execute_parallel(&self, commands: &[&Command]) -> Result<Vec<ExecutionResult>> {
        if commands.is_empty() {
            return Ok(Vec::new());
        }

        if self.context.color {
            eprintln!(
                "{} {} commands in parallel",
                "⚡".yellow().bold(),
                commands.len()
            );
        }

        let mut set = JoinSet::new();

        // 各コマンドを並列タスクとして起動
        for command in commands {
            let executor = self.clone_for_task();
            let cmd = (*command).clone();

            set.spawn(async move { executor.execute(&cmd).await });
        }

        // 全タスクの完了を待機
        let mut results = Vec::new();
        while let Some(result) = set.join_next().await {
            match result {
                Ok(Ok(exec_result)) => results.push(exec_result),
                Ok(Err(e)) => return Err(e),
                Err(e) => {
                    return Err(ExecutionError::CommandFailed {
                        command: format!("Parallel task failed: {}", e),
                        code: 1,
                    }
                    .into())
                }
            }
        }

        Ok(results)
    }

    /// タスク用のクローンを作成
    fn clone_for_task(&self) -> Self {
        Self {
            context: self.context.clone(),
            validator: CommandValidator::new(),
            sensitive_env: SensitiveEnv::new(),
        }
    }
}

/// デフォルトシェル検出
pub fn detect_shell() -> String {
    if cfg!(windows) {
        // Windows: PowerShell Core → PowerShell → cmd の優先順位
        if which::which("pwsh").is_ok() {
            "pwsh".to_string()
        } else if which::which("powershell").is_ok() {
            "powershell".to_string()
        } else {
            "cmd".to_string()
        }
    } else {
        // Unix系: SHELL環境変数 → bash
        std::env::var("SHELL")
            .ok()
            .and_then(|s| s.split('/').next_back().map(String::from))
            .unwrap_or_else(|| "bash".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::CommandSpec;

    #[tokio::test]
    async fn test_simple_command() {
        let ctx = ExecutionContext::default();
        let executor = CommandExecutor::new(ctx);

        let command = Command {
            description: "test".to_string(),
            cmd: CommandSpec::Single("echo hello".to_string()),
            env: AHashMap::new(),
            working_dir: None,
            deps: vec![],
            platform: vec![],
            tags: vec![],
            timeout: None,
            parallel: false,
            confirm: false,
        };

        let result = executor.execute(&command).await;
        assert!(result.is_ok());
        let result = result.unwrap();
        assert!(result.success);
        assert!(result.stdout.contains("hello"));
    }

    #[tokio::test]
    async fn test_variable_interpolation() {
        let mut ctx = ExecutionContext::default();
        ctx.env.insert("TEST_VAR".to_string(), "world".to_string());

        let executor = CommandExecutor::new(ctx);

        let command = Command {
            description: "test".to_string(),
            cmd: CommandSpec::Single("echo ${TEST_VAR}".to_string()),
            env: AHashMap::new(),
            working_dir: None,
            deps: vec![],
            platform: vec![],
            tags: vec![],
            timeout: None,
            parallel: false,
            confirm: false,
        };

        let result = executor.execute(&command).await.unwrap();
        assert!(result.stdout.contains("world"));
    }

    #[test]
    fn test_shell_detection() {
        let shell = detect_shell();
        assert!(!shell.is_empty());
        #[cfg(unix)]
        assert!(shell == "bash" || shell == "zsh" || shell == "fish" || shell.contains("sh"));
        #[cfg(windows)]
        assert!(shell.contains("pwsh") || shell.contains("powershell") || shell == "cmd");
    }
}
