//! TOML 設定ファイルスキーマ定義
//!
//! Serde を使用した型安全な設定デシリアライゼーション

use ahash::AHashMap;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// commands.toml のルート構造
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CommandsConfig {
    /// グローバル設定
    #[serde(default)]
    pub config: GlobalConfig,

    /// コマンド定義
    #[serde(default)]
    pub commands: AHashMap<String, Command>,

    /// エイリアス定義
    #[serde(default)]
    pub aliases: AHashMap<String, String>,

    /// フック定義
    #[serde(default)]
    pub hooks: Hooks,

    /// プラグイン設定
    #[serde(default)]
    pub plugins: PluginsConfig,
}

impl Default for CommandsConfig {
    fn default() -> Self {
        Self {
            config: GlobalConfig::default(),
            commands: AHashMap::new(),
            aliases: AHashMap::new(),
            hooks: Hooks::default(),
            plugins: PluginsConfig::default(),
        }
    }
}

impl CommandsConfig {
    /// Merge configurations (overlay takes precedence)
    pub fn merge_with(self, overlay: Self) -> Self {
        Self {
            config: self.config.merge_with(overlay.config),
            commands: {
                let mut merged = self.commands;
                merged.extend(overlay.commands);
                merged
            },
            aliases: {
                let mut merged = self.aliases;
                merged.extend(overlay.aliases);
                merged
            },
            hooks: self.hooks.merge_with(overlay.hooks),
            plugins: self.plugins.merge_with(overlay.plugins),
        }
    }
}

/// 言語設定
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum Language {
    /// 英語
    #[default]
    English,
    /// 日本語
    Japanese,
}

/// グローバル設定
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GlobalConfig {
    /// デフォルトシェル
    #[serde(default = "default_shell")]
    pub shell: String,

    /// 厳格モード
    #[serde(default = "default_true")]
    pub strict_mode: bool,

    /// 並列実行デフォルト
    #[serde(default)]
    pub parallel: bool,

    /// タイムアウト（秒）
    #[serde(default = "default_timeout")]
    pub timeout: u64,

    /// 作業ディレクトリ
    #[serde(default = "default_working_dir")]
    pub working_dir: PathBuf,

    /// 言語設定
    #[serde(default)]
    pub language: Language,

    /// グローバル環境変数
    #[serde(default)]
    pub env: AHashMap<String, String>,
}

impl Default for GlobalConfig {
    fn default() -> Self {
        Self {
            shell: default_shell(),
            strict_mode: true,
            parallel: false,
            timeout: default_timeout(),
            working_dir: default_working_dir(),
            language: Language::default(),
            env: AHashMap::new(),
        }
    }
}

impl GlobalConfig {
    /// Merge global configs (overlay takes precedence, env vars are combined)
    pub fn merge_with(self, overlay: Self) -> Self {
        Self {
            shell: overlay.shell,
            strict_mode: overlay.strict_mode,
            parallel: overlay.parallel,
            timeout: overlay.timeout,
            working_dir: overlay.working_dir,
            language: overlay.language,
            env: {
                let mut merged = self.env;
                merged.extend(overlay.env);
                merged
            },
        }
    }
}

/// コマンド定義
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Command {
    /// コマンド説明
    #[serde(default)]
    pub description: String,

    /// 実行コマンド（文字列または配列）
    #[serde(deserialize_with = "deserialize_command")]
    pub cmd: CommandSpec,

    /// 環境変数
    #[serde(default)]
    pub env: AHashMap<String, String>,

    /// 作業ディレクトリ
    #[serde(default)]
    pub working_dir: Option<PathBuf>,

    /// 依存コマンド
    #[serde(default)]
    pub deps: Vec<String>,

    /// 対応プラットフォーム
    #[serde(default)]
    pub platform: Vec<Platform>,

    /// タグ
    #[serde(default)]
    pub tags: Vec<String>,

    /// タイムアウト
    #[serde(default)]
    pub timeout: Option<u64>,

    /// 並列実行可能
    #[serde(default)]
    pub parallel: bool,

    /// 実行前確認
    #[serde(default)]
    pub confirm: bool,
}

/// コマンド仕様（文字列、配列、プラットフォーム別）
#[derive(Debug, Clone, Serialize)]
pub enum CommandSpec {
    /// 単一コマンド
    Single(String),

    /// 複数コマンド（逐次実行）
    Multiple(Vec<String>),

    /// プラットフォーム別コマンド
    Platform(PlatformCommands),
}

/// プラットフォーム別コマンド定義
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PlatformCommands {
    #[serde(default)]
    pub unix: Option<String>,

    #[serde(default)]
    pub linux: Option<String>,

    #[serde(default)]
    pub macos: Option<String>,

    #[serde(default)]
    pub windows: Option<String>,
}

/// プラットフォーム種類
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Platform {
    Unix,
    Linux,
    Macos,
    Windows,
}

/// フック定義
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Hooks {
    /// 全コマンド実行前
    #[serde(default)]
    pub pre_run: Option<String>,

    /// 全コマンド実行後
    #[serde(default)]
    pub post_run: Option<String>,

    /// コマンド別フック
    #[serde(default)]
    pub commands: AHashMap<String, CommandHooks>,
}

