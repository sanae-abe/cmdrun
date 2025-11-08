//! グラフ視覚化機能
//!
//! 依存関係グラフを様々な形式で出力

use crate::cli::GraphFormat;
use crate::command::dependency::DependencyGraph;
use crate::config::schema::CommandsConfig;
use ahash::AHashSet;
use anyhow::Result;
use colored::*;
use std::fmt::Write as FmtWrite;

/// グラフ視覚化ユーティリティ
pub struct GraphVisualizer<'a> {
    config: &'a CommandsConfig,
    graph: DependencyGraph<'a>,
}

impl<'a> GraphVisualizer<'a> {
    /// 新規GraphVisualizer作成
    pub fn new(config: &'a CommandsConfig) -> Self {
        let graph = DependencyGraph::new(config);
        Self { config, graph }
    }

    /// 指定フォーマットで出力
    pub fn visualize(
        &self,
        command: Option<&str>,
        format: GraphFormat,
        show_groups: bool,
    ) -> Result<String> {
        match format {
            GraphFormat::Tree => self.format_tree(command, show_groups),
            GraphFormat::Dot => self.format_dot(command),
            GraphFormat::Mermaid => self.format_mermaid(command),
        }
    }

    /// ツリー形式（カラフル）
    fn format_tree(&self, command: Option<&str>, show_groups: bool) -> Result<String> {
        let mut output = String::new();

        if let Some(cmd_name) = command {
            // 特定コマンドの依存関係
            let cmd = self
                .config
                .commands
                .get(cmd_name)
                .ok_or_else(|| anyhow::anyhow!("Command not found: {}", cmd_name))?;

            writeln!(
                &mut output,
                "{} {}",
                "Dependencies for:".cyan().bold(),
                cmd_name.green().bold()
            )?;
            writeln!(&mut output)?;

            if cmd.deps.is_empty() {
                writeln!(
                    &mut output,
                    "  {} {}",
                    "ℹ".blue(),
                    "No dependencies (this command runs independently)".dimmed()
                )?;
            } else {
                self.print_tree_recursive(cmd_name, &mut output, "", true)?;
            }

            // 実行グループ表示
            if show_groups {
                writeln!(&mut output)?;
                self.print_execution_groups(cmd_name, &mut output)?;
            }
        } else {
            // 全体グラフ
            writeln!(&mut output, "{}", "Dependency Graph:".cyan().bold())?;
            writeln!(&mut output)?;

            let mut commands: Vec<_> = self.config.commands.iter().collect();
            commands.sort_by_key(|(name, _)| *name);

            for (name, cmd) in commands {
                if cmd.deps.is_empty() {
                    writeln!(&mut output, "{} {}", "📦".bright_white(), name.green())?;
                } else {
                    writeln!(
                        &mut output,
                        "{} {} {}",
                        "🔗".bright_white(),
                        name.green().bold(),
                        format!("({} dependencies)", cmd.deps.len()).dimmed()
                    )?;
                    for dep in &cmd.deps {
                        writeln!(&mut output, "  {} {}", "└─►".blue(), dep.bright_yellow())?;
                    }
                    writeln!(&mut output)?;
                }
            }
        }

        Ok(output)
    }

    /// 再帰的にツリー構造を描画
    fn print_tree_recursive(
        &self,
        name: &str,
        output: &mut String,
        prefix: &str,
        is_last: bool,
    ) -> Result<()> {
        let cmd = self
            .config
            .commands
            .get(name)
            .ok_or_else(|| anyhow::anyhow!("Command not found: {}", name))?;

        // 現在のノード
        let connector = if is_last { "└─" } else { "├─" };
        let icon = if cmd.deps.is_empty() { "📦" } else { "🔗" };

        writeln!(
            output,
            "{}{} {} {}",
            prefix,
            connector.blue(),
            icon,
            name.green().bold()
        )?;

        // 依存関係を再帰的に表示
        if !cmd.deps.is_empty() {
            let new_prefix = if is_last {
                format!("{}  ", prefix)
            } else {
                format!("{}│ ", prefix)
            };

            for (idx, dep) in cmd.deps.iter().enumerate() {
                let is_last_dep = idx == cmd.deps.len() - 1;
                self.print_tree_recursive(dep, output, &new_prefix, is_last_dep)?;
            }
        }

        Ok(())
    }

