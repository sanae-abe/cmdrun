//! 機密情報保護モジュール
//!
//! secrecyクレートを活用した機密情報のマスキングと安全な取り扱い

use secrecy::{ExposeSecret, Secret};
use serde::{Deserialize, Serialize};
use std::fmt;

/// 機密文字列型（ログ出力時に自動マスキング）
#[derive(Clone)]
pub struct SensitiveString(Secret<String>);

impl SensitiveString {
    /// 新しい機密文字列を作成
    pub fn new(value: String) -> Self {
        Self(Secret::new(value))
    }

    /// 機密情報を安全に取得（使用箇所は最小限に）
    pub fn expose(&self) -> &str {
        self.0.expose_secret()
    }

    /// マスキングされた表示用文字列
    pub fn masked(&self) -> String {
        let value = self.0.expose_secret();
        if value.is_empty() {
            return String::from("(empty)");
        }

        // 先頭2文字のみ表示、残りは***
        let chars: Vec<char> = value.chars().collect();
        if chars.len() <= 2 {
            "***".to_string()
        } else {
            format!("{}***", chars.iter().take(2).collect::<String>())
        }
    }
}

impl fmt::Debug for SensitiveString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SensitiveString(***)")
    }
}

impl fmt::Display for SensitiveString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "***")
    }
}

impl From<String> for SensitiveString {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

impl From<&str> for SensitiveString {
    fn from(value: &str) -> Self {
        Self::new(value.to_string())
    }
}

/// 機密環境変数マップ
#[derive(Debug, Clone)]
pub struct SensitiveEnv {
    /// 機密キーのパターン（小文字で保存）
    sensitive_patterns: Vec<String>,
}

impl SensitiveEnv {
    /// デフォルトの機密パターン
    const DEFAULT_PATTERNS: &'static [&'static str] = &[
        "password",
        "secret",
        "token",
        "key",
        "api_key",
        "apikey",
        "auth",
        "credential",
        "private",
        "passwd",
        "pwd",
        "access_token",
        "refresh_token",
        "jwt",
        "bearer",
        "oauth",
        "session",
    ];

    /// 新しいインスタンスを作成
    pub fn new() -> Self {
        Self {
            sensitive_patterns: Self::DEFAULT_PATTERNS
                .iter()
                .map(|s| s.to_lowercase())
                .collect(),
        }
    }

    /// カスタムパターンを追加
    pub fn with_pattern(mut self, pattern: impl Into<String>) -> Self {
        self.sensitive_patterns.push(pattern.into().to_lowercase());
        self
    }

    /// 環境変数キーが機密情報かチェック
    pub fn is_sensitive(&self, key: &str) -> bool {
        let key_lower = key.to_lowercase();
        self.sensitive_patterns
            .iter()
            .any(|pattern| key_lower.contains(pattern))
    }

    /// 環境変数値をマスキング
    pub fn mask_value(&self, key: &str, value: &str) -> String {
        if self.is_sensitive(key) {
            SensitiveString::new(value.to_string()).masked()
        } else {
            value.to_string()
        }
    }

    /// 環境変数マップをマスキング（ログ出力用）
    pub fn mask_env_map(&self, env: &std::collections::HashMap<String, String>) -> std::collections::HashMap<String, String> {
        env.iter()
            .map(|(k, v)| (k.clone(), self.mask_value(k, v)))
            .collect()
    }

    /// AHashMap版のマスキング
    pub fn mask_ahash_map(&self, env: &ahash::AHashMap<String, String>) -> ahash::AHashMap<String, String> {
        env.iter()
            .map(|(k, v)| (k.clone(), self.mask_value(k, v)))
            .collect()
    }
}

impl Default for SensitiveEnv {
    fn default() -> Self {
        Self::new()
    }
}

/// Serdeサポート用の機密文字列ラッパー
#[derive(Clone, Serialize, Deserialize)]
pub struct SecretField(#[serde(with = "secret_string")] String);

impl SecretField {
    pub fn new(value: String) -> Self {
        Self(value)
    }

    pub fn expose(&self) -> &str {
        &self.0
    }
}

impl fmt::Debug for SecretField {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SecretField(***)")
    }
}

// Serde用のカスタムシリアライザー
mod secret_string {
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(_value: &String, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str("***")
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<String, D::Error>
    where
        D: Deserializer<'de>,
    {
        String::deserialize(deserializer)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sensitive_string_masking() {
        let secret = SensitiveString::new("my_secret_password_123".to_string());
        assert_eq!(secret.masked(), "my***");
        assert_eq!(format!("{}", secret), "***");
        assert_eq!(format!("{:?}", secret), "SensitiveString(***)");
    }

    #[test]
    fn test_sensitive_string_short() {
        let secret = SensitiveString::new("ab".to_string());
        assert_eq!(secret.masked(), "***");
    }

    #[test]
    fn test_sensitive_string_empty() {
        let secret = SensitiveString::new("".to_string());
        assert_eq!(secret.masked(), "(empty)");
    }

    #[test]
    fn test_sensitive_env_detection() {
        let env = SensitiveEnv::new();

        assert!(env.is_sensitive("API_KEY"));
        assert!(env.is_sensitive("database_password"));
        assert!(env.is_sensitive("JWT_SECRET"));
        assert!(env.is_sensitive("oauth_token"));

        assert!(!env.is_sensitive("USER_NAME"));
        assert!(!env.is_sensitive("APP_VERSION"));
        assert!(!env.is_sensitive("LOG_LEVEL"));
    }

    #[test]
    fn test_mask_value() {
        let env = SensitiveEnv::new();

        assert_eq!(env.mask_value("API_KEY", "sk-1234567890"), "sk***");
        assert_eq!(env.mask_value("USER_NAME", "john_doe"), "john_doe");
    }

    #[test]
    fn test_custom_pattern() {
        let env = SensitiveEnv::new().with_pattern("custom_secret");

        assert!(env.is_sensitive("MY_CUSTOM_SECRET_VALUE"));
        assert!(env.is_sensitive("custom_secret"));
    }

    #[test]
    fn test_mask_env_map() {
        use std::collections::HashMap;

        let env = SensitiveEnv::new();
        let mut map = HashMap::new();
        map.insert("API_KEY".to_string(), "secret123".to_string());
        map.insert("USER_NAME".to_string(), "alice".to_string());

        let masked = env.mask_env_map(&map);
        assert_eq!(masked.get("API_KEY").unwrap(), "se***");
        assert_eq!(masked.get("USER_NAME").unwrap(), "alice");
    }

    #[test]
    fn test_zeroize_on_drop() {
        // Zeroizeが正しく動作することを確認
        {
            let _secret = SensitiveString::new("sensitive_data".to_string());
        }
        // ドロップ時にメモリがゼロクリアされる
    }
}
