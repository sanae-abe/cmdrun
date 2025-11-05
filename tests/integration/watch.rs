//! Integration tests for file watching functionality

use cmdrun::watch::{
    CommandExecutor, FileDebouncer, PatternMatcher, WatchConfig, WatchPattern, WatchRunner,
};
use notify::EventKind;
use std::fs;
use std::path::{Path, PathBuf};
use std::thread::sleep;
use std::time::Duration;
use tempfile::TempDir;

// ============================================================================
// Configuration Tests
// ============================================================================

#[test]
fn test_watch_config_default() {
    let config = WatchConfig::default();
    assert_eq!(config.paths.len(), 1);
    assert_eq!(config.debounce_ms, 500);
    assert!(config.recursive);
    assert!(!config.ignore_gitignore);
}

#[test]
fn test_watch_config_builder() {
    let config = WatchConfig::new()
        .add_path("/tmp/test")
        .add_pattern("*.rs")
        .add_pattern("*.toml")
        .add_exclude("target/**")
        .add_exclude("*.tmp")
        .debounce(300);

    assert_eq!(config.paths.len(), 2); // default "." + added path
    assert_eq!(config.patterns.len(), 3); // default "**/*" + 2 added
    assert_eq!(config.exclude.len(), 2);
    assert_eq!(config.debounce_ms, 300);
}

#[test]
fn test_watch_config_debounce_duration() {
    let config = WatchConfig::new().debounce(1500);
    assert_eq!(config.debounce_duration(), Duration::from_millis(1500));
}

#[test]
fn test_watch_pattern_creation() {
    let pattern = WatchPattern {
        pattern: "**/*.rs".to_string(),
        case_insensitive: false,
    };

    assert_eq!(pattern.pattern, "**/*.rs");
    assert!(!pattern.case_insensitive);
}

// ============================================================================
// Pattern Matcher Tests
// ============================================================================

#[test]
fn test_pattern_matcher_basic_rust_files() {
    let temp_dir = TempDir::new().unwrap();
    let mut config = WatchConfig::new();
    config.paths = vec![temp_dir.path().to_path_buf()];
    config.patterns = vec![WatchPattern {
        pattern: "**/*.rs".to_string(),
        case_insensitive: false,
    }];
    config.ignore_gitignore = true;

    let matcher = PatternMatcher::from_config(&config, temp_dir.path()).unwrap();

    assert!(matcher.should_watch(Path::new("src/main.rs")));
    assert!(matcher.should_watch(Path::new("tests/integration/test.rs")));
    assert!(!matcher.should_watch(Path::new("README.md")));
    assert!(!matcher.should_watch(Path::new("Cargo.toml")));
}

#[test]
fn test_pattern_matcher_multiple_patterns() {
    let temp_dir = TempDir::new().unwrap();
    let mut config = WatchConfig::new();
    config.paths = vec![temp_dir.path().to_path_buf()];
    config.patterns = vec![
        WatchPattern {
            pattern: "**/*.rs".to_string(),
            case_insensitive: false,
        },
        WatchPattern {
            pattern: "**/*.toml".to_string(),
            case_insensitive: false,
        },
        WatchPattern {
            pattern: "**/*.md".to_string(),
            case_insensitive: false,
        },
    ];
    config.ignore_gitignore = true;

    let matcher = PatternMatcher::from_config(&config, temp_dir.path()).unwrap();

    assert!(matcher.should_watch(Path::new("src/lib.rs")));
    assert!(matcher.should_watch(Path::new("Cargo.toml")));
    assert!(matcher.should_watch(Path::new("README.md")));
    assert!(!matcher.should_watch(Path::new("image.png")));
}

#[test]
fn test_pattern_matcher_exclude_patterns() {
    let temp_dir = TempDir::new().unwrap();
    let mut config = WatchConfig::new();
    config.paths = vec![temp_dir.path().to_path_buf()];
    config.patterns = vec![WatchPattern {
        pattern: "**/*".to_string(),
        case_insensitive: false,
    }];
    config.exclude = vec![
        "target/**".to_string(),
        "**/*.tmp".to_string(),
        ".git/**".to_string(),
    ];
    config.ignore_gitignore = true;

    let matcher = PatternMatcher::from_config(&config, temp_dir.path()).unwrap();

    assert!(matcher.should_watch(Path::new("src/main.rs")));
    assert!(!matcher.should_watch(Path::new("target/debug/app")));
    assert!(!matcher.should_watch(Path::new("temp.tmp")));
    assert!(!matcher.should_watch(Path::new(".git/config")));
}

