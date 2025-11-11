//! E2E Test Framework for cmdrun
//!
//! ユーザー視点での完全なワークフロー検証のための統合テストフレームワーク

use std::path::PathBuf;
use std::process::{Command, Output};
use tempfile::TempDir;

/// E2Eテスト用の分離された環境を提供
///
/// # 特徴
/// - 一時ディレクトリでの完全な分離
/// - cmdrunバイナリへの直接アクセス
/// - 設定ファイルの自動管理
/// - クリーンアップの自動化
pub struct CmdrunTestEnv {
    temp_dir: TempDir,
    config_dir: PathBuf,
    binary_path: PathBuf,
}

impl CmdrunTestEnv {
    /// 新しいテスト環境を作成
    ///
    /// # Example
    /// ```no_run
    /// let env = CmdrunTestEnv::new();
    /// let output = env.run_command(&["--version"]);
    /// env.assert_success(&output);
    /// ```
    pub fn new() -> Self {
        let temp_dir = TempDir::new().expect("Failed to create temp directory");
        let config_dir = temp_dir.path().join(".cmdrun");

        // .cmdrunディレクトリを作成
        std::fs::create_dir_all(&config_dir).expect("Failed to create .cmdrun directory");

        // バイナリパスを検出（debug or release）
        let binary_path = if cfg!(debug_assertions) {
            PathBuf::from("target/debug/cmdrun")
        } else {
            PathBuf::from("target/release/cmdrun")
        };

        Self {
            temp_dir,
            config_dir,
            binary_path,
        }
    }

    /// cmdrunコマンドを実行
    ///
    /// # Arguments
    /// * `args` - コマンドライン引数のスライス
    ///
    /// # Example
    /// ```no_run
    /// let env = CmdrunTestEnv::new();
    /// let output = env.run_command(&["add", "test", "echo hello"]);
    /// ```
    pub fn run_command(&self, args: &[&str]) -> Output {
        Command::new(&self.binary_path)
            .args(args)
            .current_dir(self.temp_dir.path())
            .output()
            .expect("Failed to execute cmdrun")
    }

    /// コマンド実行が成功したことをアサート
    ///
    /// # Panics
    /// コマンドが失敗した場合、stdout/stderrとともにパニックします
    pub fn assert_success(&self, output: &Output) {
        if !output.status.success() {
            panic!(
                "❌ Command failed with exit code {:?}\n\nstdout:\n{}\n\nstderr:\n{}",
                output.status.code(),
                String::from_utf8_lossy(&output.stdout),
                String::from_utf8_lossy(&output.stderr)
            );
        }
    }

    /// コマンド実行が失敗したことをアサート
    ///
    /// # Panics
    /// コマンドが成功した場合にパニックします
    pub fn assert_failure(&self, output: &Output) {
        if output.status.success() {
            panic!(
                "❌ Expected command to fail, but it succeeded\n\nstdout:\n{}",
                String::from_utf8_lossy(&output.stdout)
            );
        }
    }

    /// stdoutに期待される文字列が含まれることをアサート
    ///
    /// # Arguments
    /// * `output` - コマンド実行結果
    /// * `expected` - 期待される文字列
    pub fn assert_stdout_contains(&self, output: &Output, expected: &str) {
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(
            stdout.contains(expected),
            "❌ Output does not contain '{}'\n\nActual stdout:\n{}",
            expected,
            stdout
        );
    }

    /// stderrに期待される文字列が含まれることをアサート
    ///
    /// # Arguments
    /// * `output` - コマンド実行結果
    /// * `expected` - 期待される文字列
    #[allow(dead_code)]
    pub fn assert_stderr_contains(&self, output: &Output, expected: &str) {
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(
            stderr.contains(expected),
            "❌ Error output does not contain '{}'\n\nActual stderr:\n{}",
            expected,
            stderr
        );
    }

    /// stdoutに期待される文字列が含まれないことをアサート
    pub fn assert_stdout_not_contains(&self, output: &Output, unexpected: &str) {
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(
            !stdout.contains(unexpected),
            "❌ Output should not contain '{}', but it does\n\nActual stdout:\n{}",
            unexpected,
            stdout
        );
    }

    /// 終了コードを検証
    ///
    /// # Arguments
    /// * `output` - コマンド実行結果
    /// * `expected_code` - 期待される終了コード
    #[allow(dead_code)]
    pub fn assert_exit_code(&self, output: &Output, expected_code: i32) {
        let actual_code = output.status.code().unwrap_or(-1);
        assert_eq!(
            actual_code, expected_code,
            "❌ Expected exit code {}, got {}",
            expected_code, actual_code
        );
    }

    /// 設定ファイルパスを取得
    pub fn config_path(&self) -> PathBuf {
        self.config_dir.join("config.toml")
    }

    /// 一時ディレクトリのパスを取得
    pub fn temp_path(&self) -> &std::path::Path {
        self.temp_dir.path()
    }

    /// 設定ファイルの内容を読み取り
    #[allow(dead_code)]
    pub fn read_config(&self) -> String {
        std::fs::read_to_string(self.config_path()).expect("Failed to read config file")
    }

    /// 設定ファイルに内容を書き込み
    #[allow(dead_code)]
    pub fn write_config(&self, content: &str) {
        std::fs::write(self.config_path(), content).expect("Failed to write config file");
    }

    /// 設定ファイルが存在することを確認
    pub fn assert_config_exists(&self) {
        assert!(
            self.config_path().exists(),
            "❌ Config file does not exist at {:?}",
            self.config_path()
        );
    }

    /// 設定ファイルに期待される内容が含まれることを確認
    #[allow(dead_code)]
    pub fn assert_config_contains(&self, expected: &str) {
        let config = self.read_config();
        assert!(
            config.contains(expected),
            "❌ Config does not contain '{}'\n\nActual config:\n{}",
            expected,
            config
        );
    }
}

impl Default for CmdrunTestEnv {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_env_creation() {
        let env = CmdrunTestEnv::new();
        assert!(env.temp_path().exists());
        assert!(env.temp_path().join(".cmdrun").exists());
    }

    #[test]
    fn test_version_command() {
        let env = CmdrunTestEnv::new();
        let output = env.run_command(&["--version"]);
        env.assert_success(&output);
        env.assert_stdout_contains(&output, "cmdrun");
    }

    #[test]
    fn test_help_command() {
        let env = CmdrunTestEnv::new();
        let output = env.run_command(&["--help"]);
        env.assert_success(&output);
        env.assert_stdout_contains(&output, "Usage");
    }
}
