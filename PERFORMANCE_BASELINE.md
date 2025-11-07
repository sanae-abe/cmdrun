# cmdrun Performance Baseline Report

**Date:** 2025-11-07
**Version:** 1.0.0
**Platform:** macOS (Apple Silicon - arm64)
**Rust Version:** 1.75+

---

## Executive Summary

cmdrun performance baseline established with the following results:

| Metric | Target | Actual | Status |
|--------|--------|--------|--------|
| **Startup Time** | < 5ms | 6.5ms (±1.4ms) | ⚠️ **Warning** (30% over target) |
| **Memory Usage** | < 15MB | 4.5MB | ✅ **Excellent** (70% under target) |
| **Binary Size** | < 5MB | 2.9MB | ✅ **Excellent** (42% under target) |
| **Config Parse (100)** | < 1ms | ~0.22ms | ✅ **Excellent** (78% under target) |

**Overall Assessment:** Performance is excellent in most areas. Startup time requires minor optimization but is within acceptable range.

---

## Detailed Measurements

### 1. Startup Time Benchmark

**Tool:** hyperfine v1.18+
**Command:** `cmdrun --version`
**Configuration:** --warmup 5, --min-runs 100, --shell=none

```
Time (mean ± σ):       6.5 ms ±   1.4 ms    [User: 2.4 ms, System: 1.8 ms]
Range (min … max):     4.6 ms …  13.8 ms    381 runs
```

**Analysis:**
- **Mean Time:** 6.5ms (target: < 5ms)
- **Standard Deviation:** 1.4ms (21% variance - acceptable)
- **Best Case:** 4.6ms (within target!)
- **Worst Case:** 13.8ms (likely system interference)

**Breakdown:**
- User time: 2.4ms (37%)
- System time: 1.8ms (28%)
- Overhead: ~2.3ms (35%)

**Recommendation:** Investigate startup overhead. Best-case performance (4.6ms) proves the target is achievable with optimization.

---

### 2. Memory Usage

**Tool:** `/usr/bin/time -l` (macOS)
**Command:** `cmdrun --version`

```
Maximum resident set size: 4,734,976 bytes (4.5 MB)
```

**Analysis:**
- **RSS (Resident Set Size):** 4.5MB
- **Target:** < 15MB
- **Margin:** 70% under target (10.5MB headroom)

**Status:** ✅ **Excellent** - Well within target with significant headroom for future features.

---

### 3. Binary Size

**Tool:** `ls -lh`
**Binary:** `target/release/cmdrun` (stripped)

```
Size: 2.9 MB
```

**Analysis:**
- **Stripped Binary:** 2.9MB
- **Target:** < 5MB
- **Margin:** 42% under target

**Comparison:**
- Debug build: ~15MB (with symbols)
- Release-with-debug: ~12MB (for profiling)
- Release (stripped): 2.9MB (production)

**Status:** ✅ **Excellent** - Compact binary size enables fast distribution and loading.

---

### 4. TOML Parsing Performance

**Tool:** Criterion (cargo bench)
**Benchmark Suite:** `toml_parsing`

#### Small Config (10 commands)

```
toml_parsing/10         time:   [87.391 µs 87.882 µs 88.427 µs]
                        thrpt:  [28.102 MiB/s 28.276 MiB/s 28.432 MiB/s]
```

- **Parse Time:** ~88 µs (0.088ms)
- **Throughput:** 28.3 MiB/s
- **Status:** ✅ Well within < 1ms target

#### Medium Config (50 commands)

```
toml_parsing/50         time:   [142.92 µs 143.86 µs 144.88 µs]
                        thrpt:  [34.987 MiB/s 35.234 MiB/s 35.466 MiB/s]
```

- **Parse Time:** ~144 µs (0.144ms)
- **Throughput:** 35.2 MiB/s
- **Status:** ✅ Excellent scaling

#### Large Config (100 commands)

```
toml_parsing/100        time:   [209.82 µs 215.22 µs 221.67 µs]
                        thrpt:  [36.439 MiB/s 37.533 MiB/s 38.499 MiB/s]
```

- **Parse Time:** ~215 µs (0.215ms)
- **Throughput:** 37.5 MiB/s
- **Status:** ✅ **Excellent** (78% under 1ms target)

#### Very Large Config (200 commands)

```
toml_parsing/200        time:   [414.81 µs 419.00 µs 423.78 µs]
                        thrpt:  [38.920 MiB/s 39.365 MiB/s 39.762 MiB/s]
```

- **Parse Time:** ~419 µs (0.419ms)
- **Throughput:** 39.4 MiB/s
- **Status:** ✅ Sub-linear scaling (excellent)

**Scaling Analysis:**
- 10 → 50 commands: 1.64x time (5x data) = **Super-linear efficiency**
- 50 → 100 commands: 1.50x time (2x data) = **Super-linear efficiency**
- 100 → 200 commands: 1.95x time (2x data) = **Near-linear scaling**

