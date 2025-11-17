//! Configuration validation
//!
//! 設定の妥当性検証と循環依存検出

use crate::config::schema::{CommandsConfig, Platform};
use crate::config::Language;
use crate::i18n::{get_message, MessageKey};
use ahash::{AHashMap, AHashSet};
use anyhow::Result;
use std::collections::VecDeque;
use tracing::{debug, warn};

/// 設定検証エラー
#[derive(Debug, thiserror::Error)]
pub enum ValidationError {
    #[error("Circular dependency detected: {0}")]
    CircularDependency(String),

    #[error("Command not found: {command} (referenced by {referenced_by})")]
    CommandNotFound {
        command: String,
        referenced_by: String,
    },

    #[error("Alias target not found: {alias} -> {target}")]
    AliasTargetNotFound { alias: String, target: String },

    #[error("Invalid command name: {0}")]
    InvalidCommandName(String),

    #[error("Platform not supported: {command} requires {platform:?}, but current platform is {current:?}")]
    PlatformNotSupported {
        command: String,
        platform: Vec<Platform>,
        current: Platform,
    },

    #[error("Empty command specification: {0}")]
    EmptyCommand(String),
}

/// 設定検証器
#[derive(Debug)]
pub struct ConfigValidator<'a> {
    config: &'a CommandsConfig,
    current_platform: Platform,
}

impl<'a> ConfigValidator<'a> {
    /// 新しい検証器を作成
    pub fn new(config: &'a CommandsConfig) -> Self {
        Self {
            config,
            current_platform: Platform::current(),
        }
    }

    /// 設定を検証
    pub fn validate(&self) -> Result<()> {
        debug!("Validating configuration");

        self.validate_command_names()?;
        self.validate_dependencies()?;
        self.validate_aliases()?;
        self.validate_platforms()?;
        self.validate_commands()?;

        debug!("Configuration validation passed");
        Ok(())
    }

    /// コマンド名の妥当性検証
    fn validate_command_names(&self) -> Result<()> {
        for name in self.config.commands.keys() {
            if name.is_empty() {
                return Err(ValidationError::InvalidCommandName(name.clone()).into());
            }

            // コマンド名は英数字、ハイフン、アンダースコア、コロンを許可
            if !name
                .chars()
                .all(|c| c.is_alphanumeric() || c == '-' || c == '_' || c == ':')
            {
                return Err(ValidationError::InvalidCommandName(name.clone()).into());
            }
        }

        Ok(())
    }

    /// 依存関係の検証（循環依存検出）
    fn validate_dependencies(&self) -> Result<()> {
        for (name, command) in &self.config.commands {
            // 依存コマンドの存在確認
            for dep in &command.deps {
                // エイリアスを解決
                let resolved_dep = self.resolve_alias(dep);

                if !self.config.commands.contains_key(resolved_dep) {
                    return Err(ValidationError::CommandNotFound {
                        command: dep.clone(),
                        referenced_by: name.clone(),
                    }
                    .into());
                }
            }

            // 循環依存検出
            self.check_circular_dependency(name)?;
        }

        Ok(())
    }

    /// 循環依存をDFSで検出
    fn check_circular_dependency(&self, start: &str) -> Result<()> {
        let mut visited = AHashSet::new();
        let mut path = Vec::new();

        self.dfs_check(start, &mut visited, &mut path)
    }

    /// 深さ優先探索で循環を検出
    fn dfs_check(
        &self,
        current: &str,
        visited: &mut AHashSet<String>,
        path: &mut Vec<String>,
    ) -> Result<()> {
        // 現在のパスに既に含まれている場合は循環
        if path.contains(&current.to_string()) {
            path.push(current.to_string());
            let cycle = path.join(" -> ");
            return Err(ValidationError::CircularDependency(cycle).into());
        }

        // 既に訪問済みの場合はスキップ
        if visited.contains(current) {
            return Ok(());
        }

        visited.insert(current.to_string());
        path.push(current.to_string());

        // 依存関係を探索
        if let Some(command) = self.config.commands.get(current) {
            for dep in &command.deps {
                let resolved_dep = self.resolve_alias(dep);
                self.dfs_check(resolved_dep, visited, path)?;
            }
        }

        path.pop();
        Ok(())
    }