    /// 実行グループを表示
    fn print_execution_groups(&self, command: &str, output: &mut String) -> Result<()> {
        let groups = self.graph.resolve(command)?;

        writeln!(
            output,
            "{} {} groups",
            "Execution Plan:".cyan().bold(),
            groups.len()
        )?;
        writeln!(output)?;

        for (idx, group) in groups.iter().enumerate() {
            writeln!(
                output,
                "{} Group {} {} {}",
                "▶".blue().bold(),
                (idx + 1).to_string().bright_white().bold(),
                "/".dimmed(),
                groups.len().to_string().dimmed()
            )?;

            for cmd_name in &group.commands {
                let cmd = self.config.commands.get(*cmd_name);
                let desc = cmd.map(|c| c.description.as_str()).unwrap_or("");
                writeln!(
                    output,
                    "  {} {} {}",
                    "•".green(),
                    cmd_name.bright_yellow().bold(),
                    desc.dimmed()
                )?;
            }

            if group.commands.len() > 1 {
                writeln!(output, "  {} Can run in parallel", "⚡".bright_white())?;
            }
            writeln!(output)?;
        }

        Ok(())
    }

    /// DOT形式（Graphviz）
    fn format_dot(&self, command: Option<&str>) -> Result<String> {
        let mut output = String::new();

        writeln!(&mut output, "digraph dependencies {{")?;
        writeln!(&mut output, "  rankdir=TB;")?;
        writeln!(
            &mut output,
            "  node [shape=box, style=rounded, fontname=\"Arial\"];"
        )?;
        writeln!(&mut output)?;

        // ノードのスタイル定義
        writeln!(&mut output, "  // Node styles")?;

        let commands_to_show: Vec<&String> = if let Some(cmd_name) = command {
            // 特定コマンドとその依存関係のみ
            let subgraph = self.extract_command_subgraph(cmd_name)?;
            subgraph.into_iter().collect()
        } else {
            // 全コマンド
            self.config.commands.keys().collect()
        };

        // ノード定義
        for name in &commands_to_show {
            let cmd = &self.config.commands[*name];
            let color = if cmd.deps.is_empty() {
                "lightblue"
            } else {
                "lightgreen"
            };

            writeln!(
                &mut output,
                "  \"{}\" [label=\"{}\\n{}\", fillcolor={}, style=filled];",
                name,
                name,
                Self::escape_dot_string(&cmd.description),
                color
            )?;
        }

        writeln!(&mut output)?;
        writeln!(&mut output, "  // Dependencies")?;

        // エッジ定義
        for name in &commands_to_show {
            let cmd = &self.config.commands[*name];
            for dep in &cmd.deps {
                if commands_to_show.contains(&dep) {
                    writeln!(&mut output, "  \"{}\" -> \"{}\";", dep, name)?;
                }
            }
        }

        writeln!(&mut output, "}}")?;

        Ok(output)
    }

    /// Mermaid形式
    fn format_mermaid(&self, command: Option<&str>) -> Result<String> {
        let mut output = String::new();

        writeln!(&mut output, "graph TD")?;
        writeln!(&mut output)?;

        let commands_to_show: Vec<&String> = if let Some(cmd_name) = command {
            let subgraph = self.extract_command_subgraph(cmd_name)?;
            subgraph.into_iter().collect()
        } else {
            self.config.commands.keys().collect()
        };

        // ノード定義（スタイル付き）
        writeln!(&mut output, "  %% Node definitions")?;
        for name in &commands_to_show {
            let cmd = &self.config.commands[*name];
            let safe_id = Self::sanitize_mermaid_id(name);

            if cmd.deps.is_empty() {
                // 依存なし = 四角
                writeln!(
                    &mut output,
                    "  {}[\"{}<br/>{}\"]",
                    safe_id,
                    name,
                    Self::escape_mermaid_string(&cmd.description)
                )?;
            } else {
                // 依存あり = 丸角四角
                writeln!(
                    &mut output,
                    "  {}(\"{}<br/>{}\"))",
                    safe_id,
                    name,
                    Self::escape_mermaid_string(&cmd.description)
                )?;
            }
        }

        writeln!(&mut output)?;
        writeln!(&mut output, "  %% Dependencies")?;

        // エッジ定義
        for name in &commands_to_show {
            let cmd = &self.config.commands[*name];
            for dep in &cmd.deps {
                if commands_to_show.contains(&dep) {
                    let safe_from = Self::sanitize_mermaid_id(dep);
                    let safe_to = Self::sanitize_mermaid_id(name);
                    writeln!(&mut output, "  {} --> {}", safe_from, safe_to)?;
                }
            }
        }

        writeln!(&mut output)?;
        writeln!(&mut output, "  %% Styling")?;
        writeln!(
            &mut output,
            "  classDef default fill:#e1f5ff,stroke:#01579b,stroke-width:2px"
        )?;

        Ok(output)
    }

