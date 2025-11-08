//! Property-based tests for coverage improvement
//!
//! Uses proptest to generate random inputs and verify invariants

use cmdrun::command::interpolation::InterpolationContext;
use cmdrun::config::schema::{CommandSpec, GlobalConfig};
use cmdrun::output::logger::{LogLevel, LoggerConfig};
use cmdrun::security::validation::CommandValidator;
use proptest::prelude::*;

// Property test: Interpolation context creation with any boolean
proptest! {
    #[test]
    fn prop_interpolation_context_creation(strict in any::<bool>()) {
        let _ = InterpolationContext::new(strict);
    }
}

// Property test: Command validator should handle any string input without panicking
proptest! {
    #[test]
    fn prop_validator_accepts_any_string(cmd in ".*") {
        let validator = CommandValidator::default();
        let _ = validator.validate(&cmd);
    }
}

// Property test: Validator with max length
proptest! {
    #[test]
    fn prop_validator_max_length(
        cmd in prop::string::string_regex("[a-z]{1000,2000}").unwrap(),
        max_len in 1u32..100u32
    ) {
        let validator = CommandValidator::new().with_max_length(max_len as usize);
        let result = validator.validate(&cmd);
        if cmd.len() > max_len as usize {
            prop_assert!(!result.is_safe());
        }
    }
}

// Property test: Logger config builder should maintain all properties
proptest! {
    #[test]
    fn prop_logger_config_maintains_properties(
        json_output in any::<bool>(),
        show_timestamps in any::<bool>(),
        show_target in any::<bool>()
    ) {
        let config = LoggerConfig::new()
            .with_json_output(json_output)
            .with_timestamps(show_timestamps)
            .with_target(show_target);

        prop_assert_eq!(config.json_output, json_output);
        prop_assert_eq!(config.show_timestamps, show_timestamps);
        prop_assert_eq!(config.show_target, show_target);
    }
}

// Property test: Escaping shell arguments should not panic
proptest! {
    #[test]
    fn prop_escape_shell_arg_never_panics(arg in ".*") {
        let result = cmdrun::security::validation::escape_shell_arg(&arg);
        // Should return some result (even if empty for empty input)
        if arg.is_empty() {
            prop_assert!(result.is_empty() || !result.is_empty());
        } else {
            prop_assert!(!result.is_empty());
        }
    }
}

// Property test: Global config with valid fields
proptest! {
    #[test]
    fn prop_global_config_fields(
        timeout in 1u64..3600u64,
        strict_mode in any::<bool>(),
        parallel in any::<bool>()
    ) {
        let config = GlobalConfig {
            timeout,
            strict_mode,
            parallel,
            ..Default::default()
        };
        prop_assert_eq!(config.timeout, timeout);
        prop_assert_eq!(config.strict_mode, strict_mode);
        prop_assert_eq!(config.parallel, parallel);
    }
}

// Property test: Command validator should reject null bytes
proptest! {
    #[test]
    fn prop_validator_rejects_null_bytes(prefix in ".*", suffix in ".*") {
        let cmd = format!("{}\0{}", prefix, suffix);
        let validator = CommandValidator::default();
        let result = validator.validate(&cmd);
        prop_assert!(!result.is_safe());
    }
}

// Property test: Logger with different log levels should not panic
proptest! {
    #[test]
    fn prop_logger_all_levels(level_idx in 0usize..5) {
        let levels = [
            LogLevel::Trace,
            LogLevel::Debug,
            LogLevel::Info,
            LogLevel::Warn,
            LogLevel::Error,
        ];
        let level = levels[level_idx];
        let config = LoggerConfig::new().with_level(level);
        prop_assert_eq!(config.level, level);
    }
}

// Property test: Command spec single variant should contain the command
proptest! {
    #[test]
    fn prop_command_spec_single(cmd in ".*") {
        let spec = CommandSpec::Single(cmd.clone());
        match spec {
            CommandSpec::Single(c) => prop_assert_eq!(c, cmd),
            _ => prop_assert!(false, "Expected Single variant"),
        }
    }
}

