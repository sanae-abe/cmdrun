//! Edge case and error handling tests

use ahash::AHashMap;
use cmdrun::config::schema::{Command, CommandSpec, CommandsConfig, GlobalConfig};
use cmdrun::security::validation::{CommandValidator, ValidationResult};

#[cfg(test)]
mod edge_case_tests {
    use super::*;

    /// Test security validation edge cases
    mod security_edge_cases {
        use super::*;

        #[test]
        fn test_unicode_characters() {
            let validator = CommandValidator::new();

            // Unicode should be allowed in basic commands
            let result = validator.validate("echo こんにちは");
            assert!(result.is_safe(), "Unicode should be allowed");

            // Emoji
            let result = validator.validate("echo 🎉");
            assert!(result.is_safe(), "Emoji should be allowed");

            // Right-to-left characters
            let result = validator.validate("echo مرحبا");
            assert!(result.is_safe(), "RTL text should be allowed");
        }

        #[test]
        fn test_boundary_length_commands() {
            let max_len = CommandValidator::DEFAULT_MAX_LENGTH;

            // Just under limit
            let validator = CommandValidator::new().with_max_length(max_len);
            let cmd = "a".repeat(max_len - 1);
            let result = validator.validate(&cmd);
            assert!(result.is_safe(), "Command under limit should be safe");

            // Exactly at limit
            let cmd = "a".repeat(max_len);
            let result = validator.validate(&cmd);
            assert!(result.is_safe(), "Command at limit should be safe");

            // Over limit
            let cmd = "a".repeat(max_len + 1);
            let result = validator.validate(&cmd);
            assert!(!result.is_safe(), "Command over limit should fail");
        }

        #[test]
        fn test_multiple_spaces_and_tabs() {
            let validator = CommandValidator::new();

            let result = validator.validate("echo    hello    world");
            assert!(result.is_safe(), "Multiple spaces should be allowed");

            // Tabs are dangerous metacharacters
            let result = validator.validate("echo\thello");
            assert!(!result.is_safe(), "Tabs should be rejected");
        }

        #[test]
        fn test_path_traversal_attempts() {
            let validator = CommandValidator::new();

            // Path traversal is safe in non-strict mode if no dangerous patterns
            let result = validator.validate("cat ../../../etc/passwd");
            assert!(
                result.is_safe(),
                "Path traversal without dangerous patterns should be safe"
            );

            // With redirect to /etc should fail
            let result = validator.validate("cat file > /etc/passwd");
            assert!(!result.is_safe(), "Redirect to /etc should fail");
        }

        #[test]
        fn test_environment_variable_expansion() {
            let validator = CommandValidator::new();

            // Without allowance, ${} should fail
            let result = validator.validate("echo ${PATH}");
            assert!(
                !result.is_safe(),
                "Variable expansion should fail without permission"
            );

            // With allowance, should pass
            let validator = CommandValidator::new()
                .allow_variable_expansion()
                .with_strict_mode(false);
            let result = validator.validate("echo ${PATH}");
            assert!(
                result.is_safe(),
                "Variable expansion should be allowed when configured"
            );
        }

        #[test]
        fn test_mixed_metacharacters() {
            let validator = CommandValidator::new().with_strict_mode(true);

            // Multiple dangerous metacharacters
            let result = validator.validate("cat file | grep test > output.txt");
            assert!(
                !result.is_safe(),
                "Multiple metacharacters should fail in strict mode"
            );

            // Non-strict with pipe allowed
            let validator = CommandValidator::new().with_strict_mode(false).allow_pipe();
            let result = validator.validate("cat file | grep test");
            assert!(result.is_safe(), "Pipe should be allowed when configured");
        }

        #[test]
        fn test_command_injection_variations() {
            let validator = CommandValidator::new();

            let dangerous_commands = vec![
                "echo test; rm -rf /",
                "echo test && rm -rf /",
                "echo test || rm -rf /",
                "echo test | sh",
                "echo test `whoami`",
                "echo test $(whoami)",
                "echo test; exec sh",
            ];

            for cmd in dangerous_commands {
                let result = validator.validate(cmd);
                assert!(
                    !result.is_safe(),
                    "Dangerous command should be rejected: {}",
                    cmd
                );
            }
        }

        #[test]
        fn test_null_byte_variations() {
            let validator = CommandValidator::new();

            // Null byte at start
            let result = validator.validate("\0echo hello");
            assert!(!result.is_safe(), "Null byte at start should fail");

            // Null byte in middle
            let result = validator.validate("echo\0hello");
            assert!(!result.is_safe(), "Null byte in middle should fail");

            // Null byte at end
            let result = validator.validate("echo hello\0");
            assert!(!result.is_safe(), "Null byte at end should fail");
        }

        #[test]
        fn test_whitespace_only_variations() {
            let validator = CommandValidator::new();

            assert!(!validator.validate("").is_safe());
            assert!(!validator.validate(" ").is_safe());
            assert!(!validator.validate("  ").is_safe());
            assert!(!validator.validate("\t").is_safe());
            assert!(!validator.validate("\n").is_safe());
            assert!(!validator.validate(" \t\n ").is_safe());
        }

        #[test]
        fn test_custom_forbidden_words() {
            let validator = CommandValidator::new()
                .add_forbidden_word("secret_command")
                .add_forbidden_word("forbidden_action");

            let result = validator.validate("run secret_command");
            assert!(
                !result.is_safe(),
                "Custom forbidden word should be rejected"
            );

            let result = validator.validate("do forbidden_action");
            assert!(
                !result.is_safe(),
                "Custom forbidden word should be rejected"
            );

            let result = validator.validate("normal command");
            assert!(result.is_safe(), "Normal command should be allowed");
        }
    }