#[test]
fn test_pattern_matcher_exclude_priority() {
    let temp_dir = TempDir::new().unwrap();
    let mut config = WatchConfig::new();
    config.paths = vec![temp_dir.path().to_path_buf()];
    config.patterns = vec![WatchPattern {
        pattern: "**/*.rs".to_string(),
        case_insensitive: false,
    }];
    config.exclude = vec!["target/**".to_string()];
    config.ignore_gitignore = true;

    let matcher = PatternMatcher::from_config(&config, temp_dir.path()).unwrap();

    // Even though target/debug/main.rs matches the include pattern,
    // it should be excluded because exclude patterns take priority
    assert!(matcher.matches_include(Path::new("target/debug/main.rs")));
    assert!(!matcher.should_watch(Path::new("target/debug/main.rs")));
    assert!(matcher.is_excluded(Path::new("target/debug/main.rs")));
}

#[test]
fn test_pattern_matcher_with_gitignore() {
    let temp_dir = TempDir::new().unwrap();

    // Create a .gitignore file
    let gitignore_path = temp_dir.path().join(".gitignore");
    fs::write(&gitignore_path, "target/\n*.log\nnode_modules/\n").unwrap();

    let mut config = WatchConfig::new();
    config.paths = vec![temp_dir.path().to_path_buf()];
    config.patterns = vec![WatchPattern {
        pattern: "**/*".to_string(),
        case_insensitive: false,
    }];
    config.ignore_gitignore = false; // Enable gitignore

    let matcher = PatternMatcher::from_config(&config, temp_dir.path()).unwrap();

    // Test that gitignore file was loaded successfully by checking
    // that files matching gitignore patterns would be excluded
    // Note: We test with relative paths as gitignore works with paths relative to base

    // Files that should NOT be excluded
    assert!(matcher.should_watch(Path::new("src/main.rs")));
    assert!(matcher.should_watch(Path::new("README.md")));

    // Files matching *.log pattern should be excluded
    assert!(!matcher.should_watch(Path::new("debug.log")));
    assert!(!matcher.should_watch(Path::new("app.log")));
}

#[test]
fn test_pattern_matcher_invalid_pattern() {
    let temp_dir = TempDir::new().unwrap();
    let mut config = WatchConfig::new();
    config.paths = vec![temp_dir.path().to_path_buf()];
    config.patterns = vec![WatchPattern {
        pattern: "[invalid".to_string(), // Invalid glob pattern
        case_insensitive: false,
    }];

    let result = PatternMatcher::from_config(&config, temp_dir.path());
    assert!(result.is_err());
}

// ============================================================================
// Debouncer Tests
// ============================================================================

#[test]
fn test_debouncer_first_event_processed() {
    let mut debouncer = FileDebouncer::new(Duration::from_millis(100));
    let path = PathBuf::from("test.txt");

    assert!(debouncer.should_process(&path));
    assert_eq!(debouncer.tracked_paths_count(), 1);
}

#[test]
fn test_debouncer_rapid_events_blocked() {
    let mut debouncer = FileDebouncer::new(Duration::from_millis(200));
    let path = PathBuf::from("src/main.rs");

    // First event should pass
    assert!(debouncer.should_process(&path));

    // Immediate subsequent events should be blocked
    assert!(!debouncer.should_process(&path));
    assert!(!debouncer.should_process(&path));
    assert!(!debouncer.should_process(&path));
}

#[test]
fn test_debouncer_delayed_events_allowed() {
    let mut debouncer = FileDebouncer::new(Duration::from_millis(50));
    let path = PathBuf::from("test.txt");

    // First event
    assert!(debouncer.should_process(&path));

    // Wait for debounce period to expire
    sleep(Duration::from_millis(60));

    // Second event after delay should be processed
    assert!(debouncer.should_process(&path));
}

#[test]
fn test_debouncer_multiple_paths_independent() {
    let mut debouncer = FileDebouncer::new(Duration::from_millis(100));
    let path1 = PathBuf::from("src/lib.rs");
    let path2 = PathBuf::from("src/main.rs");
    let path3 = PathBuf::from("README.md");

    // All first events should be processed
    assert!(debouncer.should_process(&path1));
    assert!(debouncer.should_process(&path2));
    assert!(debouncer.should_process(&path3));

    assert_eq!(debouncer.tracked_paths_count(), 3);

    // Rapid events on each path should be debounced independently
    assert!(!debouncer.should_process(&path1));
    assert!(!debouncer.should_process(&path2));
    assert!(!debouncer.should_process(&path3));
}

