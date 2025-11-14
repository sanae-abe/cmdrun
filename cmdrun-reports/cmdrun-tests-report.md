# Test Report: cmdrun-tests

**Generated:** 2025-11-12 03:10:27 UTC

## Summary

**Overall Status:** ⚠️ 85% passed

| Metric | Value |
|--------|-------|
| Total Tests | 35 |
| Passed | ✅ 30 |
| Failed | ❌ 5 |
| Skipped | ⏭️ 0 |
| Duration | 4.32s |
| Suites | 6 |

## Test Suites

### ❌ basic (60%)

**File:** `cmdrun-tests/basic.bats`

**Duration:** 0.69s

| Status | Count |
|--------|-------|
| Passed | 3 |
| Failed | 2 |
| Skipped | 0 |
| Total | 5 |

#### Failed Tests

- ❌ **[basic] Reject invalid option**
  - Error: `Test 4 failed`
- ❌ **[basic] Handle no arguments**
  - Error: `Test 5 failed`

### ❌ destructive-ops (0%)

**File:** `cmdrun-tests/destructive-ops.bats`

**Duration:** 0.41s

| Status | Count |
|--------|-------|
| Passed | 0 |
| Failed | 2 |
| Skipped | 0 |
| Total | 2 |

#### Failed Tests

- ❌ **[destructive-ops] Subcommand 'remove' requires confirmation**
  - Error: `Test 1 failed`
- ❌ **[destructive-ops] Subcommand 'remove' accepts --yes flag**
  - Error: `Test 2 failed`

### ✅ help (100%)

**File:** `cmdrun-tests/help.bats`

**Duration:** 1.71s

| Status | Count |
|--------|-------|
| Passed | 19 |
| Failed | 0 |
| Skipped | 0 |
| Total | 19 |

### ✅ multi-shell (100%)

**File:** `cmdrun-tests/multi-shell.bats`

**Duration:** 0.48s

| Status | Count |
|--------|-------|
| Passed | 3 |
| Failed | 0 |
| Skipped | 0 |
| Total | 3 |

### ✅ performance (100%)

**File:** `cmdrun-tests/performance.bats`

**Duration:** 0.39s

| Status | Count |
|--------|-------|
| Passed | 2 |
| Failed | 0 |
| Skipped | 0 |
| Total | 2 |

### ❌ security (75%)

**File:** `cmdrun-tests/security.bats`

**Duration:** 0.55s

| Status | Count |
|--------|-------|
| Passed | 3 |
| Failed | 1 |
| Skipped | 0 |
| Total | 4 |

#### Failed Tests

- ❌ **[security] Handle extremely long input**
  - Error: `Test 4 failed`

## Environment

| Property | Value |
|----------|-------|
| OS | macos 15.6.1 |
| Shell | GNU bash, バージョン 5.3.3(1)-release (aarch64-apple-darwin24.6.0) |
| BATS | Bats 1.12.0 |
| Hostname | m3-2022mac21.local |
| User | sanae.abe |

## Detailed Results

### basic

| # | Test Name | Status | Duration |
|---|-----------|--------|----------|
| 1 | [basic] Display help with --help flag | ✅ Passed | 100ms |
| 2 | [basic] Display help with -h flag | ✅ Passed | 100ms |
| 3 | [basic] Display version with --version flag | ✅ Passed | 100ms |
| 4 | [basic] Reject invalid option | ❌ Failed | 100ms |
| 5 | [basic] Handle no arguments | ❌ Failed | 100ms |

### destructive-ops

| # | Test Name | Status | Duration |
|---|-----------|--------|----------|
| 1 | [destructive-ops] Subcommand 'remove' requires confirmation | ❌ Failed | 100ms |
| 2 | [destructive-ops] Subcommand 'remove' accepts --yes flag | ❌ Failed | 100ms |

### help

| # | Test Name | Status | Duration |
|---|-----------|--------|----------|
| 1 | [help] Display help for subcommand 'run' | ✅ Passed | 100ms |
| 2 | [help] Display help for subcommand 'list' | ✅ Passed | 100ms |
| 3 | [help] Display help for subcommand 'init' | ✅ Passed | 100ms |
| 4 | [help] Display help for subcommand 'validate' | ✅ Passed | 100ms |
| 5 | [help] Display help for subcommand 'graph' | ✅ Passed | 100ms |
| 6 | [help] Display help for subcommand 'completion' | ✅ Passed | 100ms |
| 7 | [help] Display help for subcommand 'remove' | ✅ Passed | 100ms |
| 8 | [help] Display help for subcommand 'add' | ✅ Passed | 100ms |
| 9 | [help] Display help for subcommand 'open' | ✅ Passed | 100ms |
| 10 | [help] Display help for subcommand 'edit' | ✅ Passed | 100ms |
| 11 | [help] Display help for subcommand 'info' | ✅ Passed | 100ms |
| 12 | [help] Display help for subcommand 'search' | ✅ Passed | 100ms |
| 13 | [help] Display help for subcommand 'config' | ✅ Passed | 100ms |
| 14 | [help] Display help for subcommand 'watch' | ✅ Passed | 100ms |
| 15 | [help] Display help for subcommand 'env' | ✅ Passed | 100ms |
| 16 | [help] Display help for subcommand 'history' | ✅ Passed | 100ms |
| 17 | [help] Display help for subcommand 'retry' | ✅ Passed | 100ms |
| 18 | [help] Display help for subcommand 'template' | ✅ Passed | 100ms |
| 19 | [help] Display help for subcommand 'plugin' | ✅ Passed | 100ms |

### multi-shell

| # | Test Name | Status | Duration |
|---|-----------|--------|----------|
| 1 | [multi-shell] Run --help in bash | ✅ Passed | 100ms |
| 2 | [multi-shell] Run --help in zsh | ✅ Passed | 100ms |
| 3 | [multi-shell] Run --help in sh | ✅ Passed | 100ms |

### performance

| # | Test Name | Status | Duration |
|---|-----------|--------|----------|
| 1 | [performance] Startup time for --help < 100ms | ✅ Passed | 100ms |
| 2 | [performance] Memory usage stays within reasonable limits | ✅ Passed | 100ms |

### security

| # | Test Name | Status | Duration |
|---|-----------|--------|----------|
| 1 | [security] Reject command injection in option value | ✅ Passed | 100ms |
| 2 | [security] Reject null byte in option value | ✅ Passed | 100ms |
| 3 | [security] Reject path traversal attempt | ✅ Passed | 100ms |
| 4 | [security] Handle extremely long input | ❌ Failed | 100ms |

