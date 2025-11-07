//! 並列実行機能のデモンストレーション

use ahash::AHashMap;
use cmdrun::command::dependency::DependencyGraph;
use cmdrun::command::executor::{CommandExecutor, ExecutionContext};
use cmdrun::config::schema::{Command, CommandSpec, CommandsConfig, GlobalConfig, PluginsConfig};
use std::path::PathBuf;
use std::time::Instant;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("=== 並列実行機能のデモ ===\n");

    // テスト用の設定を作成
    let config = create_test_config();

    // 依存関係グラフを作成
    let dep_graph = DependencyGraph::new(&config);

    // 循環依存チェック
    println!("1. 循環依存チェック...");
    match dep_graph.check_cycles() {
        Ok(_) => println!("   ✓ 循環依存なし\n"),
        Err(e) => {
            println!("   ✗ 循環依存検出: {}\n", e);
            return Err(e.into());
        }
    }

    // root コマンドの依存関係を解決
    println!("2. 依存関係解決 (root コマンド)");
    let groups = dep_graph.resolve("root")?;

    println!("   実行グループ数: {}\n", groups.len());
    for (idx, group) in groups.iter().enumerate() {
        println!(
            "   グループ {}: {:?} ({} commands)",
            idx + 1,
            group.commands,
            group.commands.len()
        );
    }
    println!();

    // 実行コンテキスト作成
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
        echo: true,
        color: true,
    };

    let executor = CommandExecutor::new(ctx);

    // 各グループを順次実行（グループ内は並列）
    println!("3. 並列実行開始\n");
    let start = Instant::now();

    for (idx, group) in groups.iter().enumerate() {
        println!("▶ グループ {}/{} を実行中...", idx + 1, groups.len());

        // グループ内のコマンドを取得
        let commands: Vec<_> = group
            .commands
            .iter()
            .filter_map(|name| config.commands.get(*name))
            .collect();

        // 並列実行
        let results = executor.execute_parallel(&commands).await?;

        // 結果確認
        for (i, result) in results.iter().enumerate() {
            if result.success {
                println!(
                    "  ✓ コマンド {} 完了 ({:.2}s)",
                    group.commands[i],
                    result.duration.as_secs_f64()
                );
            } else {
                println!(
                    "  ✗ コマンド {} 失敗 (exit code: {})",
                    group.commands[i], result.exit_code
                );
                return Err(anyhow::anyhow!("Command failed"));
            }
        }
        println!();
    }

    let total_duration = start.elapsed();
    println!(
        "✓ すべてのコマンド完了 (合計時間: {:.2}s)",
        total_duration.as_secs_f64()
    );

    Ok(())
}

/// テスト用の依存関係設定を作成
fn create_test_config() -> CommandsConfig {
    let mut commands = AHashMap::new();

    // ルートコマンド（a と b に依存）
    commands.insert(
        "root".to_string(),
        Command {
            description: "Root command".to_string(),
            cmd: CommandSpec::Single("echo root_executed".to_string()),
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
            cmd: CommandSpec::Single("echo command_a_executed".to_string()),
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
            cmd: CommandSpec::Single("echo command_b_executed".to_string()),
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
            cmd: CommandSpec::Single("echo base_executed".to_string()),
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
        plugins: PluginsConfig::default(),
    }
}
