//! Environment management
//!
//! 環境別設定の管理と切り替え機能

use ahash::AHashMap;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tokio::fs;
use tracing::{debug, info};

use super::schema::CommandsConfig;
use super::ConfigLoader;

/// 環境設定
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EnvironmentConfig {
    /// 現在の環境名
    #[serde(default = "default_environment")]
    pub current: String,

    /// 環境定義
    #[serde(default)]
    pub environments: AHashMap<String, Environment>,
}

impl Default for EnvironmentConfig {
    fn default() -> Self {
        Self {
            current: default_environment(),
            environments: AHashMap::new(),
        }
    }
}

/// 環境定義
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Environment {
    /// 説明
    #[serde(default)]
    pub description: String,

    /// 環境変数
    #[serde(default)]
    pub variables: AHashMap<String, String>,

    /// 設定ファイルパス（オプション）
    #[serde(default)]
    pub config_file: Option<PathBuf>,

    /// .env ファイルパス（オプション）
    #[serde(default)]
    pub env_file: Option<PathBuf>,
}

impl Default for Environment {
    fn default() -> Self {
        Self {
            description: String::new(),
            variables: AHashMap::new(),
            config_file: None,
            env_file: None,
        }
    }
}

/// 環境マネージャー
#[derive(Debug)]
pub struct EnvironmentManager {
    /// 基本設定ディレクトリ
    config_dir: PathBuf,
}

impl EnvironmentManager {
    /// 新しい環境マネージャーを作成
    pub fn new(config_dir: PathBuf) -> Self {
        Self { config_dir }
    }

    /// デフォルトの環境マネージャーを作成
    pub fn default_instance() -> Result<Self> {
        let config_dir = std::env::current_dir()
            .context("Failed to get current directory")?
            .join(".cmdrun");

        Ok(Self::new(config_dir))
    }

    /// 環境設定を読み込む
    pub async fn load_environment_config(&self) -> Result<EnvironmentConfig> {
        let config_path = self.config_dir.join("config.toml");

        if !config_path.exists() {
            debug!("Environment config not found, using defaults");
            return Ok(EnvironmentConfig::default());
        }

        let content = fs::read_to_string(&config_path)
            .await
            .with_context(|| format!("Failed to read config: {}", config_path.display()))?;

        let full_config: toml::Value = toml::from_str(&content)
            .with_context(|| format!("Failed to parse config: {}", config_path.display()))?;

        // environment セクションを抽出
        if let Some(env_section) = full_config.get("environment") {
            let env_config: EnvironmentConfig = env_section
                .clone()
                .try_into()
                .context("Failed to parse environment section")?;
            Ok(env_config)
        } else {
            // 後方互換性のため、デフォルトで空の設定を返す
            debug!("No environment section found in config");
            Ok(EnvironmentConfig::default())
        }
    }

    /// 環境設定を保存
    pub async fn save_environment_config(&self, config: &EnvironmentConfig) -> Result<()> {
        let config_path = self.config_dir.join("config.toml");

        // 既存の設定を読み込み
        let mut full_config = if config_path.exists() {
            let content = fs::read_to_string(&config_path).await?;
            toml::from_str::<toml::Value>(&content)?
        } else {
            toml::Value::Table(toml::map::Map::new())
        };

        // environment セクションを更新
        let env_value =
            toml::to_string(config).context("Failed to serialize environment config")?;
        let env_table: toml::Value = toml::from_str(&env_value)?;

        if let toml::Value::Table(ref mut table) = full_config {
            table.insert("environment".to_string(), env_table);
        }

        // 設定を保存
        fs::create_dir_all(&self.config_dir).await?;
        let content = toml::to_string_pretty(&full_config)?;
        fs::write(&config_path, content).await?;

        info!(
            "Environment configuration saved to: {}",
            config_path.display()
        );
        Ok(())
    }

    /// 環境別の設定ファイルパスを取得
    pub fn get_env_config_path(&self, env_name: &str) -> PathBuf {
        if env_name.is_empty() || env_name == "default" {
            self.config_dir.join("config.toml")
        } else {
            self.config_dir.join(format!("config.{}.toml", env_name))
        }
    }

    /// 環境別の設定を読み込み（基本設定とマージ）
    pub async fn load_config_for_environment(&self, env_name: &str) -> Result<CommandsConfig> {
        let base_config_path = self.config_dir.join("config.toml");
        let env_config_path = self.get_env_config_path(env_name);

        // 基本設定を読み込み
        let mut config = if base_config_path.exists() {
            let loader = ConfigLoader::with_path(&base_config_path);
            loader.load().await?
        } else {
            CommandsConfig::default()
        };

        // 環境固有の設定があればマージ
        if env_name != "default" && env_config_path.exists() && env_config_path != base_config_path
        {
            info!(
                "Loading environment-specific config: {}",
                env_config_path.display()
            );

            let env_loader = ConfigLoader::with_path(&env_config_path);
            let env_config = env_loader.load().await?;

            config = self.merge_configs(config, env_config);
        }

        Ok(config)
    }

    /// 設定をマージ（環境固有の設定が優先）
    fn merge_configs(&self, mut base: CommandsConfig, overlay: CommandsConfig) -> CommandsConfig {
        // コマンドをマージ（上書き）
        base.commands.extend(overlay.commands);

        // エイリアスをマージ（上書き）
        base.aliases.extend(overlay.aliases);

        // 環境変数をマージ（環境固有が優先）
        base.config.env.extend(overlay.config.env);

        // グローバル設定は overlay が非デフォルト値の場合のみ上書き
        if overlay.config.shell != "bash" && overlay.config.shell != "pwsh" {
            base.config.shell = overlay.config.shell;
        }
        if overlay.config.timeout != 300 {
            base.config.timeout = overlay.config.timeout;
        }
        base.config.parallel |= overlay.config.parallel;

        base
    }

