//! Basic integration tests for cmdrun

use cmdrun::command::executor::{CommandExecutor, ExecutionContext};
use cmdrun::config::loader::ConfigLoader;
use cmdrun::config::schema::{Command, CommandSpec};

#[tokio::test]
async fn test_simple_echo() {
    let ctx = ExecutionContext::default();
    let executor = CommandExecutor::new(ctx);

    let cmd = Command {
        description: "Test echo".to_string(),
        cmd: CommandSpec::Single("echo hello".to_string()),
        env: Default::default(),
        deps: vec![],
        confirm: false,
        timeout: None,
        platform: vec![],
        working_dir: None,
        tags: vec![],
        parallel: false,
    };

    let result = executor.execute(&cmd).await.unwrap();
    assert!(result.success);
    assert_eq!(result.exit_code, 0);
}

#[tokio::test]
async fn test_multiple_commands() {
    let ctx = ExecutionContext::default();
    let executor = CommandExecutor::new(ctx);

    let cmd = Command {
        description: "Multiple commands".to_string(),
        cmd: CommandSpec::Multiple(vec!["echo first".to_string(), "echo second".to_string()]),
        env: Default::default(),
        deps: vec![],
        confirm: false,
        timeout: None,
        platform: vec![],
        working_dir: None,
        tags: vec![],
        parallel: false,
    };

    let result = executor.execute(&cmd).await.unwrap();
    assert!(result.success);
    assert_eq!(result.exit_code, 0);
}

#[tokio::test]
async fn test_command_with_env() {
    let ctx = ExecutionContext::default();
    let executor = CommandExecutor::new(ctx.clone());

    let cmd = Command {
        description: "Test with environment variable".to_string(),
        cmd: CommandSpec::Single("echo ${TEST_VAR}".to_string()),
        env: {
            let mut env = ahash::AHashMap::new();
            env.insert("TEST_VAR".to_string(), "test_value".to_string());
            env
        },
        deps: vec![],
        confirm: false,
        timeout: None,
        platform: vec![],
        working_dir: None,
        tags: vec![],
        parallel: false,
    };

    let result = executor.execute(&cmd).await.unwrap();
    assert!(result.success);
    assert!(result.stdout.contains("test_value"));
}

#[tokio::test]
async fn test_config_loader() {
    let loader = ConfigLoader::new();

    // Set the test fixture directory
    std::env::set_current_dir("tests/fixtures").ok();

    let config = loader.load().await;

    // Reset directory
    std::env::set_current_dir("../..").ok();

    assert!(config.is_ok(), "Config should load successfully");

    let config = config.unwrap();
    assert!(!config.commands.is_empty(), "Should have commands");
    assert!(
        config.commands.contains_key("test"),
        "Should have 'test' command"
    );
}

#[tokio::test]
async fn test_command_exit_code() {
    let ctx = ExecutionContext::default();
    let executor = CommandExecutor::new(ctx);

    // This command should fail
    let cmd = Command {
        description: "Failing command".to_string(),
        cmd: CommandSpec::Single("exit 42".to_string()),
        env: Default::default(),
        deps: vec![],
        confirm: false,
        timeout: None,
        platform: vec![],
        working_dir: None,
        tags: vec![],
        parallel: false,
    };

    let result = executor.execute(&cmd).await;
    assert!(
        result.is_err(),
        "Command should fail with non-zero exit code"
    );

    // Optionally check the error message contains the exit code
    if let Err(e) = result {
        let error_msg = e.to_string();
        assert!(
            error_msg.contains("42"),
            "Error should mention exit code 42"
        );
    }
}

#[tokio::test]
async fn test_timeout() {
    let ctx = ExecutionContext {
        timeout: Some(1), // 1 second timeout
        ..Default::default()
    };

    let executor = CommandExecutor::new(ctx);

    // This command should timeout
    let cmd = Command {
        description: "Long running command".to_string(),
        cmd: CommandSpec::Single("sleep 5".to_string()),
        env: Default::default(),
        deps: vec![],
        confirm: false,
        timeout: Some(1),
        platform: vec![],
        working_dir: None,
        tags: vec![],
        parallel: false,
    };

    let result = executor.execute(&cmd).await;
    assert!(result.is_err(), "Should timeout and return error");
}
