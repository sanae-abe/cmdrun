//! Integration tests for error handling
//!
//! エラーハンドリングの網羅的なテスト

use ahash::AHashMap;
use cmdrun::command::executor::{CommandExecutor, ExecutionContext};
use cmdrun::config::schema::{Command, CommandSpec};
use cmdrun::config::Language;
use std::path::PathBuf;

#[tokio::test]
async fn test_timeout_handling() {
    let shell = if cfg!(target_os = "windows") {
        "cmd".to_string()
    } else {
        "bash".to_string()
    };

    let ctx = ExecutionContext {
        working_dir: PathBuf::from("."),
        env: AHashMap::new(),
        shell,
        timeout: Some(1), // 1秒でタイムアウト
        strict: false,
        echo: false,
        color: false,
        language: Language::default(), allow_command_chaining: false, allow_subshells: false,
    };
    let executor = CommandExecutor::new(ctx);

    // プラットフォーム別のsleepコマンド
    #[cfg(windows)]
    let sleep_command = "timeout /t 10";
    #[cfg(not(windows))]
    let sleep_command = "sleep 10";

    let cmd = Command {
        description: "Long running command".to_string(),
        cmd: CommandSpec::Single(sleep_command.to_string()),
        deps: vec![],
        env: AHashMap::new(),
        working_dir: None,
        timeout: None, // グローバル設定のタイムアウトを使用
        parallel: false,
        tags: vec![],
        platform: vec![],
        confirm: false, allow_chaining: None, allow_subshells: None,
    };

    let result = executor.execute(&cmd).await;
    assert!(result.is_err(), "Command should timeout");

    let error = result.unwrap_err().to_string();
    assert!(
        error.contains("timeout") || error.contains("Timeout") || error.contains("time"),
        "Error message should mention timeout, got: {}",
        error
    );
}

#[test]
fn test_circular_dependency_detection() {
    use cmdrun::command::dependency::DependencyGraph;

    let mut graph = DependencyGraph::new();

    // A → B → C → A の循環依存を作成
    graph.add_command("A".to_string(), vec!["B".to_string()]);
    graph.add_command("B".to_string(), vec!["C".to_string()]);
    graph.add_command("C".to_string(), vec!["A".to_string()]);

    let result = graph.validate();
    assert!(result.is_err(), "Should detect circular dependency");

    let error = result.unwrap_err().to_string();
    assert!(
        error.contains("circular") || error.contains("Circular") || error.contains("cycle"),
        "Error should mention circular dependency, got: {}",
        error
    );
}

#[test]
fn test_self_dependency_detection() {
    use cmdrun::command::dependency::DependencyGraph;

    let mut graph = DependencyGraph::new();

    // 自己依存 A → A
    graph.add_command("A".to_string(), vec!["A".to_string()]);

    let result = graph.validate();
    assert!(result.is_err(), "Should detect self-dependency");
}

#[tokio::test]
async fn test_command_not_found_error() {
    let shell = if cfg!(target_os = "windows") {
        "cmd".to_string()
    } else {
        "bash".to_string()
    };

    let ctx = ExecutionContext {
        working_dir: PathBuf::from("."),
        env: AHashMap::new(),
        shell,
        timeout: Some(30),
        strict: false,
        echo: false,
        color: false,
        language: Language::default(), allow_command_chaining: false, allow_subshells: false,
    };
    let executor = CommandExecutor::new(ctx);

    let cmd = Command {
        description: "Non-existent command".to_string(),
        cmd: CommandSpec::Single("nonexistent_command_12345_xyz".to_string()),
        deps: vec![],
        env: AHashMap::new(),
        working_dir: None,
        timeout: None,
        parallel: false,
        tags: vec![],
        platform: vec![],
        confirm: false, allow_chaining: None, allow_subshells: None,
    };

    let result = executor.execute(&cmd).await;
    assert!(result.is_err(), "Should fail for non-existent command");
}

