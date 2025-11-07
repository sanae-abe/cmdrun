//! Unit tests for interpolation module to improve coverage

use ahash::AHashMap;
use cmdrun::command::interpolation::{interpolate, InterpolationContext};

#[test]
fn test_interpolate_basic() {
    let mut vars = AHashMap::new();
    vars.insert("VAR".to_string(), "value".to_string());

    let result = interpolate("${VAR}", &vars);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "value");
}

#[test]
fn test_interpolate_with_default() {
    let vars = AHashMap::new();
    let result = interpolate("${VAR:-default}", &vars);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "default");
}

#[test]
fn test_interpolate_with_value() {
    let mut vars = AHashMap::new();
    vars.insert("VAR".to_string(), "actual".to_string());

    let result = interpolate("${VAR:-default}", &vars);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "actual");
}

#[test]
fn test_interpolate_context() {
    let ctx = InterpolationContext::new(false)
        .with_env("VAR", "value")
        .with_env("NUM", "123");

    let result = ctx.interpolate("${VAR} ${NUM}");
    assert!(result.is_ok());
}

#[test]
fn test_interpolate_strict_mode() {
    let ctx = InterpolationContext::new(true);
    let result = ctx.interpolate("${UNDEFINED}");
    assert!(result.is_err());
}

#[test]
fn test_interpolate_non_strict() {
    let ctx = InterpolationContext::new(false);
    let result = ctx.interpolate("${UNDEFINED}");
    assert!(result.is_ok());
}

#[test]
fn test_interpolate_multiple_vars() {
    let mut vars = AHashMap::new();
    vars.insert("A".to_string(), "1".to_string());
    vars.insert("B".to_string(), "2".to_string());

    let result = interpolate("${A} ${B}", &vars);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "1 2");
}

#[test]
fn test_interpolate_no_variables() {
    let vars = AHashMap::new();
    let result = interpolate("plain text", &vars);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "plain text");
}

#[test]
fn test_context_with_env_map() {
    let mut env = AHashMap::new();
    env.insert("KEY1".to_string(), "val1".to_string());
    env.insert("KEY2".to_string(), "val2".to_string());

    let ctx = InterpolationContext::new(false).with_env_map(env);
    let result = ctx.interpolate("${KEY1}");
    assert!(result.is_ok());
}

#[test]
fn test_interpolate_positional_args() {
    let mut vars = AHashMap::new();
    vars.insert("1".to_string(), "first".to_string());
    vars.insert("2".to_string(), "second".to_string());

    let result = interpolate("${1} ${2}", &vars);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "first second");
}
