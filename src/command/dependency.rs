//! 依存関係解決とトポロジカルソート
//!
//! コマンドの依存関係を解析し、並列実行可能なグループを特定

use crate::config::schema::{Command, CommandsConfig};
use crate::error::{ExecutionError, Result};
use ahash::{AHashMap, AHashSet};
use std::collections::VecDeque;

/// 依存関係グラフ
#[derive(Debug)]
pub struct DependencyGraph<'a> {
    /// コマンド名から依存先へのマップ
    adjacency: AHashMap<&'a str, Vec<&'a str>>,
    /// 入次数（依存される数）
    #[allow(dead_code)]
    in_degree: AHashMap<&'a str, usize>,
    /// 全コマンド
    commands: &'a AHashMap<String, Command>,
}

/// 実行グループ（同じグループ内のコマンドは並列実行可能）
#[derive(Debug, Clone)]
pub struct ExecutionGroup<'a> {
    /// 実行するコマンド名のリスト
    pub commands: Vec<&'a str>,
}

impl<'a> DependencyGraph<'a> {
    /// 新規依存関係グラフ作成
    pub fn new(config: &'a CommandsConfig) -> Self {
        let mut adjacency = AHashMap::new();
        let mut in_degree = AHashMap::new();

        // 全コマンドを初期化
        for name in config.commands.keys() {
            adjacency.insert(name.as_str(), Vec::new());
            in_degree.insert(name.as_str(), 0);
        }

        // 依存関係を構築
        for (name, command) in &config.commands {
            for dep in &command.deps {
                // dep -> name の依存関係
                adjacency
                    .entry(dep.as_str())
                    .or_insert_with(Vec::new)
                    .push(name.as_str());
                *in_degree.entry(name.as_str()).or_insert(0) += 1;
            }
        }

        Self {
            adjacency,
            in_degree,
            commands: &config.commands,
        }
    }

