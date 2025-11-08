//! 変数展開エンジン
//!
//! セキュアで高性能な変数展開を提供（eval完全排除）
//! 対応構文:
//! - ${VAR} - 基本展開
//! - ${1}, ${2} - 位置引数展開
//! - ${VAR:-default} - デフォルト値
//! - ${VAR:?error_message} - 必須変数
//! - ${VAR:+value_if_set} - 設定時置換

use crate::error::{InterpolationError, Result};
use ahash::AHashMap;
use once_cell::sync::Lazy;
use regex::Regex;
use std::env;

/// 変数展開結果の最大サイズ（DoS防止）
const MAX_EXPANSION_LENGTH: usize = 10_240; // 10KB

/// 変数展開パターン（コンパイル時最適化）
/// 位置引数（${1}, ${2}等）と通常変数（${VAR}）の両方に対応
static VAR_PATTERN: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"\$\{([A-Za-z_][A-Za-z0-9_]*|[0-9]+)(:[?+\-])?([^}]*)?\}").unwrap());

/// 変数コンテキスト
#[derive(Debug, Clone)]
pub struct InterpolationContext {
    /// 環境変数
    env_vars: AHashMap<String, String>,
    /// 厳格モード（未定義変数でエラー）
    strict: bool,
    /// 最大展開深度（再帰防止）
    max_depth: usize,
}

impl InterpolationContext {
    /// 新規コンテキスト作成
    pub fn new(strict: bool) -> Self {
        Self {
            env_vars: AHashMap::new(),
            strict,
            max_depth: 10,
        }
    }

    /// 環境変数を追加
    pub fn with_env(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.env_vars.insert(key.into(), value.into());
        self
    }

    /// 複数の環境変数を追加
    pub fn with_env_map(mut self, env: AHashMap<String, String>) -> Self {
        self.env_vars.extend(env);
        self
    }

    /// システム環境変数をマージ
    pub fn merge_system_env(mut self) -> Self {
        for (key, value) in env::vars() {
            self.env_vars.entry(key).or_insert(value);
        }
        self
    }

    /// 変数展開実行
    pub fn interpolate(&self, input: &str) -> Result<String> {
        self.interpolate_with_depth(input, 0)
    }

    /// 深度管理付き変数展開（再帰防止）
    fn interpolate_with_depth(&self, input: &str, depth: usize) -> Result<String> {
        if depth >= self.max_depth {
            return Err(InterpolationError::RecursiveExpansion(input.to_string()).into());
        }

        // DoS防止：文字列サイズチェック
        if input.len() > MAX_EXPANSION_LENGTH {
            return Err(InterpolationError::ExpansionTooLarge(input.len()).into());
        }

        let mut result = String::with_capacity(input.len());
        let mut last_end = 0;

        for cap in VAR_PATTERN.captures_iter(input) {
            let full_match = cap.get(0).unwrap();
            let var_name = cap.get(1).unwrap().as_str();
            let operator = cap.get(2).map(|m| m.as_str());
            let operand = cap.get(3).map(|m| m.as_str()).unwrap_or("");

            // マッチ前の部分を追加
            result.push_str(&input[last_end..full_match.start()]);

            // 変数値を取得
            let value = self.resolve_variable(var_name, operator, operand)?;

            // 再帰的に展開（ネストされた変数対応）
            let expanded = if value.contains("${") {
                self.interpolate_with_depth(&value, depth + 1)?
            } else {
                value
            };

            result.push_str(&expanded);
            last_end = full_match.end();
        }

        // 残りの部分を追加
        result.push_str(&input[last_end..]);
        Ok(result)
    }

    /// 変数値解決
    fn resolve_variable(
        &self,
        var_name: &str,
        operator: Option<&str>,
        operand: &str,
    ) -> Result<String> {
        let var_value = self.env_vars.get(var_name);

        match operator {
            // ${VAR} - 基本展開
            None => match var_value {
                Some(v) => Ok(v.clone()),
                None if self.strict => {
                    Err(InterpolationError::UndefinedVariable(var_name.to_string()).into())
                }
                None => Ok(String::new()),
            },

            // ${VAR:-default} - デフォルト値
            Some(":-") => Ok(var_value
                .filter(|v| !v.is_empty())
                .cloned()
                .unwrap_or_else(|| operand.to_string())),

            // ${VAR:?error_message} - 必須変数
            Some(":?") => match var_value {
                Some(v) if !v.is_empty() => Ok(v.clone()),
                _ => {
                    let msg = if operand.is_empty() {
                        format!("{} not set", var_name)
                    } else {
                        operand.to_string()
                    };
                    Err(InterpolationError::RequiredVariableNotSet(msg).into())
                }
            },

            // ${VAR:+value_if_set} - 設定時置換
            Some(":+") => {
                if var_value.is_some() {
                    Ok(operand.to_string())
                } else {
                    Ok(String::new())
                }
            }

            // 未知の演算子
            Some(op) => {
                Err(InterpolationError::InvalidSyntax(format!("Unknown operator: {}", op)).into())
            }
        }
    }
}

/// 便利関数：単純な変数展開
pub fn interpolate(input: &str, env: &AHashMap<String, String>) -> Result<String> {
    InterpolationContext::new(false)
        .with_env_map(env.clone())
        .merge_system_env()
        .interpolate(input)
}