    /// コマンドとその依存関係を抽出
    fn extract_command_subgraph(&self, command: &str) -> Result<Vec<&String>> {
        let mut visited = AHashSet::new();
        let mut queue = vec![command];

        while let Some(name) = queue.pop() {
            if visited.contains(name) {
                continue;
            }

            visited.insert(name);

            if let Some(cmd) = self.config.commands.get(name) {
                for dep in &cmd.deps {
                    if !visited.contains(dep.as_str()) {
                        queue.push(dep.as_str());
                    }
                }
            }
        }

        let mut result: Vec<&String> = visited
            .into_iter()
            .filter_map(|name| self.config.commands.keys().find(|k| k.as_str() == name))
            .collect();

        result.sort();
        Ok(result)
    }

    /// DOT文字列エスケープ
    fn escape_dot_string(s: &str) -> String {
        s.replace('\\', "\\\\")
            .replace('"', "\\\"")
            .replace('\n', "\\n")
    }

    /// Mermaid文字列エスケープ
    fn escape_mermaid_string(s: &str) -> String {
        s.replace('"', "&quot;")
            .replace('<', "&lt;")
            .replace('>', "&gt;")
            .chars()
            .take(40) // 長すぎる説明を切り詰める
            .collect()
    }

    /// MermaidのID sanitize（英数字のみ）
    fn sanitize_mermaid_id(s: &str) -> String {
        s.chars()
            .map(|c| if c.is_alphanumeric() { c } else { '_' })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::schema::{Command, CommandSpec, GlobalConfig};
    use ahash::AHashMap;

    fn create_test_config() -> CommandsConfig {
        let mut commands = AHashMap::new();

        commands.insert(
            "build".to_string(),
            Command {
                description: "Build the project".to_string(),
                cmd: CommandSpec::Single("cargo build".to_string()),
                env: AHashMap::new(),
                working_dir: None,
                deps: vec!["lint".to_string(), "test".to_string()],
                platform: vec![],
                tags: vec![],
                timeout: None,
                parallel: false,
                confirm: false,
            },
        );

        commands.insert(
            "lint".to_string(),
            Command {
                description: "Run linter".to_string(),
                cmd: CommandSpec::Single("cargo clippy".to_string()),
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

        commands.insert(
            "test".to_string(),
            Command {
                description: "Run tests".to_string(),
                cmd: CommandSpec::Single("cargo test".to_string()),
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
    fn test_visualize_tree() {
        let config = create_test_config();
        let visualizer = GraphVisualizer::new(&config);

        let result = visualizer.visualize(Some("build"), GraphFormat::Tree, false);
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.contains("build"));
        assert!(output.contains("lint"));
        assert!(output.contains("test"));
    }

    #[test]
    fn test_visualize_dot() {
        let config = create_test_config();
        let visualizer = GraphVisualizer::new(&config);

        let result = visualizer.visualize(Some("build"), GraphFormat::Dot, false);
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.contains("digraph"));
        assert!(output.contains("build"));
        assert!(output.contains("->"));
    }

    #[test]
    fn test_visualize_mermaid() {
        let config = create_test_config();
        let visualizer = GraphVisualizer::new(&config);

        let result = visualizer.visualize(Some("build"), GraphFormat::Mermaid, false);
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.contains("graph TD"));
        assert!(output.contains("build"));
        assert!(output.contains("-->"));
    }

    // === Edge Case Tests ===

    #[test]
    fn test_visualize_nonexistent_command() {
        let config = create_test_config();
        let visualizer = GraphVisualizer::new(&config);

        let result = visualizer.visualize(Some("nonexistent"), GraphFormat::Tree, false);
        assert!(result.is_err(), "Should fail for non-existent command");
        assert!(result.unwrap_err().to_string().contains("not found"));
    }

    #[test]
    fn test_visualize_empty_config() {
        let config = CommandsConfig {
            config: GlobalConfig::default(),
            commands: AHashMap::new(),
            aliases: AHashMap::new(),
            hooks: Default::default(),
            plugins: Default::default(),
        };

        let visualizer = GraphVisualizer::new(&config);
        let result = visualizer.visualize(None, GraphFormat::Tree, false);
        assert!(result.is_ok(), "Empty config should succeed");
    }

    #[test]
    fn test_visualize_tree_with_groups() {
        let config = create_test_config();
        let visualizer = GraphVisualizer::new(&config);

        let result = visualizer.visualize(Some("build"), GraphFormat::Tree, true);
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.contains("Execution Plan"));
        assert!(output.contains("Group"));
    }

    #[test]
    fn test_visualize_all_commands_tree() {
        let config = create_test_config();
        let visualizer = GraphVisualizer::new(&config);

        let result = visualizer.visualize(None, GraphFormat::Tree, false);
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.contains("Dependency Graph"));
        assert!(output.contains("build"));
        assert!(output.contains("lint"));
        assert!(output.contains("test"));
    }

    #[test]
    fn test_visualize_all_commands_dot() {
        let config = create_test_config();
        let visualizer = GraphVisualizer::new(&config);

        let result = visualizer.visualize(None, GraphFormat::Dot, false);
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.contains("digraph dependencies"));
        assert!(output.contains("\"build\""));
        assert!(output.contains("\"lint\""));
        assert!(output.contains("\"test\""));
        assert!(output.contains("->"));
    }

