//! Integration tests for history functionality

use cmdrun::history::{HistoryEntry, HistoryRecorder, HistoryStorage};
use std::collections::HashMap;
use tempfile::NamedTempFile;

#[test]
fn test_history_storage_lifecycle() {
    let temp_file = NamedTempFile::new().unwrap();
    let mut storage = HistoryStorage::with_path(temp_file.path()).unwrap();

    // Add entries
    let entry1 = create_test_entry("build", true);
    let id1 = storage.add(&entry1).unwrap();
    assert!(id1 > 0);

    // Sleep briefly to ensure different timestamps
    std::thread::sleep(std::time::Duration::from_millis(10));

    let entry2 = create_test_entry("test", false);
    let id2 = storage.add(&entry2).unwrap();
    assert!(id2 > 0);
    // Note: IDs should be different (autoincrement), but we won't assert strict ordering

    // Retrieve by ID
    let retrieved = storage.get_by_id(id1).unwrap().unwrap();
    assert_eq!(retrieved.command, "build");
    assert!(retrieved.success);

    // List all entries
    let list = storage.list(None, None).unwrap();
    assert_eq!(list.len(), 2);

    // Get last entry
    let last = storage.get_last().unwrap().unwrap();
    assert_eq!(last.command, "test");

    // Get last failed entry
    let last_failed = storage.get_last_failed().unwrap().unwrap();
    assert_eq!(last_failed.command, "test");
    assert!(!last_failed.success);

    // Search
    let results = storage.search("build", None).unwrap();
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].command, "build");

    // Statistics
    let stats = storage.get_stats().unwrap();
    assert_eq!(stats.total, 2);
    assert_eq!(stats.successful, 1);
    assert_eq!(stats.failed, 1);
    assert_eq!(stats.success_rate(), 50.0);

    // Clear
    let deleted = storage.clear().unwrap();
    assert_eq!(deleted, 2);

    let list_after_clear = storage.list(None, None).unwrap();
    assert!(list_after_clear.is_empty());
}

#[test]
fn test_history_recorder() {
    let temp_file = NamedTempFile::new().unwrap();
    let storage = HistoryStorage::with_path(temp_file.path()).unwrap();
    let mut recorder = HistoryRecorder::with_storage(storage);

    let mut env = HashMap::new();
    env.insert("PATH".to_string(), "/usr/bin".to_string());
    env.insert("API_KEY".to_string(), "secret".to_string());

    let id = recorder
        .record(
            "deploy",
            &["production".to_string()],
            &env,
            2500,
            0,
            true,
        )
        .unwrap();

    let entry = recorder.storage().get_by_id(id).unwrap().unwrap();
    assert_eq!(entry.command, "deploy");
    assert!(entry.success);
    assert_eq!(entry.duration_ms, Some(2500));

    // Verify sensitive data is filtered
    if let Some(env_json) = entry.environment {
        let stored_env: HashMap<String, String> = serde_json::from_str(&env_json).unwrap();
        assert!(stored_env.contains_key("PATH"));
        assert!(!stored_env.contains_key("API_KEY")); // Should be filtered
    }
}

#[test]
fn test_history_export_json() {
    let temp_file = NamedTempFile::new().unwrap();
    let mut storage = HistoryStorage::with_path(temp_file.path()).unwrap();

    storage.add(&create_test_entry("cmd1", true)).unwrap();
    storage.add(&create_test_entry("cmd2", false)).unwrap();

    let json = storage.export_json(None).unwrap();

    // Verify it's valid JSON
    let parsed: Vec<HistoryEntry> = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed.len(), 2);

    // Check that commands are present (order is DESC by start_time)
    let commands: Vec<&str> = parsed.iter().map(|e| e.command.as_str()).collect();
    assert!(commands.contains(&"cmd1"));
    assert!(commands.contains(&"cmd2"));
}

#[test]
fn test_history_export_csv() {
    let temp_file = NamedTempFile::new().unwrap();
    let mut storage = HistoryStorage::with_path(temp_file.path()).unwrap();

    storage.add(&create_test_entry("cmd1", true)).unwrap();
    storage.add(&create_test_entry("cmd2", false)).unwrap();

    let csv = storage.export_csv(None).unwrap();
    assert!(csv.starts_with("id,command,args,"));
    assert!(csv.contains("cmd1"));
    assert!(csv.contains("cmd2"));

    // Count lines (header + 2 entries)
    let line_count = csv.lines().count();
    assert_eq!(line_count, 3);
}

#[test]
fn test_history_max_entries() {
    let temp_file = NamedTempFile::new().unwrap();
    let mut storage = HistoryStorage::with_path(temp_file.path()).unwrap();
    storage.set_max_entries(5);

    // Add 10 entries
    for i in 0..10 {
        let entry = create_test_entry(&format!("cmd{}", i), true);
        storage.add(&entry).unwrap();
    }

    let list = storage.list(None, None).unwrap();
    assert_eq!(list.len(), 5);

    // Verify oldest entries were removed (keeps last 5: cmd5-cmd9)
    // list is DESC order, so first=newest, last=oldest
    assert_eq!(list.first().unwrap().command, "cmd9");
    assert_eq!(list.last().unwrap().command, "cmd5");
}

#[test]
fn test_history_pagination() {
    let temp_file = NamedTempFile::new().unwrap();
    let mut storage = HistoryStorage::with_path(temp_file.path()).unwrap();

    // Add 20 entries
    for i in 0..20 {
        storage.add(&create_test_entry(&format!("cmd{}", i), true)).unwrap();
    }

    // First page
    let page1 = storage.list(Some(10), Some(0)).unwrap();
    assert_eq!(page1.len(), 10);

    // Second page
    let page2 = storage.list(Some(10), Some(10)).unwrap();
    assert_eq!(page2.len(), 10);

    // Verify no overlap
    assert_ne!(page1[0].id, page2[0].id);
}

#[test]
fn test_sensitive_data_filtering() {
    let temp_file = NamedTempFile::new().unwrap();
    let storage = HistoryStorage::with_path(temp_file.path()).unwrap();
    let mut recorder = HistoryRecorder::with_storage(storage);

    let mut env = HashMap::new();
    env.insert("PATH".to_string(), "/usr/bin".to_string());
    env.insert("API_KEY".to_string(), "super_secret".to_string());
    env.insert("PASSWORD".to_string(), "hunter2".to_string());
    env.insert("HOME".to_string(), "/home/user".to_string());

    let id = recorder.record(
        "test",
        &[],
        &env,
        100,
        0,
        true,
    ).unwrap();

    let entry = recorder.storage().get_by_id(id).unwrap().unwrap();
    if let Some(env_json) = entry.environment {
        let stored_env: HashMap<String, String> = serde_json::from_str(&env_json).unwrap();
        // Safe keys should be present
        assert!(stored_env.contains_key("PATH"));
        assert!(stored_env.contains_key("HOME"));
        // Sensitive keys should be filtered out
        assert!(!stored_env.contains_key("API_KEY"));
        assert!(!stored_env.contains_key("PASSWORD"));
    }
}

// Helper function to create test entries
fn create_test_entry(command: &str, success: bool) -> HistoryEntry {
    use chrono::Utc;

    HistoryEntry {
        id: 0,
        command: command.to_string(),
        args: Some(r#"["arg1"]"#.to_string()),
        start_time: Utc::now().timestamp_millis(),
        duration_ms: Some(1000),
        exit_code: if success { Some(0) } else { Some(1) },
        success,
        working_dir: Some("/tmp".to_string()),
        environment: Some(r#"{"PATH": "/usr/bin"}"#.to_string()),
    }
}