    /// Test configuration edge cases
    mod config_edge_cases {
        use super::*;

        #[test]
        fn test_empty_config() {
            let config = CommandsConfig {
                config: GlobalConfig::default(),
                commands: AHashMap::new(),
                aliases: AHashMap::new(),
                hooks: Default::default(),
            };

            assert_eq!(config.commands.len(), 0);
            assert_eq!(config.aliases.len(), 0);
        }

        #[test]
        fn test_command_with_empty_deps() {
            let mut commands = AHashMap::new();
            commands.insert(
                "test".to_string(),
                Command {
                    description: "Test".to_string(),
                    cmd: CommandSpec::Single("echo test".to_string()),
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

            let config = CommandsConfig {
                config: GlobalConfig::default(),
                commands,
                aliases: AHashMap::new(),
                hooks: Default::default(),
            };

            let cmd = config.commands.get("test").unwrap();
            assert!(cmd.deps.is_empty());
        }

        #[test]
        fn test_command_with_multiple_deps() {
            let mut commands = AHashMap::new();
            commands.insert(
                "deploy".to_string(),
                Command {
                    description: "Deploy".to_string(),
                    cmd: CommandSpec::Single("deploy.sh".to_string()),
                    env: AHashMap::new(),
                    working_dir: None,
                    deps: vec![
                        "build".to_string(),
                        "test".to_string(),
                        "lint".to_string(),
                        "security_scan".to_string(),
                    ],
                    platform: vec![],
                    tags: vec![],
                    timeout: None,
                    parallel: false,
                    confirm: false,
                },
            );

            let config = CommandsConfig {
                config: GlobalConfig::default(),
                commands,
                aliases: AHashMap::new(),
                hooks: Default::default(),
            };

            let cmd = config.commands.get("deploy").unwrap();
            assert_eq!(cmd.deps.len(), 4);
        }

        #[test]
        fn test_command_with_long_description() {
            let long_desc = "A".repeat(1000);
            let mut commands = AHashMap::new();
            commands.insert(
                "test".to_string(),
                Command {
                    description: long_desc.clone(),
                    cmd: CommandSpec::Single("echo test".to_string()),
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

            let config = CommandsConfig {
                config: GlobalConfig::default(),
                commands,
                aliases: AHashMap::new(),
                hooks: Default::default(),
            };

            let cmd = config.commands.get("test").unwrap();
            assert_eq!(cmd.description.len(), 1000);
        }

        #[test]
        fn test_command_with_special_characters_in_name() {
            // Valid command names (alphanumeric, dash, underscore)
            let valid_names = vec!["test-cmd", "test_cmd", "testCmd", "test123", "123test"];

            for name in valid_names {
                let mut commands = AHashMap::new();
                commands.insert(
                    name.to_string(),
                    Command {
                        description: "Test".to_string(),
                        cmd: CommandSpec::Single("echo test".to_string()),
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

                let config = CommandsConfig {
                    config: GlobalConfig::default(),
                    commands,
                    aliases: AHashMap::new(),
                    hooks: Default::default(),
                };

                assert!(config.commands.contains_key(name));
            }
        }
    }

    /// Test boundary conditions
    mod boundary_tests {
        use super::*;

        #[test]
        fn test_zero_length_input() {
            let validator = CommandValidator::new();
            let result = validator.validate("");
            assert!(!result.is_safe(), "Empty command should fail");
        }

        #[test]
        fn test_single_character_commands() {
            let validator = CommandValidator::new();

            // Single letter commands
            for ch in 'a'..='z' {
                let cmd = ch.to_string();
                let result = validator.validate(&cmd);
                assert!(result.is_safe(), "Single letter '{}' should be safe", ch);
            }
        }

        #[test]
        fn test_maximum_dependency_depth() {
            let mut commands = AHashMap::new();

            // Create a deep dependency chain
            for i in 0..100 {
                let name = format!("cmd{}", i);
                let deps = if i > 0 {
                    vec![format!("cmd{}", i - 1)]
                } else {
                    vec![]
                };

                commands.insert(
                    name.clone(),
                    Command {
                        description: format!("Command {}", i),
                        cmd: CommandSpec::Single(format!("echo {}", i)),
                        env: AHashMap::new(),
                        working_dir: None,
                        deps,
                        platform: vec![],
                        tags: vec![],
                        timeout: None,
                        parallel: false,
                        confirm: false,
                    },
                );
            }

            let config = CommandsConfig {
                config: GlobalConfig::default(),
                commands,
                aliases: AHashMap::new(),
                hooks: Default::default(),
            };

            // Should handle deep dependency chains
            assert_eq!(config.commands.len(), 100);
        }

        #[test]
        fn test_many_commands() {
            let mut commands = AHashMap::new();

            // Create many commands
            for i in 0..1000 {
                commands.insert(
                    format!("cmd{}", i),
                    Command {
                        description: format!("Command {}", i),
                        cmd: CommandSpec::Single(format!("echo {}", i)),
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
            }

            let config = CommandsConfig {
                config: GlobalConfig::default(),
                commands,
                aliases: AHashMap::new(),
                hooks: Default::default(),
            };

            assert_eq!(config.commands.len(), 1000);
        }
    }
}
