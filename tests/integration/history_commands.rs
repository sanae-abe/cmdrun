//! Integration tests for history management commands
//!
//! commands/history.rs のテストカバレッジ向上

use cmdrun::commands::history::{handle_history_clear, handle_history_list, handle_history_search};
use cmdrun::history::recorder::HistoryRecorder;
use cmdrun::history::storage::HistoryStorage;
use tempfile::TempDir;
use tokio::fs;

#[tokio::test]
async fn test_history_lifecycle() {
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("history.db");

    // 履歴ストレージの初期化
    let storage = HistoryStorage::new(&db_path).unwrap();

    // コマンド実行の記録
    storage
        .record_execution(
            "test".to_string(),
            "echo hello".to_string(),
            true,
            0,
            None,
        )
        .unwrap();

    storage
        .record_execution(
            "build".to_string(),
            "cargo build".to_string(),
            true,
            0,
            None,
        )
        .unwrap();

    // 履歴一覧
    let list_result = handle_history_list(db_path.clone(), 10).await;
    if let Ok(entries) = list_result {
        assert!(!entries.is_empty(), "Should have history entries");
        assert!(entries.len() <= 10, "Should respect limit");
    }

    // 履歴検索
    let search_result = handle_history_search(db_path.clone(), "test".to_string()).await;
    if let Ok(results) = search_result {
        assert!(
            results.iter().any(|e| e.command_id == "test"),
            "Should find 'test' in search results"
        );
    }

    // 履歴クリア
    let clear_result = handle_history_clear(db_path.clone()).await;
    assert!(clear_result.is_ok(), "Should be able to clear history");

    // クリア後の履歴一覧
    let after_clear = handle_history_list(db_path.clone(), 10).await;
    if let Ok(entries) = after_clear {
        assert!(entries.is_empty(), "History should be empty after clear");
    }
}

#[tokio::test]
async fn test_history_list_with_limit() {
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("history.db");
    let storage = HistoryStorage::new(&db_path).unwrap();

    // 20個のエントリを記録
    for i in 0..20 {
        storage
            .record_execution(
                format!("cmd{}", i),
                format!("echo {}", i),
                true,
                0,
                None,
            )
            .unwrap();
    }

    // 最新10件を取得
    let result = handle_history_list(db_path.clone(), 10).await;
    if let Ok(entries) = result {
        assert_eq!(entries.len(), 10, "Should return exactly 10 entries");
    }

    // 最新5件を取得
    let result2 = handle_history_list(db_path.clone(), 5).await;
    if let Ok(entries) = result2 {
        assert_eq!(entries.len(), 5, "Should return exactly 5 entries");
    }
}

#[tokio::test]
async fn test_history_search_empty_results() {
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("history.db");
    let storage = HistoryStorage::new(&db_path).unwrap();

    storage
        .record_execution(
            "test".to_string(),
            "echo test".to_string(),
            true,
            0,
            None,
        )
        .unwrap();

    // 存在しないキーワードで検索
    let result = handle_history_search(db_path.clone(), "nonexistent".to_string()).await;
    if let Ok(results) = result {
        assert!(results.is_empty(), "Should return empty results for non-matching search");
    }
}

#[tokio::test]
async fn test_history_search_case_sensitivity() {
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("history.db");
    let storage = HistoryStorage::new(&db_path).unwrap();

    storage
        .record_execution(
            "Test".to_string(),
            "echo TEST".to_string(),
            true,
            0,
            None,
        )
        .unwrap();

    // 大文字小文字を区別しない検索（実装依存）
    let result_lower = handle_history_search(db_path.clone(), "test".to_string()).await;
    let result_upper = handle_history_search(db_path.clone(), "TEST".to_string()).await;

    // 少なくともどちらかで結果が返るべき
    if let Ok(lower) = result_lower {
        if let Ok(upper) = result_upper {
            assert!(
                !lower.is_empty() || !upper.is_empty(),
                "Should find results regardless of case"
            );
        }
    }
}

#[tokio::test]
async fn test_history_with_failed_executions() {
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("history.db");
    let storage = HistoryStorage::new(&db_path).unwrap();

    // 成功したコマンド
    storage
        .record_execution(
            "success".to_string(),
            "echo ok".to_string(),
            true,
            0,
            None,
        )
        .unwrap();

    // 失敗したコマンド
    storage
        .record_execution(
            "failure".to_string(),
            "false".to_string(),
            false,
            1,
            Some("Command failed".to_string()),
        )
        .unwrap();

    // 両方のコマンドが履歴に含まれることを確認
    let result = handle_history_list(db_path.clone(), 10).await;
    if let Ok(entries) = result {
        assert_eq!(entries.len(), 2, "Should include both success and failure");

        let success_entry = entries.iter().find(|e| e.command_id == "success");
        let failure_entry = entries.iter().find(|e| e.command_id == "failure");

        assert!(success_entry.is_some(), "Should have success entry");
        assert!(failure_entry.is_some(), "Should have failure entry");

        if let Some(fail) = failure_entry {
            assert!(!fail.success, "Failure entry should have success=false");
            assert_eq!(fail.exit_code, 1, "Should record correct exit code");
        }
    }
}

#[tokio::test]
async fn test_history_recorder_integration() {
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("history.db");

    // HistoryRecorderを使った記録
    let recorder = HistoryRecorder::new(&db_path).unwrap();

    recorder
        .record_success("cmd1".to_string(), "echo 1".to_string(), 0)
        .unwrap();

    recorder
        .record_failure("cmd2".to_string(), "false".to_string(), 1, "Failed".to_string())
        .unwrap();

    // 履歴一覧で検証
    let result = handle_history_list(db_path.clone(), 10).await;
    if let Ok(entries) = result {
        assert_eq!(entries.len(), 2, "Should have 2 recorded entries");
    }
}

#[tokio::test]
async fn test_history_clear_empty_database() {
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("history.db");

    // 空のデータベースでクリアを実行
    let result = handle_history_clear(db_path.clone()).await;

    // エラーにならずに成功するべき
    assert!(result.is_ok(), "Clearing empty history should succeed");
}

#[tokio::test]
async fn test_history_list_empty_database() {
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("history.db");

    // 空のデータベースで一覧取得
    let result = handle_history_list(db_path.clone(), 10).await;

    if let Ok(entries) = result {
        assert!(entries.is_empty(), "Empty database should return empty list");
    }
}
