//! Integration tests for logger module
//!
//! Tests logging configuration and functionality

use cmdrun::output::logger::{LogLevel, LoggerConfig};

/// Test default logger config
#[test]
fn test_default_logger_config() {
    let config = LoggerConfig::default();
    assert_eq!(config.level, LogLevel::Info);
    assert!(!config.json_output);
    assert!(config.show_timestamps);
    assert!(!config.show_target);
    assert!(config.log_file.is_none());
}

/// Test logger config builder pattern
#[test]
fn test_logger_config_builder() {
    let config = LoggerConfig::new()
        .with_level(LogLevel::Debug)
        .with_json_output(true)
        .with_timestamps(false)
        .with_target(true)
        .with_log_file("/tmp/test.log".to_string());

    assert_eq!(config.level, LogLevel::Debug);
    assert!(config.json_output);
    assert!(!config.show_timestamps);
    assert!(config.show_target);
    assert_eq!(config.log_file, Some("/tmp/test.log".to_string()));
}

/// Test all log levels
#[test]
fn test_all_log_levels() {
    let levels = vec![
        LogLevel::Trace,
        LogLevel::Debug,
        LogLevel::Info,
        LogLevel::Warn,
        LogLevel::Error,
    ];

    for level in levels {
        let config = LoggerConfig::new().with_level(level);
        assert_eq!(config.level, level);
    }
}

/// Test log level conversion to tracing::Level
#[test]
fn test_log_level_to_tracing_level() {
    use tracing::Level;

    assert_eq!(Level::from(LogLevel::Trace), Level::TRACE);
    assert_eq!(Level::from(LogLevel::Debug), Level::DEBUG);
    assert_eq!(Level::from(LogLevel::Info), Level::INFO);
    assert_eq!(Level::from(LogLevel::Warn), Level::WARN);
    assert_eq!(Level::from(LogLevel::Error), Level::ERROR);
}

/// Test config with various combinations
#[test]
fn test_config_combinations() {
    let configs = vec![
        LoggerConfig::new().with_level(LogLevel::Trace),
        LoggerConfig::new().with_json_output(true),
        LoggerConfig::new().with_timestamps(false),
        LoggerConfig::new().with_target(true),
        LoggerConfig::new()
            .with_level(LogLevel::Debug)
            .with_json_output(true),
        LoggerConfig::new()
            .with_level(LogLevel::Error)
            .with_timestamps(false)
            .with_target(true),
    ];

    // Just verify they can be created
    assert_eq!(configs.len(), 6);
}

/// Test logger config clone
#[test]
fn test_config_clone() {
    let config1 = LoggerConfig::new()
        .with_level(LogLevel::Debug)
        .with_json_output(true);

    let config2 = config1.clone();

    assert_eq!(config1.level, config2.level);
    assert_eq!(config1.json_output, config2.json_output);
}

/// Test logger config debug formatting
#[test]
fn test_config_debug() {
    let config = LoggerConfig::new().with_level(LogLevel::Debug);
    let debug_str = format!("{:?}", config);

    assert!(debug_str.contains("LoggerConfig"));
    assert!(debug_str.contains("Debug"));
}

/// Test log level equality
#[test]
fn test_log_level_equality() {
    assert_eq!(LogLevel::Info, LogLevel::Info);
    assert_ne!(LogLevel::Info, LogLevel::Debug);
    assert_ne!(LogLevel::Trace, LogLevel::Error);
}

/// Test log level copy
#[test]
fn test_log_level_copy() {
    let level1 = LogLevel::Info;
    let level2 = level1; // Copy
    assert_eq!(level1, level2);
}

/// Test builder pattern immutability
#[test]
fn test_builder_immutability() {
    let base_config = LoggerConfig::new();
    let config1 = base_config.clone().with_level(LogLevel::Debug);
    let config2 = base_config.clone().with_level(LogLevel::Error);

    assert_eq!(config1.level, LogLevel::Debug);
    assert_eq!(config2.level, LogLevel::Error);
}

/// Test JSON output flag
#[test]
fn test_json_output_flag() {
    let config_false = LoggerConfig::new().with_json_output(false);
    let config_true = LoggerConfig::new().with_json_output(true);

    assert!(!config_false.json_output);
    assert!(config_true.json_output);
}

