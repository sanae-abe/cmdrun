//! コマンドインジェクション対策と入力検証
//!
//! シェルメタ文字の検証、危険なパターンの検出

use regex::Regex;
use std::collections::HashSet;
use thiserror::Error;

/// 検証エラー
#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum ValidationError {
    #[error("Command contains dangerous shell metacharacters: {0}")]
    DangerousMetacharacters(String),

    #[error("Command contains potentially dangerous pattern: {0}")]
    DangerousPattern(String),

    #[error("Command exceeds maximum length: {actual} > {max}")]
    ExceedsMaxLength { actual: usize, max: usize },

    #[error("Command contains null bytes")]
    ContainsNullBytes,

    #[error("Command is empty or whitespace only")]
    EmptyCommand,

    #[error("Command contains forbidden word: {0}")]
    ForbiddenWord(String),
}

/// 検証結果
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValidationResult {
    /// 安全
    Safe,
    /// 警告付きで許可
    Warning(String),
    /// 危険で拒否
    Denied(ValidationError),
}

impl ValidationResult {
    /// 安全かチェック
    pub fn is_safe(&self) -> bool {
        matches!(self, ValidationResult::Safe | ValidationResult::Warning(_))
    }

    /// エラーを取得
    pub fn error(&self) -> Option<&ValidationError> {
        match self {
            ValidationResult::Denied(err) => Some(err),
            _ => None,
        }
    }
}

/// コマンドバリデーター
#[derive(Debug, Clone)]
pub struct CommandValidator {
    /// 最大コマンド長
    max_length: usize,
    /// 厳格モード
    strict: bool,
    /// 許可されたシェルメタ文字
    allowed_metacharacters: HashSet<char>,
    /// 禁止ワードリスト
    forbidden_words: HashSet<String>,
    /// 変数展開を許可
    allow_var_expansion: bool,
}

impl CommandValidator {
    /// デフォルトの最大コマンド長
    pub const DEFAULT_MAX_LENGTH: usize = 4096;

