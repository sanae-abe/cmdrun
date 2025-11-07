# Fuzzing Tests for cmdrun

This directory contains fuzzing tests using [cargo-fuzz](https://github.com/rust-fuzz/cargo-fuzz) and [libFuzzer](https://llvm.org/docs/LibFuzzer.html).

## Overview

Fuzzing is an automated testing technique that provides random, malformed, or unexpected inputs to find bugs, crashes, and security vulnerabilities. For cmdrun, fuzzing is critical for:

- **Security**: Detecting shell injection vulnerabilities in variable expansion and command validation
- **Robustness**: Finding edge cases in TOML parsing and command execution
- **Reliability**: Ensuring the tool handles malformed inputs gracefully

## Fuzz Targets

### 🔴 High Priority (Security-Critical)

#### 1. `fuzz_interpolation` - Variable Expansion Security
**File**: `fuzz_targets/fuzz_interpolation.rs`

**Purpose**: Tests variable interpolation for shell injection vulnerabilities

**Tested Components**:
- Basic variable expansion: `${VAR}`
- Default values: `${VAR:-default}`
- Required variables: `${VAR:?error}`
- Conditional expansion: `${VAR:+value}`
- Nested variable expansion
- Recursive expansion limits

**Security Focus**:
- Shell metacharacter injection
- Recursive expansion DoS
- Variable name validation
- Operator parsing security

**Run**:
```bash
cargo +nightly fuzz run fuzz_interpolation
```

#### 2. `fuzz_validation` - Input Validation
**File**: `fuzz_targets/fuzz_validation.rs`

**Purpose**: Tests command validation for injection attacks

**Tested Components**:
- Shell metacharacter detection
- Dangerous pattern matching (eval, sudo, chmod, etc.)
- Null byte injection
- Command length limits
- Forbidden word detection
- Shell argument escaping

**Security Focus**:
- Command injection prevention
- Path traversal attacks
- Privilege escalation attempts
- System destruction commands (rm -rf /, dd, mkfs, format)

**Run**:
```bash
cargo +nightly fuzz run fuzz_validation
```

### 🟡 Medium Priority

#### 3. `fuzz_toml_config` - TOML Parsing
**File**: `fuzz_targets/fuzz_toml_config.rs`

**Purpose**: Tests TOML configuration parsing robustness

**Tested Components**:
- TOML syntax parsing (toml crate)
- TOML editing (toml_edit crate)
- CommandsConfig deserialization
- Individual section parsing

**Run**:
```bash
cargo +nightly fuzz run fuzz_toml_config
```

#### 4. `fuzz_command_parts` - Command Parsing Components
**File**: `fuzz_targets/fuzz_command_parts.rs`

**Purpose**: Tests various command parsing utilities

**Tested Components**:
- Shell word splitting (shell-words)
- Path expansion (shellexpand)
- Regex compilation
- Glob pattern parsing
- Combined interpolation + validation

**Run**:
```bash
cargo +nightly fuzz run fuzz_command_parts
```

## Prerequisites

### Install Nightly Toolchain
```bash
rustup toolchain install nightly
```

### Install cargo-fuzz
```bash
cargo install cargo-fuzz
```

## Usage

### Run Individual Fuzz Target
```bash
# Run for 60 seconds
cargo +nightly fuzz run <target> -- -max_total_time=60

# Run with specific options
cargo +nightly fuzz run fuzz_interpolation -- \
  -max_total_time=300 \
  -max_len=4096 \
  -print_final_stats=1
```

### Run All Targets (5 minutes each)
```bash
for target in fuzz_interpolation fuzz_validation fuzz_toml_config fuzz_command_parts; do
  echo "Fuzzing $target..."
  timeout 300s cargo +nightly fuzz run $target -- -max_total_time=300 || true
done
```

### Reproduce a Crash
If fuzzing finds a crash, it saves the input to `fuzz/artifacts/<target>/`:

```bash
# Reproduce the crash
cargo +nightly fuzz run <target> fuzz/artifacts/<target>/<crash-file>

# Example
cargo +nightly fuzz run fuzz_interpolation fuzz/artifacts/fuzz_interpolation/crash-1234
```

### Debug a Crash
```bash
# Run with debug symbols
cargo +nightly fuzz run <target> fuzz/artifacts/<target>/<crash-file> -- -runs=1
```

### Coverage Information
```bash
# Generate coverage report
cargo +nightly fuzz coverage <target>

# View coverage HTML
cargo +nightly fuzz coverage <target> --html
open fuzz/coverage/<target>/index.html
```

## CI/CD Integration

Fuzzing runs automatically on GitHub Actions:

### Scheduled Runs
- **Weekly**: Every Sunday at 2:00 UTC
- **Duration**: 5 minutes per target (20 minutes total)
- **Artifacts**: Crash inputs saved for 30 days

### Manual Trigger
```bash
# Via GitHub Actions UI: Actions → Fuzzing → Run workflow
# Or via GitHub CLI:
gh workflow run fuzzing.yml -f duration=600 -f target=fuzz_interpolation
```

### Crash Handling
- **Automatic Issue Creation**: Creates GitHub issue with crash details
- **Artifact Upload**: Crash inputs uploaded as artifacts
- **Notification**: Workflow failure triggers team notification

## Fuzzing Strategy

### Time Allocation
1. **Daily Development**: 1-2 minutes per target (smoke test)
2. **Weekly CI**: 5 minutes per target
3. **Release Preparation**: 30-60 minutes per target
4. **Security Audit**: 6-24 hours per target

### Priority Order
1. `fuzz_interpolation` - Variable expansion (shell injection risk)
2. `fuzz_validation` - Command validation (command injection risk)
3. `fuzz_command_parts` - Parsing utilities
4. `fuzz_toml_config` - Configuration parsing

### Coverage Goals
- **Interpolation**: >90% code coverage
- **Validation**: >95% code coverage (security-critical)
- **TOML Config**: >80% code coverage
- **Command Parts**: >85% code coverage

## Performance Optimization

### Fuzzing Speed
```bash
# Use multiple jobs for parallel fuzzing
cargo +nightly fuzz run <target> -- -jobs=4 -workers=4

# Reduce max length for faster fuzzing
cargo +nightly fuzz run <target> -- -max_len=1024
```

### Corpus Minimization
```bash
# Minimize corpus (remove redundant inputs)
cargo +nightly fuzz cmin <target>

# Merge multiple corpora
cargo +nightly fuzz cmin <target> corpus1 corpus2
```

## Security Best Practices

### 1. Regular Fuzzing Schedule
- Run fuzzing before every release
- Include in security audit process
- Monitor CI fuzzing results weekly

### 2. Crash Triage
- **Critical**: Shell injection, command injection, privilege escalation
- **High**: DoS, memory safety, data corruption
- **Medium**: Error handling, edge cases
- **Low**: Performance degradation

### 3. Regression Testing
For every crash found:
1. Create minimal reproduction case
2. Add unit test in `tests/security/`
3. Fix vulnerability
4. Verify fix with fuzzing
5. Document in SECURITY.md

### 4. Corpus Seeding
Add known attack patterns to corpus:
```bash
mkdir -p fuzz/corpus/fuzz_validation
echo 'echo hello; rm -rf /' > fuzz/corpus/fuzz_validation/injection1
echo '$(whoami)' > fuzz/corpus/fuzz_validation/injection2
echo '`cat /etc/passwd`' > fuzz/corpus/fuzz_validation/injection3
```

## Troubleshooting

### Out of Memory
```bash
# Limit memory usage
cargo +nightly fuzz run <target> -- -rss_limit_mb=2048
```

### Slow Fuzzing
```bash
# Reduce input length
cargo +nightly fuzz run <target> -- -max_len=512

# Use faster sanitizer
RUSTFLAGS="" cargo +nightly fuzz run <target>
```

### No Crashes Found
- Good! But verify with longer runs (24+ hours)
- Check corpus diversity: `ls -la fuzz/corpus/<target>/`
- Seed corpus with attack patterns

## Integration with Security Testing

### Combined Security Workflow
1. **Static Analysis**: `cargo clippy` (SAST)
2. **Dependency Audit**: `cargo audit` (SCA)
3. **Fuzzing**: `cargo fuzz` (dynamic testing)
4. **Manual Review**: Code review + penetration testing
5. **Monitoring**: Runtime security monitoring

### Security Metrics
- **Zero Tolerance**: Shell injection, command injection
- **High Priority**: Memory safety, DoS, data corruption
- **Target**: 95%+ coverage on security-critical code

## Resources

- [cargo-fuzz Book](https://rust-fuzz.github.io/book/cargo-fuzz.html)
- [libFuzzer Documentation](https://llvm.org/docs/LibFuzzer.html)
- [OWASP Fuzzing Guide](https://owasp.org/www-community/Fuzzing)
- [Rust Fuzzing Authority](https://github.com/rust-fuzz)

## Contributing

When adding new fuzz targets:

1. Create target in `fuzz_targets/<name>.rs`
2. Add `[[bin]]` entry in `fuzz/Cargo.toml`
3. Update this README
4. Add to CI matrix in `.github/workflows/fuzzing.yml`
5. Run initial fuzzing (30+ minutes)
6. Document coverage and findings

## License

Same as cmdrun project (MIT)