/// Test timestamps flag
#[test]
fn test_timestamps_flag() {
    let config_false = LoggerConfig::new().with_timestamps(false);
    let config_true = LoggerConfig::new().with_timestamps(true);

    assert!(!config_false.show_timestamps);
    assert!(config_true.show_timestamps);
}

/// Test target flag
#[test]
fn test_target_flag() {
    let config_false = LoggerConfig::new().with_target(false);
    let config_true = LoggerConfig::new().with_target(true);

    assert!(!config_false.show_target);
    assert!(config_true.show_target);
}

/// Test log file configuration
#[test]
fn test_log_file_configuration() {
    let config_none = LoggerConfig::new();
    let config_some = LoggerConfig::new().with_log_file("/var/log/cmdrun.log".to_string());

    assert!(config_none.log_file.is_none());
    assert_eq!(
        config_some.log_file,
        Some("/var/log/cmdrun.log".to_string())
    );
}

/// Test multiple log file updates
#[test]
fn test_multiple_log_file_updates() {
    let config = LoggerConfig::new()
        .with_log_file("/tmp/log1.log".to_string())
        .with_log_file("/tmp/log2.log".to_string());

    assert_eq!(config.log_file, Some("/tmp/log2.log".to_string()));
}

/// Test chaining all builder methods
#[test]
fn test_full_builder_chain() {
    let config = LoggerConfig::new()
        .with_level(LogLevel::Trace)
        .with_json_output(true)
        .with_timestamps(false)
        .with_target(true)
        .with_log_file("/tmp/full.log".to_string());

    assert_eq!(config.level, LogLevel::Trace);
    assert!(config.json_output);
    assert!(!config.show_timestamps);
    assert!(config.show_target);
    assert_eq!(config.log_file, Some("/tmp/full.log".to_string()));
}

/// Test log level ordering
#[test]
fn test_log_level_ordering() {
    use std::mem::discriminant;

    let levels = [
        LogLevel::Error,
        LogLevel::Warn,
        LogLevel::Info,
        LogLevel::Debug,
        LogLevel::Trace,
    ];

    // Verify all levels are distinct
    for (i, level1) in levels.iter().enumerate() {
        for (j, level2) in levels.iter().enumerate() {
            if i == j {
                assert_eq!(discriminant(level1), discriminant(level2));
            } else {
                assert_ne!(discriminant(level1), discriminant(level2));
            }
        }
    }
}

/// Test config with empty log file path (edge case)
#[test]
fn test_empty_log_file_path() {
    let config = LoggerConfig::new().with_log_file("".to_string());
    assert_eq!(config.log_file, Some("".to_string()));
}

/// Test config with special characters in log file path
#[test]
fn test_special_chars_log_file_path() {
    let paths = vec![
        "/tmp/log with spaces.log",
        "/tmp/日本語.log",
        "/tmp/log-with-dashes.log",
        "/tmp/log_with_underscores.log",
    ];

    for path in paths {
        let config = LoggerConfig::new().with_log_file(path.to_string());
        assert_eq!(config.log_file, Some(path.to_string()));
    }
}

/// Test that logging functions can be called (compilation test)
#[test]
fn test_logging_functions_compile() {
    // These are compilation tests - they verify the functions exist and have correct signatures
    cmdrun::output::logger::log_command("test", "echo test");
    cmdrun::output::logger::log_command_success("test", 100);
    cmdrun::output::logger::log_command_failure("test", 1, "error");
    cmdrun::output::logger::log_dependency_resolution("test", &vec!["dep1".to_string()]);
    cmdrun::output::logger::log_config_loaded("/tmp/config.toml", 5);
}

/// Test log level debug formatting
#[test]
fn test_log_level_debug_format() {
    let levels = vec![
        (LogLevel::Trace, "Trace"),
        (LogLevel::Debug, "Debug"),
        (LogLevel::Info, "Info"),
        (LogLevel::Warn, "Warn"),
        (LogLevel::Error, "Error"),
    ];

    for (level, expected) in levels {
        let debug_str = format!("{:?}", level);
        assert!(debug_str.contains(expected));
    }
}