#[test]
fn test_debouncer_clear_path() {
    let mut debouncer = FileDebouncer::new(Duration::from_millis(100));
    let path = PathBuf::from("test.txt");

    debouncer.should_process(&path);
    assert_eq!(debouncer.tracked_paths_count(), 1);

    debouncer.clear_path(&path);
    assert_eq!(debouncer.tracked_paths_count(), 0);

    // After clearing, next event should be processed immediately
    assert!(debouncer.should_process(&path));
}

#[test]
fn test_debouncer_clear_all() {
    let mut debouncer = FileDebouncer::new(Duration::from_millis(100));

    debouncer.should_process(&PathBuf::from("file1.txt"));
    debouncer.should_process(&PathBuf::from("file2.txt"));
    debouncer.should_process(&PathBuf::from("file3.txt"));
    assert_eq!(debouncer.tracked_paths_count(), 3);

    debouncer.clear_all();
    assert_eq!(debouncer.tracked_paths_count(), 0);
}

#[test]
fn test_debouncer_cleanup_old_entries() {
    let mut debouncer = FileDebouncer::new(Duration::from_millis(50));
    let old_path = PathBuf::from("old.txt");
    let recent_path = PathBuf::from("recent.txt");

    // Process old path
    debouncer.should_process(&old_path);

    // Wait to make it "old"
    sleep(Duration::from_millis(60));

    // Process recent path
    debouncer.should_process(&recent_path);

    assert_eq!(debouncer.tracked_paths_count(), 2);

    // Cleanup with retention period
    debouncer.cleanup(Duration::from_millis(50));

    // Old path should be removed, recent should remain
    assert_eq!(debouncer.tracked_paths_count(), 1);
}

#[test]
fn test_debouncer_realistic_scenario() {
    let mut debouncer = FileDebouncer::new(Duration::from_millis(100));
    let path = PathBuf::from("src/main.rs");

    // Simulate multiple rapid saves (common in editors)
    assert!(debouncer.should_process(&path)); // First save
    assert!(!debouncer.should_process(&path)); // Auto-save 1
    assert!(!debouncer.should_process(&path)); // Auto-save 2
    assert!(!debouncer.should_process(&path)); // Auto-save 3

    // Wait for debounce period
    sleep(Duration::from_millis(110));

    // User makes another change
    assert!(debouncer.should_process(&path)); // Should process
}

// ============================================================================
// Command Executor Tests
// ============================================================================

#[test]
fn test_executor_creation() {
    let temp_dir = TempDir::new().unwrap();
    let executor = CommandExecutor::new(temp_dir.path().to_path_buf());

    assert_eq!(executor.working_dir(), temp_dir.path());
    #[cfg(unix)]
    assert!(!executor.shell().is_empty());
    #[cfg(windows)]
    assert!(!executor.shell().is_empty());
}

#[test]
fn test_executor_builder_pattern() {
    let temp_dir = TempDir::new().unwrap();
    let executor = CommandExecutor::new(PathBuf::from("/tmp"))
        .with_working_dir(temp_dir.path().to_path_buf())
        .with_shell("/bin/bash".to_string());

    assert_eq!(executor.working_dir(), temp_dir.path());
    assert_eq!(executor.shell(), "/bin/bash");
}

#[tokio::test]
async fn test_executor_simple_command() {
    let temp_dir = TempDir::new().unwrap();
    let executor = CommandExecutor::new(temp_dir.path().to_path_buf());
    let changed_file = PathBuf::from("test.txt");

    #[cfg(unix)]
    let result = executor
        .execute("echo 'Hello, World!'", &changed_file)
        .await;
    #[cfg(windows)]
    let result = executor.execute("echo Hello, World!", &changed_file).await;

    assert!(result.is_ok(), "Simple echo command should succeed");
}

#[tokio::test]
async fn test_executor_with_working_directory() {
    let temp_dir = TempDir::new().unwrap();
    let test_file = temp_dir.path().join("test.txt");
    fs::write(&test_file, "test content").unwrap();

    let executor = CommandExecutor::new(temp_dir.path().to_path_buf());
    let changed_file = PathBuf::from("test.txt");

    #[cfg(unix)]
    let result = executor.execute("ls test.txt", &changed_file).await;
    #[cfg(windows)]
    let result = executor.execute("dir test.txt", &changed_file).await;

    assert!(
        result.is_ok(),
        "Command in working directory should succeed"
    );
}

