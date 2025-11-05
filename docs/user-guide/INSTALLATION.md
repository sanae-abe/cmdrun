# Installation Guide

This guide covers all available methods to install `cmdrun` on various platforms.

## Table of Contents

- [Quick Install](#quick-install)
- [Installation Methods](#installation-methods)
  - [Cargo (Recommended)](#cargo-recommended)
  - [Homebrew (macOS/Linux)](#homebrew-macoslinux)
  - [Scoop (Windows)](#scoop-windows)
  - [Manual Installation](#manual-installation)
  - [Building from Source](#building-from-source)
- [Shell Completion Setup](#shell-completion-setup)
- [Verification](#verification)
- [Troubleshooting](#troubleshooting)
- [Updating](#updating)
- [Uninstallation](#uninstallation)

---

## Quick Install

### macOS/Linux
```bash
# Using Cargo (Rust package manager)
cargo install cmdrun

# Using Homebrew (planned)
brew install sanae-abe/tap/cmdrun
```

### Windows
```powershell
# Using Cargo
cargo install cmdrun

# Using Scoop (planned)
scoop bucket add cmdrun https://github.com/sanae-abe/scoop-bucket
scoop install cmdrun
```

---

## Installation Methods

### Cargo (Recommended)

The easiest and most reliable way to install `cmdrun` is via Cargo, Rust's package manager.

#### Prerequisites
- Rust toolchain (rustc 1.70.0 or later)
- Cargo (usually comes with Rust)

#### Install Rust
If you don't have Rust installed:

**macOS/Linux:**
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

**Windows:**
Download and run [rustup-init.exe](https://rustup.rs/)

#### Install cmdrun
```bash
cargo install cmdrun
```

**Installation details:**
- Binary location: `~/.cargo/bin/cmdrun` (Unix) or `%USERPROFILE%\.cargo\bin\cmdrun.exe` (Windows)
- Automatically added to PATH during Rust installation
- Typical install time: 1-3 minutes (depending on system)

#### Install specific version
```bash
# Install latest version
cargo install cmdrun

# Install specific version
cargo install cmdrun --version 0.1.0

# Force reinstall
cargo install cmdrun --force
```

---

### Homebrew (macOS/Linux)

**Status:** Planned for future release

Homebrew will provide the easiest installation experience for macOS and Linux users.

#### Planned Usage
```bash
# Add tap (repository)
brew tap sanae-abe/tap

# Install cmdrun
brew install cmdrun

# Update
brew upgrade cmdrun
```

**Current workaround:** Use [Cargo installation](#cargo-recommended) or [manual installation](#manual-installation).

---

### Scoop (Windows)

**Status:** Planned for future release

Scoop will provide a native Windows package manager experience.

#### Planned Usage
```powershell
# Add bucket (repository)
scoop bucket add cmdrun https://github.com/sanae-abe/scoop-bucket

# Install cmdrun
scoop install cmdrun

# Update
scoop update cmdrun
```

**Current workaround:** Use [Cargo installation](#cargo-recommended) or [manual installation](#manual-installation).

---

### Manual Installation

Download pre-built binaries from the [Releases page](https://github.com/sanae-abe/cmdrun/releases).

#### macOS (Apple Silicon)
```bash
# Download latest release
curl -LO https://github.com/sanae-abe/cmdrun/releases/latest/download/cmdrun-aarch64-apple-darwin.tar.gz

# Extract
tar xzf cmdrun-aarch64-apple-darwin.tar.gz

# Move to PATH
sudo mv cmdrun /usr/local/bin/

# Verify
cmdrun --version
```

#### macOS (Intel)
```bash
# Download latest release
curl -LO https://github.com/sanae-abe/cmdrun/releases/latest/download/cmdrun-x86_64-apple-darwin.tar.gz

# Extract
tar xzf cmdrun-x86_64-apple-darwin.tar.gz

# Move to PATH
sudo mv cmdrun /usr/local/bin/

# Verify
cmdrun --version
```

#### Linux (x86_64)
```bash
# Download latest release
curl -LO https://github.com/sanae-abe/cmdrun/releases/latest/download/cmdrun-x86_64-unknown-linux-gnu.tar.gz

# Extract
tar xzf cmdrun-x86_64-unknown-linux-gnu.tar.gz

# Move to PATH
sudo mv cmdrun /usr/local/bin/

# Verify
cmdrun --version
```

#### Linux (ARM64)
```bash
# Download latest release
curl -LO https://github.com/sanae-abe/cmdrun/releases/latest/download/cmdrun-aarch64-unknown-linux-gnu.tar.gz

# Extract
tar xzf cmdrun-aarch64-unknown-linux-gnu.tar.gz

# Move to PATH
sudo mv cmdrun /usr/local/bin/

# Verify
cmdrun --version
```

#### Windows (x86_64)
```powershell
# Download from releases page manually or use:
Invoke-WebRequest -Uri "https://github.com/sanae-abe/cmdrun/releases/latest/download/cmdrun-x86_64-pc-windows-msvc.zip" -OutFile "cmdrun.zip"

# Extract
Expand-Archive -Path cmdrun.zip -DestinationPath .

# Move to a directory in PATH (e.g., C:\Program Files\cmdrun\)
Move-Item cmdrun.exe "C:\Program Files\cmdrun\cmdrun.exe"

# Add to PATH (PowerShell as Administrator)
$env:Path += ";C:\Program Files\cmdrun"
[Environment]::SetEnvironmentVariable("Path", $env:Path, [EnvironmentVariableTarget]::Machine)

# Verify
cmdrun --version
```

---

### Building from Source

For developers or users who want the latest features from the main branch.

#### Prerequisites
- Rust toolchain (rustc 1.70.0 or later)
- Git

#### Clone and Build
```bash
# Clone repository
git clone https://github.com/sanae-abe/cmdrun.git
cd cmdrun

# Build in release mode (optimized)
cargo build --release

# Binary will be at: ./target/release/cmdrun
./target/release/cmdrun --version

# Install to ~/.cargo/bin/
cargo install --path .
```

#### Build with Specific Features
```bash
# Build with all features
cargo build --release --all-features

# Build with minimal features
cargo build --release --no-default-features
```

#### Development Build
```bash
# Build in debug mode (faster compilation, slower runtime)
cargo build

# Binary will be at: ./target/debug/cmdrun
./target/debug/cmdrun --version
```

---

## Shell Completion Setup

Enable shell completion for a better CLI experience.

### Bash

```bash
# Generate completion script
cmdrun completions bash > ~/.local/share/bash-completion/completions/cmdrun

# Or add to ~/.bashrc
eval "$(cmdrun completions bash)"
```

### Zsh

```bash
# Generate completion script
cmdrun completions zsh > "${fpath[1]}/_cmdrun"

# Or add to ~/.zshrc
eval "$(cmdrun completions zsh)"
```

### Fish

```bash
# Generate completion script
cmdrun completions fish > ~/.config/fish/completions/cmdrun.fish

# Reload completions
source ~/.config/fish/completions/cmdrun.fish
```

### PowerShell

```powershell
# Add to PowerShell profile
cmdrun completions powershell | Out-String | Invoke-Expression

# Or save to profile
cmdrun completions powershell >> $PROFILE
```

**Note:** Shell completion generation is planned for a future release. The above commands are examples of the planned functionality.

---

## Verification

After installation, verify that `cmdrun` is working correctly:

### Check Version
```bash
cmdrun --version
# Expected output: cmdrun 0.1.0 (or current version)
```

### Check Installation Path
```bash
# Unix-like systems
which cmdrun
# Expected: /usr/local/bin/cmdrun or ~/.cargo/bin/cmdrun

# Windows
where.exe cmdrun
# Expected: C:\Users\YourName\.cargo\bin\cmdrun.exe or similar
```

### Test Basic Functionality
```bash
# Show help
cmdrun --help

# List commands (will show empty if no commands.toml exists)
cmdrun list
```

### Create Test Project
```bash
# Create a test directory
mkdir cmdrun-test && cd cmdrun-test

# Create a simple commands.toml
cat > commands.toml << 'EOF'
[commands.hello]
description = "Say hello"
cmd = "echo 'Hello from cmdrun!'"
EOF

# Run the command
cmdrun run hello
# Expected output: Hello from cmdrun!
```

---

## Troubleshooting

### Command Not Found

**Issue:** `cmdrun: command not found` or `'cmdrun' is not recognized`

**Solutions:**

1. **Check PATH:**
   ```bash
   # Unix-like
   echo $PATH | grep -o "[^:]*cargo[^:]*"

   # Windows PowerShell
   $env:Path -split ';' | Select-String cargo
   ```

2. **Add Cargo bin to PATH:**
   ```bash
   # Bash/Zsh (~/.bashrc or ~/.zshrc)
   export PATH="$HOME/.cargo/bin:$PATH"

   # Fish (~/.config/fish/config.fish)
   set -gx PATH $HOME/.cargo/bin $PATH

   # Windows PowerShell (run as Administrator)
   $env:Path += ";$env:USERPROFILE\.cargo\bin"
   [Environment]::SetEnvironmentVariable("Path", $env:Path, [EnvironmentVariableTarget]::User)
   ```

3. **Restart shell:**
   ```bash
   # Unix-like
   exec $SHELL

   # Windows: Close and reopen PowerShell/Command Prompt
   ```

### Cargo Installation Fails

**Issue:** Compilation errors during `cargo install cmdrun`

**Solutions:**

1. **Update Rust toolchain:**
   ```bash
   rustup update stable
   cargo install cmdrun
   ```

2. **Check Rust version:**
   ```bash
   rustc --version
   # Should be 1.70.0 or later
   ```

3. **Clear Cargo cache:**
   ```bash
   cargo clean
   rm -rf ~/.cargo/registry/cache
   cargo install cmdrun
   ```

4. **Install with verbose output:**
   ```bash
   cargo install cmdrun --verbose
   # Review error messages for specific issues
   ```

### Permission Denied

**Issue:** Permission errors when moving binary to system directory

**Solutions:**

1. **Use sudo (Unix-like):**
   ```bash
   sudo mv cmdrun /usr/local/bin/
   ```

2. **Use user directory:**
   ```bash
   # Create user bin directory
   mkdir -p ~/.local/bin
   mv cmdrun ~/.local/bin/

   # Add to PATH
   export PATH="$HOME/.local/bin:$PATH"
   ```

3. **Windows: Run as Administrator**
   - Right-click PowerShell → Run as Administrator

### Binary Doesn't Execute

**Issue:** Binary downloaded but won't run

**Solutions:**

1. **Add execute permission (Unix-like):**
   ```bash
   chmod +x cmdrun
   ```

2. **Check file type:**
   ```bash
   file cmdrun
   # Should show: ELF 64-bit executable (Linux) or Mach-O executable (macOS)
   ```

3. **Verify download integrity:**
   ```bash
   # Download checksum file
   curl -LO https://github.com/sanae-abe/cmdrun/releases/latest/download/checksums.txt

   # Verify (macOS/Linux)
   sha256sum -c checksums.txt
   ```

4. **Windows: Unblock file**
   ```powershell
   Unblock-File -Path cmdrun.exe
   ```

### Slow Performance

**Issue:** cmdrun runs slower than expected

**Solutions:**

1. **Verify release build:**
   ```bash
   # If built from source, ensure release mode was used
   cargo build --release
   ```

2. **Check for debug symbols:**
   ```bash
   # Strip debug symbols for smaller, faster binary
   strip target/release/cmdrun
   ```

3. **Update to latest version:**
   ```bash
   cargo install cmdrun --force
   ```

---

## Updating

### Cargo Installation
```bash
# Update to latest version
cargo install cmdrun --force
```

### Homebrew (Planned)
```bash
brew upgrade cmdrun
```

### Scoop (Planned)
```powershell
scoop update cmdrun
```

### Manual Installation
1. Download the latest release
2. Replace existing binary
3. Verify version: `cmdrun --version`

---

## Uninstallation

### Cargo Installation
```bash
cargo uninstall cmdrun
```

### Homebrew (Planned)
```bash
brew uninstall cmdrun
brew untap sanae-abe/tap
```

### Scoop (Planned)
```powershell
scoop uninstall cmdrun
scoop bucket rm cmdrun
```

### Manual Installation
```bash
# Unix-like
sudo rm /usr/local/bin/cmdrun

# Windows
Remove-Item "C:\Program Files\cmdrun\cmdrun.exe"
```

### Remove Configuration (Optional)
```bash
# Remove all cmdrun-related files (be careful!)
rm -rf ~/.cmdrun
rm commands.toml  # In project directories
```

---

## Next Steps

After installation:

1. **Read the [Quick Start Guide](../README.md#quick-start)** to create your first `commands.toml`
2. **Check the [Configuration Reference](../technical/CONFIGURATION.md)** for advanced features
3. **Explore [CLI Reference](../technical/CLI.md)** for all available commands
4. **Join [Discussions](https://github.com/sanae-abe/cmdrun/discussions)** for help and tips

---

## Platform-Specific Notes

### macOS
- **Gatekeeper:** First run may require: System Preferences → Security & Privacy → Allow
- **Homebrew location:** Intel: `/usr/local/bin/`, Apple Silicon: `/opt/homebrew/bin/`

### Linux
- **SELinux:** May require: `chcon -t bin_t /usr/local/bin/cmdrun`
- **AppArmor:** Should work without configuration

### Windows
- **Antivirus:** May flag first-time execution (false positive)
- **PATH order:** Ensure Cargo bin directory is before other conflicting paths
- **PowerShell execution policy:** May need `Set-ExecutionPolicy RemoteSigned`

### FreeBSD
- **Installation:** Use Cargo method (package planned for future release)
- **Shell:** Default shell is `sh`, may need to configure in `commands.toml`

---

**Need help?** See [Troubleshooting](#troubleshooting) or [open an issue](https://github.com/sanae-abe/cmdrun/issues).
