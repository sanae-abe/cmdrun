//! Configuration loader
//!
//! TOML 設定ファイルの読み込みと階層的なマージ処理

use crate::config::schema::CommandsConfig;
use anyhow::{Context, Result};
use std::path::{Path, PathBuf};
use tokio::fs;
use tracing::{debug, info};

/// 設定ファイル名（優先順位順）
const CONFIG_FILENAMES: &[&str] = &["commands.toml", ".cmdrun.toml", "cmdrun.toml"];

/// 設定ファイルローダー
#[derive(Debug, Clone)]
pub struct ConfigLoader {
    /// 明示的に指定された設定ファイルパス
    explicit_path: Option<PathBuf>,
}

impl ConfigLoader {
    /// 新しいローダーを作成
    pub fn new() -> Self {
        Self {
            explicit_path: None,
        }
    }

    /// 明示的なパスを指定してローダーを作成
    pub fn with_path<P: Into<PathBuf>>(path: P) -> Self {
        Self {
            explicit_path: Some(path.into()),
        }
    }

    /// 設定ファイルを読み込む
    ///
    /// 優先順位:
    /// 1. 明示的に指定されたパス
    /// 2. カレントディレクトリからの探索
    /// 3. ホームディレクトリの ~/.cmdrun/
    pub async fn load(&self) -> Result<CommandsConfig> {
        let config_path = if let Some(path) = &self.explicit_path {
            debug!("Using explicitly specified config: {}", path.display());
            path.clone()
        } else {
            self.find_config().await?
        };

        info!("Loading configuration from: {}", config_path.display());
        self.load_from_path(&config_path).await
    }

    /// 設定ファイルを探索
    async fn find_config(&self) -> Result<PathBuf> {
        // カレントディレクトリから上位ディレクトリへ探索
        let current_dir = std::env::current_dir().context("Failed to get current directory")?;

        if let Some(path) = self.search_upwards(&current_dir).await? {
            return Ok(path);
        }

        // ホームディレクトリを探索
        if let Some(home_dir) = dirs::home_dir() {
            let cmdrun_dir = home_dir.join(".cmdrun");
            if let Some(path) = self.check_directory(&cmdrun_dir).await? {
                return Ok(path);
            }
        }

        anyhow::bail!(
            "Configuration file not found. Searched for: {}",
            CONFIG_FILENAMES.join(", ")
        )
    }

    /// ディレクトリから上位へ向かって設定ファイルを探索
    async fn search_upwards(&self, start_dir: &Path) -> Result<Option<PathBuf>> {
        let mut current = start_dir.to_path_buf();

        loop {
            if let Some(path) = self.check_directory(&current).await? {
                return Ok(Some(path));
            }

            // 親ディレクトリへ移動
            if !current.pop() {
                break;
            }
        }

        Ok(None)
    }

    /// ディレクトリ内で設定ファイルをチェック
    async fn check_directory(&self, dir: &Path) -> Result<Option<PathBuf>> {
        for filename in CONFIG_FILENAMES {
            let path = dir.join(filename);
            if path.exists() && path.is_file() {
                debug!("Found config file: {}", path.display());
                return Ok(Some(path));
            }
        }
        Ok(None)
    }

    /// 指定されたパスから設定ファイルを読み込む
    async fn load_from_path(&self, path: &Path) -> Result<CommandsConfig> {
        let content = fs::read_to_string(path)
            .await
            .with_context(|| format!("Failed to read config file: {}", path.display()))?;

        let config: CommandsConfig = toml::from_str(&content)
            .with_context(|| format!("Failed to parse TOML config: {}", path.display()))?;

        debug!(
            "Loaded {} commands, {} aliases",
            config.commands.len(),
            config.aliases.len()
        );

        Ok(config)
    }

    /// 複数の設定ファイルをマージして読み込む（将来の機能拡張用）
    #[allow(dead_code)]
    async fn load_with_merge(&self, paths: &[PathBuf]) -> Result<CommandsConfig> {
        if paths.is_empty() {
            anyhow::bail!("No configuration files specified");
        }

        let mut merged = self.load_from_path(&paths[0]).await?;

        for path in &paths[1..] {
            let config = self.load_from_path(path).await?;
            merged = self.merge_configs(merged, config);
        }

        Ok(merged)
    }

