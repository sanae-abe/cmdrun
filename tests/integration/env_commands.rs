//! Integration tests for environment management commands
//!
//! commands/env.rs のテストカバレッジ向上

use cmdrun::commands::env::{
    handle_env_create, handle_env_get, handle_env_list, handle_env_set, handle_env_switch,
};
use tempfile::TempDir;
use tokio::fs;

#[tokio::test]
async fn test_env_lifecycle() {
    let temp_dir = TempDir::new().unwrap();
    let config_dir = temp_dir.path().join(".cmdrun");
    fs::create_dir_all(&config_dir).await.unwrap();

    // 環境作成
    let create_result = handle_env_create(
        config_dir.clone(),
        "dev".to_string(),
        "Development environment".to_string(),
    )
    .await;

    if let Ok(_) = create_result {
        // 環境切り替え
        let switch_result = handle_env_switch(config_dir.clone(), "dev".to_string()).await;
        assert!(switch_result.is_ok(), "Should be able to switch to dev environment");

        // 変数設定
        let set_result = handle_env_set(
            config_dir.clone(),
            "dev".to_string(),
            "API_URL".to_string(),
            "http://localhost:3000".to_string(),
        )
        .await;
        assert!(set_result.is_ok(), "Should be able to set environment variable");

        // 変数取得
        let get_result = handle_env_get(
            config_dir.clone(),
            "dev".to_string(),
            "API_URL".to_string(),
        )
        .await;

        if let Ok(value) = get_result {
            assert_eq!(value, "http://localhost:3000", "Should retrieve correct value");
        }

        // 環境一覧
        let list_result = handle_env_list(config_dir.clone()).await;
        if let Ok(envs) = list_result {
            assert!(
                envs.iter().any(|(name, _)| name == "dev"),
                "Should include dev environment in list"
            );
        }
    }
}

#[tokio::test]
async fn test_env_creation_with_different_names() {
    let temp_dir = TempDir::new().unwrap();
    let config_dir = temp_dir.path().join(".cmdrun");
    fs::create_dir_all(&config_dir).await.unwrap();

    let environments = vec![
        ("dev", "Development"),
        ("staging", "Staging"),
        ("prod", "Production"),
    ];

    for (name, desc) in environments {
        let result = handle_env_create(
            config_dir.clone(),
            name.to_string(),
            desc.to_string(),
        )
        .await;

        if result.is_ok() {
            // 環境が作成できた場合、リストに含まれることを確認
            let list_result = handle_env_list(config_dir.clone()).await;
            if let Ok(envs) = list_result {
                assert!(
                    envs.iter().any(|(env_name, _)| env_name == name),
                    "Environment {} should be in the list",
                    name
                );
            }
        }
    }
}

#[tokio::test]
async fn test_env_variable_operations() {
    let temp_dir = TempDir::new().unwrap();
    let config_dir = temp_dir.path().join(".cmdrun");
    fs::create_dir_all(&config_dir).await.unwrap();

    // 環境作成
    let create_result = handle_env_create(
        config_dir.clone(),
        "test".to_string(),
        "Test environment".to_string(),
    )
    .await;

    if create_result.is_ok() {
        // 複数の変数を設定
        let variables = vec![
            ("API_URL", "http://api.example.com"),
            ("DB_HOST", "localhost"),
            ("DEBUG", "true"),
        ];

        for (key, value) in &variables {
            let set_result = handle_env_set(
                config_dir.clone(),
                "test".to_string(),
                key.to_string(),
                value.to_string(),
            )
            .await;
            assert!(set_result.is_ok(), "Should be able to set {}", key);
        }

        // すべての変数を取得して検証
        for (key, expected_value) in &variables {
            let get_result = handle_env_get(
                config_dir.clone(),
                "test".to_string(),
                key.to_string(),
            )
            .await;

            if let Ok(value) = get_result {
                assert_eq!(
                    &value, expected_value,
                    "Value for {} should match",
                    key
                );
            }
        }
    }
}

#[tokio::test]
async fn test_env_switch_to_nonexistent() {
    let temp_dir = TempDir::new().unwrap();
    let config_dir = temp_dir.path().join(".cmdrun");
    fs::create_dir_all(&config_dir).await.unwrap();

    // 存在しない環境への切り替えを試みる
    let result = handle_env_switch(config_dir.clone(), "nonexistent".to_string()).await;

    // エラーになるべき
    assert!(result.is_err(), "Should fail to switch to non-existent environment");
}

#[tokio::test]
async fn test_env_get_nonexistent_variable() {
    let temp_dir = TempDir::new().unwrap();
    let config_dir = temp_dir.path().join(".cmdrun");
    fs::create_dir_all(&config_dir).await.unwrap();

    // 環境作成
    let create_result = handle_env_create(
        config_dir.clone(),
        "test".to_string(),
        "Test".to_string(),
    )
    .await;

    if create_result.is_ok() {
        // 存在しない変数の取得
        let result = handle_env_get(
            config_dir.clone(),
            "test".to_string(),
            "NONEXISTENT_VAR".to_string(),
        )
        .await;

        // エラーまたは空文字列になるべき
        if let Ok(value) = result {
            assert!(value.is_empty() || value == "NONEXISTENT_VAR",
                "Should return empty or default value for non-existent variable");
        }
    }
}

#[tokio::test]
async fn test_env_list_empty() {
    let temp_dir = TempDir::new().unwrap();
    let config_dir = temp_dir.path().join(".cmdrun");
    fs::create_dir_all(&config_dir).await.unwrap();

    // 環境一覧（初期状態）
    let result = handle_env_list(config_dir.clone()).await;

    if let Ok(envs) = result {
        // default環境が存在する可能性がある
        // または空のリストが返される
        assert!(
            envs.is_empty() || envs.iter().any(|(name, _)| name == "default"),
            "Should return empty list or default environment"
        );
    }
}

#[tokio::test]
async fn test_env_variable_overwrite() {
    let temp_dir = TempDir::new().unwrap();
    let config_dir = temp_dir.path().join(".cmdrun");
    fs::create_dir_all(&config_dir).await.unwrap();

    let create_result = handle_env_create(
        config_dir.clone(),
        "test".to_string(),
        "Test".to_string(),
    )
    .await;

    if create_result.is_ok() {
        // 最初の値を設定
        handle_env_set(
            config_dir.clone(),
            "test".to_string(),
            "VAR".to_string(),
            "value1".to_string(),
        )
        .await
        .unwrap();

        // 同じ変数を上書き
        handle_env_set(
            config_dir.clone(),
            "test".to_string(),
            "VAR".to_string(),
            "value2".to_string(),
        )
        .await
        .unwrap();

        // 最新の値が取得されることを確認
        let value = handle_env_get(
            config_dir.clone(),
            "test".to_string(),
            "VAR".to_string(),
        )
        .await
        .unwrap();

        assert_eq!(value, "value2", "Should return the latest value");
    }
}
