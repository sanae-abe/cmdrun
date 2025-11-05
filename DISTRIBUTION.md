# Distribution Guide

This document explains the distribution and release process for cmdrun.

## Table of Contents

- [Release Workflow](#release-workflow)
- [Distribution Channels](#distribution-channels)
- [GitHub Actions CI/CD](#github-actions-cicd)
- [Version Management](#version-management)
- [Platform Support](#platform-support)
- [Testing Releases](#testing-releases)
- [Troubleshooting](#troubleshooting)

## Release Workflow

### 1. Prepare Release

```bash
# 1. Update version in Cargo.toml
vim Cargo.toml  # Update version = "X.Y.Z"

# 2. Update CHANGELOG.md
vim CHANGELOG.md  # Document changes

# 3. Build and test locally
cargo build --release
cargo test --all-features
cargo clippy --all-targets --all-features

# 4. Commit changes
git add Cargo.toml Cargo.lock CHANGELOG.md
git commit -m "chore: prepare release vX.Y.Z"
```

### 2. Create Release Tag

```bash
# Create and push tag
git tag -a vX.Y.Z -m "Release vX.Y.Z"
git push origin main
git push origin vX.Y.Z
```

### 3. Automated Release Process

Once the tag is pushed, GitHub Actions automatically:

1. Creates a GitHub Release
2. Builds binaries for all supported platforms:
   - Linux (x86_64, ARM64)
   - macOS (Intel, Apple Silicon)
   - Windows (x86_64, ARM64)
3. Uploads release artifacts
4. Publishes to crates.io

## Distribution Channels

### 1. crates.io (Primary)

**Installation:**
```bash
cargo install cmdrun
```

**Publishing:**
- Automated via GitHub Actions on tag push
- Requires `CARGO_TOKEN` secret in repository settings
- Manual: `cargo publish`

**Prerequisites:**
- Update `Cargo.toml` with correct metadata
- Ensure README.md is complete
- All tests pass
- Documentation is up-to-date

### 2. GitHub Releases (Binary Distribution)

**Supported Platforms:**
- Linux x86_64: `cmdrun-X.Y.Z-x86_64-unknown-linux-gnu.tar.gz`
- Linux ARM64: `cmdrun-X.Y.Z-aarch64-unknown-linux-gnu.tar.gz`
- macOS Intel: `cmdrun-X.Y.Z-x86_64-apple-darwin.tar.gz`
- macOS Apple Silicon: `cmdrun-X.Y.Z-aarch64-apple-darwin.tar.gz`
- Windows x86_64: `cmdrun-X.Y.Z-x86_64-pc-windows-msvc.zip`
- Windows ARM64: `cmdrun-X.Y.Z-aarch64-pc-windows-msvc.zip`

**Download:**
```bash
# Automated install script
curl -sSL https://raw.githubusercontent.com/sanae-abe/cmdrun/main/scripts/install.sh | bash

# Manual download
wget https://github.com/sanae-abe/cmdrun/releases/download/vX.Y.Z/cmdrun-X.Y.Z-[platform].tar.gz
tar xzf cmdrun-X.Y.Z-[platform].tar.gz
mv cmdrun ~/.local/bin/
```

### 3. Homebrew (Future)

**Status:** Prepared, not yet published

**Setup:**
1. Create tap repository: `sanae-abe/homebrew-tap`
2. Move `Formula/cmdrun.rb` to tap repository
3. Update SHA256 checksums after first release

**Installation (after setup):**
```bash
brew tap sanae-abe/tap
brew install cmdrun
```

**Updating Formula:**
```ruby
# In Formula/cmdrun.rb, update after each release:
version "X.Y.Z"
sha256 "[calculated-checksum]"  # Run: shasum -a 256 cmdrun-X.Y.Z-[platform].tar.gz
```

### 4. Docker (Future)

**Status:** Planned

**Example:**
```dockerfile
FROM rust:1.75 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
COPY --from=builder /app/target/release/cmdrun /usr/local/bin/
ENTRYPOINT ["cmdrun"]
```

## GitHub Actions CI/CD

### Workflows

#### 1. CI (`ci.yml`)
- **Triggers:** Push to main/develop, Pull Requests
- **Jobs:**
  - Test Suite (Linux, macOS, Windows)
  - Security Audit
  - Code Coverage

#### 2. Release (`release.yml`)
- **Triggers:** Tag push (vX.Y.Z)
- **Jobs:**
  - Create GitHub Release
  - Build cross-platform binaries
  - Upload release artifacts
  - Publish to crates.io

#### 3. Coverage (`coverage.yml`)
- **Triggers:** Push to main, Pull Requests
- **Jobs:**
  - Generate code coverage reports
  - Upload to codecov.io

### Required Secrets

Configure these in GitHub repository settings:

```bash
CARGO_TOKEN         # crates.io API token (for publishing)
CODECOV_TOKEN       # Codecov token (optional, for coverage)
```

**Setting up CARGO_TOKEN:**
1. Go to https://crates.io/settings/tokens
2. Create new token with publish permissions
3. Add to GitHub: Settings → Secrets → New repository secret

## Version Management

### Semantic Versioning

cmdrun follows [SemVer](https://semver.org/):

- **MAJOR** (1.0.0): Breaking changes
- **MINOR** (0.1.0): New features (backward compatible)
- **PATCH** (0.0.1): Bug fixes

### Version Bump Checklist

- [ ] Update `Cargo.toml` version
- [ ] Update `Cargo.lock` (`cargo build`)
- [ ] Update `CHANGELOG.md`
- [ ] Update `Formula/cmdrun.rb` version (if applicable)
- [ ] Commit changes
- [ ] Create and push git tag
- [ ] Verify GitHub Actions release workflow

## Platform Support

### Tier 1 (Fully Tested)
- Linux x86_64 (ubuntu-latest)
- macOS x86_64 (macos-latest)
- Windows x86_64 (windows-latest)

### Tier 2 (Cross-compiled)
- Linux ARM64
- macOS ARM64 (Apple Silicon)
- Windows ARM64

### Minimum Requirements
- **Rust:** 1.75+ (MSRV - Minimum Supported Rust Version)
- **OS:**
  - Linux: glibc 2.17+
  - macOS: 10.15+
  - Windows: Windows 10+

## Testing Releases

### Local Testing

```bash
# Build release binary
cargo build --release

# Test binary
./target/release/cmdrun --version
./target/release/cmdrun init
./target/release/cmdrun list

# Test installation script (in VM/container)
bash scripts/install.sh --version v1.0.0
```

### Pre-release Testing

```bash
# Create pre-release tag
git tag -a v1.0.0-rc.1 -m "Release Candidate 1"
git push origin v1.0.0-rc.1

# Test installation from pre-release
cargo install --git https://github.com/sanae-abe/cmdrun --tag v1.0.0-rc.1
```

### Platform-specific Testing

**Linux:**
```bash
# Test on different distros (Docker)
docker run --rm -it ubuntu:22.04 bash
curl -sSL https://raw.githubusercontent.com/sanae-abe/cmdrun/main/scripts/install.sh | bash
cmdrun --version
```

**macOS:**
```bash
# Test on Intel and Apple Silicon
arch -x86_64 cmdrun --version  # Intel
arch -arm64 cmdrun --version   # Apple Silicon
```

**Windows:**
```powershell
# Test installation
Invoke-WebRequest -Uri "https://github.com/sanae-abe/cmdrun/releases/download/vX.Y.Z/cmdrun-X.Y.Z-x86_64-pc-windows-msvc.zip" -OutFile cmdrun.zip
Expand-Archive cmdrun.zip
.\cmdrun\cmdrun.exe --version
```

## Troubleshooting

### Release Build Issues

**Problem:** Binary size too large
```bash
# Solution: Verify release profile optimization
cargo build --release
strip target/release/cmdrun  # Linux/macOS only
du -h target/release/cmdrun
```

**Problem:** Cross-compilation fails
```bash
# Solution: Use cross for ARM builds
cargo install cross
cross build --release --target aarch64-unknown-linux-gnu
```

### GitHub Actions Issues

**Problem:** Release workflow not triggered
- Check tag format: Must be `vX.Y.Z` (e.g., v1.0.0)
- Verify workflow file syntax: `.github/workflows/release.yml`

**Problem:** crates.io publish fails
- Verify `CARGO_TOKEN` secret is set correctly
- Check crate name availability on crates.io
- Ensure version is unique (not already published)

**Problem:** Binary upload fails
- Check release asset names match workflow configuration
- Verify build completed successfully for all platforms

### Homebrew Formula Issues

**Problem:** SHA256 mismatch
```bash
# Recalculate checksum
shasum -a 256 cmdrun-X.Y.Z-[platform].tar.gz
# Update in Formula/cmdrun.rb
```

**Problem:** Formula installation fails
```bash
# Test formula locally
brew install --build-from-source Formula/cmdrun.rb
brew test cmdrun
```

## Best Practices

### 1. Pre-release Checklist

- [ ] All tests pass locally
- [ ] Documentation is up-to-date
- [ ] CHANGELOG.md is updated
- [ ] Version bumped in all relevant files
- [ ] Breaking changes are documented
- [ ] Migration guide provided (if needed)

### 2. Release Communication

- [ ] Post release notes on GitHub
- [ ] Update documentation site
- [ ] Announce on relevant channels
- [ ] Close related issues

### 3. Post-release Verification

- [ ] Test installation from all distribution channels
- [ ] Verify binaries work on all supported platforms
- [ ] Check documentation links
- [ ] Monitor issue tracker for release-related bugs

## Resources

- [Cargo Book - Publishing](https://doc.rust-lang.org/cargo/reference/publishing.html)
- [GitHub Actions - Releases](https://docs.github.com/en/actions/publishing-packages)
- [Homebrew Formula Cookbook](https://docs.brew.sh/Formula-Cookbook)
- [Semantic Versioning](https://semver.org/)

---

For questions or issues with the distribution process, please:
- Open an issue: https://github.com/sanae-abe/cmdrun/issues
- Check documentation: https://github.com/sanae-abe/cmdrun/tree/main/docs
