# Release Checklist

Use this checklist for every release to ensure consistency and quality.

## Pre-release (1-2 days before)

### Code Quality
- [ ] All tests pass: `cargo test --all-features`
- [ ] No clippy warnings: `cargo clippy --all-targets --all-features -- -D warnings`
- [ ] Code formatted: `cargo fmt --check`
- [ ] Security audit clean: `cargo audit`
- [ ] Documentation builds: `cargo doc --no-deps`

### Version Updates
- [ ] Update version in `Cargo.toml`
- [ ] Update version in `Formula/cmdrun.rb`
- [ ] Run `cargo build` to update `Cargo.lock`
- [ ] Update `CHANGELOG.md` with all changes since last release
- [ ] Update version references in documentation if needed

### Documentation
- [ ] README.md is up-to-date
- [ ] All examples work with current version
- [ ] API documentation is complete
- [ ] Migration guide written (if breaking changes)
- [ ] DISTRIBUTION.md reflects current process

### Testing
- [ ] Integration tests pass on all platforms
- [ ] Manual testing on Linux, macOS, Windows
- [ ] Test install script: `bash scripts/install.sh`
- [ ] Verify examples in `examples/` directory
- [ ] Performance benchmarks show no regression

## Release Day

### Create Release
- [ ] Commit all changes: `git commit -m "chore: prepare release vX.Y.Z"`
- [ ] Push to main: `git push origin main`
- [ ] Create annotated tag: `git tag -a vX.Y.Z -m "Release vX.Y.Z"`
- [ ] Push tag: `git push origin vX.Y.Z`
- [ ] Monitor GitHub Actions workflow

### Verify Automated Steps
- [ ] GitHub Release created successfully
- [ ] All platform binaries built and uploaded (6 platforms)
- [ ] crates.io publish completed successfully
- [ ] CI/CD workflow completed without errors

### Manual Verification
- [ ] Test binary download from GitHub Releases
- [ ] Test `cargo install cmdrun` from crates.io
- [ ] Verify checksums in GitHub Release
- [ ] Test install script: `curl -sSL ... | bash`
- [ ] Check crates.io page looks correct

## Post-release (within 24 hours)

### Communication
- [ ] Edit GitHub Release notes (add highlights, breaking changes)
- [ ] Post announcement (if major release)
- [ ] Close milestone (if using milestones)
- [ ] Update project board

### Homebrew (if applicable)
- [ ] Calculate SHA256 checksums for all binaries
- [ ] Update `Formula/cmdrun.rb` with new checksums
- [ ] Test Homebrew formula: `brew install --build-from-source`
- [ ] Create PR to homebrew-tap repository

### Documentation
- [ ] Update docs.rs link in README if needed
- [ ] Verify all documentation links work
- [ ] Update version badges

### Monitoring
- [ ] Watch for new issues related to release
- [ ] Monitor download statistics
- [ ] Check for build failures on user systems

## Version-specific Notes

### Patch Release (X.Y.Z where Z > 0)
- Focus on bug fixes only
- No breaking changes
- Minimal testing required
- Can be released quickly

### Minor Release (X.Y.0 where Y > 0)
- New features allowed
- No breaking changes
- Full testing required
- Update documentation thoroughly

### Major Release (X.0.0)
- Breaking changes expected
- Comprehensive testing required
- Migration guide mandatory
- Deprecation warnings in previous version
- Communication plan essential
- Consider beta/RC releases

## Common Issues & Solutions

### Issue: GitHub Actions workflow fails
**Solution:** Check workflow logs, verify secrets are set, ensure tag format is correct

### Issue: crates.io publish fails
**Solution:** Check for duplicate version, verify CARGO_TOKEN, ensure Cargo.toml is valid

### Issue: Binary doesn't work on specific platform
**Solution:** Test cross-compilation, check platform-specific code, verify dependencies

### Issue: Install script fails
**Solution:** Test in clean environment, verify URL paths, check archive structure

## Rollback Procedure

If critical issues are discovered post-release:

1. **Immediate:**
   - Yank version from crates.io: `cargo yank --version X.Y.Z`
   - Mark GitHub Release as "Pre-release" or add warning
   - Post issue with known problems

2. **Within 24 hours:**
   - Prepare hotfix release (X.Y.Z+1)
   - Follow expedited release process
   - Communicate fix timeline

3. **Documentation:**
   - Update CHANGELOG with issue description
   - Document resolution in release notes

## Templates

### Commit Message
```
chore: prepare release vX.Y.Z

- Update version to X.Y.Z
- Update CHANGELOG.md
- Update dependencies
```

### Git Tag Message
```
Release vX.Y.Z

See CHANGELOG.md for details.
```

### GitHub Release Notes Template
```markdown
## What's New

### Features
- Feature 1 description
- Feature 2 description

### Bug Fixes
- Bug fix 1
- Bug fix 2

### Performance Improvements
- Performance improvement 1

### Breaking Changes
- Breaking change 1 (include migration guide)

## Installation

### Cargo
\`\`\`bash
cargo install cmdrun
\`\`\`

### Binary Download
Download for your platform below.

## Checksums

See attached checksums.txt

## Full Changelog
See [CHANGELOG.md](https://github.com/sanae-abe/cmdrun/blob/main/CHANGELOG.md)
```

## Automation Improvements (Future)

- [ ] Automated changelog generation from commits
- [ ] Automated version bumping script
- [ ] Pre-release CI/CD checks before allowing tag push
- [ ] Automated Homebrew formula updates
- [ ] Automated documentation deployment
- [ ] Release notification automation

---

**Last Updated:** 2025-11-05
**Next Review:** Before next major release
