//! Unit tests for command executor to improve coverage

use ahash::AHashMap;
use cmdrun::command::executor::{CommandExecutor, ExecutionContext};
use cmdrun::config::schema::{Command, CommandSpec};
use std::path::PathBuf;

#[tokio::test]
async fn test_executor_creation() {
    let ctx = ExecutionContext {
        working_dir: PathBuf::from("."),
        env: AHashMap::new(),
        shell: "bash".to_string(),
        timeout: Some(30),
        strict: true,
        echo: true,
        color: true,
        language: cmdrun::config::Language::default(),
    };

    let _executor = CommandExecutor::new(ctx);
    // Executor created successfully
}

#[tokio::test]
async fn test_execute_simple_command() {
    let ctx = ExecutionContext {
        working_dir: PathBuf::from("."),
        env: AHashMap::new(),
        shell: "bash".to_string(),
        timeout: Some(30),
        strict: false,
        echo: false,
        color: false,
        language: cmdrun::config::Language::default(),
    };

    let executor = CommandExecutor::new(ctx);
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
        confirm: false,
    };

    let result = executor.execute(&cmd).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_execute_parallel() {
    let ctx = ExecutionContext {
        working_dir: PathBuf::from("."),
        env: AHashMap::new(),
        shell: "bash".to_string(),
        timeout: Some(30),
        strict: false,
        echo: false,
        color: false,
        language: cmdrun::config::Language::default(),
    };

    let executor = CommandExecutor::new(ctx);
    let cmd1 = Command {
        description: "Test 1".to_string(),
        cmd: CommandSpec::Single("echo test1".to_string()),
        deps: vec![],
        env: AHashMap::new(),
        working_dir: None,
        timeout: None,
        parallel: false,
        tags: vec![],
        platform: vec![],
        confirm: false,
    };
    let cmd2 = Command {
        description: "Test 2".to_string(),
        cmd: CommandSpec::Single("echo test2".to_string()),
        deps: vec![],
        env: AHashMap::new(),
        working_dir: None,
        timeout: None,
        parallel: false,
        tags: vec![],
        platform: vec![],
        confirm: false,
    };

    let commands = vec![&cmd1, &cmd2];
    let result = executor.execute_parallel(&commands).await;
    assert!(result.is_ok());
}
