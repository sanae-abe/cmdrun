# cmdrun - Test Documentation

> Comprehensive testing guide for cmdrun: A fast, secure, and cross-platform command runner

**Last Updated**: 2025-11-10
**Test Coverage**: ~55% (target: 70% by Phase 2, 85% by Phase 3)

---

## 📋 Table of Contents

- [Quick Start](#quick-start)
- [Test Structure](#test-structure)
- [Running Tests](#running-tests)
- [Coverage Report](#coverage-report)
- [Test Categories](#test-categories)
- [Writing Tests](#writing-tests)
- [Best Practices](#best-practices)
- [CI/CD Integration](#cicd-integration)
- [Troubleshooting](#troubleshooting)

---

## 🚀 Quick Start

### Run All Tests

```bash
# Run all tests (unit + integration + e2e)
cargo test

# Run tests with output
cargo test -- --nocapture

# Run tests in parallel (default)
cargo test -- --test-threads=4
```

### Run Specific Test Categories

```bash
# Unit tests only
cargo test --lib

# Integration tests only
cargo test --test '*'

# E2E tests
cargo test --test e2e_tests

# Security tests
cargo test --test security_injection

# Property-based tests
cargo test proptest
```

### Generate Coverage Report

```bash
# Install cargo-tarpaulin (one-time setup)
cargo install cargo-tarpaulin

# Generate coverage report (HTML + terminal output)
cargo tarpaulin --out Html --out Stdout --timeout 300

# Generate coverage with detailed line-by-line report
cargo tarpaulin --out Html --output-dir coverage --timeout 300

# Open coverage report in browser
open tarpaulin-report.html  # macOS
xdg-open tarpaulin-report.html  # Linux
```

---

## 📁 Test Structure

```
tests/
├── README.md                          # This file
│
├── e2e/                               # E2E Tests (End-to-End)
│   ├── framework.rs                   # Test framework (CmdrunTestEnv)
│   ├── cli_workflow.rs                # CLI workflow tests (10 scenarios)
│   └── mod.rs                         # Module entry point
│
├── e2e_tests.rs                       # E2E test entry point
│
├── integration/                       # Integration Tests
│   ├── basic.rs                       # Basic command execution
│   ├── dependencies.rs                # Dependency resolution
│   ├── parallel.rs                    # Parallel execution
│   ├── watch.rs                       # File watching
│   ├── environment.rs                 # Environment management
│   ├── history.rs                     # Command history
│   ├── env_commands.rs                # Environment variable commands
│   ├── history_commands.rs            # History management commands
│   ├── error_handling.rs              # Error handling scenarios
│   ├── cli_commands.rs                # CLI command tests
│   ├── cross_platform.rs              # Cross-platform compatibility tests
│   ├── test_add.rs                    # Add command tests
│   ├── test_config.rs                 # Config command tests
│   ├── test_search.rs                 # Search command tests
│   └── test_logger.rs                 # Logger tests
│
├── lib_integration/                   # Library Integration Tests
│   ├── mod.rs                         # Module entry point
│   ├── test_add_commands.rs           # Add command library tests
│   ├── test_config_commands.rs        # Config command library tests
│   └── test_search_commands.rs        # Search command library tests
│
├── lib_integration_tests.rs           # Library integration entry point
│
├── security/                          # Security Tests
│   └── injection.rs                   # Injection vulnerability tests
│
├── plugin/                            # Plugin Tests
│   └── basic.rs                       # Basic plugin functionality
│
├── unit_*.rs                          # Unit Tests
│   ├── unit_interpolation.rs          # Variable interpolation tests
│   ├── unit_dependency_graph.rs       # Dependency graph tests
│   ├── unit_executor.rs               # Executor tests
│   ├── unit_color_output.rs           # Color output tests
│   ├── unit_i18n.rs                   # Internationalization tests
│   └── unit_typo_detector.rs          # Typo detection tests
│
├── proptest_coverage.rs               # Property-based tests
├── edge_cases.rs                      # Edge case tests
├── test_remove.rs                     # Remove command tests
│
└── fixtures/                          # Test fixtures and data
```

---

## 🏃 Running Tests

### By Test Type

#### 1. Unit Tests

Test individual functions and modules in isolation.

```bash
# All unit tests
cargo test --lib

# Specific unit test file
cargo test unit_interpolation
cargo test unit_dependency_graph
cargo test unit_executor
cargo test unit_color_output
cargo test unit_i18n
cargo test unit_typo_detector
```

#### 2. Integration Tests

Test interactions between multiple modules and components.

```bash
# All integration tests
cargo test --test '*'

# Specific integration test
cargo test --test basic
cargo test --test dependencies
cargo test --test watch
cargo test --test environment
cargo test --test history
cargo test --test error_handling
```

#### 3. E2E Tests (End-to-End)

Test complete user workflows from CLI invocation to output.

```bash
# All E2E tests
cargo test --test e2e_tests

# Run with detailed output
cargo test --test e2e_tests -- --nocapture
```

#### 4. Security Tests

Test security-critical functionality (injection prevention, etc.).

```bash
# All security tests
cargo test --test security_injection

# Run with detailed output
cargo test --test security_injection -- --nocapture
```

#### 5. Property-based Tests

Test code properties using randomized inputs (powered by `proptest`).

```bash
# All property-based tests
cargo test proptest

# Run with increased test cases
PROPTEST_CASES=10000 cargo test proptest
```

### By Module/Feature

```bash
# Command execution tests
cargo test executor

# Configuration tests
cargo test config

# Watch functionality tests
cargo test watch

# Environment management tests
cargo test environment

# History tests
cargo test history

# i18n tests
cargo test i18n

# Dependency resolution tests
cargo test dependency
```

### Performance and Optimization

```bash
# Run tests in release mode (faster execution)
cargo test --release

# Run tests with optimizations but keep debug info
cargo test --profile release-with-debug

# Limit parallel test execution
cargo test -- --test-threads=2

# Run tests sequentially (useful for debugging)
cargo test -- --test-threads=1
```

---

## 📊 Coverage Report

### Current Coverage Status

**Overall Coverage**: ~55% (as of 2025-11-10)

**Phase 1 Completed** (2025-11-10):
- Baseline: 38.16% → Target: 55% ✅
- Added: ~1,831 lines of test code
- Focus: E2E framework, i18n, error handling, env/history commands

**Coverage by Module**:

| Module | Coverage | Target | Status |
|--------|----------|--------|--------|
| `commands/env.rs` | 70% | 70% | ✅ Phase 1 |
| `commands/history.rs` | 70% | 70% | ✅ Phase 1 |
| `i18n.rs` | 40% | 40% | ✅ Phase 1 |
| `main.rs` | 50% | 50% | ✅ Phase 1 (via E2E) |
| `command/executor.rs` | 65% | 80% | 🚧 In progress |
| `commands/completion.rs` | 0% | 60% | ⏳ Phase 2 |
| `commands/plugin.rs` | 0% | 60% | ⏳ Phase 2 |
| `watch/watcher.rs` | 7.4% | 60% | ⏳ Phase 2 |

### Generating Coverage Reports

#### Using cargo-tarpaulin (Recommended)

```bash
# Install (one-time)
cargo install cargo-tarpaulin

# Generate HTML report
cargo tarpaulin --out Html --timeout 300

# Generate multiple formats
cargo tarpaulin --out Html --out Lcov --out Stdout --timeout 300

# Exclude specific files
cargo tarpaulin --out Html --exclude-files 'tests/*' --timeout 300

# Generate coverage for specific package
cargo tarpaulin --out Html --packages cmdrun --timeout 300
```

#### Using llvm-cov (Alternative)

```bash
# Install (one-time)
rustup component add llvm-tools-preview
cargo install cargo-llvm-cov

# Generate coverage
cargo llvm-cov --html

# Open report
cargo llvm-cov --open
```

### Coverage Targets

**Phase 2** (Target: 70% by 2025-12-15):
- Shell completion tests
- Plugin system tests
- Enhanced error handling tests
- Watch functionality tests
- CLI integration tests

**Phase 3** (Target: 85% by 2026-01-31):
- Performance tests automation
- Cross-platform integration tests
- Mutation testing
- Enhanced property-based testing

---

## 🧪 Test Categories

### 1. E2E Tests (End-to-End)

**Location**: `tests/e2e/`
**Purpose**: Test complete user workflows from CLI to output

**Framework**: Custom `CmdrunTestEnv` (isolated test environment)

**Test Scenarios** (10 workflows):
1. ✅ Add and execute command
2. ✅ List commands
3. ✅ Remove command
4. ✅ Execute with dependencies
5. ✅ Parallel execution
6. ✅ Environment management
7. ✅ History tracking
8. ✅ Watch mode
9. ✅ Error handling
10. ✅ Help and version display

**Running E2E Tests**:
```bash
cargo test --test e2e_tests
```

### 2. Integration Tests

**Location**: `tests/integration/`
**Purpose**: Test interactions between modules

**Key Test Areas**:
- Command execution lifecycle
- Dependency resolution
- Environment variable management
- File watching
- History recording
- Error handling and recovery
- Configuration loading and parsing
- **Cross-platform compatibility** (Windows/macOS/Linux)

**Running Integration Tests**:
```bash
cargo test --test basic
cargo test --test dependencies
cargo test --test error_handling
cargo test --test cross_platform  # Cross-platform compatibility tests
```

#### Cross-Platform Integration Tests

**Location**: `tests/integration/cross_platform.rs`
**Purpose**: Ensure cmdrun works correctly across different operating systems and shells

**Test Coverage**:

1. **Path Handling Tests**:
   - Windows: Backslashes, drive letters (C:\), UNC paths (\\server\share)
   - macOS: Forward slashes, /Users/ home directories, case-insensitivity
   - Linux: Forward slashes, /home/ directories, case-sensitivity, symlinks

2. **Shell-Specific Tests**:
   - Windows: cmd.exe, PowerShell (pwsh), batch file execution
   - Unix: bash, sh, zsh, fish, shell script execution

3. **Line Ending Tests**:
   - LF (Unix: \n)
   - CRLF (Windows: \r\n)
   - Mixed line endings in configuration files

4. **Cross-Platform Commands**:
   - Echo command (works everywhere)
   - Exit code preservation
   - Environment variable expansion

5. **File System Encoding**:
   - UTF-8 filenames (Unix)
   - Unicode filenames (Windows)

**Running Cross-Platform Tests**:
```bash
# Run all cross-platform tests
cargo test --test cross_platform

# Run platform-specific tests only
cargo test --test cross_platform windows_  # Windows-only tests
cargo test --test cross_platform macos_    # macOS-only tests
cargo test --test cross_platform linux_    # Linux-only tests

# Run shell-specific tests
cargo test --test cross_platform shell

# Run line ending tests
cargo test --test cross_platform line_ending
```

**Platform-Specific Execution**:
These tests use conditional compilation (`#[cfg(windows)]`, `#[cfg(unix)]`, `#[cfg(target_os = "macos")]`) to run only on appropriate platforms.

### 3. Unit Tests

**Location**: `tests/unit_*.rs` and `src/**/*.rs` (inline)
**Purpose**: Test individual functions and modules

**Key Test Areas**:
- Variable interpolation (`unit_interpolation.rs`)
- Dependency graph construction (`unit_dependency_graph.rs`)
- Command executor logic (`unit_executor.rs`)
- Color output formatting (`unit_color_output.rs`)
- i18n translation completeness (`unit_i18n.rs`)
- Typo detection algorithm (`unit_typo_detector.rs`)

**Running Unit Tests**:
```bash
cargo test --lib
cargo test unit_i18n
```

### 4. Security Tests

**Location**: `tests/security/`
**Purpose**: Validate security-critical functionality

**Key Test Areas**:
- Shell injection prevention
- Path traversal protection
- Environment variable sanitization
- Safe command execution

**Running Security Tests**:
```bash
cargo test --test security_injection
```

### 5. Property-based Tests

**Location**: `tests/proptest_coverage.rs`
**Purpose**: Test code properties with randomized inputs

**Framework**: `proptest`

**Test Properties**:
- Dependency graph correctness
- Command parsing invariants
- Configuration validation

**Running Property-based Tests**:
```bash
cargo test proptest
PROPTEST_CASES=10000 cargo test proptest  # More test cases
```

### 6. Plugin Tests

**Location**: `tests/plugin/`
**Purpose**: Test plugin system functionality

**Key Test Areas**:
- Plugin loading
- Plugin execution
- Plugin lifecycle management

**Running Plugin Tests**:
```bash
cargo test --test plugin
```

---

## ✍️ Writing Tests

### Test Organization

Follow the **Given-When-Then** pattern for clarity:

```rust
#[test]
fn test_example() {
    // Given: Setup test environment
    let temp_dir = tempfile::tempdir().unwrap();
    let config_path = temp_dir.path().join("cmdrun.toml");

    // When: Execute the action
    let result = execute_command(&config_path);

    // Then: Verify the outcome
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), expected_value);
}
```

### E2E Test Example

```rust
use crate::e2e::framework::CmdrunTestEnv;

#[test]
fn test_add_and_execute_workflow() {
    // Given: Isolated test environment
    let mut env = CmdrunTestEnv::new();

    // When: Add a command
    let add_result = env.run(&["add", "hello", "echo Hello"]);

    // Then: Verify command was added
    add_result.assert_success();
    add_result.assert_stdout_contains("Command 'hello' added");

    // When: Execute the command
    let exec_result = env.run(&["exec", "hello"]);

    // Then: Verify execution
    exec_result.assert_success();
    exec_result.assert_stdout_contains("Hello");
}
```

### Integration Test Example

```rust
#[tokio::test]
async fn test_parallel_execution() {
    // Given: Test config with parallel commands
    let config = create_test_config();

    // When: Execute commands in parallel
    let result = execute_parallel(&config).await;

    // Then: Verify all commands succeeded
    assert!(result.is_ok());
    assert_eq!(result.unwrap().len(), 3);
}
```

### Unit Test Example

```rust
#[test]
fn test_variable_interpolation() {
    // Given: Environment variables
    let env = HashMap::from([
        ("VAR1".to_string(), "value1".to_string()),
    ]);

    // When: Interpolate variables
    let result = interpolate("Hello ${VAR1}", &env);

    // Then: Verify interpolation
    assert_eq!(result, "Hello value1");
}
```

### Property-based Test Example

```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_dependency_graph_no_cycles(
        commands in prop::collection::vec(any::<String>(), 1..10)
    ) {
        // Given: Random command names
        let graph = DependencyGraph::new();

        // When: Add commands without cycles
        for cmd in commands {
            graph.add_node(&cmd);
        }

        // Then: Graph should have no cycles
        prop_assert!(!graph.has_cycles());
    }
}
```

---

## 🎯 Best Practices

### 1. Test Naming

Use descriptive test names that explain what is being tested:

```rust
// ✅ Good
#[test]
fn test_add_command_with_valid_name_succeeds() { }

// ❌ Bad
#[test]
fn test_add() { }
```

### 2. Test Isolation

Ensure tests are isolated and can run in any order:

```rust
// ✅ Good: Use temporary directories
#[test]
fn test_with_isolation() {
    let temp_dir = tempfile::tempdir().unwrap();
    // Test uses temp_dir, automatically cleaned up
}

// ❌ Bad: Use shared state
#[test]
fn test_without_isolation() {
    let config = "/tmp/shared-config.toml";  // Conflicts with other tests
}
```

### 3. Assertion Clarity

Use descriptive assertions:

```rust
// ✅ Good
assert_eq!(result.status, CommandStatus::Success,
    "Command should succeed for valid input");

// ❌ Bad
assert!(result.status == CommandStatus::Success);
```

### 4. Test Coverage

Aim for comprehensive coverage of:
- ✅ Happy path scenarios
- ✅ Error conditions
- ✅ Edge cases
- ✅ Boundary conditions

### 5. Performance

Keep tests fast:
- Use `#[tokio::test]` for async tests
- Avoid unnecessary sleeps
- Mock external dependencies
- Use `cargo test --release` for performance-critical tests

---

## 🔄 CI/CD Integration

### GitHub Actions Workflow

The project uses GitHub Actions for continuous testing:

**Configuration**: `.github/workflows/test.yml`

```yaml
name: Tests

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - name: Run tests
        run: cargo test --all-features
      - name: Generate coverage
        run: cargo tarpaulin --out Lcov --timeout 300
      - name: Upload coverage
        uses: codecov/codecov-action@v3
```

### Coverage Thresholds

**Current Threshold**: 55%
**Phase 2 Target**: 70%
**Phase 3 Target**: 85%

CI will fail if coverage drops below the current threshold.

---

## 🐛 Troubleshooting

### Tests Hanging

**Issue**: Tests hang indefinitely

**Solution**:
```bash
# Run with timeout
cargo test -- --test-threads=1 --nocapture

# Check for deadlocks in async code
RUST_LOG=debug cargo test
```

### Flaky Tests

**Issue**: Tests pass/fail inconsistently

**Common Causes**:
- Race conditions in async code
- Shared state between tests
- Time-dependent assertions

**Solutions**:
- Use `tokio::time::pause()` for time-dependent tests
- Ensure proper test isolation
- Add retry logic for flaky external dependencies

### Coverage Generation Fails

**Issue**: `cargo tarpaulin` fails

**Solution**:
```bash
# Increase timeout
cargo tarpaulin --out Html --timeout 600

# Exclude problematic files
cargo tarpaulin --out Html --exclude-files 'tests/*' --timeout 300

# Use alternative tool
cargo llvm-cov --html
```

### Slow Tests

**Issue**: Test suite takes too long

**Solutions**:
```bash
# Run tests in release mode
cargo test --release

# Limit parallel execution
cargo test -- --test-threads=2

# Run only changed tests (requires nextest)
cargo install cargo-nextest
cargo nextest run
```

---

## 📚 Additional Resources

### Documentation

- **Test Analysis Report**: [docs/testing/test-analysis-report.md](../docs/testing/test-analysis-report.md)
- **Rust Testing Guide**: https://doc.rust-lang.org/book/ch11-00-testing.html
- **proptest Documentation**: https://github.com/proptest-rs/proptest
- **tokio Testing**: https://tokio.rs/tokio/topics/testing

### Tools

- **cargo-tarpaulin**: Code coverage tool
- **cargo-nextest**: Next-generation test runner
- **cargo-watch**: Auto-run tests on file changes
- **cargo-mutants**: Mutation testing

### Installation

```bash
# Coverage tools
cargo install cargo-tarpaulin
cargo install cargo-llvm-cov

# Test runners
cargo install cargo-nextest
cargo install cargo-watch

# Mutation testing
cargo install cargo-mutants
```

---

## 📞 Contact & Contribution

For questions or contributions related to tests:

1. Check existing issues: https://github.com/sanae-abe/cmdrun/issues
2. Create a new issue with `[Test]` prefix
3. Refer to test analysis report for current priorities

**Test Improvement Priorities** (see [todos.md](../todos.md)):
- Phase 2: Shell completion, plugin system, watch functionality
- Phase 3: Performance tests, cross-platform tests, mutation testing

---

**Last Updated**: 2025-11-10
**Maintained By**: Sanae Abe (@sanae-abe)
**License**: MIT
