# cmdrun Troubleshooting Guide

This guide helps you diagnose and fix common issues with cmdrun.

## Table of Contents

- [Installation Issues](#installation-issues)
- [Configuration Issues](#configuration-issues)
- [Execution Issues](#execution-issues)
- [Watch Mode Issues](#watch-mode-issues)
- [Platform-Specific Issues](#platform-specific-issues)
- [Performance Issues](#performance-issues)
- [Error Messages](#error-messages)
- [Debugging Techniques](#debugging-techniques)

---

## Installation Issues

### Issue: "cmdrun: command not found" after installation

**Symptoms:**
```bash
$ cmdrun --version
cmdrun: command not found
```

**Cause:** The cargo bin directory is not in your PATH.

**Solution:**

1. **Verify installation:**
   ```bash
   ls -la ~/.cargo/bin/cmdrun
   ```

   If the file doesn't exist, reinstall:
   ```bash
   cd cmdrun
   cargo install --path . --force
   ```

2. **Add cargo bin to PATH:**

   **Linux/macOS (bash):**
   ```bash
   echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.bashrc
   source ~/.bashrc
   ```

   **Linux/macOS (zsh):**
   ```bash
   echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.zshrc
   source ~/.zshrc
   ```

   **Windows (PowerShell):**
   ```powershell
   $env:PATH += ";$env:USERPROFILE\.cargo\bin"
   # Make permanent:
   [Environment]::SetEnvironmentVariable("PATH", $env:PATH, "User")
   ```

3. **Verify:**
   ```bash
   cmdrun --version
   ```

---

### Issue: Rust compilation fails

**Symptoms:**
```
error: could not compile `cmdrun`
```

**Causes & Solutions:**

1. **Outdated Rust version:**
   ```bash
   # Check version
   rustc --version

   # Update Rust (must be 1.75+)
   rustup update stable
   ```

2. **Missing dependencies:**

   **Debian/Ubuntu:**
   ```bash
   sudo apt update
   sudo apt install build-essential pkg-config libssl-dev
   ```

   **Fedora/RHEL:**
   ```bash
   sudo dnf install gcc pkg-config openssl-devel
   ```

   **macOS:**
   ```bash
   xcode-select --install
   ```

3. **Disk space:**
   ```bash
   # Check available space (needs ~1GB for build)
   df -h .

   # Clean cargo cache if needed
   cargo clean
   ```

---

## Configuration Issues

### Issue: Configuration file not found

**Symptoms:**
```
Error: Configuration file not found at ~/.config/cmdrun/commands.toml
```

**Solutions:**

1. **Create default config:**
   ```bash
   # Linux/macOS
   mkdir -p ~/.config/cmdrun
   touch ~/.config/cmdrun/commands.toml

   # Windows (PowerShell)
   New-Item -ItemType Directory -Force -Path "$env:APPDATA\cmdrun"
   New-Item -ItemType File -Force -Path "$env:APPDATA\cmdrun\commands.toml"
   ```

2. **Let cmdrun create it:**
   ```bash
   cmdrun add test "echo test" "Test command"
   ```

3. **Use custom config:**
   ```bash
   cmdrun -c ./commands.toml list
   ```

---

### Issue: "Failed to parse TOML"

**Symptoms:**
```
Error: Failed to parse configuration file
  Caused by: TOML parse error at line 5, column 10
```

**Common TOML Syntax Errors:**

1. **Missing quotes:**
   ```toml
   # ❌ Wrong
   [commands.dev]
   cmd = npm run dev

   # ✅ Correct
   [commands.dev]
   cmd = "npm run dev"
   ```

2. **Invalid table name:**
   ```toml
   # ❌ Wrong
   [commands.my command]

   # ✅ Correct
   [commands."my-command"]
   # or
   [commands.my_command]
   ```

3. **Array syntax:**
   ```toml
   # ❌ Wrong
   cmd = ["npm test" "npm build"]

   # ✅ Correct
   cmd = ["npm test", "npm build"]
   ```

4. **Mixed types:**
   ```toml
   # ❌ Wrong
   deps = "test"  # Should be array

   # ✅ Correct
   deps = ["test"]
   ```

**Validation:**
```bash
# Validate config syntax
cmdrun config show

# Or use online TOML validator
# https://www.toml-lint.com/
```

---

### Issue: Circular dependency detected

**Symptoms:**
```
Error: Circular dependency detected: deploy -> build -> test -> deploy
```

**Cause:** Commands depend on each other in a cycle.

**Example Problem:**
```toml
[commands.deploy]
deps = ["build"]

[commands.build]
deps = ["test"]

[commands.test]
deps = ["deploy"]  # ❌ Circular!
```

**Solution:** Remove circular dependency:
```toml
[commands.deploy]
deps = ["build"]

[commands.build]
deps = ["test"]

[commands.test]
cmd = "cargo test"
# No deps - breaks the cycle
```

**Debugging:**
```bash
# Visualize dependency graph (if available)
cmdrun graph deploy

# Or manually trace dependencies
cmdrun info deploy
cmdrun info build
cmdrun info test
```

---

## Execution Issues

### Issue: Command not found in config

**Symptoms:**
```
Error: Command 'dev' not found
```

**Solutions:**

1. **List available commands:**
   ```bash
   cmdrun list
   ```

2. **Check config file:**
   ```bash
   cmdrun config show
   ```

3. **Check spelling:**
   ```bash
   # Search for similar commands
   cmdrun search de
   ```

4. **Add command:**
   ```bash
   cmdrun add dev "npm run dev" "Development server"
   ```

---

### Issue: Variable expansion doesn't work

**Symptoms:**
```bash
$ cmdrun run deploy
# Output: rsync dist/ ${DEPLOY_USER}@${DEPLOY_HOST}:/var/www
# Variables not expanded!
```

**Causes & Solutions:**

1. **Environment variable not set:**
   ```bash
   # Check if variable exists
   echo $DEPLOY_USER

   # Set it
   export DEPLOY_USER="admin"
   export DEPLOY_HOST="server.example.com"

   # Run again
   cmdrun run deploy
   ```

2. **Use default values:**
   ```toml
   [commands.deploy]
   cmd = "rsync dist/ ${DEPLOY_USER:-admin}@${DEPLOY_HOST:-localhost}:/var/www"
   ```

3. **Use required variables for better errors:**
   ```toml
   [commands.deploy]
   cmd = "rsync dist/ ${DEPLOY_USER:?DEPLOY_USER not set}@${DEPLOY_HOST:?DEPLOY_HOST not set}:/var/www"
   ```

   Now you'll get a clear error:
   ```
   Error: Required variable 'DEPLOY_USER' is not set: DEPLOY_USER not set
   ```

4. **Check strict mode:**
   ```toml
   [config]
   strict_mode = false  # Undefined vars become empty string
   ```

---

### Issue: Positional arguments don't work

**Symptoms:**
```bash
$ cmdrun run convert input.png jpeg
# ${1} and ${2} not replaced
```

**Solutions:**

1. **Check syntax:**
   ```toml
   # ✅ Correct
   [commands.convert]
   cmd = "sharp -i ${1} -f ${2:-webp}"

   # ❌ Wrong (shell-style $1)
   cmd = "sharp -i $1 -f $2"
   ```

2. **Pass arguments after command name:**
   ```bash
   cmdrun run convert input.png jpeg 90
   #             ^command ^args
   ```

3. **Debug variable expansion:**
   ```bash
   # Enable verbose mode to see expanded command
   cmdrun -v run convert input.png jpeg
   ```

---

### Issue: Permission denied

**Symptoms:**
```
Error: Permission denied (os error 13)
```

**Solutions:**

1. **Script not executable:**
   ```bash
   # Make script executable
   chmod +x script.sh

   # Or use shell directly
   cmdrun run script
   # where cmd = "bash script.sh"
   ```

2. **Working directory permissions:**
   ```toml
   [commands.example]
   cmd = "ls"
   working_dir = "/root"  # ❌ No permission
   ```

   Fix:
   ```toml
   working_dir = "."  # Current directory
   ```

3. **File path issues:**
   ```bash
   # Use absolute paths
   [commands.script]
   cmd = "/home/user/scripts/deploy.sh"

   # Or expand home directory
   cmd = "~/scripts/deploy.sh"
   ```

---

### Issue: Command timeout

**Symptoms:**
```
Error: Command timed out after 300 seconds
```

**Solutions:**

1. **Increase timeout:**
   ```toml
   [commands.long-task]
   cmd = "npm run build"
   timeout = 600  # 10 minutes
   ```

2. **Set global timeout:**
   ```toml
   [config]
   timeout = 600  # Default for all commands
   ```

3. **Disable timeout (not recommended):**
   ```toml
   [commands.infinite-task]
   cmd = "watch-forever.sh"
   # timeout = 0  # No timeout (use with caution!)
   ```

---

## Watch Mode Issues

### Issue: Watch mode not detecting file changes

**Symptoms:**
File changes don't trigger command execution.

**Solutions:**

1. **Check glob pattern:**
   ```bash
   # ❌ Wrong (non-recursive)
   cmdrun watch build --pattern "*.rs"

   # ✅ Correct (recursive)
   cmdrun watch build --pattern "**/*.rs"
   ```

2. **Check .gitignore:**
   Watch mode respects `.gitignore` by default.

   ```bash
   # To watch ignored files
   cmdrun watch build --pattern "**/*.rs" --no-gitignore

   # Or modify .gitignore
   ```

3. **Check exclusions:**
   ```bash
   # Default exclusions: .git, target, node_modules, etc.
   # To watch anyway:
   cmdrun watch build --pattern "**/*.rs" --no-default-ignore
   ```

4. **Verify path exists:**
   ```bash
   # Check path argument
   cmdrun watch build --path src --pattern "**/*.rs"
   #                        ^^^
   # Make sure 'src' exists
   ```

---

### Issue: Watch mode triggers too many times

**Symptoms:**
Command executes multiple times for single file save.

**Cause:** Editors create multiple file system events (write, modify, close).

**Solutions:**

1. **Increase debounce:**
   ```bash
   # Default: 500ms
   cmdrun watch build --pattern "**/*.rs" --debounce 1000
   #                                                  ^^^^
   #                                                  1 second
   ```

2. **Use gitignore to exclude temp files:**
   ```gitignore
   # .gitignore
   *.swp        # Vim
   *.tmp        # Temp files
   *~           # Backup files
   .#*          # Emacs
   ```

---

### Issue: High CPU usage in watch mode

**Symptoms:**
cmdrun watch consumes lots of CPU.

**Solutions:**

1. **Reduce watch scope:**
   ```bash
   # ❌ Watching too much
   cmdrun watch build --pattern "**/*"

   # ✅ Watch specific files
   cmdrun watch build --pattern "**/*.rs"
   ```

2. **Exclude large directories:**
   ```bash
   cmdrun watch build \
     --pattern "**/*.rs" \
     --ignore "**/target/**" \
     --ignore "**/node_modules/**"
   ```

3. **Use default ignores:**
   ```bash
   # Default ignores are optimized for performance
   cmdrun watch build --pattern "**/*.rs"
   # (automatically excludes .git, target, etc.)
   ```

---

## Platform-Specific Issues

### Linux Issues

#### Issue: "inotify watch limit reached"

**Symptoms:**
```
Error: Failed to watch directory: inotify watch limit reached
```

**Solution:**
```bash
# Check current limit
cat /proc/sys/fs/inotify/max_user_watches

# Increase limit (temporary)
sudo sysctl fs.inotify.max_user_watches=524288

# Increase limit (permanent)
echo "fs.inotify.max_user_watches=524288" | sudo tee -a /etc/sysctl.conf
sudo sysctl -p
```

---

### macOS Issues

#### Issue: "Operation not permitted" on Catalina+

**Symptoms:**
```
Error: Operation not permitted (os error 1)
```

**Cause:** macOS security (System Integrity Protection).

**Solution:**
```bash
# Grant Full Disk Access to Terminal
# System Preferences > Security & Privacy > Privacy > Full Disk Access
# Add Terminal or your terminal app
```

#### Issue: "xcrun: error: invalid active developer path"

**Symptoms:**
Build fails with Xcode error.

**Solution:**
```bash
# Install Xcode Command Line Tools
xcode-select --install
```

---

### Windows Issues

#### Issue: PowerShell execution policy

**Symptoms:**
```
cmdrun : File cannot be loaded because running scripts is disabled
```

**Solution:**
```powershell
# Check policy
Get-ExecutionPolicy

# Set policy (run as Administrator)
Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope CurrentUser
```

#### Issue: Path with spaces

**Symptoms:**
Commands with spaces in paths fail.

**Solution:**
```toml
# ❌ Wrong
[commands.example]
cmd = "C:\\Program Files\\app\\run.exe"

# ✅ Correct (escape or quote)
cmd = "\"C:\\Program Files\\app\\run.exe\""

# Or use forward slashes
cmd = "C:/Program Files/app/run.exe"
```

#### Issue: Line ending issues (CRLF vs LF)

**Symptoms:**
Scripts fail with "command not found" on Windows.

**Solution:**
```bash
# Convert CRLF to LF
dos2unix script.sh

# Or configure git
git config --global core.autocrlf input
```

---

## Performance Issues

### Issue: Slow startup time

**Symptoms:**
cmdrun takes >100ms to start.

**Expected:** ~4ms startup time.

**Debugging:**

1. **Measure:**
   ```bash
   time cmdrun --version
   ```

2. **Check binary size:**
   ```bash
   ls -lh ~/.cargo/bin/cmdrun
   # Should be ~5MB or less
   ```

3. **Rebuild with optimization:**
   ```bash
   cd cmdrun
   cargo clean
   cargo build --release
   cargo install --path . --force
   ```

4. **Check for debug build:**
   ```bash
   # ❌ Debug build (slow)
   cargo install --path . --debug

   # ✅ Release build (fast)
   cargo install --path .
   ```

---

### Issue: Slow config loading

**Symptoms:**
Large config file loads slowly.

**Solutions:**

1. **Split config files:**
   ```bash
   # Instead of one large file
   # Use multiple smaller configs
   cmdrun -c ~/work/commands.toml list
   cmdrun -c ~/personal/commands.toml list
   ```

2. **Remove unused commands:**
   ```bash
   cmdrun remove old-command
   ```

3. **Optimize TOML:**
   ```toml
   # ❌ Redundant
   [commands.test1]
   cmd = "cargo test"

   [commands.test2]
   cmd = "cargo test"

   # ✅ Use one with different name
   [commands.test]
   cmd = "cargo test"
   ```

---

## Error Messages

### "Required variable not set"

**Full Error:**
```
Error: Required variable 'API_KEY' is not set: API_KEY environment variable required
```

**Solution:**
```bash
# Set the environment variable
export API_KEY="your-api-key"

# Or use default value in config
[commands.api-call]
cmd = "curl -H 'Authorization: Bearer ${API_KEY:-test-key}' ..."
```

---

### "Invalid command ID"

**Full Error:**
```
Error: Invalid command ID 'my command'. Must contain only alphanumeric, underscore, or hyphen.
```

**Solution:**
```bash
# ❌ Wrong (contains space)
cmdrun add "my command" "echo test"

# ✅ Correct
cmdrun add my-command "echo test"
cmdrun add my_command "echo test"
```

---

### "Path traversal detected"

**Full Error:**
```
Error: Path traversal detected: ../../etc/passwd
```

**Cause:** Security protection against directory traversal attacks.

**Solution:**
Use absolute paths or paths within current directory:
```toml
# ❌ Wrong
[commands.read]
working_dir = "../../etc"

# ✅ Correct
working_dir = "/etc"  # Absolute path
# or
working_dir = "./config"  # Relative to current dir
```

---

## Debugging Techniques

### Enable verbose output

```bash
# Verbose mode
cmdrun -v run your-command
cmdrun --verbose run your-command
```

**Output includes:**
- Parsed configuration
- Resolved dependencies
- Expanded variables
- Execution details

---

### Enable trace logging

```bash
# Set log level
RUST_LOG=trace cmdrun run your-command
RUST_LOG=debug cmdrun run your-command
RUST_LOG=info cmdrun run your-command
```

**Levels:**
- `trace` - Everything (very verbose)
- `debug` - Debug information
- `info` - General information
- `warn` - Warnings only
- `error` - Errors only

---

### Get detailed error information

```bash
# Full backtrace
RUST_BACKTRACE=1 cmdrun run your-command
RUST_BACKTRACE=full cmdrun run your-command
```

**Example output:**
```
Error: Command execution failed
   0: cmdrun::command::executor::execute
             at src/command/executor.rs:45
   1: cmdrun::commands::run::handle_run
             at src/commands/run.rs:23
   ... (full stack trace)
```

---

### Validate configuration

```bash
# Show parsed configuration
cmdrun config show

# Validate specific command
cmdrun info your-command

# List all commands (check for duplicates)
cmdrun list
```

---

### Test command interpolation

```bash
# Dry-run mode (planned feature)
# For now, use echo:
[commands.test-vars]
cmd = "echo ${VAR1} ${VAR2:-default}"

cmdrun run test-vars
```

---

### Check file permissions

```bash
# Verify working directory
ls -la $(pwd)

# Verify script permissions
ls -la script.sh

# Verify config file
ls -la ~/.config/cmdrun/commands.toml
```

---

### Verify environment

```bash
# Check environment variables
env | grep CMDRUN
printenv

# Check shell
echo $SHELL

# Check PATH
echo $PATH
```

---

### Use strace/dtrace (advanced)

**Linux (strace):**
```bash
strace -e trace=file cmdrun run your-command 2>&1 | grep ENOENT
```

**macOS (dtrace):**
```bash
sudo dtruss -f cmdrun run your-command
```

---

## Getting Help

If you can't find a solution here:

1. **Check documentation:**
   - [FAQ](FAQ.md)
   - [Configuration Reference](CONFIGURATION.md)
   - [CLI Reference](CLI.md)

2. **Search GitHub Issues:**
   https://github.com/sanae-abe/cmdrun/issues

3. **Create a new issue:**
   Include:
   - cmdrun version (`cmdrun --version`)
   - OS and version
   - Full error message
   - Config file (sanitized)
   - Steps to reproduce

4. **Community support:**
   - GitHub Discussions: https://github.com/sanae-abe/cmdrun/discussions

---

## Reporting Bugs

**Before reporting:**
- [ ] Update to latest version
- [ ] Check if issue already exists
- [ ] Try with minimal config

**Include in bug report:**
```bash
# cmdrun version
cmdrun --version

# OS information
uname -a  # Linux/macOS
# or
winver    # Windows

# Rust version (if building from source)
rustc --version

# Full error output
RUST_BACKTRACE=1 cmdrun run command 2>&1 | tee error.log

# Config file (remove sensitive data)
cat ~/.config/cmdrun/commands.toml
```

**Template:**
```markdown
## Environment
- cmdrun version: 1.0.0
- OS: Ubuntu 22.04
- Shell: bash 5.1

## Steps to Reproduce
1. Create config with...
2. Run `cmdrun run ...`
3. Observe error...

## Expected Behavior
Should execute command successfully

## Actual Behavior
Error: ...

## Config File
```toml
[commands.example]
cmd = "..."
```

## Additional Context
(Any other relevant information)
```

---

**Last Updated:** 2025-11-07
**Version:** 1.0.0
