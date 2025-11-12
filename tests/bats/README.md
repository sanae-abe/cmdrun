# cmdrun BATS Security Tests

Shell-based integration tests for cmdrun security validation using BATS (Bash Automated Testing System).

## Overview

This directory contains BATS tests that validate cmdrun's security features, including:
- Shell injection prevention
- Null byte injection protection
- Path traversal attack mitigation
- Long input handling (buffer overflow prevention)

## Prerequisites

### Install BATS

**macOS:**
```bash
brew install bats-core
```

**Ubuntu/Debian:**
```bash
sudo apt-get install bats
```

**From Source:**
```bash
git clone https://github.com/bats-core/bats-core.git
cd bats-core
sudo ./install.sh /usr/local
```

### Build cmdrun

The tests expect the cmdrun binary at `./target/release/cmdrun`:

```bash
cargo build --release
```

## Running Tests

### Quick Start

Run all security tests:

```bash
./tests/bats/run-tests.sh
```

### Manual Execution

Run BATS directly:

```bash
bats tests/bats/security.bats
```

Run with verbose output:

```bash
bats --verbose tests/bats/security.bats
```

Run specific test by line number:

```bash
bats tests/bats/security.bats:30  # Run test starting at line 30
```

## Test Details

### security.bats (4 tests)

| Test ID | Description | OWASP Category | Severity |
|---------|-------------|----------------|----------|
| security-001 | Command injection prevention | A03:2021 - Injection | Critical |
| security-002 | Null byte injection protection | A03:2021 - Injection | Critical |
| security-003 | Path traversal mitigation | A01:2021 - Broken Access Control | Critical |
| security-004 | Long input handling | A04:2021 - Insecure Design | Informational |

#### Test 1: Command Injection Prevention
```bash
cmdrun --config 'test; rm -rf /'
# Expected: Non-zero exit code (rejection)
```

Validates that cmdrun rejects attempts to inject shell commands through option values.

#### Test 2: Null Byte Injection Protection
```bash
cmdrun --config $'/tmp/test\x00malicious'
# Expected: Non-zero exit code (rejection)
```

Ensures null byte characters in file paths are properly rejected.

#### Test 3: Path Traversal Mitigation
```bash
cmdrun --config ../../../etc/passwd
# Expected: Non-zero exit code (rejection)
```

Verifies that path traversal attempts are blocked.

#### Test 4: Long Input Handling
```bash
cmdrun list --config 'AAAA...(10000 chars)...'
# Expected: Zero exit code (graceful handling)
```

Confirms cmdrun can handle extremely long input without crashing (buffer overflow prevention).

## CI/CD Integration

### GitHub Actions Example

```yaml
name: Security Tests

on: [push, pull_request]

jobs:
  security-tests:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Install BATS
        run: sudo apt-get install -y bats
      - name: Install Rust
        uses: actions-rust-lang/setup-rust-toolchain@v1
      - name: Build cmdrun
        run: cargo build --release
      - name: Run security tests
        run: ./tests/bats/run-tests.sh
```

### GitLab CI Example

```yaml
security-tests:
  stage: test
  image: rust:latest
  before_script:
    - apt-get update && apt-get install -y bats
    - cargo build --release
  script:
    - ./tests/bats/run-tests.sh
```

## Expected Output

### Success (100% pass rate)

```
=== cmdrun BATS Security Test Runner ===

Running security tests...

 ✓ [security] Reject command injection in option value
 ✓ [security] Reject null byte in option value
 ✓ [security] Reject path traversal attempt
 ✓ [security] Handle extremely long input

4 tests, 0 failures

✅ All security tests passed!
```

### Failure Example

```
 ✗ [security] Reject command injection in option value
   (in test file tests/bats/security.bats, line 38)
     `[ "$status" -ne 0 ]' failed

1 test, 1 failure

❌ Some security tests failed
```

## Troubleshooting

### Binary Not Found

**Error:**
```
/usr/bin/env: 'cmdrun': No such file or directory
```

**Solution:**
```bash
cargo build --release
```

### BATS Not Installed

**Error:**
```
bash: bats: command not found
```

**Solution:**
```bash
# macOS
brew install bats-core

# Ubuntu/Debian
sudo apt-get install bats
```

### Permission Denied

**Error:**
```
bash: ./tests/bats/run-tests.sh: Permission denied
```

**Solution:**
```bash
chmod +x tests/bats/run-tests.sh
```

## Extending Tests

### Adding New Security Tests

1. Edit `tests/bats/security.bats`
2. Add new `@test` block:

```bash
@test "[security] Your test description" {
    # Test ID: security-005
    # Tags: category, severity

    # Execute command
    run cmdrun <your-test-command>

    # Assert exit code
    [ "$status" -ne 0 ]  # For rejection tests
    # OR
    [ "$status" -eq 0 ]  # For acceptance tests
}
```

3. Run tests to verify:

```bash
./tests/bats/run-tests.sh
```

### Creating New Test Suites

1. Create new BATS file: `tests/bats/your-suite.bats`
2. Copy the setup/teardown template from `security.bats`
3. Add tests following BATS syntax
4. Update `run-tests.sh` to include new suite:

```bash
# In run-tests.sh, modify the bats command:
bats tests/bats/*.bats
```

## References

- [BATS Documentation](https://bats-core.readthedocs.io/)
- [OWASP Top 10 2021](https://owasp.org/Top10/)
- [cmdrun Security Guidelines](../../docs/technical/SECURITY.md)
- [Rust Integration Tests](../comprehensive_behavior_test.rs)

## Related Files

- `tests/comprehensive_behavior_test.rs` - Rust-based integration tests
- `docs/MANUAL_TESTING_GUIDE.md` - Manual testing procedures
- `src/security/validation.rs` - Security validation implementation
- `.github/workflows/` - CI/CD configuration (if exists)

## Maintenance

These tests should be updated when:
- New security features are added
- Security vulnerabilities are discovered
- cmdrun's CLI interface changes
- OWASP guidelines are updated

Last updated: 2025-01-12