impl Hooks {
    /// Merge hooks (overlay takes precedence)
    pub fn merge_with(self, overlay: Self) -> Self {
        Self {
            pre_run: overlay.pre_run.or(self.pre_run),
            post_run: overlay.post_run.or(self.post_run),
            commands: {
                let mut merged = self.commands;
                merged.extend(overlay.commands);
                merged
            },
        }
    }
}

/// コマンド別フック
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CommandHooks {
    #[serde(default)]
    pub pre_run: Option<String>,

    #[serde(default)]
    pub post_run: Option<String>,
}

/// プラグイン設定
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PluginsConfig {
    /// 有効なプラグイン一覧
    #[serde(default)]
    pub enabled: Vec<String>,

    /// プラグイン別設定
    #[serde(flatten)]
    pub plugins: AHashMap<String, crate::plugin::PluginConfig>,
}

impl PluginsConfig {
    /// Merge plugin configs (overlay takes complete precedence)
    pub fn merge_with(self, overlay: Self) -> Self {
        Self {
            enabled: overlay.enabled,
            plugins: {
                let mut merged = self.plugins;
                merged.extend(overlay.plugins);
                merged
            },
        }
    }
}

// ===================================================================
// カスタムデシリアライザー
// ===================================================================

use serde::de::{self, Deserializer, Visitor};
use std::fmt;

/// CommandSpec のデシリアライズ
fn deserialize_command<'de, D>(deserializer: D) -> Result<CommandSpec, D::Error>
where
    D: Deserializer<'de>,
{
    struct CommandVisitor;

    impl<'de> Visitor<'de> for CommandVisitor {
        type Value = CommandSpec;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a string, array of strings, or platform-specific commands")
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(CommandSpec::Single(value.to_string()))
        }

        fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
        where
            A: de::SeqAccess<'de>,
        {
            let mut commands = Vec::new();
            while let Some(cmd) = seq.next_element::<String>()? {
                commands.push(cmd);
            }
            Ok(CommandSpec::Multiple(commands))
        }

        fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
        where
            A: de::MapAccess<'de>,
        {
            let platform_commands =
                PlatformCommands::deserialize(de::value::MapAccessDeserializer::new(map))?;
            Ok(CommandSpec::Platform(platform_commands))
        }
    }

    deserializer.deserialize_any(CommandVisitor)
}

// ===================================================================
// デフォルト値関数
// ===================================================================

fn default_shell() -> String {
    if cfg!(windows) {
        "pwsh".to_string()
    } else {
        "bash".to_string()
    }
}

fn default_true() -> bool {
    true
}

fn default_timeout() -> u64 {
    300
}

fn default_working_dir() -> PathBuf {
    PathBuf::from(".")
}

// ===================================================================
// ユーティリティメソッド
// ===================================================================

impl CommandSpec {
    /// プラットフォームに応じたコマンド取得
    pub fn resolve_for_platform(&self, platform: &Platform) -> Option<Vec<String>> {
        match self {
            CommandSpec::Single(cmd) => Some(vec![cmd.clone()]),
            CommandSpec::Multiple(cmds) => Some(cmds.clone()),
            CommandSpec::Platform(platform_cmds) => {
                let cmd = match platform {
                    Platform::Linux => platform_cmds.linux.as_ref().or(platform_cmds.unix.as_ref()),
                    Platform::Macos => platform_cmds.macos.as_ref().or(platform_cmds.unix.as_ref()),
                    Platform::Unix => platform_cmds.unix.as_ref(),
                    Platform::Windows => platform_cmds.windows.as_ref(),
                };
                cmd.map(|c| vec![c.clone()])
            }
        }
    }
}

impl Platform {
    /// 現在のプラットフォーム検出
    pub fn current() -> Self {
        if cfg!(target_os = "windows") {
            Platform::Windows
        } else if cfg!(target_os = "macos") {
            Platform::Macos
        } else if cfg!(target_os = "linux") {
            Platform::Linux
        } else {
            Platform::Unix
        }
    }

    /// プラットフォームが対応しているか確認
    pub fn is_supported(&self, supported: &[Platform]) -> bool {
        if supported.is_empty() {
            return true; // 指定なしは全プラットフォーム対応
        }

        supported.contains(self)
            || (matches!(self, Platform::Linux | Platform::Macos)
                && supported.contains(&Platform::Unix))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_single_command() {
        let toml = r#"
            description = "test"
            cmd = "echo hello"
        "#;
        let cmd: Command = toml::from_str(toml).unwrap();
        assert!(matches!(cmd.cmd, CommandSpec::Single(_)));
    }

    #[test]
    fn test_deserialize_multiple_commands() {
        let toml = r#"
            description = "test"
            cmd = ["echo hello", "echo world"]
        "#;
        let cmd: Command = toml::from_str(toml).unwrap();
        assert!(matches!(cmd.cmd, CommandSpec::Multiple(_)));
    }

    #[test]
    fn test_deserialize_platform_commands() {
        let toml = r#"
            description = "test"
            [cmd]
            unix = "ls"
            windows = "dir"
        "#;
        let cmd: Command = toml::from_str(toml).unwrap();
        assert!(matches!(cmd.cmd, CommandSpec::Platform(_)));
    }

    #[test]
    fn test_platform_detection() {
        let platform = Platform::current();
        #[cfg(target_os = "linux")]
        assert_eq!(platform, Platform::Linux);
        #[cfg(target_os = "macos")]
        assert_eq!(platform, Platform::Macos);
        #[cfg(target_os = "windows")]
        assert_eq!(platform, Platform::Windows);
    }
}