/// 便利関数：厳格モード変数展開
pub fn interpolate_strict(input: &str, env: &AHashMap<String, String>) -> Result<String> {
    InterpolationContext::new(true)
        .with_env_map(env.clone())
        .merge_system_env()
        .interpolate(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_env() -> AHashMap<String, String> {
        let mut env = AHashMap::new();
        env.insert("USER".to_string(), "alice".to_string());
        env.insert("HOME".to_string(), "/home/alice".to_string());
        env.insert("EMPTY".to_string(), String::new());
        env
    }

    #[test]
    fn test_basic_interpolation() {
        let env = test_env();
        let ctx = InterpolationContext::new(false).with_env_map(env);

        let result = ctx.interpolate("Hello, ${USER}!").unwrap();
        assert_eq!(result, "Hello, alice!");
    }

    #[test]
    fn test_multiple_variables() {
        let env = test_env();
        let ctx = InterpolationContext::new(false).with_env_map(env);

        let result = ctx.interpolate("${USER} lives in ${HOME}").unwrap();
        assert_eq!(result, "alice lives in /home/alice");
    }

    #[test]
    fn test_default_value() {
        let env = test_env();
        let ctx = InterpolationContext::new(false).with_env_map(env);

        let result = ctx.interpolate("${UNDEFINED:-default}").unwrap();
        assert_eq!(result, "default");

        let result = ctx.interpolate("${USER:-default}").unwrap();
        assert_eq!(result, "alice");
    }

    #[test]
    fn test_required_variable() {
        let env = test_env();
        let ctx = InterpolationContext::new(false).with_env_map(env);

        let result = ctx.interpolate("${USER:?USER is required}");
        assert!(result.is_ok());

        let result = ctx.interpolate("${UNDEFINED:?UNDEFINED is required}");
        assert!(result.is_err());
    }

    #[test]
    fn test_conditional_value() {
        let env = test_env();
        let ctx = InterpolationContext::new(false).with_env_map(env);

        let result = ctx.interpolate("${USER:+user_is_set}").unwrap();
        assert_eq!(result, "user_is_set");

        let result = ctx.interpolate("${UNDEFINED:+user_is_set}").unwrap();
        assert_eq!(result, "");
    }

    #[test]
    fn test_nested_interpolation() {
        let mut env = test_env();
        env.insert("PATH_VAR".to_string(), "${HOME}/bin".to_string());
        let ctx = InterpolationContext::new(false).with_env_map(env);

        let result = ctx.interpolate("${PATH_VAR}").unwrap();
        assert_eq!(result, "/home/alice/bin");
    }

    #[test]
    fn test_strict_mode() {
        let env = test_env();
        let ctx = InterpolationContext::new(true).with_env_map(env);

        let result = ctx.interpolate("${UNDEFINED}");
        assert!(result.is_err());
    }

    #[test]
    fn test_recursive_expansion_limit() {
        let mut env = AHashMap::new();
        env.insert("A".to_string(), "${B}".to_string());
        env.insert("B".to_string(), "${C}".to_string());
        env.insert("C".to_string(), "${D}".to_string());
        env.insert("D".to_string(), "${E}".to_string());
        env.insert("E".to_string(), "${F}".to_string());
        env.insert("F".to_string(), "${G}".to_string());
        env.insert("G".to_string(), "${H}".to_string());
        env.insert("H".to_string(), "${I}".to_string());
        env.insert("I".to_string(), "${J}".to_string());
        env.insert("J".to_string(), "${K}".to_string());
        env.insert("K".to_string(), "value".to_string());

        let ctx = InterpolationContext::new(false).with_env_map(env);
        let result = ctx.interpolate("${A}");
        assert!(result.is_err()); // 深度超過
    }

    #[test]
    fn test_positional_arguments() {
        let mut env = AHashMap::new();
        env.insert("1".to_string(), "first".to_string());
        env.insert("2".to_string(), "second".to_string());
        env.insert("3".to_string(), "third".to_string());
        let ctx = InterpolationContext::new(false).with_env_map(env);

        let result = ctx.interpolate("${1} ${2} ${3}").unwrap();
        assert_eq!(result, "first second third");
    }

    #[test]
    fn test_positional_arguments_with_defaults() {
        let mut env = AHashMap::new();
        env.insert("1".to_string(), "input.png".to_string());
        let ctx = InterpolationContext::new(false).with_env_map(env);

        // 1は定義済み、2は未定義でデフォルト値使用
        let result = ctx
            .interpolate("sharp -i ${1} -o ${2:-output.webp}")
            .unwrap();
        assert_eq!(result, "sharp -i input.png -o output.webp");
    }

    #[test]
    fn test_positional_and_named_variables() {
        let mut env = AHashMap::new();
        env.insert("1".to_string(), "file.txt".to_string());
        env.insert("USER".to_string(), "alice".to_string());
        let ctx = InterpolationContext::new(false).with_env_map(env);

        let result = ctx.interpolate("${USER} processes ${1}").unwrap();
        assert_eq!(result, "alice processes file.txt");
    }

    #[test]
    fn test_empty_variable_with_default() {
        let env = test_env();
        let ctx = InterpolationContext::new(false).with_env_map(env);

        let result = ctx.interpolate("${EMPTY:-default}").unwrap();
        assert_eq!(result, "default");
    }

    #[test]
    fn test_no_interpolation() {
        let ctx = InterpolationContext::new(false);
        let result = ctx.interpolate("no variables here").unwrap();
        assert_eq!(result, "no variables here");
    }
}