    /// 危険なシェルメタ文字（厳格モード）
    const DANGEROUS_METACHARACTERS: &'static [char] = &[
        ';', '&', '|', '>', '<', '`', '$', '(', ')', '{', '}', '[', ']', '\\', '"', '\'', '\n',
        '\r', '\t',
    ];

    /// 基本的に許可されるメタ文字（非厳格モード）
    const ALLOWED_BASIC_METACHARACTERS: &'static [char] =
        &['-', '_', '.', '/', ':', '=', ',', ' ', '\'', '\\', '"'];

    /// 危険なパターン（正規表現）
    const DANGEROUS_PATTERNS: &'static [&'static str] = &[
        r"\$\(.*\)",              // コマンド置換: $(...)
        r"`.*`",                  // コマンド置換: `...`
        r"\$\{.*\}",              // 変数展開: ${...}（一部許可の場合もある）
        r"[;&|]",                 // コマンド連結
        r">>\s*\/dev\/",          // デバイスファイル書き込み
        r"[<>]\s*\/etc\/",        // システムファイル操作
        r"\|\s*sh",               // シェルへのパイプ
        r"\|\s*bash",             // bashへのパイプ
        r"eval\s+",               // evalコマンド
        r"exec\s+",               // execコマンド
        r"^sh\s+-c\s+",           // sh -c 実行
        r"\s+sh\s+-c\s+",         // sh -c 実行（スペース後）
        r"chmod\s+[0-7]{3,4}",    // chmod権限変更
        r"chown\s+",              // chown所有者変更
        r"sudo\s+",               // sudo実行
        r"su\s+",                 // suユーザー切り替え
        r"mkfs\.\w+",             // ディスクフォーマット: mkfs.ext4等
        r"mkfs\s+/dev/",          // ディスクフォーマット: mkfs /dev/sda等
        r"^format\s+[a-zA-Z]:",   // Windowsディスクフォーマット: format c:等
        r"\s+format\s+[a-zA-Z]:", // Windowsディスクフォーマット（スペース後）
    ];

    /// デフォルトの禁止ワード
    const DEFAULT_FORBIDDEN_WORDS: &'static [&'static str] = &[
        "rm -rf /",
        "dd if=",
        "mkfs.",       // mkfs.ext4等のディスクフォーマットコマンド
        "mkfs ",       // mkfs /dev/sdaのようなコマンド
        ":(){:|:&};:", // フォークボム
    ];

    /// 新しいバリデーターを作成
    pub fn new() -> Self {
        Self {
            max_length: Self::DEFAULT_MAX_LENGTH,
            strict: true,
            allowed_metacharacters: Self::ALLOWED_BASIC_METACHARACTERS.iter().copied().collect(),
            forbidden_words: Self::DEFAULT_FORBIDDEN_WORDS
                .iter()
                .map(|s| s.to_string())
                .collect(),
            allow_var_expansion: false,
        }
    }

    /// 最大長を設定
    pub fn with_max_length(mut self, max_length: usize) -> Self {
        self.max_length = max_length;
        self
    }

    /// 厳格モードを設定
    pub fn with_strict_mode(mut self, strict: bool) -> Self {
        self.strict = strict;
        self
    }

    /// 許可されたメタ文字を追加
    pub fn allow_metacharacter(mut self, ch: char) -> Self {
        self.allowed_metacharacters.insert(ch);
        self
    }

    /// 禁止ワードを追加
    pub fn add_forbidden_word(mut self, word: impl Into<String>) -> Self {
        self.forbidden_words.insert(word.into());
        self
    }

    /// コマンドを検証
    pub fn validate(&self, command: &str) -> ValidationResult {
        // 空チェック
        if command.trim().is_empty() {
            return ValidationResult::Denied(ValidationError::EmptyCommand);
        }

        // ヌルバイトチェック
        if command.contains('\0') {
            return ValidationResult::Denied(ValidationError::ContainsNullBytes);
        }

        // 長さチェック
        if command.len() > self.max_length {
            return ValidationResult::Denied(ValidationError::ExceedsMaxLength {
                actual: command.len(),
                max: self.max_length,
            });
        }

        // 禁止ワードチェック
        for word in &self.forbidden_words {
            if command.contains(word.as_str()) {
                return ValidationResult::Denied(ValidationError::ForbiddenWord(word.clone()));
            }
        }

        // メタ文字チェック
        if let Some(result) = self.check_metacharacters(command) {
            return result;
        }

        // 危険パターンチェック（厳格モード）
        if self.strict {
            if let Some(result) = self.check_dangerous_patterns(command) {
                return result;
            }
        }

        ValidationResult::Safe
    }

    /// メタ文字をチェック
    fn check_metacharacters(&self, command: &str) -> Option<ValidationResult> {
        let mut dangerous_chars = Vec::new();

        for ch in command.chars() {
            if Self::DANGEROUS_METACHARACTERS.contains(&ch)
                && !self.allowed_metacharacters.contains(&ch)
            {
                dangerous_chars.push(ch);
            }
        }

        if !dangerous_chars.is_empty() {
            let chars_str = dangerous_chars
                .iter()
                .map(|c| format!("'{}'", c))
                .collect::<Vec<_>>()
                .join(", ");

            return Some(ValidationResult::Denied(
                ValidationError::DangerousMetacharacters(chars_str),
            ));
        }

        None
    }

    /// 危険なパターンをチェック
    fn check_dangerous_patterns(&self, command: &str) -> Option<ValidationResult> {
        for pattern_str in Self::DANGEROUS_PATTERNS {
            // 変数展開が許可されている場合、${...}パターンはスキップ
            if self.allow_var_expansion && *pattern_str == r"\$\{.*\}" {
                continue;
            }

            if let Ok(pattern) = Regex::new(pattern_str) {
                if pattern.is_match(command) {
                    return Some(ValidationResult::Denied(ValidationError::DangerousPattern(
                        pattern_str.to_string(),
                    )));
                }
            }
        }

        None
    }

    /// 変数展開を許可（${VAR} 形式）
    pub fn allow_variable_expansion(mut self) -> Self {
        // $, {, } を許可
        self.allowed_metacharacters.insert('$');
        self.allowed_metacharacters.insert('{');
        self.allowed_metacharacters.insert('}');
        self.allow_var_expansion = true;
        self
    }

    /// パイプを許可
    pub fn allow_pipe(mut self) -> Self {
        self.allowed_metacharacters.insert('|');
        self
    }

    /// リダイレクトを許可
    pub fn allow_redirect(mut self) -> Self {
        self.allowed_metacharacters.insert('>');
        self.allowed_metacharacters.insert('<');
        self
    }
}

impl Default for CommandValidator {
    fn default() -> Self {
        Self::new()
    }
}

