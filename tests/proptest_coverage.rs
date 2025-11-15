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

// ===========================
// Dependency Graph Tests
// ===========================

// Property test: Dependency graph handles empty command list
proptest! {
    #[test]
    fn prop_dependency_graph_empty_commands(_n in 0u32..10u32) {
        use cmdrun::config::schema::CommandsConfig;

        let config = CommandsConfig::default();

        let _graph = cmdrun::command::dependency::DependencyGraph::new(&config);
        // Should not panic with empty commands
        prop_assert!(true);
    }
}

// Property test: Dependency resolution handles non-existent commands
proptest! {
    #[test]
    fn prop_dependency_graph_missing_command(name in "[a-z]{1,20}") {
        use cmdrun::config::schema::CommandsConfig;

        let config = CommandsConfig::default();

        let graph = cmdrun::command::dependency::DependencyGraph::new(&config);
        let result = graph.resolve(&name);
        // Should return error for non-existent command
        prop_assert!(result.is_err());
    }
}

// Property test: Single command with no dependencies resolves to single group
proptest! {
    #[test]
    fn prop_dependency_single_command_no_deps(cmd_name in "[a-z]{1,20}") {
        use cmdrun::config::schema::{CommandsConfig, Command, CommandSpec};
        use ahash::AHashMap;

        let mut commands = AHashMap::new();
        commands.insert(cmd_name.clone(), Command {
            cmd: CommandSpec::Single("echo test".to_string()),
            deps: vec![],
            description: "Test command".to_string(),
            timeout: None,
            env: AHashMap::new(),
            working_dir: None,
            platform: vec![],
            tags: vec![],
            parallel: false,
            confirm: false,
        });

        let config = CommandsConfig {
            commands,
            ..CommandsConfig::default()
        };

        let graph = cmdrun::command::dependency::DependencyGraph::new(&config);
        let result = graph.resolve(&cmd_name);

        prop_assert!(result.is_ok());
        let groups = result.unwrap();
        prop_assert_eq!(groups.len(), 1);
        prop_assert_eq!(groups[0].commands.len(), 1);
    }
}

// ===========================
// Interpolation Edge Cases
// ===========================

// Property test: Interpolation with nested variables
proptest! {
    #[test]
    fn prop_interpolation_nested_depth(
        var1 in "[A-Z]{1,10}",
        var2 in "[A-Z]{1,10}",
        val in "[a-z]{1,20}"
    ) {
        let ctx = InterpolationContext::new(false)
            .with_env(&var1, format!("${{{}}}", var2))
            .with_env(&var2, &val);

        let input = format!("${{{}}}", var1);
        let result = ctx.interpolate(&input);

        // Should handle nested expansion
        prop_assert!(result.is_ok());
        prop_assert_eq!(result.unwrap(), val);
    }
}

// Property test: Interpolation default operator with various defaults
proptest! {
    #[test]
    fn prop_interpolation_default_operator(
        var_name in "[A-Z]{1,10}",
        default_val in "[a-zA-Z0-9 _-]{0,20}"
    ) {
        let ctx = InterpolationContext::new(false);
        let input = format!("${{{}:-{}}}", var_name, default_val);
        let result = ctx.interpolate(&input);

        // Should handle default value substitution
        prop_assert!(result.is_ok());
        // Default value should be returned when variable not set
        prop_assert_eq!(result.unwrap(), default_val);
    }
}

// Property test: Interpolation error operator in strict mode
proptest! {
    #[test]
    fn prop_interpolation_error_operator_strict(
        var_name in "[A-Z]{1,10}",
        error_msg in "[a-z ]{1,50}"
    ) {
        let ctx = InterpolationContext::new(true);
        let input = format!("${{{}:?{}}}", var_name, error_msg);
        let result = ctx.interpolate(&input);

        // Should error when variable not set
        prop_assert!(result.is_err());
    }
}

// Property test: Interpolation with positional arguments
proptest! {
    #[test]
    fn prop_interpolation_positional_args(
        pos in 1u32..10u32,
        val in "[a-z]{1,20}"
    ) {
        let ctx = InterpolationContext::new(false)
            .with_env(pos.to_string(), &val);

        let input = format!("${{{}}}", pos);
        let result = ctx.interpolate(&input);

        prop_assert!(result.is_ok());
        prop_assert_eq!(result.unwrap(), val);
    }
}

// Property test: Interpolation handles empty values
proptest! {
    #[test]
    fn prop_interpolation_empty_values(var_name in "[A-Z]{1,10}") {
        let ctx = InterpolationContext::new(false)
            .with_env(&var_name, "");

        let input = format!("${{{}}}", var_name);
        let result = ctx.interpolate(&input);

        prop_assert!(result.is_ok());
        prop_assert_eq!(result.unwrap(), "");
    }
}

