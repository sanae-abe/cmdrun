//! Configuration loader
//!
//! TOML 設定ファイルの読み込みと階層的なマージ処理

use crate::config::schema::CommandsConfig;
use crate::config::Language;
use crate::i18n::{get_message, MessageKey};
use anyhow::{Context, Result};
use std::path::{Path, PathBuf};
use tokio::fs;
use tracing::{debug, info};

/// 設定ファイルと読み込み情報
#[derive(Debug, Clone)]
pub struct LoadedConfig {
    /// 設定内容
    pub config: CommandsConfig,
    /// グローバル設定ファイルパス
    pub global_path: Option<PathBuf>,
    /// ローカル設定ファイルパス
    pub local_path: Option<PathBuf>,
}

/// 設定ファイル名（優先順位順）
const CONFIG_FILENAMES: &[&str] = &["commands.toml", ".cmdrun.toml", "cmdrun.toml"];

/// 設定ファイルローダー
#[derive(Debug, Clone)]
pub struct ConfigLoader {
    /// 明示的に指定された設定ファイルパス
    explicit_path: Option<PathBuf>,
    /// 実際に使用されたグローバル設定ファイルパス
    pub loaded_global_path: Option<PathBuf>,
    /// 実際に使用されたローカル設定ファイルパス
    pub loaded_local_path: Option<PathBuf>,
    /// グローバル設定のみを使用するフラグ
    global_only: bool,
}

impl ConfigLoader {
    /// 新しいローダーを作成
    pub fn new() -> Self {
        Self {
            explicit_path: None,
            loaded_global_path: None,
            loaded_local_path: None,
            global_only: false,
        }
    }

    /// グローバル設定のみを使用するローダーを作成
    pub fn global_only() -> Self {
        Self {
            explicit_path: None,
            loaded_global_path: None,
            loaded_local_path: None,
            global_only: true,
        }
    }

    /// 明示的なパスを指定してローダーを作成
    pub fn with_path<P: Into<PathBuf>>(path: P) -> Result<Self> {
        let path = path.into();

        // パストラバーサル対策：パスを正規化
        let canonical_path = std::fs::canonicalize(&path)
            .with_context(|| format!("Invalid config path: {}", path.display()))?;

        // セキュリティ警告：プロジェクト外のパス
        if let Ok(current_dir) = std::env::current_dir() {
            if !canonical_path.starts_with(&current_dir) {
                debug!(
                    "⚠️  Config file outside project directory: {}",
                    canonical_path.display()
                );
            }
        }

        Ok(Self {
            explicit_path: Some(canonical_path),
            loaded_global_path: None,
            loaded_local_path: None,
            global_only: false,
        })
    }

    /// 設定ファイルを読み込む（グローバル + ローカルのマージ）
    ///
    /// 優先順位:
    /// 1. 明示的に指定されたパス（マージなし）
    /// 2. ローカル設定（必須） + グローバル設定（任意）
    pub async fn load(&self) -> Result<CommandsConfig> {
        let loaded = self.load_with_paths().await?;
        Ok(loaded.config)
    }

    /// 設定ファイルを読み込み、パス情報も返す
    pub async fn load_with_paths(&self) -> Result<LoadedConfig> {
        if let Some(path) = &self.explicit_path {
            debug!("Using explicitly specified config: {}", path.display());
            let config = self.load_from_path(path).await?;
            return Ok(LoadedConfig {
                config,
                global_path: None,
                local_path: Some(path.clone()),
            });
        }

        // グローバル設定（任意）
        let (global_config, global_path) = match self.find_global_config().await {
            Some(path) => {
                info!("Loading global config: {}", path.display());
                let config = self.load_from_path(&path).await?;
                (Some(config), Some(path))
            }
            None => {
                debug!("No global config found");
                (None, None)
            }
        };

        // グローバルのみモードの場合、ローカル設定を探さない
        if self.global_only {
            let config = global_config.context("Global configuration file not found")?;
            return Ok(LoadedConfig {
                config,
                global_path,
                local_path: None,
            });
        }

        // ローカル設定（必須）
        let local_path = self.find_local_config().await?;
        info!("Loading local config: {}", local_path.display());
        let local_config = self.load_from_path(&local_path).await?;

        // マージ（ローカルが優先）
        let merged_config = match global_config {
            Some(global) => {
                debug!("Merging global and local configurations");
                global.merge_with(local_config)
            }
            None => local_config,
        };

        Ok(LoadedConfig {
            config: merged_config,
            global_path,
            local_path: Some(local_path),
        })
    }