    /// 現在の環境名を取得
    pub async fn get_current_environment(&self) -> Result<String> {
        let config = self.load_environment_config().await?;
        Ok(config.current)
    }

    /// 環境を切り替え
    pub async fn switch_environment(&self, env_name: &str) -> Result<()> {
        let mut config = self.load_environment_config().await?;

        // 環境が存在するか確認（default は常に存在）
        if env_name != "default" && !config.environments.contains_key(env_name) {
            anyhow::bail!("Environment '{}' not found", env_name);
        }

        config.current = env_name.to_string();
        self.save_environment_config(&config).await?;

        info!("Switched to environment: {}", env_name);
        Ok(())
    }

    /// 環境一覧を取得
    pub async fn list_environments(&self) -> Result<Vec<(String, String)>> {
        let config = self.load_environment_config().await?;

        let mut envs = vec![("default".to_string(), "Default environment".to_string())];

        for (name, env) in &config.environments {
            envs.push((name.clone(), env.description.clone()));
        }

        envs.sort_by(|a, b| a.0.cmp(&b.0));
        Ok(envs)
    }

    /// 環境変数を設定
    pub async fn set_variable(&self, env_name: &str, key: String, value: String) -> Result<()> {
        let mut config = self.load_environment_config().await?;

        // 環境が存在しない場合は作成
        if env_name != "default" {
            config
                .environments
                .entry(env_name.to_string())
                .or_insert_with(Environment::default);
        }

        if env_name == "default" {
            // デフォルト環境の場合は基本設定ファイルに保存
            anyhow::bail!(
                "Cannot set variables for 'default' environment. Use a specific environment name."
            );
        } else {
            // 特定環境の変数を設定
            if let Some(env) = config.environments.get_mut(env_name) {
                env.variables.insert(key.clone(), value.clone());
            }
        }

        self.save_environment_config(&config).await?;
        info!("Set {}={} in environment '{}'", key, value, env_name);
        Ok(())
    }

    /// 環境を作成
    pub async fn create_environment(&self, name: String, description: String) -> Result<()> {
        let mut config = self.load_environment_config().await?;

        if config.environments.contains_key(&name) {
            anyhow::bail!("Environment '{}' already exists", name);
        }

        config.environments.insert(
            name.clone(),
            Environment {
                description,
                variables: AHashMap::new(),
                config_file: None,
                env_file: None,
            },
        );

        self.save_environment_config(&config).await?;
        info!("Created environment: {}", name);
        Ok(())
    }
}

fn default_environment() -> String {
    "default".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_environment_manager_default() {
        let temp_dir = TempDir::new().unwrap();
        let config_dir = temp_dir.path().join(".cmdrun");
        fs::create_dir_all(&config_dir).await.unwrap();

        let manager = EnvironmentManager::new(config_dir);
        let current = manager.get_current_environment().await.unwrap();

        assert_eq!(current, "default");
    }

    #[tokio::test]
    async fn test_create_and_switch_environment() {
        let temp_dir = TempDir::new().unwrap();
        let config_dir = temp_dir.path().join(".cmdrun");
        fs::create_dir_all(&config_dir).await.unwrap();

        let manager = EnvironmentManager::new(config_dir);

        // 環境を作成
        manager
            .create_environment("dev".to_string(), "Development environment".to_string())
            .await
            .unwrap();

        // 環境を切り替え
        manager.switch_environment("dev").await.unwrap();

        let current = manager.get_current_environment().await.unwrap();
        assert_eq!(current, "dev");
    }

    #[tokio::test]
    async fn test_set_variable() {
        let temp_dir = TempDir::new().unwrap();
        let config_dir = temp_dir.path().join(".cmdrun");
        fs::create_dir_all(&config_dir).await.unwrap();

        let manager = EnvironmentManager::new(config_dir);

        // 環境を作成
        manager
            .create_environment("prod".to_string(), "Production environment".to_string())
            .await
            .unwrap();

        // 変数を設定
        manager
            .set_variable(
                "prod",
                "API_URL".to_string(),
                "https://api.example.com".to_string(),
            )
            .await
            .unwrap();

        // 設定を確認
        let config = manager.load_environment_config().await.unwrap();
        let prod_env = config.environments.get("prod").unwrap();
        assert_eq!(
            prod_env.variables.get("API_URL").unwrap(),
            "https://api.example.com"
        );
    }

    #[tokio::test]
    async fn test_list_environments() {
        let temp_dir = TempDir::new().unwrap();
        let config_dir = temp_dir.path().join(".cmdrun");
        fs::create_dir_all(&config_dir).await.unwrap();

        let manager = EnvironmentManager::new(config_dir);

        manager
            .create_environment("dev".to_string(), "Development".to_string())
            .await
            .unwrap();
        manager
            .create_environment("staging".to_string(), "Staging".to_string())
            .await
            .unwrap();

        let envs = manager.list_environments().await.unwrap();

        assert_eq!(envs.len(), 3); // default + dev + staging
        assert!(envs.iter().any(|(name, _)| name == "default"));
        assert!(envs.iter().any(|(name, _)| name == "dev"));
        assert!(envs.iter().any(|(name, _)| name == "staging"));
    }
}