    #[test]
    fn test_visualize_all_commands_mermaid() {
        let config = create_test_config();
        let visualizer = GraphVisualizer::new(&config);

        let result = visualizer.visualize(None, GraphFormat::Mermaid, false);
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.contains("graph TD"));
        assert!(output.contains("build"));
        assert!(output.contains("lint"));
        assert!(output.contains("test"));
    }

    #[test]
    fn test_escape_dot_string() {
        assert_eq!(GraphVisualizer::<'_>::escape_dot_string("hello"), "hello");
        assert_eq!(
            GraphVisualizer::<'_>::escape_dot_string("hello\\world"),
            "hello\\\\world"
        );
        assert_eq!(
            GraphVisualizer::<'_>::escape_dot_string("hello\"world"),
            "hello\\\"world"
        );
        assert_eq!(
            GraphVisualizer::<'_>::escape_dot_string("hello\nworld"),
            "hello\\nworld"
        );
    }

    #[test]
    fn test_escape_mermaid_string() {
        assert_eq!(
            GraphVisualizer::<'_>::escape_mermaid_string("hello"),
            "hello"
        );
        assert_eq!(
            GraphVisualizer::<'_>::escape_mermaid_string("hello\"world"),
            "hello&quot;world"
        );
        assert_eq!(
            GraphVisualizer::<'_>::escape_mermaid_string("hello<world>"),
            "hello&lt;world&gt;"
        );

        // Test truncation at 40 chars
        let long_str = "a".repeat(50);
        let escaped = GraphVisualizer::<'_>::escape_mermaid_string(&long_str);
        assert_eq!(escaped.len(), 40);
    }

    #[test]
    fn test_sanitize_mermaid_id() {
        assert_eq!(GraphVisualizer::<'_>::sanitize_mermaid_id("hello"), "hello");
        assert_eq!(
            GraphVisualizer::<'_>::sanitize_mermaid_id("hello-world"),
            "hello_world"
        );
        assert_eq!(
            GraphVisualizer::<'_>::sanitize_mermaid_id("hello@world"),
            "hello_world"
        );
        assert_eq!(
            GraphVisualizer::<'_>::sanitize_mermaid_id("hello123"),
            "hello123"
        );
    }

    #[test]
    fn test_complex_dependency_graph() {
        let mut commands = AHashMap::new();

        // Create a complex dependency graph
        commands.insert(
            "deploy".to_string(),
            Command {
                description: "Deploy to production".to_string(),
                cmd: CommandSpec::Single("deploy.sh".to_string()),
                env: AHashMap::new(),
                working_dir: None,
                deps: vec!["build".to_string(), "test".to_string()],
                platform: vec![],
                tags: vec![],
                timeout: None,
                parallel: false,
                confirm: false,
            },
        );

        commands.insert(
            "build".to_string(),
            Command {
                description: "Build project".to_string(),
                cmd: CommandSpec::Single("cargo build".to_string()),
                env: AHashMap::new(),
                working_dir: None,
                deps: vec!["lint".to_string()],
                platform: vec![],
                tags: vec![],
                timeout: None,
                parallel: false,
                confirm: false,
            },
        );

        commands.insert(
            "test".to_string(),
            Command {
                description: "Run tests".to_string(),
                cmd: CommandSpec::Single("cargo test".to_string()),
                env: AHashMap::new(),
                working_dir: None,
                deps: vec!["lint".to_string()],
                platform: vec![],
                tags: vec![],
                timeout: None,
                parallel: false,
                confirm: false,
            },
        );

        commands.insert(
            "lint".to_string(),
            Command {
                description: "Run linter".to_string(),
                cmd: CommandSpec::Single("cargo clippy".to_string()),
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

        let config = CommandsConfig {
            config: GlobalConfig::default(),
            commands,
            aliases: AHashMap::new(),
            hooks: Default::default(),
            plugins: Default::default(),
        };

        let visualizer = GraphVisualizer::new(&config);

        // Test tree format
        let result = visualizer.visualize(Some("deploy"), GraphFormat::Tree, true);
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.contains("deploy"));
        assert!(output.contains("build"));
        assert!(output.contains("test"));
        assert!(output.contains("lint"));

        // Test DOT format
        let result = visualizer.visualize(Some("deploy"), GraphFormat::Dot, false);
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.contains("digraph"));
        assert!(output.contains("\"deploy\""));
        assert!(output.contains("\"build\" -> \"deploy\""));

        // Test Mermaid format
        let result = visualizer.visualize(Some("deploy"), GraphFormat::Mermaid, false);
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.contains("graph TD"));
        assert!(output.contains("deploy"));
    }

    #[test]
    fn test_command_without_dependencies() {
        let mut commands = AHashMap::new();

        commands.insert(
            "standalone".to_string(),
            Command {
                description: "Standalone command".to_string(),
                cmd: CommandSpec::Single("echo hello".to_string()),
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

        let config = CommandsConfig {
            config: GlobalConfig::default(),
            commands,
            aliases: AHashMap::new(),
            hooks: Default::default(),
            plugins: Default::default(),
        };

        let visualizer = GraphVisualizer::new(&config);
        let result = visualizer.visualize(Some("standalone"), GraphFormat::Tree, false);
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.contains("No dependencies") || output.contains("runs independently"));
    }

    #[test]
    fn test_special_characters_in_descriptions() {
        let mut commands = AHashMap::new();

        commands.insert(
            "special".to_string(),
            Command {
                description: "Test \"quotes\" and <brackets> and & symbols".to_string(),
                cmd: CommandSpec::Single("echo test".to_string()),
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

        let config = CommandsConfig {
            config: GlobalConfig::default(),
            commands,
            aliases: AHashMap::new(),
            hooks: Default::default(),
            plugins: Default::default(),
        };

        let visualizer = GraphVisualizer::new(&config);

        // DOT format should escape properly
        let result = visualizer.visualize(Some("special"), GraphFormat::Dot, false);
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.contains("\\\""));

        // Mermaid format should escape properly
        let result = visualizer.visualize(Some("special"), GraphFormat::Mermaid, false);
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.contains("&quot;") || output.contains("&lt;") || output.contains("&gt;"));
    }
}
