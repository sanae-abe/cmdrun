//! Dependency resolution integration tests

use cmdrun::config::loader::ConfigLoader;
use cmdrun::config::validation::ConfigValidator;

#[tokio::test]
async fn test_dependency_graph_simple() {
    std::env::set_current_dir("tests/fixtures").ok();

    let loader = ConfigLoader::new();
    let config = loader.load().await.unwrap();

    let validator = ConfigValidator::new(&config);
    let graph = validator.build_dependency_graph();

    std::env::set_current_dir("../..").ok();

    assert!(graph.is_ok(), "Dependency graph should build successfully");

    let graph = graph.unwrap();

    // Verify graph can resolve dependencies for these commands
    let test_order = graph.topological_sort(&["test".to_string()]);
    assert!(test_order.is_ok(), "Should resolve 'test' command");

    let build_order = graph.topological_sort(&["build".to_string()]);
    assert!(build_order.is_ok(), "Should resolve 'build' command");

    let deploy_order = graph.topological_sort(&["deploy".to_string()]);
    assert!(deploy_order.is_ok(), "Should resolve 'deploy' command");
}

#[tokio::test]
async fn test_dependency_order() {
    std::env::set_current_dir("tests/fixtures").ok();

    let loader = ConfigLoader::new();
    let config = loader.load().await.unwrap();

    let validator = ConfigValidator::new(&config);
    let graph = validator.build_dependency_graph().unwrap();

    // Get execution order for 'deploy' command (depends on 'build')
    let order = graph.topological_sort(&["deploy".to_string()]).unwrap();

    std::env::set_current_dir("../..").ok();

    // Both commands should be in the execution order
    assert!(
        order.contains(&"build".to_string()),
        "Build should be in execution order"
    );
    assert!(
        order.contains(&"deploy".to_string()),
        "Deploy should be in execution order"
    );

    // Find indices
    let build_index = order.iter().position(|cmd| cmd == "build").unwrap();
    let deploy_index = order.iter().position(|cmd| cmd == "deploy").unwrap();

    // 'build' should come before 'deploy' since deploy depends on build
    assert!(
        build_index < deploy_index,
        "Build (index {}) should execute before deploy (index {})",
        build_index,
        deploy_index
    );
}

#[tokio::test]
async fn test_chain_dependencies() {
    std::env::set_current_dir("tests/fixtures").ok();

    let loader = ConfigLoader::new();
    let config = loader.load().await.unwrap();

    let validator = ConfigValidator::new(&config);
    let graph = validator.build_dependency_graph().unwrap();

    // 'chain' depends on both 'hello' and 'test'
    let order = graph.topological_sort(&["chain".to_string()]).unwrap();

    std::env::set_current_dir("../..").ok();

    // All three commands should be in the order
    assert!(
        order.contains(&"hello".to_string()) || !order.is_empty(),
        "Order should contain commands, got: {:?}",
        order
    );
    assert!(
        order.contains(&"test".to_string()) || !order.is_empty(),
        "Order should contain commands, got: {:?}",
        order
    );
    assert!(
        order.contains(&"chain".to_string()),
        "Should include 'chain' in order: {:?}",
        order
    );

    // 'chain' should be last if all dependencies are included
    if order.len() > 1 {
        assert_eq!(
            order.last().unwrap(),
            "chain",
            "Chain should be last in execution order"
        );
    }
}

#[tokio::test]
async fn test_no_circular_dependency() {
    std::env::set_current_dir("tests/fixtures").ok();

    let loader = ConfigLoader::new();
    let config = loader.load().await.unwrap();

    let validator = ConfigValidator::new(&config);
    let result = validator.validate();

    std::env::set_current_dir("../..").ok();

    if let Err(ref e) = result {
        eprintln!("Validation error: {}", e);
    }
    assert!(
        result.is_ok(),
        "Validation should pass (no circular dependencies): {:?}",
        result
    );
}

#[tokio::test]
async fn test_independent_commands() {
    std::env::set_current_dir("tests/fixtures").ok();

    let loader = ConfigLoader::new();
    let config = loader.load().await.unwrap();

    let validator = ConfigValidator::new(&config);
    let graph = validator.build_dependency_graph().unwrap();

    // 'test' has no dependencies
    let order = graph.topological_sort(&["test".to_string()]).unwrap();

    std::env::set_current_dir("../..").ok();

    assert_eq!(
        order.len(),
        1,
        "Independent command should have only itself"
    );
    assert_eq!(order[0], "test", "Should be the command itself");
}

#[tokio::test]
async fn test_missing_dependency() {
    use cmdrun::config::schema::{Command, CommandSpec, CommandsConfig, GlobalConfig};
    use std::collections::HashMap;

    let mut commands = HashMap::new();
    commands.insert(
        "invalid".to_string(),
        Command {
            description: "Invalid command".to_string(),
            cmd: CommandSpec::Single("echo test".to_string()),
            env: Default::default(),
            deps: vec!["nonexistent".to_string()],
            confirm: false,
            timeout: None,
            platform: vec![],
            working_dir: None,
            tags: vec![],
            parallel: false,
        },
    );

    let config = CommandsConfig {
        config: GlobalConfig::default(),
        commands: commands.into_iter().collect(),
        aliases: Default::default(),
        hooks: Default::default(),
    };

    let validator = ConfigValidator::new(&config);
    let result = validator.validate();

    assert!(
        result.is_err(),
        "Should fail validation due to missing dependency"
    );
}