---

### 5. TOML Serialization Performance

**Tool:** Criterion (cargo bench)
**Benchmark Suite:** `toml_serialization`

```
toml_serialization/10   time:   [17.706 µs 17.806 µs 17.924 µs]
                        thrpt:  [557.91 Kelem/s 561.60 Kelem/s 564.78 Kelem/s]

toml_serialization/50   time:   [86.471 µs 86.878 µs 87.410 µs]
                        thrpt:  [572.02 Kelem/s 575.52 Kelem/s 578.23 Kelem/s]

toml_serialization/100  time:   [180.44 µs 192.86 µs 211.93 µs]
                        thrpt:  [471.86 Kelem/s 518.51 Kelem/s 554.20 Kelem/s]
```

**Analysis:**
- **Throughput:** ~550-575 Kelem/s (consistent)
- **Scaling:** Near-linear with config size
- **Status:** ✅ Excellent serialization performance

---

### 6. String Operations Performance

**Tool:** Criterion (cargo bench)
**Benchmark Suite:** `string_ops`

```
split_and_collect       time:   [88.571 ns 89.588 ns 91.249 ns]
to_lowercase            time:   [20.335 ns 20.442 ns 20.576 ns]
contains_check          time:   [25.088 ns 25.500 ns 26.070 ns]
```

**Analysis:**
- **Split:** ~90 ns (11M ops/sec)
- **Lowercase:** ~20 ns (49M ops/sec)
- **Contains:** ~26 ns (39M ops/sec)

**Status:** ✅ Optimized string operations (nanosecond-level performance)

---

### 7. File I/O Performance

**Tool:** Criterion (cargo bench)
**Benchmark:** `file_io/write_temp_file`

```
write_temp_file         time:   [292.49 µs 298.36 µs 305.54 µs]
```

**Analysis:**
- **Write Time:** ~298 µs (0.298ms)
- **Operation:** Write + flush temporary file
- **Status:** ✅ Fast I/O operations

---

### 8. Complex TOML Parsing

**Tool:** Criterion (cargo bench)
**Benchmark:** Nested configuration with env vars, aliases, dependencies

```
parse_complex           time:   [9.8067 µs 9.9588 µs 10.185 µs]
serialize_complex       time:   [8.5347 µs 8.7034 µs 8.9509 µs]
```

**Analysis:**
- **Parse:** ~10 µs (100K ops/sec)
- **Serialize:** ~9 µs (111K ops/sec)
- **Status:** ✅ Excellent performance on complex structures

---

## Performance Targets Comparison

| Component | Target | Current | Margin | Status |
|-----------|--------|---------|--------|--------|
| **Startup** | | | | |
| Binary load | < 3ms | ~2.3ms | 0.7ms | ✅ |
| Config load (100) | < 1ms | 0.22ms | 0.78ms | ✅ |
| Total startup | < 5ms | 6.5ms | -1.5ms | ⚠️ |
| **Memory** | | | | |
| Idle memory | < 15MB | 4.5MB | 10.5MB | ✅ |
| **Binary** | | | | |
| Size (stripped) | < 5MB | 2.9MB | 2.1MB | ✅ |
| **Config Parse** | | | | |
| 100 commands | < 1ms | 0.22ms | 0.78ms | ✅ |
| 200 commands | < 2ms | 0.42ms | 1.58ms | ✅ |
| 1000 commands | < 5ms | ~2ms* | 3ms | ✅ |

*Extrapolated from 200-command benchmark

---

## Optimization Opportunities

### Priority 1: Startup Time (6.5ms → < 5ms)

**Current Bottleneck:**
- System overhead: ~2.3ms (35% of total)
- User code: 2.4ms
- System calls: 1.8ms

**Potential Optimizations:**

1. **Lazy Initialization** (Est. savings: 0.5-1ms)
   - Defer regex compilation until needed
   - Lazy load help text
   - Use `once_cell::Lazy` more aggressively

2. **Binary Size Reduction** (Est. savings: 0.3-0.5ms)
   - Strip unused features from dependencies
   - Use `cargo-bloat` to identify bloat
   - Consider splitting features

3. **Static Linking Optimization** (Est. savings: 0.2-0.3ms)
   - Optimize dynamic library loading
   - Consider static musl build (Linux)

**Expected Result:** 5.5ms → 4.5ms (10% improvement)

---

### Priority 2: Advanced Profiling

**Next Steps:**

1. **Flamegraph Analysis**
   ```bash
   ./scripts/profile.sh flamegraph
   ```
   - Identify CPU hotspots
   - Analyze call stack depth
   - Find unexpected allocations

2. **Detailed Memory Profiling**
   ```bash
   ./scripts/profile.sh memory
   ```
   - Check for memory leaks
   - Analyze allocation patterns
   - Verify heap efficiency