    /// エイリアスを検証
    fn validate_aliases(&self) -> Result<()> {
        for (alias, target) in &self.config.aliases {
            // エイリアスターゲットの存在確認
            if !self.config.commands.contains_key(target) {
                return Err(ValidationError::AliasTargetNotFound {
                    alias: alias.clone(),
                    target: target.clone(),
                }
                .into());
            }

            // エイリアス名の妥当性検証
            if alias.is_empty() {
                return Err(ValidationError::InvalidCommandName(alias.clone()).into());
            }
        }

        Ok(())
    }

    /// プラットフォーム対応の検証
    fn validate_platforms(&self) -> Result<()> {
        for (name, command) in &self.config.commands {
            // プラットフォーム指定がある場合、現在のプラットフォームが対応しているか確認
            if !command.platform.is_empty()
                && !self.current_platform.is_supported(&command.platform)
            {
                warn!(
                    "Command '{}' is not supported on current platform {:?}",
                    name, self.current_platform
                );
            }
        }

        Ok(())
    }

    /// コマンド仕様の検証
    fn validate_commands(&self) -> Result<()> {
        for (name, command) in &self.config.commands {
            // プラットフォーム別コマンドの場合、現在のプラットフォームで実行可能か確認
            if let Some(cmds) = command.cmd.resolve_for_platform(&self.current_platform) {
                if cmds.is_empty() || cmds.iter().all(|s| s.trim().is_empty()) {
                    return Err(ValidationError::EmptyCommand(name.clone()).into());
                }
            } else {
                warn!(
                    "Command '{}' has no implementation for platform {:?}",
                    name, self.current_platform
                );
            }
        }

        Ok(())
    }

    /// エイリアスを解決
    fn resolve_alias<'b>(&self, name: &'b str) -> &'b str
    where
        'a: 'b,
    {
        self.config
            .aliases
            .get(name)
            .map(|s| s.as_str())
            .unwrap_or(name)
    }

    /// 依存グラフを構築（トポロジカルソート）
    pub fn build_dependency_graph(&self) -> Result<DependencyGraph> {
        let mut graph = DependencyGraph::new();

        for (name, command) in &self.config.commands {
            let deps: Vec<String> = command
                .deps
                .iter()
                .map(|dep| self.resolve_alias(dep).to_string())
                .collect();
            graph.add_command(name.clone(), deps);
        }

        Ok(graph)
    }

    /// 実行順序を計算（トポロジカルソート）
    pub fn compute_execution_order(&self, commands: &[String]) -> Result<Vec<String>> {
        let graph = self.build_dependency_graph()?;
        graph.topological_sort(commands)
    }
}

/// 依存グラフ
#[derive(Debug, Clone)]
pub struct DependencyGraph {
    /// コマンド -> 依存コマンドリスト
    dependencies: AHashMap<String, Vec<String>>,
}

impl DependencyGraph {
    pub fn new() -> Self {
        Self {
            dependencies: AHashMap::new(),
        }
    }

    pub fn add_command(&mut self, name: String, deps: Vec<String>) {
        self.dependencies.insert(name, deps);
    }

    /// トポロジカルソート（Kahn's algorithm）
    pub fn topological_sort(&self, targets: &[String]) -> Result<Vec<String>> {
        let mut in_degree = AHashMap::new();
        let mut adj_list: AHashMap<String, Vec<String>> = AHashMap::new();

        // ターゲットコマンドとその依存関係を収集
        let mut to_process = VecDeque::from(targets.to_vec());
        let mut visited = AHashSet::new();

        while let Some(cmd) = to_process.pop_front() {
            if visited.contains(&cmd) {
                continue;
            }
            visited.insert(cmd.clone());

            if let Some(deps) = self.dependencies.get(&cmd) {
                in_degree.entry(cmd.clone()).or_insert(0);

                for dep in deps {
                    adj_list.entry(dep.clone()).or_default().push(cmd.clone());
                    *in_degree.entry(cmd.clone()).or_insert(0) += 1;
                    to_process.push_back(dep.clone());
                }
            } else {
                in_degree.entry(cmd.clone()).or_insert(0);
            }
        }

        // Kahn's algorithm
        let mut queue = VecDeque::new();
        for (cmd, &degree) in &in_degree {
            if degree == 0 {
                queue.push_back(cmd.clone());
            }
        }

        let mut result = Vec::new();
        while let Some(cmd) = queue.pop_front() {
            result.push(cmd.clone());

            if let Some(dependents) = adj_list.get(&cmd) {
                for dependent in dependents {
                    if let Some(degree) = in_degree.get_mut(dependent) {
                        *degree -= 1;
                        if *degree == 0 {
                            queue.push_back(dependent.clone());
                        }
                    }
                }
            }
        }

        // 訪問したノード数と結果の長さが一致するか確認（循環検出）
        if result.len() != in_degree.len() {
            anyhow::bail!(
                "{}",
                get_message(MessageKey::ErrorCircularDependency, Language::English)
            );
        }

        Ok(result)
    }
}