    /// 環境を考慮して設定ファイルを読み込む
    ///
    /// 優先順位:
    /// 1. グローバル設定（任意）
    /// 2. ローカル設定（必須）
    /// 3. 環境別設定（任意、最優先）
    pub async fn load_with_environment(&self) -> Result<CommandsConfig> {
        use crate::config::environment::EnvironmentManager;

        // 基本設定を読み込み
        let mut config = self.load().await?;

        // 現在の環境を取得
        let env_manager = EnvironmentManager::default_instance()
            .context("Failed to initialize environment manager")?;

        let current_env = env_manager.get_current_environment().await?;

        // デフォルト環境以外の場合、環境別設定をマージ
        if current_env != "default" {
            if let Some(env_config) = self.load_environment_config(&current_env).await? {
                info!("Merging environment-specific config: {}", current_env);
                config = config.merge_with(env_config);

                // 環境固有の環境変数を追加
                if let Ok(env_config_data) = env_manager.load_environment_config().await {
                    if let Some(env) = env_config_data.environments.get(&current_env) {
                        config.config.env.extend(env.variables.clone());
                    }
                }
            }
        }

        Ok(config)
    }

    /// 環境別設定ファイルを読み込む
    async fn load_environment_config(&self, env_name: &str) -> Result<Option<CommandsConfig>> {
        // 現在のディレクトリで環境別設定を探す
        let current_dir = std::env::current_dir().context("Failed to get current directory")?;

        // commands.{env}.toml の形式
        let env_filename = format!("commands.{}.toml", env_name);
        let env_path = current_dir.join(&env_filename);

        if env_path.exists() && env_path.is_file() {
            info!("Loading environment config: {}", env_path.display());
            let config = self.load_from_path(&env_path).await?;
            return Ok(Some(config));
        }

        // .cmdrun/config.{env}.toml も探す
        let cmdrun_env_path = current_dir
            .join(".cmdrun")
            .join(format!("config.{}.toml", env_name));
        if cmdrun_env_path.exists() && cmdrun_env_path.is_file() {
            info!("Loading environment config: {}", cmdrun_env_path.display());
            let config = self.load_from_path(&cmdrun_env_path).await?;
            return Ok(Some(config));
        }

        debug!("No environment-specific config found for: {}", env_name);
        Ok(None)
    }

    /// グローバル設定ファイルを検索
    async fn find_global_config(&self) -> Option<PathBuf> {
        let global_dir = dirs::config_dir()?.join("cmdrun");
        self.check_directory(&global_dir).await.ok().flatten()
    }

    /// ローカル設定ファイルを検索（必須）
    async fn find_local_config(&self) -> Result<PathBuf> {
        let current_dir = std::env::current_dir().context("Failed to get current directory")?;

        if let Some(path) = self.search_upwards(&current_dir).await? {
            return Ok(path);
        }

        anyhow::bail!(
            "{}. Searched for: {}",
            get_message(MessageKey::ErrorLocalConfigNotFound, Language::English),
            CONFIG_FILENAMES.join(", ")
        )
    }

    /// 設定ファイルを探索（後方互換）
    #[allow(dead_code)]
    async fn find_config(&self) -> Result<PathBuf> {
        self.find_local_config().await
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
            anyhow::bail!(
                "{}",
                get_message(MessageKey::ErrorNoConfigFilesSpecified, Language::English)
            );
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

        let loader = ConfigLoader::with_path(&config_path).unwrap();
        let config = loader.load().await.unwrap();

        assert_eq!(config.config.shell, "bash");
        assert_eq!(config.commands.len(), 1);
        assert!(config.commands.contains_key("test"));
    }

    #[tokio::test]
    async fn test_config_not_found() {
        // Path validation now happens in with_path, so we expect it to fail there
        let result = ConfigLoader::with_path("/nonexistent/path/commands.toml");
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

        let loader = ConfigLoader::with_path(&config_path).unwrap();
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

#[cfg(test)]
mod global_config_tests {
    use super::*;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_global_config_merge() {
        let temp_dir = TempDir::new().unwrap();

        // Create global config
        let global_dir = temp_dir.path().join(".config/cmdrun");
        fs::create_dir_all(&global_dir).await.unwrap();
        let global_path = global_dir.join("commands.toml");
        fs::write(
            &global_path,
            r#"
[config]
shell = "zsh"
timeout = 600

[config.env]
GLOBAL_VAR = "from_global"

[commands.global-cmd]
description = "Global command"
cmd = "echo global"
"#,
        )
        .await
        .unwrap();

        // Create local config
        let local_dir = temp_dir.path().join("project");
        fs::create_dir_all(&local_dir).await.unwrap();
        let local_path = local_dir.join("commands.toml");
        fs::write(
            &local_path,
            r#"
[config]
shell = "bash"

[config.env]
LOCAL_VAR = "from_local"

[commands.local-cmd]
description = "Local command"
cmd = "echo local"
"#,
        )
        .await
        .unwrap();

        // Test with mock config_dir
        println!("Global: {:?}", global_path);
        println!("Local: {:?}", local_path);
    }
}
