# Mutation Testing Guide for cmdrun

> **Comprehensive guide to mutation testing with cargo-mutants**

## 📋 Table of Contents

- [What is Mutation Testing?](#what-is-mutation-testing)
- [Why Mutation Testing?](#why-mutation-testing)
- [Getting Started](#getting-started)
- [Running Mutation Tests](#running-mutation-tests)
- [Interpreting Results](#interpreting-results)
- [Improving Test Quality](#improving-test-quality)
- [CI Integration](#ci-integration)
- [Best Practices](#best-practices)
- [Troubleshooting](#troubleshooting)

---

## What is Mutation Testing?

Mutation testing is a technique to evaluate the **quality of your tests**, not just code coverage.

### How it works:

1. **Mutate**: cargo-mutants introduces small changes (mutations) to your code
2. **Test**: Run your test suite against the mutated code
3. **Evaluate**:
   - If tests **fail** → Mutation is **CAUGHT** ✅ (good test quality)
   - If tests **pass** → Mutation is **MISSED** ❌ (test gap found)
   - If code doesn't **compile** → Mutation is **UNVIABLE** ⚠️

### Example Mutations:

```rust
// Original code
fn is_valid(x: i32) -> bool {
    x > 0 && x < 100
}

// Mutant 1: Delete ! (boolean negation)
fn is_valid(x: i32) -> bool {
    x > 0 && x < 100  // No change, but if there was !, it would be deleted
}

// Mutant 2: Replace && with ||
fn is_valid(x: i32) -> bool {
    x > 0 || x < 100  // Logic changed
}

// Mutant 3: Replace > with >=
fn is_valid(x: i32) -> bool {
    x >= 0 && x < 100  // Boundary changed
}
```

If your tests don't catch these mutations, they're not testing edge cases properly.

---

## Why Mutation Testing?

### Traditional Code Coverage Limitations

```rust
// 100% line coverage, but poor test quality
fn add(a: i32, b: i32) -> i32 {
    a + b  // Line is executed
}

#[test]
fn test_add() {
    add(2, 3);  // Doesn't check the result! ❌
}
```

**Code coverage says**: ✅ 100% covered
**Mutation testing says**: ❌ If `a + b` is changed to `a - b`, test still passes

### Mutation Testing Reveals the Gap

```rust
#[test]
fn test_add() {
    assert_eq!(add(2, 3), 5);  // ✅ Catches mutations
}
```

Now if the mutation changes `+` to `-`, the test will fail.

### Benefits for cmdrun

1. **Security Assurance**: Catch gaps in security-critical code (interpolation, validation)
2. **Reliability**: Ensure core logic (executor, dependency resolution) is well-tested
3. **Regression Prevention**: High-quality tests prevent bugs from sneaking in
4. **Confidence**: Deploy with confidence that your tests actually work

---

## Getting Started

### 1. Install cargo-mutants

```bash
cargo install cargo-mutants --version 25.3.1
```

### 2. Verify Installation

```bash
cargo mutants --version
# Output: cargo-mutants 25.3.1
```

### 3. Configuration

Configuration is in `.cargo/mutants.toml`:

```toml
# Timeout multiplier for mutation tests
timeout_multiplier = 4.0

# Minimum timeout for tests (in seconds)
minimum_test_timeout = 20

# Test tool (cargo or nextest)
test_tool = "cargo"

# Exclude patterns
exclude_globs = [
    "tests/fixtures/**",
    "benches/**",
    "examples/**",
]

# Prioritized modules
examine_globs = [
    "src/security/**/*.rs",
    "src/command/executor.rs",
    "src/command/interpolation.rs",
]
```

---

## Running Mutation Tests

### Quick Start (Single File)

```bash
# Test a specific file (fastest, recommended for development)
cargo mutants --file src/commands/search.rs
```

### Targeted Testing (Module)

```bash
# Test all files in a directory
cargo mutants --file "src/security/**/*.rs"
```

### Full Suite (All Configured Modules)

```bash
# Run mutation tests on all examine_globs
cargo mutants
```

**⚠️ Warning**: Full suite can take 30-60 minutes

### Advanced Options

```bash
# Parallel execution (faster, uses more CPU)
cargo mutants --jobs 4

# Custom timeout
cargo mutants --timeout 180

# List all possible mutants without running
cargo mutants --list

# Don't shuffle (reproducible results)
cargo mutants --no-shuffle

# JSON output for CI
cargo mutants --output json
```

---

## Interpreting Results

### Result Types

1. **CAUGHT** ✅ - Mutation detected by tests (good)
2. **MISSED** ❌ - Mutation not detected (test gap)
3. **UNVIABLE** ⚠️ - Mutation causes compile error (neutral)
4. **TIMEOUT** ⏱️ - Tests took too long (may need adjustment)

### Example Output

```
Found 54 mutants to test
ok       Unmutated baseline in 82.4s build + 1.9s test
MISSED   src/command/executor.rs:118:12: delete ! in CommandExecutor::execute
CAUGHT   src/command/executor.rs:120:15: replace > with >= in CommandExecutor::execute
UNVIABLE src/command/executor.rs:125:9: replace Result<()> with Ok(String::new())
54 mutants tested in 14m 41s: 18 missed, 20 caught, 16 unviable
```

### Mutation Score

```
Mutation Score = (CAUGHT / (CAUGHT + MISSED)) * 100
```

**For cmdrun project**:
- **Executor.rs example**: 20 caught / (20 + 18) = **52.6%**
- **Target**:
  - Security-critical modules: **>80%**
  - Core logic: **>70%**
  - Commands: **>60%**
  - Overall: **>60%**

---

## Improving Test Quality

### Example: executor.rs Missed Mutants

#### Missed Mutant #1:
```
MISSED src/command/executor.rs:155:9: replace check_platform -> Result<()> with Ok(())
```

**Problem**: Function returns success without actually checking platform

**Solution**: Add assertion test
```rust
#[test]
fn test_check_platform_validates_correctly() {
    let executor = CommandExecutor::new();
    let cmd = create_test_command_with_windows_platform();

    #[cfg(not(target_os = "windows"))]
    {
        let result = executor.check_platform(&cmd);
        assert!(result.is_err(), "Should fail on non-Windows platform");
    }

    #[cfg(target_os = "windows")]
    {
        let result = executor.check_platform(&cmd);
        assert!(result.is_ok(), "Should succeed on Windows platform");
    }
}
```

#### Missed Mutant #2:
```
MISSED src/command/executor.rs:345:9: replace print_command with ()
```

**Problem**: Function does nothing and tests don't verify output

**Solution**: Capture and verify output
```rust
#[test]
fn test_print_command_outputs_correctly() {
    let executor = CommandExecutor::new();
    let cmd = create_test_command("test_cmd");

    // Capture stdout
    let output = std::io::stdout();
    executor.print_command(&cmd);

    // Verify output contains command name
    // (Use a capture mechanism or integration test)
}
```

### General Strategies

1. **Assert Return Values**: Don't just call functions, check their results
2. **Test Edge Cases**: Boundaries, empty inputs, error conditions
3. **Verify Side Effects**: Output, state changes, external calls
4. **Use Property-Based Testing**: Generate random inputs with proptest
5. **Integration Tests**: Test how functions work together

---

## CI Integration

### GitHub Actions Workflow

Located at: `.github/workflows/mutation-testing.yml`

**Triggers**:
- **Weekly**: Every Sunday at 00:00 UTC (automatic)
- **Manual**: Workflow dispatch with custom parameters

**Features**:
- 📊 Automatic mutation score calculation
- 📝 PR comments with results summary
- 📦 Artifact upload (90-day retention)
- 🎯 Threshold checking (warning if <50%)
- 🏷️ Badge generation

### Manual Trigger (GitHub UI)

1. Go to Actions tab
2. Select "Mutation Testing" workflow
3. Click "Run workflow"
4. Options:
   - `target`: Specific file (e.g., `src/command/executor.rs`)
   - `timeout`: Timeout in seconds (default: 120)

### Local Development Workflow

```bash
# Before committing changes to critical files
cargo mutants --file src/security/validation.rs

# If mutations are missed
# → Add tests to catch them
# → Re-run mutation tests
# → Commit when mutation score improves
```

---

## Best Practices

### 1. Incremental Approach

❌ **Don't**: Run full mutation testing on the entire codebase daily
✅ **Do**: Target specific modules during development

```bash
# During feature development
cargo mutants --file src/commands/new_feature.rs

# Before merging PR
cargo mutants --file "src/commands/**/*.rs"
```

### 2. Prioritize Critical Code

**Focus order**:
1. Security modules (`src/security/**`)
2. Core logic (`src/command/executor.rs`, `src/command/dependency.rs`)
3. Command handlers (`src/commands/**`)
4. Utilities and helpers

### 3. Set Realistic Targets

| Module Type | Target Score | Priority |
|------------|--------------|----------|
| Security   | >80%         | Critical |
| Core Logic | >70%         | High     |
| Commands   | >60%         | Medium   |
| Utilities  | >50%         | Low      |

### 4. Iterative Improvement

```bash
# Week 1: Establish baseline
cargo mutants > baseline-report.txt

# Week 2: Improve one module
cargo mutants --file src/command/executor.rs
# Add tests for missed mutants

# Week 3: Re-measure
cargo mutants --file src/command/executor.rs
# Compare with baseline
```

### 5. Don't Obsess Over 100%

- Some mutants are **unviable** (compile errors)
- Some mutations are **equivalent** (no semantic change)
- **80-85%** is excellent for most code
- **90%+** for security-critical code only

---

## Troubleshooting

### Issue: Tests timeout

**Symptom**:
```
TIMEOUT src/command/executor.rs:100:5: replace execute -> Result<()> with Ok(())
```

**Solution**:
```bash
# Increase timeout multiplier
cargo mutants --timeout 240  # 4 minutes per mutant

# Or in .cargo/mutants.toml
timeout_multiplier = 6.0
```

### Issue: Too many unviable mutants

**Symptom**:
```
54 mutants tested: 2 missed, 5 caught, 47 unviable
```

**Cause**: Strongly-typed code (good!) causes many mutations to fail compilation

**Solution**: This is normal and good. Focus on improving caught/(caught+missed) ratio.

### Issue: Mutation testing is too slow

**Solutions**:

1. **Parallel jobs**:
   ```bash
   cargo mutants --jobs 4
   ```

2. **In-place testing** (faster, but risky):
   ```bash
   cargo mutants --in-place
   # ⚠️ Don't edit files while running
   ```

3. **Target specific files**:
   ```bash
   cargo mutants --file src/commands/search.rs
   ```

4. **Use faster test tool**:
   ```bash
   cargo install cargo-nextest
   # In .cargo/mutants.toml:
   test_tool = "nextest"
   ```

### Issue: Baseline tests fail

**Symptom**:
```
ERROR: Baseline tests failed
```

**Solution**:
```bash
# Fix failing tests first
cargo test --lib --bins

# Then retry mutation testing
cargo mutants
```

---

## Mutation Testing Results (cmdrun)

### Initial Baseline (2025-11-13)

#### search.rs
- **Total**: 3 mutants
- **Caught**: 2 (66.7%)
- **Missed**: 1 (33.3%)
- **Score**: 66.7%

#### executor.rs
- **Total**: 54 mutants
- **Caught**: 20 (37.0%)
- **Missed**: 18 (33.3%)
- **Unviable**: 16 (29.6%)
- **Score**: 52.6%

**Next Steps**:
1. Add tests for missed mutants in executor.rs (priority: high)
2. Focus on platform checking, parallel execution, and helper functions
3. Target: Improve executor.rs score to >70%

---

## Resources

- **Official Documentation**: https://mutants.rs/
- **GitHub Repository**: https://github.com/sourcefrog/cargo-mutants
- **cargo-mutants Book**: https://mutants.rs/
- **Rust Testing Book**: https://doc.rust-lang.org/book/ch11-00-testing.html
- **Property-Based Testing**: https://github.com/proptest-rs/proptest

---

## Contributing

When adding new features to cmdrun:

1. Write tests that catch mutations
2. Run mutation testing on changed files
3. Add tests for missed mutants before merging
4. Update this guide with new patterns discovered

---

**Last Updated**: 2025-11-13
**cargo-mutants Version**: 25.3.1
**Mutation Testing Status**: ✅ Enabled (Weekly CI + Manual)