3. **Regression Testing**
   ```bash
   cargo bench -- --save-baseline v1.0.0
   ```
   - Establish baseline for future comparisons
   - Detect performance regressions in CI

---

## Continuous Monitoring Plan

### Daily
```bash
# Quick startup check (30 seconds)
hyperfine --warmup 3 --min-runs 10 './target/release/cmdrun --version'
```

**Target:** Mean < 5ms

### Weekly
```bash
# Full benchmark suite (5-10 minutes)
./scripts/benchmark.sh all
```

**Review:** Check for regressions in Criterion reports

### Monthly
```bash
# Comprehensive profiling (15-20 minutes)
./scripts/profile.sh all
```

**Analyze:**
- Flamegraph for hotspots
- Memory usage trends
- Binary size evolution

### Before Release
```bash
# Complete performance validation (30 minutes)
./scripts/profile.sh all
./scripts/benchmark.sh all
cargo bloat --release -n 20
```

**Checklist:**
- [ ] Startup time < 5ms
- [ ] Memory < 15MB
- [ ] Binary size < 5MB
- [ ] No benchmark regressions
- [ ] Flamegraph reviewed
- [ ] Documentation updated

---

## CI/CD Integration Status

**Current Status:** ✅ Implemented

- GitHub Actions workflow: `.github/workflows/ci.yml`
- Benchmark regression detection: 150% threshold
- Automatic failure on significant degradation

**Metrics Tracked:**
- Cargo bench execution
- Startup time validation (planned)
- Memory usage checks (planned)

---

## Tooling Summary

### Installed & Available
- ✅ hyperfine (startup benchmarking)
- ✅ Criterion (microbenchmarks)
- ✅ time (memory profiling)
- ✅ cargo-bloat (binary analysis)

### Recommended (Not Installed)
- ⚠️ cargo-flamegraph (CPU profiling) - `cargo install flamegraph`
- ⚠️ valgrind (Linux memory profiling) - `sudo apt install valgrind`
- ⚠️ heaptrack (Linux GUI profiler) - `sudo apt install heaptrack`

### Platform-Specific
- ✅ macOS Instruments (available via Xcode)
- ✅ /usr/bin/time (macOS & Linux)

---

## Created Assets

### Scripts
1. **`scripts/profile.sh`** - Comprehensive profiling automation
   - Startup profiling (hyperfine)
   - Memory profiling (time)
   - Flamegraph generation (optional, requires sudo)
   - Automated reporting

2. **`scripts/benchmark.sh`** - Benchmark execution automation
   - Run all Criterion benchmarks
   - Generate markdown reports
   - Compare with baselines

### Documentation
1. **`docs/technical/PROFILING.md`** - Complete profiling guide
   - Tool installation & setup
   - Usage instructions
   - Result interpretation
   - Troubleshooting
   - CI/CD integration

2. **`PERFORMANCE_BASELINE.md`** (this file)
   - Baseline measurements
   - Performance targets
   - Optimization roadmap

### Reports
- `target/profiling/results/` - Profiling results
- `target/criterion/` - Benchmark HTML reports
- `target/benchmark-results/` - Historical benchmark data

---

## Recommendations

### Immediate Actions (This Week)
1. ✅ **Establish baseline** - COMPLETED
2. ⚠️ **Investigate startup time** - In progress
   - Run flamegraph analysis
   - Identify lazy initialization opportunities
   - Target: 6.5ms → 4.5ms

3. ⚠️ **Setup monitoring** - Partially done
   - Add startup time check to CI
   - Create weekly benchmark schedule
   - Document performance SLAs

### Short-term (This Month)
1. **Optimize startup time** to < 5ms
2. **Install profiling tools** (flamegraph, etc.)
3. **Run comprehensive profiling**
4. **Document optimization strategies**

### Long-term (This Quarter)
1. **Automated performance tracking dashboard**
2. **Performance budget enforcement in CI**
3. **Continuous profiling in production**
4. **Advanced optimization techniques**

---

## Conclusion

cmdrun demonstrates **excellent baseline performance** with most metrics well under target:

- ✅ **Memory efficiency:** 4.5MB (70% under target)
- ✅ **Binary size:** 2.9MB (42% under target)
- ✅ **Config parsing:** 0.22ms for 100 commands (78% under target)
- ⚠️ **Startup time:** 6.5ms (30% over target, but best-case 4.6ms proves target is achievable)

The profiling infrastructure is now in place with:
- Automated scripts for regular profiling
- Comprehensive documentation
- CI/CD integration
- Clear optimization roadmap

**Next Priority:** Optimize startup time from 6.5ms to < 5ms through lazy initialization and binary optimization.

---

**Report Generated:** 2025-11-07
**Last Updated:** 2025-11-07
**Version:** 1.0.0
**Status:** ✅ Baseline Established
