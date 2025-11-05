//! 並列実行機能のテスト

use cmdrun::command::dependency::DependencyGraph;
use cmdrun::command::executor::{CommandExecutor, ExecutionContext};
use cmdrun::config::schema::{Command, CommandSpec, CommandsConfig, GlobalConfig};
use ahash::AHashMap;
use std::path::PathBuf;
use std::time::Instant;

/// テスト用の設定を作成
fn create_test_config() -> CommandsConfig {
    let mut commands = AHashMap::new();

    // 依存関係のないコマンド
    commands.insert(
        "fast1".to_string(),
        Command {
            description: "Fast command 1".to_string(),
            cmd: CommandSpec::Single("echo fast1".to_string()),
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
        "fast2".to_string(),
        Command {
            description: "Fast command 2".to_string(),
            cmd: CommandSpec::Single("echo fast2".to_string()),
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
        "fast3".to_string(),
        Command {
            description: "Fast command 3".to_string(),
            cmd: CommandSpec::Single("echo fast3".to_string()),
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
    }
}

/// 依存関係のあるテスト設定を作成
fn create_dependency_config() -> CommandsConfig {
    let mut commands = AHashMap::new();

    // ルートコマンド（a と b に依存）
    commands.insert(
        "root".to_string(),
        Command {
            description: "Root command".to_string(),
            cmd: CommandSpec::Single("echo root".to_string()),
            env: AHashMap::new(),
            working_dir: None,
            deps: vec!["a".to_string(), "b".to_string()],
            platform: vec![],
            tags: vec![],
            timeout: None,
            parallel: false,
            confirm: false,
        },
    );

    // a は base に依存
    commands.insert(
        "a".to_string(),
        Command {
            description: "Command A".to_string(),
            cmd: CommandSpec::Single("echo a".to_string()),
            env: AHashMap::new(),
            working_dir: None,
            deps: vec!["base".to_string()],
            platform: vec![],
            tags: vec![],
            timeout: None,
            parallel: false,
            confirm: false,
        },
    );

    // b も base に依存
    commands.insert(
        "b".to_string(),
        Command {
            description: "Command B".to_string(),
            cmd: CommandSpec::Single("echo b".to_string()),
            env: AHashMap::new(),
            working_dir: None,
            deps: vec!["base".to_string()],
            platform: vec![],
            tags: vec![],
            timeout: None,
            parallel: false,
            confirm: false,
        },
    );

    // base（依存なし）
    commands.insert(
        "base".to_string(),
        Command {
            description: "Base command".to_string(),
            cmd: CommandSpec::Single("echo base".to_string()),
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
    }
}

#[tokio::test]
async fn test_parallel_execution() {
    let config = create_test_config();
    let ctx = ExecutionContext {
        working_dir: PathBuf::from("."),
        env: AHashMap::new(),
        shell: if cfg!(windows) {
            "cmd".to_string()
        } else {
            "bash".to_string()
        },
        timeout: Some(10),
        strict: true,
        echo: false,
        color: false,
    };

    let executor = CommandExecutor::new(ctx);

    // 3つのコマンドを並列実行
    let commands: Vec<_> = ["fast1", "fast2", "fast3"]
        .iter()
        .filter_map(|name| config.commands.get(*name))
        .collect();

    let start = Instant::now();
    let results = executor.execute_parallel(&commands).await;
    let duration = start.elapsed();

    assert!(results.is_ok());
    let results = results.unwrap();
    assert_eq!(results.len(), 3);

    // すべて成功していること
    for result in &results {
        assert!(result.success);
    }

    // 並列実行なので、逐次実行より速いはず（厳密な検証は難しいので存在確認のみ）
    println!("Parallel execution took: {:?}", duration);
}

#[tokio::test]
async fn test_dependency_resolution() {
    let config = create_dependency_config();
    let dep_graph = DependencyGraph::new(&config);

    // root コマンドの依存関係を解決
    let groups = dep_graph.resolve("root").unwrap();

    // 3つのグループが生成されるべき
    // Group 1: base
    // Group 2: a, b (並列実行可能)
    // Group 3: root
    assert_eq!(groups.len(), 3);

    // Group 1: base
    assert_eq!(groups[0].commands.len(), 1);
    assert!(groups[0].commands.contains(&"base"));

    // Group 2: a と b
    assert_eq!(groups[1].commands.len(), 2);
    assert!(groups[1].commands.contains(&"a"));
    assert!(groups[1].commands.contains(&"b"));

    // Group 3: root
    assert_eq!(groups[2].commands.len(), 1);
    assert!(groups[2].commands.contains(&"root"));
}

#[tokio::test]
async fn test_parallel_with_dependencies() {
    let config = create_dependency_config();
    let ctx = ExecutionContext {
        working_dir: PathBuf::from("."),
        env: AHashMap::new(),
        shell: if cfg!(windows) {
            "cmd".to_string()
        } else {
            "bash".to_string()
        },
        timeout: Some(10),
        strict: true,
        echo: false,
        color: false,
    };

    let executor = CommandExecutor::new(ctx);
    let dep_graph = DependencyGraph::new(&config);

    // 依存関係を解決
    let groups = dep_graph.resolve("root").unwrap();

    // 各グループを順次実行
    for group in groups {
        let commands: Vec<_> = group
            .commands
            .iter()
            .filter_map(|name| config.commands.get(*name))
            .collect();

        let results = executor.execute_parallel(&commands).await;
        assert!(results.is_ok());

        let results = results.unwrap();
        for result in results {
            assert!(result.success);
        }
    }
}

#[tokio::test]
async fn test_circular_dependency_detection() {
    let mut config = create_dependency_config();

    // 循環依存を作成: base -> root
    config
        .commands
        .get_mut("base")
        .unwrap()
        .deps
        .push("root".to_string());

    let dep_graph = DependencyGraph::new(&config);

    // 循環依存が検出されるべき
    let result = dep_graph.check_cycles();
    assert!(result.is_err());
}

#[tokio::test]
async fn test_parallel_error_handling() {
    let mut config = create_test_config();

    // エラーを起こすコマンドを追加
    config.commands.insert(
        "fail".to_string(),
        Command {
            description: "Failing command".to_string(),
            cmd: CommandSpec::Single("exit 1".to_string()),
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

    let ctx = ExecutionContext {
        working_dir: PathBuf::from("."),
        env: AHashMap::new(),
        shell: if cfg!(windows) {
            "cmd".to_string()
        } else {
            "bash".to_string()
        },
        timeout: Some(10),
        strict: true,
        echo: false,
        color: false,
    };

    let executor = CommandExecutor::new(ctx);

    // 失敗するコマンドを含む並列実行
    let commands: Vec<_> = ["fast1", "fail", "fast2"]
        .iter()
        .filter_map(|name| config.commands.get(*name))
        .collect();

    let result = executor.execute_parallel(&commands).await;

    // エラーが返されるべき
    assert!(result.is_err());
}