#[tokio::test]
async fn test_executor_with_environment_variables() {
    let temp_dir = TempDir::new().unwrap();
    let executor = CommandExecutor::new(temp_dir.path().to_path_buf());
    let changed_file = PathBuf::from("test.txt");
    let env = vec![
        ("TEST_VAR".to_string(), "test_value".to_string()),
        ("CUSTOM_VAR".to_string(), "custom".to_string()),
    ];

    #[cfg(unix)]
    let result = executor
        .execute_with_env("echo $TEST_VAR", &changed_file, &env)
        .await;
    #[cfg(windows)]
    let result = executor
        .execute_with_env("echo %TEST_VAR%", &changed_file, &env)
        .await;

    assert!(result.is_ok(), "Command with environment should succeed");
}

#[tokio::test]
async fn test_executor_changed_file_env_var() {
    let temp_dir = TempDir::new().unwrap();
    let executor = CommandExecutor::new(temp_dir.path().to_path_buf());
    let changed_file = PathBuf::from("src/main.rs");

    // The CMDRUN_CHANGED_FILE environment variable should be set automatically
    #[cfg(unix)]
    let result = executor
        .execute("echo \"Changed: $CMDRUN_CHANGED_FILE\"", &changed_file)
        .await;
    #[cfg(windows)]
    let result = executor
        .execute("echo Changed: %CMDRUN_CHANGED_FILE%", &changed_file)
        .await;

    assert!(
        result.is_ok(),
        "Command should have access to CMDRUN_CHANGED_FILE"
    );
}

// ============================================================================
// Watch Runner Tests
// ============================================================================

#[tokio::test]
async fn test_watch_runner_creation() {
    let temp_dir = TempDir::new().unwrap();
    let config = WatchConfig::new()
        .add_path(temp_dir.path())
        .add_pattern("*.txt");

    let runner = WatchRunner::new(config, "echo test".to_string(), temp_dir.path());
    assert!(runner.is_ok(), "WatchRunner should be created successfully");
}

#[tokio::test]
async fn test_watch_runner_with_invalid_pattern() {
    let temp_dir = TempDir::new().unwrap();
    let config = WatchConfig::new()
        .add_path(temp_dir.path())
        .add_pattern("[invalid"); // Invalid glob pattern

    let runner = WatchRunner::new(config, "echo test".to_string(), temp_dir.path());
    assert!(
        runner.is_err(),
        "WatchRunner should fail with invalid pattern"
    );
}

#[tokio::test]
async fn test_watch_runner_matcher_access() {
    let temp_dir = TempDir::new().unwrap();
    let mut config = WatchConfig::new();
    config.paths = vec![temp_dir.path().to_path_buf()];
    config.patterns = vec![WatchPattern {
        pattern: "**/*.rs".to_string(),
        case_insensitive: false,
    }];

    let runner = WatchRunner::new(config, "cargo check".to_string(), temp_dir.path()).unwrap();

    let matcher = runner.matcher();
    assert!(
        matcher.should_watch(Path::new("src/main.rs")),
        "Matcher should match .rs files"
    );
    assert!(
        !matcher.should_watch(Path::new("README.md")),
        "Matcher should not match .md files"
    );
}

#[tokio::test]
async fn test_watch_runner_executor_access() {
    let temp_dir = TempDir::new().unwrap();
    let config = WatchConfig::new().add_path(temp_dir.path());

    let mut runner = WatchRunner::new(config, "echo test".to_string(), temp_dir.path()).unwrap();

    let executor = runner.executor_mut();
    assert_eq!(
        executor.working_dir(),
        temp_dir.path(),
        "Executor should have correct working directory"
    );
}

// ============================================================================
// Integration Scenarios
// ============================================================================