/// 安全なコマンド引数エスケープ
pub fn escape_shell_arg(arg: &str) -> String {
    // シェル引数を安全にエスケープ
    if arg
        .chars()
        .all(|c| c.is_alphanumeric() || c == '-' || c == '_' || c == '.' || c == '/')
    {
        // 安全な文字のみの場合はそのまま
        arg.to_string()
    } else {
        // シングルクォートで囲み、内部のシングルクォートをエスケープ
        format!("'{}'", arg.replace('\'', r"'\''"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_safe_command() {
        let validator = CommandValidator::new();
        assert!(matches!(
            validator.validate("echo hello"),
            ValidationResult::Safe
        ));
        assert!(matches!(
            validator.validate("ls -la /tmp"),
            ValidationResult::Safe
        ));
    }

    #[test]
    fn test_dangerous_metacharacters() {
        let validator = CommandValidator::new();

        // セミコロンによるコマンド連結（禁止ワードを含まないパターン）
        let result = validator.validate("echo hello; ls");
        assert!(matches!(
            result,
            ValidationResult::Denied(ValidationError::DangerousMetacharacters(_))
        ));

        // パイプ
        let result = validator.validate("cat file | sh");
        assert!(matches!(
            result,
            ValidationResult::Denied(ValidationError::DangerousMetacharacters(_))
        ));
    }

    #[test]
    fn test_command_substitution() {
        let validator = CommandValidator::new();

        let result = validator.validate("echo $(whoami)");
        // コマンド置換は厳格モードで拒否される（メタ文字またはパターン）
        assert!(!result.is_safe(), "Command substitution should be rejected");

        let result = validator.validate("echo `whoami`");
        // バッククォートは厳格モードで拒否される（メタ文字またはパターン）
        assert!(
            !result.is_safe(),
            "Backtick substitution should be rejected"
        );
    }

    #[test]
    fn test_null_bytes() {
        let validator = CommandValidator::new();
        let result = validator.validate("echo hello\0world");
        assert!(matches!(
            result,
            ValidationResult::Denied(ValidationError::ContainsNullBytes)
        ));
    }

    #[test]
    fn test_empty_command() {
        let validator = CommandValidator::new();
        let result = validator.validate("   ");
        assert!(matches!(
            result,
            ValidationResult::Denied(ValidationError::EmptyCommand)
        ));
    }

    #[test]
    fn test_max_length() {
        let validator = CommandValidator::new().with_max_length(10);
        let result = validator.validate("echo hello world");
        assert!(matches!(
            result,
            ValidationResult::Denied(ValidationError::ExceedsMaxLength { .. })
        ));
    }

    #[test]
    fn test_forbidden_words() {
        let validator = CommandValidator::new();
        let result = validator.validate("rm -rf /");
        assert!(matches!(
            result,
            ValidationResult::Denied(ValidationError::ForbiddenWord(_))
        ));
    }

    #[test]
    fn test_variable_expansion_allowed() {
        let validator = CommandValidator::new()
            .allow_variable_expansion()
            .with_strict_mode(false);

        let result = validator.validate("echo ${HOME}");
        assert!(result.is_safe());
    }

    #[test]
    fn test_pipe_allowed() {
        let validator = CommandValidator::new().allow_pipe().with_strict_mode(false);

        let result = validator.validate("ls -la | grep test");
        assert!(result.is_safe());
    }

    #[test]
    fn test_redirect_allowed() {
        let validator = CommandValidator::new()
            .allow_redirect()
            .with_strict_mode(false);

        let result = validator.validate("echo hello > output.txt");
        assert!(result.is_safe());
    }

    #[test]
    fn test_escape_shell_arg() {
        assert_eq!(escape_shell_arg("hello"), "hello");
        assert_eq!(escape_shell_arg("hello world"), "'hello world'");
        assert_eq!(escape_shell_arg("hello'world"), "'hello'\\''world'");
        assert_eq!(escape_shell_arg("/path/to/file"), "/path/to/file");
    }

    #[test]
    fn test_dangerous_system_commands() {
        let validator = CommandValidator::new();

        let result = validator.validate("sudo rm -rf /");
        assert!(!result.is_safe());

        let result = validator.validate("chmod 777 /etc/passwd");
        assert!(!result.is_safe());

        let result = validator.validate("eval 'malicious code'");
        assert!(!result.is_safe());
    }

    #[test]
    fn test_fork_bomb() {
        let validator = CommandValidator::new();
        let result = validator.validate(":(){:|:&};:");
        assert!(matches!(
            result,
            ValidationResult::Denied(ValidationError::ForbiddenWord(_))
        ));
    }

    #[test]
    fn test_validation_result_is_safe() {
        assert!(ValidationResult::Safe.is_safe());
        assert!(ValidationResult::Warning("test".to_string()).is_safe());
        assert!(!ValidationResult::Denied(ValidationError::EmptyCommand).is_safe());
    }

    #[test]
    fn test_validation_result_error() {
        assert!(ValidationResult::Safe.error().is_none());
        assert!(ValidationResult::Warning("test".to_string())
            .error()
            .is_none());

        let error = ValidationResult::Denied(ValidationError::EmptyCommand);
        assert!(error.error().is_some());
        assert!(matches!(
            error.error().unwrap(),
            ValidationError::EmptyCommand
        ));
    }

    #[test]
    fn test_multiple_metacharacter_types() {
        let validator = CommandValidator::new();

        // Mix of different metacharacters
        let result = validator.validate("cat file; echo done");
        assert!(!result.is_safe());

        let result = validator.validate("ls | grep test | wc -l");
        assert!(!result.is_safe());

        let result = validator.validate("echo test > file.txt < input.txt");
        assert!(!result.is_safe());
    }

    #[test]
    fn test_validator_with_max_length_variations() {
        // Test at boundary
        let validator = CommandValidator::new().with_max_length(20);

        let result = validator.validate("echo hello world"); // 16 chars
        assert!(result.is_safe());

        let result = validator.validate("echo hello world!"); // 17 chars
        assert!(result.is_safe());

        let result = validator.validate("echo hello world!!"); // 18 chars
        assert!(result.is_safe());

        let result = validator.validate("echo hello world!!!"); // 19 chars
        assert!(result.is_safe());

        let result = validator.validate("echo hello world!!!!"); // 20 chars
        assert!(result.is_safe());

        let result = validator.validate("echo hello world!!!!!"); // 21 chars
        assert!(!result.is_safe());
    }

    #[test]
    fn test_allow_metacharacter_fluent_api() {
        let validator = CommandValidator::new()
            .allow_metacharacter('|')
            .allow_metacharacter('>')
            .with_strict_mode(false);

        let result = validator.validate("echo test | grep test");
        assert!(result.is_safe());

        let result = validator.validate("echo test > output.txt");
        assert!(result.is_safe());
    }

    #[test]
    fn test_custom_forbidden_words_multiple() {
        let validator = CommandValidator::new()
            .add_forbidden_word("secret")
            .add_forbidden_word("private")
            .add_forbidden_word("confidential");

        assert!(!validator.validate("cat secret.txt").is_safe());
        assert!(!validator.validate("rm private.log").is_safe());
        assert!(!validator.validate("echo confidential").is_safe());
        assert!(validator.validate("echo public").is_safe());
    }

    #[test]
    fn test_escape_shell_arg_edge_cases() {
        // Empty string - returns as-is (empty string is safe)
        let result = escape_shell_arg("");
        assert!(result.is_empty() || result == "''");

        // Only special characters
        assert_eq!(escape_shell_arg("!!!"), "'!!!'");

        // Mixed alphanumeric and special
        assert_eq!(escape_shell_arg("test-file.txt"), "test-file.txt");
        assert_eq!(escape_shell_arg("test file.txt"), "'test file.txt'");

        // Multiple single quotes
        assert_eq!(escape_shell_arg("it's ain't"), "'it'\\''s ain'\\''t'");

        // Unicode characters - not all alphanumeric, so escaped
        let result = escape_shell_arg("こんにちは");
        // Unicode may not be treated as alphanumeric by the function
        assert!(result == "こんにちは" || result == "'こんにちは'");
    }

    #[test]
    fn test_dangerous_patterns_with_strict_mode() {
        let strict = CommandValidator::new().with_strict_mode(true);
        let _non_strict = CommandValidator::new().with_strict_mode(false);

        // Test eval - always dangerous
        assert!(!strict.validate("eval echo test").is_safe());
        // Non-strict mode doesn't check dangerous patterns, only metacharacters
        // So eval might pass in non-strict if no forbidden metacharacters

        // Test exec - always dangerous in strict mode
        assert!(!strict.validate("exec sh").is_safe());

        // Test chmod - always dangerous in strict mode
        assert!(!strict.validate("chmod 755 file").is_safe());
    }

    #[test]
    fn test_variable_expansion_with_allowed_flag() {
        let validator = CommandValidator::new().allow_variable_expansion();

        // $, {, } are allowed
        assert!(validator.validate("echo ${VAR}").is_safe());
        assert!(validator
            .validate("export PATH=${PATH}:/new/path")
            .is_safe());
    }

    #[test]
    fn test_newline_and_carriage_return() {
        let validator = CommandValidator::new();

        // Newline
        assert!(!validator.validate("echo test\nrm -rf /").is_safe());

        // Carriage return
        assert!(!validator.validate("echo test\rrm -rf /").is_safe());

        // Both
        assert!(!validator.validate("echo test\r\nrm -rf /").is_safe());
    }

    #[test]
    fn test_default_validator() {
        let validator = CommandValidator::default();

        assert!(validator.validate("echo hello").is_safe());
        assert!(!validator.validate("echo hello; rm -rf /").is_safe());
    }
}