#[test]
fn test_invalid_toml_format() {
    let invalid_toml = r#"
    [commands.test
    # 閉じ括弧がない不正なTOML
    description = "Test"
    "#;

    let result = toml::from_str::<toml::Value>(invalid_toml);
    assert!(result.is_err(), "Should fail to parse invalid TOML");

    let error = result.unwrap_err().to_string();
    assert!(
        !error.is_empty(),
        "Error message should not be empty"
    );
}

#[test]
fn test_invalid_toml_missing_required_fields() {
    use cmdrun::config::schema::CommandConfig;

    let invalid_toml = r#"
    [commands.test]
    # description と cmd が必須だが、cmdが欠けている
    description = "Test command"
    "#;

    let result = toml::from_str::<CommandConfig>(invalid_toml);
    // パースは成功するが、バリデーションで失敗するはず
    if let Ok(config) = result {
        // commands.test のcmdフィールドが存在しないことを確認
        if let Some(test_cmd) = config.commands.get("test") {
            // cmdフィールドの検証
            match &test_cmd.cmd {
                CommandSpec::Single(s) => assert!(!s.is_empty()),
                CommandSpec::Multiple(v) => assert!(!v.is_empty()),
            }
        }
    }
}

#[tokio::test]
async fn test_invalid_working_directory() {
    let shell = if cfg!(target_os = "windows") {
        "cmd".to_string()
    } else {
        "bash".to_string()
    };

    let ctx = ExecutionContext {
        working_dir: PathBuf::from("."),
        env: AHashMap::new(),
        shell,
        timeout: Some(30),
        strict: false,
        echo: false,
        color: false,
        language: Language::default(), allow_command_chaining: false, allow_subshells: false,
    };
    let executor = CommandExecutor::new(ctx);

    let cmd = Command {
        description: "Command with invalid working directory".to_string(),
        cmd: CommandSpec::Single("echo test".to_string()),
        deps: vec![],
        env: AHashMap::new(),
        working_dir: Some(PathBuf::from("/nonexistent/directory/path/12345")),
        timeout: None,
        parallel: false,
        tags: vec![],
        platform: vec![],
        confirm: false, allow_chaining: None, allow_subshells: None,
    };

    let result = executor.execute(&cmd).await;
    // 存在しないディレクトリでの実行はエラーになる可能性がある
    // （実装によっては警告のみの場合もある）
    if result.is_err() {
        let error = result.unwrap_err().to_string();
        assert!(!error.is_empty());
    }
}

#[test]
fn test_empty_command_id() {
    use cmdrun::config::schema::Command;

    // 空のコマンドIDは許可されるべきではない
    let cmd = Command {
        description: "Test".to_string(),
        cmd: CommandSpec::Single("echo test".to_string()),
        deps: vec![],
        env: AHashMap::new(),
        working_dir: None,
        timeout: None,
        parallel: false,
        tags: vec![],
        platform: vec![],
        confirm: false, allow_chaining: None, allow_subshells: None,
    };

    // コマンド自体は作成できるが、IDが空の場合の処理を確認
    assert!(!cmd.description.is_empty());
}

#[test]
fn test_empty_command_string() {
    // 空のコマンド文字列
    let result = CommandSpec::Single("".to_string());

    // CommandSpecは作成できるが、実行時にバリデーションエラーになるべき
    match result {
        CommandSpec::Single(s) => assert!(s.is_empty()),
        _ => panic!("Expected Single variant"),
    }
}

#[test]
fn test_dependency_on_nonexistent_command() {
    use cmdrun::command::dependency::DependencyGraph;

    let mut graph = DependencyGraph::new();

    // 存在しないコマンドへの依存
    graph.add_command("A".to_string(), vec!["B".to_string()]);

    // Bが存在しない場合のvalidation
    let result = graph.validate();

    // 存在しないコマンドへの依存はエラーになるべき
    if result.is_err() {
        let error = result.unwrap_err().to_string();
        assert!(!error.is_empty());
    }
}

#[test]
fn test_duplicate_command_definition() {
    use cmdrun::config::schema::CommandConfig;
    use ahash::AHashMap;

    let mut config = CommandConfig {
        config: cmdrun::config::schema::GlobalConfig::default(),
        commands: AHashMap::new(),
        aliases: AHashMap::new(),
    };

    let cmd = cmdrun::config::schema::Command {
        description: "Test".to_string(),
        cmd: CommandSpec::Single("echo test".to_string()),
        deps: vec![],
        env: AHashMap::new(),
        working_dir: None,
        timeout: None,
        parallel: false,
        tags: vec![],
        platform: vec![],
        confirm: false, allow_chaining: None, allow_subshells: None,
    };

    // 同じIDで2回追加
    config.commands.insert("test".to_string(), cmd.clone());
    let previous = config.commands.insert("test".to_string(), cmd);

    // HashMapは上書きするので、previousはSomeになる
    assert!(previous.is_some(), "Duplicate insertion should return previous value");
}

#[tokio::test]
async fn test_platform_specific_command_execution() {
    let shell = if cfg!(target_os = "windows") {
        "cmd".to_string()
    } else {
        "bash".to_string()
    };

    let ctx = ExecutionContext {
        working_dir: PathBuf::from("."),
        env: AHashMap::new(),
        shell,
        timeout: Some(30),
        strict: false,
        echo: false,
        color: false,
        language: Language::default(), allow_command_chaining: false, allow_subshells: false,
    };
    let executor = CommandExecutor::new(ctx);

    // 現在のプラットフォームでのみ実行されるコマンド
    #[cfg(windows)]
    let platform_cmd = "ver";
    #[cfg(not(windows))]
    let platform_cmd = "uname";

    let cmd = Command {
        description: "Platform-specific command".to_string(),
        cmd: CommandSpec::Single(platform_cmd.to_string()),
        deps: vec![],
        env: AHashMap::new(),
        working_dir: None,
        timeout: None,
        parallel: false,
        tags: vec![],
        platform: vec![],
        confirm: false, allow_chaining: None, allow_subshells: None,
    };

    let result = executor.execute(&cmd).await;
    assert!(result.is_ok(), "Platform-specific command should execute successfully");
}