#[tokio::test]
async fn test_integration_rust_project_watch() {
    let temp_dir = TempDir::new().unwrap();

    // Create a realistic Rust project structure
    let src_dir = temp_dir.path().join("src");
    fs::create_dir(&src_dir).unwrap();
    fs::write(src_dir.join("main.rs"), "fn main() {}").unwrap();
    fs::write(src_dir.join("lib.rs"), "pub fn test() {}").unwrap();

    let target_dir = temp_dir.path().join("target");
    fs::create_dir(&target_dir).unwrap();

    let config = WatchConfig::new()
        .add_path(temp_dir.path())
        .add_pattern("**/*.rs")
        .add_exclude("target/**")
        .debounce(100);

    let matcher = PatternMatcher::from_config(&config, temp_dir.path()).unwrap();

    // Should watch source files
    assert!(matcher.should_watch(Path::new("src/main.rs")));
    assert!(matcher.should_watch(Path::new("src/lib.rs")));

    // Should not watch target directory
    assert!(!matcher.should_watch(Path::new("target/debug/main")));
}

#[tokio::test]
async fn test_integration_web_project_watch() {
    let temp_dir = TempDir::new().unwrap();

    let config = WatchConfig::new()
        .add_path(temp_dir.path())
        .add_pattern("**/*.js")
        .add_pattern("**/*.css")
        .add_pattern("**/*.html")
        .add_exclude("node_modules/**")
        .add_exclude("dist/**")
        .debounce(300);

    let matcher = PatternMatcher::from_config(&config, temp_dir.path()).unwrap();

    // Should watch web files
    assert!(matcher.should_watch(Path::new("src/index.js")));
    assert!(matcher.should_watch(Path::new("styles/main.css")));
    assert!(matcher.should_watch(Path::new("public/index.html")));

    // Should not watch excluded directories
    assert!(!matcher.should_watch(Path::new("node_modules/package/index.js")));
    assert!(!matcher.should_watch(Path::new("dist/bundle.js")));
}

#[tokio::test]
async fn test_integration_debounce_with_multiple_rapid_changes() {
    let mut debouncer = FileDebouncer::new(Duration::from_millis(100));
    let file1 = PathBuf::from("src/main.rs");
    let file2 = PathBuf::from("src/lib.rs");

    // Simulate rapid changes in two files
    assert!(debouncer.should_process(&file1)); // Process
    assert!(debouncer.should_process(&file2)); // Process (different file)
    assert!(!debouncer.should_process(&file1)); // Debounced
    assert!(!debouncer.should_process(&file2)); // Debounced

    // Wait for debounce period
    sleep(Duration::from_millis(110));

    // Both should be processed again
    assert!(debouncer.should_process(&file1));
    assert!(debouncer.should_process(&file2));
}

#[test]
fn test_integration_complex_pattern_matching() {
    let temp_dir = TempDir::new().unwrap();

    let mut config = WatchConfig::new();
    config.paths = vec![temp_dir.path().to_path_buf()];
    config.patterns = vec![
        WatchPattern {
            pattern: "src/**/*.rs".to_string(),
            case_insensitive: false,
        },
        WatchPattern {
            pattern: "tests/**/*.rs".to_string(),
            case_insensitive: false,
        },
        WatchPattern {
            pattern: "*.toml".to_string(),
            case_insensitive: false,
        },
    ];
    config.exclude = vec![
        "target/**".to_string(),
        "**/*.tmp".to_string(),
        "**/.*".to_string(), // Hidden files
    ];
    config.ignore_gitignore = true;

    let matcher = PatternMatcher::from_config(&config, temp_dir.path()).unwrap();

    // Should match
    assert!(matcher.should_watch(Path::new("src/main.rs")));
    assert!(matcher.should_watch(Path::new("src/module/lib.rs")));
    assert!(matcher.should_watch(Path::new("tests/integration/test.rs")));
    assert!(matcher.should_watch(Path::new("Cargo.toml")));

    // Should not match (excluded)
    assert!(!matcher.should_watch(Path::new("target/debug/main")));
    assert!(!matcher.should_watch(Path::new("src/temp.tmp")));
    assert!(!matcher.should_watch(Path::new(".hidden")));

    // Should not match (pattern doesn't match)
    assert!(!matcher.should_watch(Path::new("README.md")));
    assert!(!matcher.should_watch(Path::new("doc/guide.txt")));
}

#[test]
fn test_event_kind_coverage() {
    use notify::event::{CreateKind, ModifyKind, RemoveKind};

    // Test that we can represent different event kinds
    let create_event = EventKind::Create(CreateKind::File);
    let modify_event = EventKind::Modify(ModifyKind::Data(notify::event::DataChange::Any));
    let remove_event = EventKind::Remove(RemoveKind::File);

    // These should all be valid event kinds
    let _ = format!("{:?}", create_event);
    let _ = format!("{:?}", modify_event);
    let _ = format!("{:?}", remove_event);
}