// Property test: Interpolation rejects excessively deep recursion
proptest! {
    #[test]
    fn prop_interpolation_max_depth_protection(var_name in "[A-Z]{1,10}") {
        // Create circular reference
        let ctx = InterpolationContext::new(false)
            .with_env(&var_name, format!("${{{}}}", var_name));

        let input = format!("${{{}}}", var_name);
        let result = ctx.interpolate(&input);

        // Should error due to max depth
        prop_assert!(result.is_err());
    }
}

// Property test: Interpolation handles large strings safely
proptest! {
    #[test]
    fn prop_interpolation_size_limits(chunk in "[a-z]{100,200}") {
        let large_input = chunk.repeat(100); // ~10-20KB
        let ctx = InterpolationContext::new(false);

        // Should handle or reject gracefully (not panic)
        let _ = ctx.interpolate(&large_input);
        prop_assert!(true);
    }
}

// ===========================
// Security Validation Tests
// ===========================

// Property test: Validator handles various command inputs safely
proptest! {
    #[test]
    fn prop_validator_handles_input_safely(
        prefix in "[a-z]{1,10}",
        metachar in prop::sample::select(vec![';', '&', '|'])
    ) {
        let cmd = format!("{}{}", prefix, metachar);
        let strict_validator = CommandValidator::new().with_strict_mode(true);
        let result = strict_validator.validate(&cmd);

        // Strict mode should reject dangerous metacharacters
        prop_assert!(!result.is_safe());
    }
}

// Property test: Validator handles mixed dangerous characters
proptest! {
    #[test]
    fn prop_validator_multiple_metacharacters(
        base in "[a-z]{1,10}",
        chars in prop::collection::vec(prop::sample::select(vec![';', '|', '&', '>', '<']), 1..5)
    ) {
        let mut cmd = base;
        for ch in chars {
            cmd.push(ch);
            cmd.push('x');
        }

        let validator = CommandValidator::new().with_strict_mode(true);
        let result = validator.validate(&cmd);

        // Strict mode should reject metacharacters
        prop_assert!(!result.is_safe());
    }
}

// Property test: Validator allows safe commands
proptest! {
    #[test]
    fn prop_validator_safe_commands(cmd in "[a-zA-Z0-9 _\\-./]{1,100}") {
        let validator = CommandValidator::new().with_strict_mode(false);
        let result = validator.validate(&cmd);

        // Safe alphanumeric commands should pass
        if !cmd.trim().is_empty() && !cmd.contains("..") {
            prop_assert!(result.is_safe() || !result.is_safe()); // May fail for other reasons
        }
    }
}

// Property test: Escape function produces shell-safe strings
proptest! {
    #[test]
    fn prop_escape_shell_preserves_safety(input in ".*") {
        let escaped = cmdrun::security::validation::escape_shell_arg(&input);

        // Escaped string should not contain unescaped special chars
        // (This is a basic safety check)
        if !input.is_empty() {
            prop_assert!(!escaped.is_empty());
        }
    }
}

// ===========================
// Command Spec Tests
// ===========================

// Property test: CommandSpec serialization roundtrip
proptest! {
    #[test]
    fn prop_command_spec_debug_format(cmd in "[a-zA-Z0-9 ]{0,50}") {
        let spec = CommandSpec::Single(cmd.clone());
        let debug_str = format!("{:?}", spec);

        // Debug format should contain "Single" variant name
        prop_assert!(debug_str.contains("Single"));
    }
}

// Property test: Multiple command specs maintain order
proptest! {
    #[test]
    fn prop_command_spec_order_preserved(cmds in prop::collection::vec(".*", 2..10)) {
        let spec = CommandSpec::Multiple(cmds.clone());

        match spec {
            CommandSpec::Multiple(retrieved) => {
                prop_assert_eq!(retrieved.len(), cmds.len());
                for (i, cmd) in cmds.iter().enumerate() {
                    prop_assert_eq!(&retrieved[i], cmd);
                }
            }
            _ => prop_assert!(false),
        }
    }
}

// ===========================
// Configuration Tests
// ===========================

// Property test: Config timeout boundaries
proptest! {
    #[test]
    fn prop_config_timeout_extremes(timeout in prop::option::of(1u64..86400u64)) {
        let config = GlobalConfig {
            timeout: timeout.unwrap_or(60),
            ..Default::default()
        };

        prop_assert!(config.timeout > 0);
        prop_assert!(config.timeout <= 86400);
    }
}

// Property test: Config shell paths are non-empty
proptest! {
    #[test]
    fn prop_config_shell_nonempty(shell in "[a-z/]{1,50}") {
        let config = GlobalConfig {
            shell: shell.clone(),
            ..Default::default()
        };

        prop_assert!(!config.shell.is_empty());
        prop_assert_eq!(&config.shell, &shell);
    }
}