impl Default for DependencyGraph {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::schema::{Command, CommandSpec, GlobalConfig, Hooks};

    fn create_test_config() -> CommandsConfig {
        let mut commands = AHashMap::new();

        commands.insert(
            "build".to_string(),
            Command {
                description: "Build".to_string(),
                cmd: CommandSpec::Single("cargo build".to_string()),
                env: AHashMap::new(),
                working_dir: None,
                deps: vec![],
                platform: vec![],
                tags: vec![],
                timeout: None,
                parallel: false,
                confirm: false,
                allow_chaining: None,
                allow_subshells: None,
            },
        );

        commands.insert(
            "test".to_string(),
            Command {
                description: "Test".to_string(),
                cmd: CommandSpec::Single("cargo test".to_string()),
                env: AHashMap::new(),
                working_dir: None,
                deps: vec!["build".to_string()],
                platform: vec![],
                tags: vec![],
                timeout: None,
                parallel: false,
                confirm: false,
                allow_chaining: None,
                allow_subshells: None,
            },
        );

        CommandsConfig {
            config: GlobalConfig::default(),
            commands,
            aliases: AHashMap::new(),
            hooks: Hooks::default(),
            plugins: Default::default(),
        }
    }

    #[test]
    fn test_validate_simple_config() {
        let config = create_test_config();
        let validator = ConfigValidator::new(&config);
        assert!(validator.validate().is_ok());
    }

    #[test]
    fn test_detect_circular_dependency() {
        let mut config = create_test_config();

        // 循環依存を作成: build -> test -> build
        config
            .commands
            .get_mut("build")
            .unwrap()
            .deps
            .push("test".to_string());

        let validator = ConfigValidator::new(&config);
        let result = validator.validate();
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Circular dependency"));
    }

    #[test]
    fn test_missing_dependency() {
        let mut config = create_test_config();

        config
            .commands
            .get_mut("test")
            .unwrap()
            .deps
            .push("nonexistent".to_string());

        let validator = ConfigValidator::new(&config);
        let result = validator.validate();
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Command not found"));
    }

    #[test]
    fn test_topological_sort() {
        let config = create_test_config();
        let validator = ConfigValidator::new(&config);

        let order = validator
            .compute_execution_order(&["test".to_string()])
            .unwrap();

        // build が test より先に実行されるべき
        let build_idx = order.iter().position(|s| s == "build").unwrap();
        let test_idx = order.iter().position(|s| s == "test").unwrap();
        assert!(build_idx < test_idx);
    }

    #[test]
    fn test_alias_resolution() {
        let mut config = create_test_config();
        config.aliases.insert("t".to_string(), "test".to_string());

        let validator = ConfigValidator::new(&config);
        assert!(validator.validate().is_ok());
    }

    #[test]
    fn test_invalid_command_name() {
        let mut config = create_test_config();
        config.commands.insert(
            "invalid name!".to_string(),
            Command {
                description: "Invalid".to_string(),
                cmd: CommandSpec::Single("echo test".to_string()),
                env: AHashMap::new(),
                working_dir: None,
                deps: vec![],
                platform: vec![],
                tags: vec![],
                timeout: None,
                parallel: false,
                confirm: false,
                allow_chaining: None,
                allow_subshells: None,
            },
        );

        let validator = ConfigValidator::new(&config);
        let result = validator.validate();
        assert!(result.is_err());
    }
}
