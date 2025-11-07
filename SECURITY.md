# Security Policy

## Supported Versions

We actively support the following versions of cmdrun with security updates:

| Version | Supported          |
| ------- | ------------------ |
| 1.0.x   | :white_check_mark: |
| < 1.0   | :x:                |

## Reporting a Vulnerability

**Please do not report security vulnerabilities through public GitHub issues.**

Instead, please report security vulnerabilities by email to:

**sanae.a.sunny@gmail.com**

### What to Include

Please include the following information in your report:

1. **Type of vulnerability** (e.g., shell injection, path traversal, etc.)
2. **Affected component** (e.g., variable interpolation, watch mode, etc.)
3. **Steps to reproduce** the vulnerability
4. **Potential impact** of the vulnerability
5. **Suggested fix** (if you have one)
6. **Your contact information** for follow-up questions

### What to Expect

- **Initial response**: Within 48 hours
- **Status update**: Within 7 days
- **Fix timeline**: Depends on severity
  - Critical: Within 7 days
  - High: Within 14 days
  - Medium: Within 30 days
  - Low: Next release

### Disclosure Policy

We follow **responsible disclosure** practices:

1. **Coordinated disclosure**: We will work with you to understand and fix the issue
2. **Public disclosure**: After the fix is released, we will publish a security advisory
3. **Credit**: We will credit you in the advisory (unless you prefer to remain anonymous)

### Security Measures

cmdrun implements multiple security layers:

#### 1. **Shell Injection Prevention**
- **shell-words** crate for safe command parsing
- No use of `eval()` or dynamic code execution
- Strict input validation

#### 2. **Path Traversal Protection**
- Path normalization and validation
- Restriction to allowed directories
- Symlink resolution with checks

#### 3. **Variable Expansion Safety**
- Safe variable interpolation (no arbitrary code execution)
- Environment variable validation
- Protection against malicious variable values

#### 4. **Dependency Security**
- Regular `cargo audit` scans (automated via CI)
- Minimal dependency tree
- Only trusted, well-maintained crates

#### 5. **Fuzzing (cargo-fuzz)**
- **Continuous fuzzing** via GitHub Actions (weekly)
- **4 fuzz targets** covering security-critical components:
  - Variable expansion (shell injection prevention)
  - Command validation (injection attack prevention)
  - TOML parsing (malformed input handling)
  - Command parsing utilities
- See [FUZZING_REPORT.md](FUZZING_REPORT.md) for details

#### 6. **Input Validation**
- TOML schema validation
- Command argument sanitization
- File path validation

## Security Best Practices for Users

### 1. **Review Command Definitions**
Always review commands before adding them to your configuration:

```toml
# ❌ Dangerous: User input without validation
[commands.deploy]
cmd = "ssh ${HOST} 'rm -rf /'" # NEVER DO THIS

# ✅ Safe: Validated input with defaults
[commands.deploy]
cmd = "ssh ${DEPLOY_HOST:?DEPLOY_HOST required} 'systemctl restart app'"
```

### 2. **Use Confirmation for Destructive Commands**
Add `confirm = true` for potentially dangerous operations:

```toml
[commands.cleanup]
cmd = "rm -rf ./temp"
confirm = true  # Asks for confirmation before execution
```

### 3. **Avoid Storing Secrets in Config**
**Never** store sensitive information in `commands.toml`:

```toml
# ❌ NEVER DO THIS
[commands.deploy]
env = { API_KEY = "secret123" }

# ✅ Use environment variables instead
[commands.deploy]
cmd = "deploy --key ${API_KEY:?API_KEY not set}"
```

### 4. **Use Non-Strict Mode Carefully**
Non-strict mode allows pipes and redirects but reduces security:

```toml
[config]
strict_mode = false  # Use only when necessary
```

### 5. **Keep cmdrun Updated**
Regularly update to the latest version:

```bash
cargo install cmdrun --force
```

## Security Audits

We conduct security audits:

- **Automated Testing**:
  - `cargo audit` in CI on every commit
  - `cargo fuzz` weekly fuzzing runs (Sunday 2:00 UTC)
- **Manual Testing**: Before each major release
- **External Reviews**: We welcome third-party security reviews
- **Fuzzing**: See [FUZZING_REPORT.md](FUZZING_REPORT.md) for detailed fuzzing results

## Known Security Considerations

### 1. **Shell Command Execution**
cmdrun executes shell commands as configured. While we prevent injection, the commands themselves can be dangerous if misconfigured.

**Mitigation**:
- Review all command definitions
- Use `confirm = true` for destructive commands
- Limit shell access in production environments

### 2. **File Watch Permissions**
Watch mode monitors file changes. Malicious files could trigger unintended command execution.

**Mitigation**:
- Use specific glob patterns (not `**/*`)
- Exclude untrusted directories
- Use `.gitignore` integration to exclude sensitive files

### 3. **TOML Parsing**
While TOML is a safe format, maliciously crafted configs could cause issues.

**Mitigation**:
- Validate config files with `cmdrun validate`
- Review configs from untrusted sources
- Use version control for config files

## Vulnerability History

No security vulnerabilities have been reported or discovered in cmdrun to date.

We will maintain this section with any future security advisories.

## Security Contact

**Email**: sanae.a.sunny@gmail.com
**GPG Key**: Available on request

---

**Last Updated**: 2025-11-07
**Version**: 1.0.0
