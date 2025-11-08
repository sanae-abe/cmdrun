//! Integration tests for interactive TUI mode

use cmdrun::config::schema::{Command, CommandSpec, CommandsConfig, GlobalConfig};
use cmdrun::tui::app::App;
use cmdrun::tui::fuzzy::fuzzy_match_commands;

/// Test fuzzy matching functionality
#[test]
fn test_fuzzy_match_basic() {
    let commands = vec![
        "build".to_string(),
        "build-prod".to_string(),
        "test".to_string(),
        "dev".to_string(),
    ];

    // Exact match should be first
    let results = fuzzy_match_commands("build", &commands);
    assert_eq!(results[0], "build");

    // Partial match should work
    let results = fuzzy_match_commands("bld", &commands);
    assert!(results.len() >= 2);
    assert!(results.contains(&"build".to_string()));
    assert!(results.contains(&"build-prod".to_string()));
}

/// Test App initialization
#[test]
fn test_app_initialization() {
    let config = create_test_config();
    let app = App::new(config.clone(), None);

    assert_eq!(app.search_input, "");
    assert_eq!(app.selected_index, 0);
    assert_eq!(app.filtered_commands.len(), config.commands.len());
}

/// Test search input updates
#[test]
fn test_app_search_update() {
    let config = create_test_config();
    let mut app = App::new(config, None);

    app.update_search("build".to_string());
    assert_eq!(app.search_input, "build");
    assert!(app.filtered_commands.len() <= 3); // Should filter commands
}

/// Test navigation
#[test]
fn test_app_navigation() {
    let config = create_test_config();
    let mut app = App::new(config, None);

    assert_eq!(app.selected_index, 0);

    app.select_next();
    assert_eq!(app.selected_index, 1);

    app.select_next();
    assert_eq!(app.selected_index, 2);

    app.select_previous();
    assert_eq!(app.selected_index, 1);

    app.select_previous();
    assert_eq!(app.selected_index, 0);

    // Should not go below 0
    app.select_previous();
    assert_eq!(app.selected_index, 0);
}

/// Test navigation boundaries
#[test]
fn test_app_navigation_boundaries() {
    let config = create_test_config();
    let mut app = App::new(config, None);

    // Move to end
    let max_idx = app.filtered_commands.len() - 1;
    for _ in 0..max_idx + 5 {
        app.select_next();
    }

    // Should not exceed max
    assert_eq!(app.selected_index, max_idx);
}

/// Test character input
#[test]
fn test_app_char_input() {
    let config = create_test_config();
    let mut app = App::new(config, None);

    app.push_char('b');
    assert_eq!(app.search_input, "b");

    app.push_char('u');
    assert_eq!(app.search_input, "bu");

    app.pop_char();
    assert_eq!(app.search_input, "b");

    app.pop_char();
    assert_eq!(app.search_input, "");
}

/// Test clear search
#[test]
fn test_app_clear_search() {
    let config = create_test_config();
    let mut app = App::new(config.clone(), None);

    app.update_search("build".to_string());
    assert_eq!(app.search_input, "build");

    app.clear_search();
    assert_eq!(app.search_input, "");
    assert_eq!(app.filtered_commands.len(), config.commands.len());
}

/// Test selected command retrieval
#[test]
fn test_app_selected_command() {
    let config = create_test_config();
    let mut app = App::new(config, None);

    let selected = app.selected_command();
    assert!(selected.is_some());

    // Filter to specific command
    app.update_search("build".to_string());
    let selected = app.selected_command();
    assert!(selected.is_some());
}

/// Helper function to create a test configuration
fn create_test_config() -> CommandsConfig {
    use ahash::AHashMap;

    let mut commands = AHashMap::new();

    commands.insert(
        "build".to_string(),
        Command {
            description: "Build the project".to_string(),
            cmd: CommandSpec::Single("cargo build".to_string()),
            env: AHashMap::new(),
            working_dir: None,
            deps: vec![],
            platform: vec![],
            tags: vec![],
            timeout: None,
            parallel: false,
            confirm: false,
        },
    );

    commands.insert(
        "build-release".to_string(),
        Command {
            description: "Build release version".to_string(),
            cmd: CommandSpec::Single("cargo build --release".to_string()),
            env: AHashMap::new(),
            working_dir: None,
            deps: vec![],
            platform: vec![],
            tags: vec![],
            timeout: None,
            parallel: false,
            confirm: false,
        },
    );

    commands.insert(
        "test".to_string(),
        Command {
            description: "Run tests".to_string(),
            cmd: CommandSpec::Single("cargo test".to_string()),
            env: AHashMap::new(),
            working_dir: None,
            deps: vec![],
            platform: vec![],
            tags: vec![],
            timeout: None,
            parallel: false,
            confirm: false,
        },
    );

    commands.insert(
        "dev".to_string(),
        Command {
            description: "Start development server".to_string(),
            cmd: CommandSpec::Single("cargo run".to_string()),
            env: AHashMap::new(),
            working_dir: None,
            deps: vec![],
            platform: vec![],
            tags: vec![],
            timeout: None,
            parallel: false,
            confirm: false,
        },
    );

    CommandsConfig {
        config: GlobalConfig::default(),
        commands,
        aliases: AHashMap::new(),
        hooks: Default::default(),
        plugins: Default::default(),
    }
}