    /// 特定のコマンドとその依存関係を解決し、実行順序を取得
    pub fn resolve(&self, command_name: &'a str) -> Result<Vec<ExecutionGroup<'a>>> {
        // 依存関係のサブグラフを抽出
        let subgraph = self.extract_subgraph(command_name)?;

        // トポロジカルソート（Kahn's Algorithm）
        self.topological_sort_groups(&subgraph)
    }

    /// サブグラフ抽出（特定コマンドとその依存関係のみ）
    fn extract_subgraph(&self, start: &'a str) -> Result<AHashSet<&'a str>> {
        let mut visited = AHashSet::new();
        let mut queue = VecDeque::new();

        // コマンドの存在確認
        if !self.commands.contains_key(start) {
            return Err(ExecutionError::CommandFailed {
                command: start.to_string(),
                code: 1,
            }
            .into());
        }

        queue.push_back(start);
        visited.insert(start);

        // 依存関係を逆順にたどる
        while let Some(node) = queue.pop_front() {
            if let Some(command) = self.commands.get(node) {
                for dep in &command.deps {
                    let dep_str = dep.as_str();
                    if !visited.contains(dep_str) {
                        if !self.commands.contains_key(dep) {
                            return Err(ExecutionError::CommandFailed {
                                command: format!("Dependency not found: {}", dep),
                                code: 1,
                            }
                            .into());
                        }
                        visited.insert(dep_str);
                        queue.push_back(dep_str);
                    }
                }
            }
        }

        Ok(visited)
    }

    /// トポロジカルソートして並列実行可能なグループに分割
    fn topological_sort_groups(
        &self,
        subgraph: &AHashSet<&'a str>,
    ) -> Result<Vec<ExecutionGroup<'a>>> {
        let mut groups = Vec::new();
        let mut in_degree = AHashMap::new();
        let mut queue = VecDeque::new();

        // サブグラフの入次数を計算
        for &node in subgraph {
            let mut degree = 0;
            if let Some(command) = self.commands.get(node) {
                for dep in &command.deps {
                    if subgraph.contains(dep.as_str()) {
                        degree += 1;
                    }
                }
            }
            in_degree.insert(node, degree);

            if degree == 0 {
                queue.push_back(node);
            }
        }

        // レベル別にグループ化
        while !queue.is_empty() {
            let level_size = queue.len();
            let mut current_group = Vec::new();

            for _ in 0..level_size {
                if let Some(node) = queue.pop_front() {
                    current_group.push(node);

                    // 隣接ノードの入次数を減らす
                    if let Some(neighbors) = self.adjacency.get(node) {
                        for &neighbor in neighbors {
                            if subgraph.contains(neighbor) {
                                if let Some(degree) = in_degree.get_mut(neighbor) {
                                    *degree -= 1;
                                    if *degree == 0 {
                                        queue.push_back(neighbor);
                                    }
                                }
                            }
                        }
                    }
                }
            }

            if !current_group.is_empty() {
                groups.push(ExecutionGroup {
                    commands: current_group,
                });
            }
        }

        // 循環依存チェック
        let total_processed: usize = groups.iter().map(|g| g.commands.len()).sum();
        if total_processed != subgraph.len() {
            return Err(ExecutionError::CommandFailed {
                command: "Circular dependency detected".to_string(),
                code: 1,
            }
            .into());
        }

        Ok(groups)
    }

    /// 依存関係の循環チェック
    pub fn check_cycles(&self) -> Result<()> {
        for name in self.commands.keys() {
            let mut visited = AHashSet::new();
            let mut rec_stack = AHashSet::new();

            if self.has_cycle(name.as_str(), &mut visited, &mut rec_stack) {
                return Err(ExecutionError::CommandFailed {
                    command: format!("Circular dependency involving: {}", name),
                    code: 1,
                }
                .into());
            }
        }
        Ok(())
    }

    /// DFS で循環を検出
    fn has_cycle(
        &self,
        node: &'a str,
        visited: &mut AHashSet<&'a str>,
        rec_stack: &mut AHashSet<&'a str>,
    ) -> bool {
        if rec_stack.contains(node) {
            return true;
        }

        if visited.contains(node) {
            return false;
        }

        visited.insert(node);
        rec_stack.insert(node);

        if let Some(command) = self.commands.get(node) {
            for dep in &command.deps {
                if self.has_cycle(dep.as_str(), visited, rec_stack) {
                    return true;
                }
            }
        }

        rec_stack.remove(node);
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::schema::{CommandSpec, GlobalConfig};

    fn create_test_config() -> CommandsConfig {
        let mut commands = AHashMap::new();

        // a -> b, c
        commands.insert(
            "a".to_string(),
            Command {
                description: "Command A".to_string(),
                cmd: CommandSpec::Single("echo a".to_string()),
                env: AHashMap::new(),
                working_dir: None,
                deps: vec!["b".to_string(), "c".to_string()],
                platform: vec![],
                tags: vec![],
                timeout: None,
                parallel: false,
                confirm: false,
            },
        );

        // b -> d
        commands.insert(
            "b".to_string(),
            Command {
                description: "Command B".to_string(),
                cmd: CommandSpec::Single("echo b".to_string()),
                env: AHashMap::new(),
                working_dir: None,
                deps: vec!["d".to_string()],
                platform: vec![],
                tags: vec![],
                timeout: None,
                parallel: false,
                confirm: false,
            },
        );

        // c -> d
        commands.insert(
            "c".to_string(),
            Command {
                description: "Command C".to_string(),
                cmd: CommandSpec::Single("echo c".to_string()),
                env: AHashMap::new(),
                working_dir: None,
                deps: vec!["d".to_string()],
                platform: vec![],
                tags: vec![],
                timeout: None,
                parallel: false,
                confirm: false,
            },
        );

        // d (no deps)
        commands.insert(
            "d".to_string(),
            Command {
                description: "Command D".to_string(),
                cmd: CommandSpec::Single("echo d".to_string()),
                env: AHashMap::new(),
                working_dir: None,
                deps: vec![],
                platform: vec![],
                tags: vec![],
                timeout: None,
                parallel: false,
                confirm: false,
            },
        );

        CommandsConfig {
            config: GlobalConfig::default(),
            commands,
            aliases: AHashMap::new(),
            hooks: Default::default(),
            plugins: Default::default(),
        }
    }

    #[test]
    fn test_dependency_resolution() {
        let config = create_test_config();
        let graph = DependencyGraph::new(&config);

        let groups = graph.resolve("a").unwrap();

        // グループ1: d
        // グループ2: b, c (並列実行可能)
        // グループ3: a
        assert_eq!(groups.len(), 3);
        assert_eq!(groups[0].commands, vec!["d"]);
        assert_eq!(groups[1].commands.len(), 2);
        assert!(groups[1].commands.contains(&"b"));
        assert!(groups[1].commands.contains(&"c"));
        assert_eq!(groups[2].commands, vec!["a"]);
    }

    #[test]
    fn test_cycle_detection() {
        let mut config = create_test_config();

        // d -> a で循環を作成
        config
            .commands
            .get_mut("d")
            .unwrap()
            .deps
            .push("a".to_string());

        let graph = DependencyGraph::new(&config);
        assert!(graph.check_cycles().is_err());
    }

    #[test]
    fn test_missing_dependency() {
        let mut config = create_test_config();

        // 存在しない依存関係を追加
        config
            .commands
            .get_mut("a")
            .unwrap()
            .deps
            .push("nonexistent".to_string());

        let graph = DependencyGraph::new(&config);
        assert!(graph.resolve("a").is_err());
    }
}
