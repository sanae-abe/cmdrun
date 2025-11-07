# cmdrun Profiling Guide

Comprehensive guide for continuous performance monitoring and profiling of cmdrun.

## Table of Contents

- [Overview](#overview)
- [Quick Start](#quick-start)
- [Profiling Tools](#profiling-tools)
- [Running Profiling](#running-profiling)
- [Interpreting Results](#interpreting-results)
- [Performance Targets](#performance-targets)
- [Troubleshooting](#troubleshooting)
- [CI/CD Integration](#cicd-integration)

---

## Overview

cmdrun maintains strict performance targets:

| Metric | Target | Typical Result |
|--------|--------|----------------|
| Startup Time | < 5ms | ~4ms |
| Memory Footprint | < 15MB | ~10MB |
| Config Parse (100 cmd) | < 1ms | ~0.5ms |
| Binary Size | < 5MB | ~3MB (stripped) |

Regular profiling ensures we stay within these targets.

---

## Quick Start

### Prerequisites

```bash
# Required
cargo install hyperfine

# Optional (for detailed profiling)
cargo install flamegraph  # CPU profiling
cargo install cargo-bloat # Binary size analysis

# Linux only (optional)
sudo apt install valgrind heaptrack linux-perf
```

### Run Quick Profiling

```bash
# Profile startup time (2-3 minutes)
./scripts/profile.sh startup

# Profile memory usage (1-2 minutes)
./scripts/profile.sh memory

# Run all benchmarks (5-10 minutes)
./scripts/benchmark.sh all

# Full profiling suite (10-15 minutes, requires sudo for flamegraph)
./scripts/profile.sh all
```

---

## Profiling Tools

### 1. hyperfine - Startup Time Measurement

**Purpose:** Accurate command-line benchmark tool for measuring startup time.

**Installation:**
```bash
cargo install hyperfine
```

**Usage:**
```bash
# Basic benchmark
hyperfine 'cmdrun --version'

# Compare multiple commands
hyperfine \
  'cmdrun --version' \
  'npm --version' \
  'make --version'

# With warmup and export
hyperfine \
  --warmup 3 \
  --min-runs 100 \
  --export-markdown results.md \
  'cmdrun --version'
```

**Expected Output:**
```
Benchmark 1: cmdrun --version
  Time (mean ± σ):       4.2 ms ±   0.3 ms    [User: 1.1 ms, System: 2.8 ms]
  Range (min … max):     3.8 ms …   5.1 ms    500 runs
```

**Interpreting Results:**
- **Mean time:** Average execution time (should be < 5ms)
- **Standard deviation (σ):** Consistency (lower is better)
- **Range:** Min/max times (watch for outliers)

---

### 2. time - Basic Memory Profiling

**Purpose:** Built-in command for basic memory and time measurement.

**macOS Usage:**
```bash
/usr/bin/time -l cmdrun --version

# Look for:
# maximum resident set size (bytes): 10485760  # ~10MB
# user time: 0.001s
# system time: 0.002s
```

**Linux Usage:**
```bash
/usr/bin/time -v cmdrun --version

# Look for:
# Maximum resident set size (kbytes): 10240  # ~10MB
# User time (seconds): 0.001
# System time (seconds): 0.002
```

**Interpreting Memory Results:**

| Platform | Field | Target |
|----------|-------|--------|
| macOS | maximum resident set size | < 15,728,640 bytes (15MB) |
| Linux | Maximum resident set size | < 15360 kbytes (15MB) |

---

### 3. cargo-flamegraph - CPU Profiling

**Purpose:** Visualize CPU time spent in different functions.

**Installation:**
```bash
cargo install flamegraph

# Linux: also install perf
sudo apt install linux-perf

# macOS: requires sudo
```

**Usage:**
```bash
# Build with debug symbols
cargo build --profile release-with-debug

# Generate flamegraph (macOS requires sudo)
sudo cargo flamegraph -- list --config examples/basic.toml

# Output: flamegraph.svg (open in browser)
```

**Interpreting Flamegraph:**
- **Width:** Time spent in function (wider = more time)
- **Height:** Call stack depth
- **Color:** Random (for visual distinction)

**What to look for:**
- Wide bars at the top = hotspots
- Unexpected function calls
- Regex compilation in hot paths
- Excessive allocations

---

### 4. valgrind/massif - Memory Profiling (Linux)

**Purpose:** Detailed heap memory profiling.

**Installation:**
```bash
sudo apt install valgrind
```

**Usage:**
```bash
# Run massif
valgrind --tool=massif --massif-out-file=massif.out \
  ./target/release-with-debug/cmdrun --version

# Analyze results
ms_print massif.out
```

**Interpreting Results:**
```
    KB
10.24^                                                           #
     |                                                         ::#
     |                                                       ::::#
     |                                                    @:::::::#
     |                                                 @@@:::::::#
```

- **Peak memory:** Highest point (should be < 15MB)
- **Growth pattern:** Steady or spiky
- **Allocations:** Number and size

---

### 5. heaptrack - Memory Profiling (Linux)

**Purpose:** User-friendly heap memory profiler with GUI.

**Installation:**
```bash
sudo apt install heaptrack heaptrack-gui
```

**Usage:**
```bash
# Profile application
heaptrack ./target/release-with-debug/cmdrun --version

# Analyze with GUI
heaptrack_gui heaptrack.cmdrun.*.gz
```

**Features:**
- Allocation flame graphs
- Memory leak detection
- Peak memory analysis
- Temporary allocation tracking

---

### 6. Instruments (macOS)

**Purpose:** Native macOS profiling tool.

**Usage:**
```bash
# Build with debug symbols
cargo build --profile release-with-debug

# Time Profiler
instruments -t "Time Profiler" \
  ./target/release-with-debug/cmdrun list --config examples/basic.toml

# Allocations
instruments -t "Allocations" \
  ./target/release-with-debug/cmdrun list --config examples/basic.toml

# Open Instruments GUI
open /Applications/Xcode.app/Contents/Applications/Instruments.app
```

**Key Instruments:**
- **Time Profiler:** CPU usage, hot functions
- **Allocations:** Memory allocations, leaks
- **System Trace:** Overall system activity

---

### 7. cargo-bloat - Binary Size Analysis

**Purpose:** Analyze what takes up space in the binary.

**Installation:**
```bash
cargo install cargo-bloat
```

**Usage:**
```bash
# Analyze release binary
cargo bloat --release

# Top 20 functions
cargo bloat --release -n 20

# Show crate-level breakdown
cargo bloat --release --crates
```

**Expected Output:**
```
 File  .text     Size Crate
 0.5%   1.2%   8.5KiB std
 0.4%   1.0%   7.0KiB cmdrun
 0.3%   0.8%   5.5KiB clap
...
```

**Target:** Binary size < 5MB (stripped)

---

## Running Profiling

### Automated Profiling Scripts

We provide two main scripts:

#### 1. profile.sh - Performance Profiling

```bash
# Usage
./scripts/profile.sh [startup|memory|flamegraph|all]

# Startup time only (fastest, ~2 min)
./scripts/profile.sh startup

# Memory profiling only (~2 min)
./scripts/profile.sh memory

# Flamegraph only (requires sudo, ~3 min)
./scripts/profile.sh flamegraph

# All profiling (interactive, ~10 min)
./scripts/profile.sh all
```

**Output Location:**
```
target/profiling/results/
├── startup_20250107_143022.txt
├── startup_20250107_143022.md
├── startup_20250107_143022.json
├── memory_20250107_143022.txt
├── flamegraph_20250107_143022.svg
└── report_20250107_143022.md
```

#### 2. benchmark.sh - Criterion Benchmarks

```bash
# Usage
./scripts/benchmark.sh [all|command|toml|report]

# Run all benchmarks (~5-10 min)
./scripts/benchmark.sh all

# Command execution benchmarks only
./scripts/benchmark.sh command

# TOML parsing benchmarks only
./scripts/benchmark.sh toml

# Generate report from existing results
./scripts/benchmark.sh report
```

**Output Location:**
```
target/criterion/
├── report/index.html          # Main report
├── shell_command/             # Command benchmarks
├── toml_parsing/              # TOML benchmarks
└── ...

target/benchmark-results/
├── bench_all_20250107_143022.txt
└── report_20250107_143022.md
```

---

### Manual Profiling Workflow

#### Step 1: Build Profiling Binary

```bash
# Build with debug symbols (for profiling)
cargo build --profile release-with-debug

# Verify binary
ls -lh target/release-with-debug/cmdrun
file target/release-with-debug/cmdrun
```

#### Step 2: Profile Startup Time

```bash
# Quick test
time ./target/release-with-debug/cmdrun --version

# Accurate benchmark
hyperfine \
  --warmup 3 \
  --min-runs 100 \
  './target/release-with-debug/cmdrun --version'
```

#### Step 3: Profile Memory

```bash
# macOS
/usr/bin/time -l ./target/release-with-debug/cmdrun --version

# Linux
/usr/bin/time -v ./target/release-with-debug/cmdrun --version
```

#### Step 4: Generate Flamegraph (Optional)

```bash
# macOS (requires sudo)
sudo cargo flamegraph \
  --output=flamegraph.svg \
  --profile release-with-debug \
  -- list --config examples/basic.toml

# Open in browser
open flamegraph.svg
```

#### Step 5: Run Benchmarks

```bash
# Run all criterion benchmarks
cargo bench

# Open HTML report
open target/criterion/report/index.html
```

---

## Interpreting Results

### Startup Time Analysis

**Target: < 5ms**

**Good Result:**
```
Time (mean ± σ):       4.2 ms ±   0.3 ms
```
✅ Mean < 5ms, low standard deviation

**Warning Result:**
```
Time (mean ± σ):       6.8 ms ±   1.2 ms
```
⚠️ Mean > 5ms, investigate startup code

**Problem Result:**
```
Time (mean ± σ):      15.3 ms ±   3.5 ms
```
❌ Significantly over target, high variance

**Common Causes:**
- Debug build instead of release
- Lazy static initialization
- Excessive regex compilation
- Large binary size

**Solutions:**
- Verify `--release` build
- Use `once_cell::Lazy` for initialization
- Pre-compile regex patterns
- Reduce dependencies

---

### Memory Usage Analysis

**Target: < 15MB**

**macOS Output:**
```
maximum resident set size: 10485760 bytes  # 10MB
```
✅ 10MB < 15MB target

**Linux Output:**
```
Maximum resident set size (kbytes): 10240  # 10MB
```
✅ 10MB < 15MB target

**Warning Signs:**
- Memory > 15MB for simple commands
- Memory growing over time (leak)
- Unexpected allocations in profiler

**Common Causes:**
- Unbounded collections (HashMap, Vec)
- String allocations (clone, to_string)
- Regex cache not working
- TOML parsing overhead

**Solutions:**
- Use `AHashMap` for smaller memory footprint
- Use `&str` instead of `String` where possible
- Verify `once_cell::Lazy` for regex
- Stream large TOML files

---

### Flamegraph Analysis

**What to Look For:**

1. **Wide Bars (Hotspots):**
   - Top-level functions consuming most time
   - Good: Wide bars in `tokio::spawn`, `Command::spawn`
   - Bad: Wide bars in `String::clone`, `regex::new`

2. **Call Stack Depth:**
   - Deep stacks = complex logic
   - Good: Depth < 10 for main path
   - Bad: Depth > 20 (stack overflow risk)

3. **Unexpected Functions:**
   - `alloc::` = allocations (minimize in hot path)
   - `regex::Regex::new` = runtime compilation (should be cached)
   - `serde::de::` = deserialization (expected for config parsing)

4. **Time Distribution:**
   - Expected: 60-70% in user code, 30-40% in libraries
   - Problem: > 50% in single function

**Example Analysis:**

```
cmdrun::main (100%)
├── cmdrun::cli::parse (5%)
├── cmdrun::config::load (15%)
│   ├── toml::from_str (10%)
│   └── std::fs::read_to_string (5%)
└── cmdrun::command::execute (80%)
    ├── tokio::spawn (60%)
    └── std::process::Command (20%)
```

✅ **Good:** Most time in command execution (expected)

---

### Benchmark Results

**Criterion Output:**

```
shell_command/echo_hello
                        time:   [4.123 ms 4.156 ms 4.192 ms]
                        change: [-2.3% -1.1% +0.2%] (p = 0.08 > 0.05)
                        No change in performance detected.

toml_parsing/100
                        time:   [523.45 µs 528.32 µs 533.67 µs]
                        change: [-5.2% -3.8% -2.1%] (p = 0.00 < 0.05)
                        Performance has improved.
```

**Interpretation:**

- **time:** Mean ± confidence interval
- **change:** % change from previous baseline
- **p-value:**
  - p < 0.05: Statistically significant change
  - p > 0.05: No significant change

**Status Indicators:**

- ✅ **Performance has improved:** Good!
- ⚠️ **No change in performance detected:** Neutral
- ❌ **Performance has regressed:** Investigate!

---

## Performance Targets

### Detailed Targets

| Component | Metric | Target | Measure With |
|-----------|--------|--------|--------------|
| **Startup** | | | |
| Binary load | Time | < 3ms | `hyperfine` |
| CLI parse | Time | < 0.5ms | Flamegraph |
| Config load | Time | < 1ms (100 cmd) | `cargo bench` |
| **Memory** | | | |
| Idle memory | RSS | < 15MB | `time -l/-v` |
| Peak memory | RSS | < 50MB | `valgrind` |
| **Config** | | | |
| Parse (100 cmd) | Time | < 1ms | `cargo bench` |
| Parse (1000 cmd) | Time | < 5ms | `cargo bench` |
| **Command** | | | |
| Execution overhead | Time | < 1ms | `hyperfine` |
| Parallel (10 cmd) | Time | ~= slowest cmd | Integration test |
| **Binary** | | | |
| Size (stripped) | Bytes | < 5MB | `ls -lh` |
| Size (debug) | Bytes | < 15MB | `ls -lh` |

---

### Regression Detection

**CI Integration:**

Our CI pipeline automatically detects performance regressions:

```yaml
# .github/workflows/ci.yml
- name: Run benchmarks
  run: cargo bench --no-fail-fast

- name: Check for regressions
  run: |
    # Fail if performance degraded by > 50%
    cargo bench -- --save-baseline main
```

**Thresholds:**

- **Warning:** > 20% slower
- **Error:** > 50% slower (CI fails)

**Manual Comparison:**

```bash
# Save baseline
cargo bench -- --save-baseline main

# Make changes...

# Compare
cargo bench -- --baseline main
```

---

## Troubleshooting

### Issue: Startup Time > 10ms

**Symptoms:**
- `hyperfine 'cmdrun --version'` shows > 10ms

**Diagnosis:**
```bash
# Check if debug build
file ~/.cargo/bin/cmdrun
# Should say "not stripped" for release

# Check binary size
ls -lh ~/.cargo/bin/cmdrun
# Should be ~3-5MB

# Profile startup
sudo cargo flamegraph -- --version
# Look for expensive initialization
```

**Solutions:**
```bash
# Rebuild with optimizations
cargo clean
cargo build --release
cargo install --path . --force

# Verify
hyperfine 'cmdrun --version'
```

---

### Issue: High Memory Usage

**Symptoms:**
- Memory > 50MB for simple operations
- Memory grows over time

**Diagnosis:**
```bash
# Linux: valgrind
valgrind --leak-check=full \
  ./target/release-with-debug/cmdrun --version

# Linux: heaptrack
heaptrack ./target/release-with-debug/cmdrun --version
heaptrack_gui heaptrack.cmdrun.*.gz

# macOS: Instruments
instruments -t "Allocations" \
  ./target/release-with-debug/cmdrun --version
```

**Common Causes:**
- Unbounded `Vec` growth
- String cloning
- Regex compilation
- TOML retention

**Solutions:**
- Use `.capacity()` for pre-allocation
- Use `&str` where possible
- Cache regex with `once_cell::Lazy`
- Drop config after parsing

---

### Issue: Benchmark Variance

**Symptoms:**
- Criterion shows high standard deviation
- Inconsistent results across runs

**Diagnosis:**
```bash
# Run more iterations
cargo bench -- --sample-size 500

# Check system load
top  # Ensure no other heavy processes
```

**Solutions:**
- Close other applications
- Disable CPU frequency scaling (Linux):
  ```bash
  sudo cpupower frequency-set --governor performance
  ```
- Increase warmup:
  ```bash
  cargo bench -- --warm-up-time 5
  ```

---

### Issue: CI Benchmark Failures

**Symptoms:**
- CI benchmarks fail or timeout
- Inconsistent results in CI

**Solutions:**

1. **Reduce benchmark time:**
   ```rust
   // benches/*.rs
   group.measurement_time(Duration::from_secs(5));  // Reduce from 10s
   group.sample_size(50);  // Reduce from 100
   ```

2. **Use baseline comparison:**
   ```yaml
   # .github/workflows/ci.yml
   - run: cargo bench -- --save-baseline ci
   - run: cargo bench -- --baseline ci
   ```

3. **Skip benchmarks in CI (if needed):**
   ```yaml
   - run: cargo test --release  # Skip benches
   ```

---

## CI/CD Integration

### GitHub Actions

```yaml
name: Performance

on:
  pull_request:
  push:
    branches: [main]

jobs:
  benchmark:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Install Rust
        uses: dtolnay/rust-toolchain@stable

      - name: Install hyperfine
        run: cargo install hyperfine

      - name: Build release
        run: cargo build --release

      - name: Benchmark startup
        run: |
          hyperfine \
            --warmup 3 \
            --min-runs 50 \
            --export-markdown startup.md \
            './target/release/cmdrun --version'

      - name: Check startup time
        run: |
          # Extract mean time and check < 5ms
          mean=$(grep "Time (mean" startup.md | awk '{print $4}')
          if (( $(echo "$mean > 5.0" | bc -l) )); then
            echo "❌ Startup time ${mean}ms exceeds 5ms target"
            exit 1
          fi
          echo "✅ Startup time: ${mean}ms"

      - name: Run benchmarks
        run: cargo bench --no-fail-fast

      - name: Upload results
        uses: actions/upload-artifact@v4
        with:
          name: benchmark-results
          path: |
            target/criterion/
            startup.md
```

---

### Local Pre-commit Hook

```bash
# .git/hooks/pre-commit

#!/bin/bash
set -e

echo "Running quick performance checks..."

# Check startup time
time_ms=$(hyperfine --warmup 3 --min-runs 10 './target/release/cmdrun --version' 2>&1 | grep "Time (mean" | awk '{print $4}')

if (( $(echo "$time_ms > 5.0" | bc -l) )); then
  echo "❌ Startup time ${time_ms}ms exceeds 5ms target"
  exit 1
fi

echo "✅ Performance checks passed"
```

---

## Best Practices

### Regular Profiling Schedule

- **Daily:** Startup time (1 min)
  ```bash
  hyperfine 'cmdrun --version'
  ```

- **Weekly:** Full benchmark suite (10 min)
  ```bash
  ./scripts/benchmark.sh all
  ```

- **Monthly:** Deep profiling (30 min)
  ```bash
  ./scripts/profile.sh all
  ```

- **Before Release:** Comprehensive analysis (1 hour)
  ```bash
  ./scripts/profile.sh all
  ./scripts/benchmark.sh all
  cargo bloat --release
  ```

---

### Performance Checklist

Before merging performance-sensitive changes:

- [ ] Run `hyperfine 'cmdrun --version'` (< 5ms)
- [ ] Run `cargo bench` (no regressions)
- [ ] Check memory with `time -l/-v` (< 15MB)
- [ ] Review flamegraph for hotspots
- [ ] Verify binary size (< 5MB)
- [ ] Update benchmarks if new features
- [ ] Document performance characteristics

---

## Related Documentation

- [PERFORMANCE_GUIDE.md](PERFORMANCE_GUIDE.md) - Optimization techniques
- [ARCHITECTURE.md](ARCHITECTURE.md) - System design
- [PERFORMANCE.md](PERFORMANCE.md) - Performance metrics

---

**Last Updated:** 2025-11-07
**Version:** 1.0.0
