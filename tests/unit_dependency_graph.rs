//! Unit tests for dependency graph to improve coverage

use ahash::AHashMap;
use cmdrun::command::dependency::DependencyGraph;
use cmdrun::config::schema::{Command, CommandSpec, CommandsConfig, PluginsConfig};

fn create_test_config() -> CommandsConfig {
    let mut commands = AHashMap::new();
    commands.insert(
        "test".to_string(),
        Command {
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
        },
    );
    commands.insert(
        "build".to_string(),
        Command {
            description: "Build".to_string(),
            cmd: CommandSpec::Single("echo build".to_string()),
            deps: vec!["test".to_string()],
            env: AHashMap::new(),
            working_dir: None,
            timeout: None,
            parallel: false,
            tags: vec![],
            platform: vec![],
            confirm: false,
        },
    );

    CommandsConfig {
        config: Default::default(),
        commands,
        aliases: AHashMap::new(),
        hooks: Default::default(),
        plugins: PluginsConfig::default(),
    }
}

#[test]
fn test_dependency_graph_creation() {
    let config = create_test_config();
    let graph = DependencyGraph::new(&config);
    assert!(format!("{:?}", graph).contains("DependencyGraph"));
}

#[test]
fn test_dependency_graph_resolve() {
    let config = create_test_config();
    let graph = DependencyGraph::new(&config);
    let result = graph.resolve("build");
    assert!(result.is_ok());
}

#[test]
fn test_dependency_graph_check_cycles() {
    let config = create_test_config();
    let graph = DependencyGraph::new(&config);
    let result = graph.check_cycles();
    assert!(result.is_ok());
}

#[test]
fn test_circular_dependency_detection() {
    let mut commands = AHashMap::new();
    commands.insert(
        "a".to_string(),
        Command {
            description: "A".to_string(),
            cmd: CommandSpec::Single("echo a".to_string()),
            deps: vec!["b".to_string()],
            env: AHashMap::new(),
            working_dir: None,
            timeout: None,
            parallel: false,
            tags: vec![],
            platform: vec![],
            confirm: false,
        },
    );
    commands.insert(
        "b".to_string(),
        Command {
            description: "B".to_string(),
            cmd: CommandSpec::Single("echo b".to_string()),
            deps: vec!["a".to_string()],
            env: AHashMap::new(),
            working_dir: None,
            timeout: None,
            parallel: false,
            tags: vec![],
            platform: vec![],
            confirm: false,
        },
    );

    let config = CommandsConfig {
        config: Default::default(),
        commands,
        aliases: AHashMap::new(),
        hooks: Default::default(),
        plugins: PluginsConfig::default(),
    };

    let graph = DependencyGraph::new(&config);
    let result = graph.check_cycles();
    assert!(result.is_err());
}

#[test]
fn test_missing_dependency() {
    let mut commands = AHashMap::new();
    commands.insert(
        "cmd".to_string(),
        Command {
            description: "Command".to_string(),
            cmd: CommandSpec::Single("echo cmd".to_string()),
            deps: vec!["missing".to_string()],
            env: AHashMap::new(),
            working_dir: None,
            timeout: None,
            parallel: false,
            tags: vec![],
            platform: vec![],
            confirm: false,
        },
    );

    let config = CommandsConfig {
        config: Default::default(),
        commands,
        aliases: AHashMap::new(),
        hooks: Default::default(),
        plugins: PluginsConfig::default(),
    };

    let graph = DependencyGraph::new(&config);
    let result = graph.resolve("cmd");
    assert!(result.is_err());
}