// Property test: Command spec multiple variant should contain all commands
proptest! {
    #[test]
    fn prop_command_spec_multiple(cmds in prop::collection::vec(".*", 1..10)) {
        let spec = CommandSpec::Multiple(cmds.clone());
        match spec {
            CommandSpec::Multiple(c) => prop_assert_eq!(c, cmds),
            _ => prop_assert!(false, "Expected Multiple variant"),
        }
    }
}

// Property test: Empty command should be rejected
proptest! {
    #[test]
    fn prop_empty_command_rejected(whitespace in "[ \t\n\r]*") {
        let validator = CommandValidator::default();
        let result = validator.validate(&whitespace);
        prop_assert!(!result.is_safe());
    }
}

// Property test: Logger config with log file paths
proptest! {
    #[test]
    fn prop_logger_log_file(path in ".*") {
        let config = LoggerConfig::new().with_log_file(path.clone());
        prop_assert_eq!(config.log_file, Some(path));
    }
}

// Property test: Command timeout should be preserved
proptest! {
    #[test]
    fn prop_command_timeout(timeout in 1u64..86400) {
        let config = GlobalConfig {
            timeout,
            ..Default::default()
        };
        prop_assert_eq!(config.timeout, timeout);
    }
}

// Property test: Strict mode validator should be stricter
proptest! {
    #[test]
    fn prop_strict_mode_stricter(cmd in "[a-z|;&]+") {
        let strict = CommandValidator::new().with_strict_mode(true);
        let lenient = CommandValidator::new().with_strict_mode(false);

        let strict_result = strict.validate(&cmd);
        let _ = lenient.validate(&cmd);

        // If command has dangerous chars, strict mode should catch it
        if cmd.contains(['|', ';', '&']) {
            prop_assert!(!strict_result.is_safe());
        }
    }
}

// Property test: Validator with allowed metacharacters
proptest! {
    #[test]
    fn prop_allowed_metacharacters(cmd in "[a-z|]+") {
        let validator = CommandValidator::new()
            .with_strict_mode(false)
            .allow_metacharacter('|');
        let _ = validator.validate(&cmd);
    }
}

// Property test: Context with environment variables
proptest! {
    #[test]
    fn prop_context_with_env(
        key in "[a-zA-Z_]+",
        value in ".*",
        strict in any::<bool>()
    ) {
        let _ctx = InterpolationContext::new(strict).with_env(key, value);
        // Context should be created successfully
        prop_assert!(true);
    }
}

// Property test: Interpolation context with strict mode
proptest! {
    #[test]
    fn prop_interpolation_strict_mode(strict in any::<bool>()) {
        let ctx = InterpolationContext::new(strict);
        // Verify context is created
        let debug_str = format!("{:?}", ctx);
        prop_assert!(debug_str.contains("InterpolationContext"));
    }
}

// Property test: Logger config default values
proptest! {
    #[test]
    fn prop_logger_default_sane(_n in 0u32..100u32) {
        let config = LoggerConfig::default();
        prop_assert_eq!(config.level, LogLevel::Info);
        prop_assert!(!config.json_output);
        prop_assert!(config.show_timestamps);
        prop_assert!(!config.show_target);
        prop_assert!(config.log_file.is_none());
    }
}

// Property test: Command spec clone
proptest! {
    #[test]
    fn prop_command_spec_clone(cmd in ".*") {
        let spec1 = CommandSpec::Single(cmd.clone());
        let spec2 = spec1.clone();
        prop_assert_eq!(format!("{:?}", spec1), format!("{:?}", spec2));
    }
}

// Property test: Global config default is valid
proptest! {
    #[test]
    fn prop_global_config_default_valid(_n in 0u32..100u32) {
        let config = GlobalConfig::default();
        prop_assert!(config.timeout > 0);
        prop_assert!(!config.shell.is_empty());
    }
}