    /// 設定をマージ（後の設定が優先）
    fn merge_configs(&self, mut base: CommandsConfig, overlay: CommandsConfig) -> CommandsConfig {
        // コマンドをマージ（上書き）
        base.commands.extend(overlay.commands);

        // エイリアスをマージ（上書き）
        base.aliases.extend(overlay.aliases);

        // グローバル設定は overlay を優先
        if overlay.config.shell != "bash" && overlay.config.shell != "pwsh" {
            base.config.shell = overlay.config.shell;
        }
        if overlay.config.timeout != 300 {
            base.config.timeout = overlay.config.timeout;
        }
        base.config.parallel |= overlay.config.parallel;
        base.config.env.extend(overlay.config.env);

        // フックをマージ
        if overlay.hooks.pre_run.is_some() {
            base.hooks.pre_run = overlay.hooks.pre_run;
        }
        if overlay.hooks.post_run.is_some() {
            base.hooks.post_run = overlay.hooks.post_run;
        }
        base.hooks.commands.extend(overlay.hooks.commands);

        base
    }
}

impl Default for ConfigLoader {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use tokio::fs::File;
    use tokio::io::AsyncWriteExt;

    #[tokio::test]
    async fn test_load_simple_config() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("commands.toml");

        let toml_content = r#"
[config]
shell = "bash"

[commands.test]
description = "Run tests"
cmd = "cargo test"
"#;

        let mut file = File::create(&config_path).await.unwrap();
        file.write_all(toml_content.as_bytes()).await.unwrap();

        let loader = ConfigLoader::with_path(&config_path);
        let config = loader.load().await.unwrap();

        assert_eq!(config.config.shell, "bash");
        assert_eq!(config.commands.len(), 1);
        assert!(config.commands.contains_key("test"));
    }

    #[tokio::test]
    async fn test_config_not_found() {
        let loader = ConfigLoader::with_path("/nonexistent/path/commands.toml");
        let result = loader.load().await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_invalid_toml() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("commands.toml");

        let invalid_toml = "this is not valid TOML [[[";
        let mut file = File::create(&config_path).await.unwrap();
        file.write_all(invalid_toml.as_bytes()).await.unwrap();
        file.flush().await.unwrap();
        drop(file);

        // Verify file exists and has invalid content
        assert!(config_path.exists());
        let content = fs::read_to_string(&config_path).await.unwrap();
        assert_eq!(content, invalid_toml);

        let loader = ConfigLoader::with_path(&config_path);
        let result = loader.load().await;
        assert!(
            result.is_err(),
            "Expected error when parsing invalid TOML, but got Ok"
        );
    }

    #[tokio::test]
    async fn test_merge_configs() {
        let loader = ConfigLoader::new();

        let base = CommandsConfig {
            config: crate::config::schema::GlobalConfig {
                shell: "bash".to_string(),
                timeout: 100,
                ..Default::default()
            },
            commands: {
                let mut map = ahash::AHashMap::new();
                map.insert(
                    "test".to_string(),
                    crate::config::schema::Command {
                        description: "Test".to_string(),
                        cmd: crate::config::schema::CommandSpec::Single("echo test".to_string()),
                        env: Default::default(),
                        working_dir: None,
                        deps: vec![],
                        platform: vec![],
                        tags: vec![],
                        timeout: None,
                        parallel: false,
                        confirm: false,
                    },
                );
                map
            },
            aliases: Default::default(),
            hooks: Default::default(),
            plugins: Default::default(),
        };

        let overlay = CommandsConfig {
            config: crate::config::schema::GlobalConfig {
                shell: "zsh".to_string(),
                timeout: 200,
                ..Default::default()
            },
            commands: {
                let mut map = ahash::AHashMap::new();
                map.insert(
                    "build".to_string(),
                    crate::config::schema::Command {
                        description: "Build".to_string(),
                        cmd: crate::config::schema::CommandSpec::Single("cargo build".to_string()),
                        env: Default::default(),
                        working_dir: None,
                        deps: vec![],
                        platform: vec![],
                        tags: vec![],
                        timeout: None,
                        parallel: false,
                        confirm: false,
                    },
                );
                map
            },
            aliases: Default::default(),
            hooks: Default::default(),
            plugins: Default::default(),
        };

        let merged = loader.merge_configs(base, overlay);

        assert_eq!(merged.config.shell, "zsh");
        assert_eq!(merged.config.timeout, 200);
        assert_eq!(merged.commands.len(), 2);
        assert!(merged.commands.contains_key("test"));
        assert!(merged.commands.contains_key("build"));
    }
}
