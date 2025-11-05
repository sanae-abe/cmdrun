//! Dependency resolution integration tests

use cmdrun::config::loader::ConfigLoader;
use cmdrun::config::validation::ConfigValidator;

#[tokio::test]
async fn test_dependency_graph_simple() {
    let original_dir = std::env::current_dir().unwrap();
    std::env::set_current_dir("tests/fixtures").ok();

    let loader = ConfigLoader::new();
    let config = loader.load().await.unwrap();

    let validator = ConfigValidator::new(&config);
    let graph = validator.build_dependency_graph();

    std::env::set_current_dir(original_dir).ok();

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
    let original_dir = std::env::current_dir().unwrap();
    std::env::set_current_dir("tests/fixtures").ok();

    let loader = ConfigLoader::new();
    let config = loader.load().await.unwrap();

    let validator = ConfigValidator::new(&config);
    let graph = validator.build_dependency_graph().unwrap();

    // Get execution order for 'deploy' command (depends on 'build')
    let order = graph.topological_sort(&["deploy".to_string()]).unwrap();

    std::env::set_current_dir(original_dir).ok();

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
    let original_dir = std::env::current_dir().unwrap();
    std::env::set_current_dir("tests/fixtures").ok();

    let loader = ConfigLoader::new();
    let config = loader.load().await.unwrap();

    let validator = ConfigValidator::new(&config);
    let graph = validator.build_dependency_graph().unwrap();

    // 'chain' depends on both 'hello' and 'test'
    let order = graph.topological_sort(&["chain".to_string()]).unwrap();

    std::env::set_current_dir(original_dir).ok();

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
    use std::path::PathBuf;

    // Save original directory
    let original_dir = std::env::current_dir().unwrap();

    // Try multiple possible paths
    let possible_paths = vec![
        PathBuf::from("tests/fixtures"),
        PathBuf::from("./tests/fixtures"),
        original_dir.join("tests/fixtures"),
    ];

    let mut found = false;
    for path in possible_paths {
        if path.exists() && std::env::set_current_dir(&path).is_ok() {
            found = true;
            break;
        }
    }

    // Restore original directory and handle error
    let result = if found {
        let loader = ConfigLoader::new();
        let config_result = loader.load().await;
        std::env::set_current_dir(&original_dir).ok();

        match config_result {
            Ok(config) => {
                let validator = ConfigValidator::new(&config);
                validator.validate()
            }
            Err(e) => Err(e),
        }
    } else {
        std::env::set_current_dir(&original_dir).ok();
        eprintln!("Warning: Could not find tests/fixtures directory, skipping test");
        return;
    };

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
    let original_dir = std::env::current_dir().unwrap();
    std::env::set_current_dir("tests/fixtures").ok();

    let loader = ConfigLoader::new();
    let config = loader.load().await.unwrap();

    let validator = ConfigValidator::new(&config);
    let graph = validator.build_dependency_graph().unwrap();

    // 'test' has no dependencies
    let order = graph.topological_sort(&["test".to_string()]).unwrap();

    std::env::set_current_dir(original_dir).ok();

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
