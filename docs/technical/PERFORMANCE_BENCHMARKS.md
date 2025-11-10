# Performance Benchmarks

cmdrun maintains strict performance targets to ensure it remains a fast, lightweight command runner.

## 🎯 Performance Targets

| Metric | Target | Rationale |
|--------|--------|-----------|
| **Startup Time** | ≤ 4ms | Must be faster than shell alias overhead |
| **Memory Usage** | ≤ 10MB | Minimal impact on system resources |
| **Binary Size** | ≤ 5MB | Fast downloads and minimal disk usage |
| **Config Loading** | ≤ 1ms | Instant response for typical configs (< 100 commands) |

## 📊 Benchmark Categories

### 1. Startup Time Benchmarks (`startup_time.rs`)

Tests the time it takes for cmdrun to start and execute basic commands:

- **`version`**: `cmdrun --version` (fastest path)
- **`help`**: `cmdrun --help` (CLI parsing)
- **`list_no_config`**: `cmdrun list` without config file
- **`cold_startup`**: First-time config loading
- **`argument_parsing`**: Complex CLI argument processing
- **`config_loading`**: Config files of varying sizes (5-100 commands)

### 2. TOML Processing (`toml_parsing.rs`)

Measures configuration file processing performance:

- **Parsing**: TOML → Rust structures (10-200 commands)
- **Serialization**: Rust structures → TOML
- **Complex structures**: Nested config with dependencies
- **String operations**: Common string processing tasks
- **File I/O**: Reading and writing config files

### 3. Command Execution (`command_execution.rs`)

Tests command processing and execution:

- **Shell commands**: Basic process execution overhead
- **Variable interpolation**: Regex pattern matching and replacement
- **HashMap operations**: Dependency resolution lookups
- **Path operations**: Cross-platform path handling

## 🔧 Running Benchmarks

### Local Benchmarks

```bash
# Run all benchmarks
cargo bench

# Run specific benchmark suites
cargo bench --bench startup_time
cargo bench --bench toml_parsing
cargo bench --bench command_execution

# Generate HTML reports
cargo bench -- --output-format json > benchmarks.json
```

### Performance Testing Script

```bash
# Quick performance check
./scripts/performance_test.sh

# Test specific binary
./scripts/performance_test.sh ./target/release/cmdrun

# With custom iterations
ITERATIONS=20 ./scripts/performance_test.sh
```

### Automated CI/CD Testing

Performance tests run automatically:

- **On PR**: Regression detection against main branch
- **Weekly**: Comprehensive profiling and trend analysis
- **Manual**: On-demand performance validation

## 📈 CI/CD Integration

### GitHub Actions Workflows

1. **`.github/workflows/benchmark.yml`**:
   - Runs on every PR and main branch push
   - Measures startup time, memory usage, binary size
   - Compares against performance targets
   - Generates reports and artifacts

2. **Performance Artifacts**:
   - JSON benchmark results (90-day retention)
   - HTML performance reports
   - Memory profiling data (weekly)
   - Regression comparison reports

### Performance Gates

CI fails if any metric exceeds target:

```bash
# Binary size check
if [ $BINARY_SIZE -le 5242880 ]; then  # 5MB
  echo "✅ Binary size OK"
else
  echo "❌ Binary size exceeds 5MB"
  exit 1
fi

# Startup time check
if (( $(echo "$STARTUP_TIME <= 4.0" | bc -l) )); then
  echo "✅ Startup time OK"
else
  echo "❌ Startup time exceeds 4ms"
  exit 1
fi
```

## 🔍 Benchmark Analysis

### Understanding Results

1. **Startup Time**:
   - Measured over 10 iterations after 3 warmup runs
   - Uses high-precision timing (`date +%s%N`)
   - Accounts for process creation overhead

2. **Memory Usage**:
   - Peak RSS (Resident Set Size) measurement
   - Uses GNU time: `/usr/bin/time -f "%M"`
   - Includes all loaded libraries and data

3. **Binary Size**:
   - Release build with LTO and optimizations
   - Strip debug symbols in production builds
   - Measured with `stat` command

### Regression Detection

Performance regressions are detected when:

- **Startup time** increases by >10%
- **Memory usage** increases by >10%
- **Binary size** increases by >5%

### Historical Tracking

- Benchmark results stored as GitHub Actions artifacts
- Performance trends tracked over time
- Automatic alerts for sustained regressions

## 🛠️ Optimization Techniques

### Startup Time Optimization

1. **Lazy Loading**:
   ```rust
   // Load config only when needed
   let config = if needs_config(args) {
       Some(load_config()?)
   } else {
       None
   };
   ```

2. **Fast Paths**:
   ```rust
   // Quick exit for version/help
   match args.command {
       Command::Version => return print_version(),
       Command::Help => return print_help(),
       _ => {} // Continue with full initialization
   }
   ```

3. **Minimal Dependencies**:
   - Use `ahash` instead of default hasher
   - Prefer `smallvec` for small collections
   - Avoid heavy dependencies in hot paths

### Memory Optimization

1. **String Interning**:
   ```rust
   // Reuse common strings
   static COMMON_COMMANDS: &[&str] = &["build", "test", "dev"];
   ```

2. **Efficient Data Structures**:
   ```rust
   // Use references where possible
   pub struct Command<'a> {
       name: &'a str,
       description: &'a str,
   }
   ```

3. **Arena Allocation**:
   ```rust
   // For temporary allocations during config parsing
   use bumpalo::Bump;
   let arena = Bump::new();
   ```

### Binary Size Optimization

1. **Link-Time Optimization (LTO)**:
   ```toml
   [profile.release]
   lto = true
   codegen-units = 1
   panic = "abort"
   strip = true
   ```

2. **Feature Gating**:
   ```rust
   #[cfg(feature = "plugin-system")]
   mod plugins;
   ```

3. **Dead Code Elimination**:
   ```bash
   # Check for unused dependencies
   cargo +nightly udeps

   # Analyze binary size
   cargo bloat --release --crates
   ```

## 📋 Benchmark Maintenance

### Adding New Benchmarks

1. Create benchmark in `benches/` directory
2. Use Criterion framework for consistent measurement
3. Add to CI workflow if testing critical path
4. Document expected performance characteristics

### Updating Performance Targets

Performance targets should be updated when:

- Significant new features justify increased resource usage
- Infrastructure improvements enable better performance
- User feedback indicates current targets are too strict/loose

### Troubleshooting Performance Issues

1. **Profile with Multiple Tools**:
   ```bash
   # CPU profiling
   cargo bench --bench startup_time -- --profile-time=5

   # Memory profiling
   valgrind --tool=massif target/release/cmdrun --version

   # System-level profiling
   perf record target/release/cmdrun list
   ```

2. **Analyze Hot Paths**:
   ```rust
   // Add timing to suspect functions
   let start = std::time::Instant::now();
   expensive_operation();
   println!("Operation took: {:?}", start.elapsed());
   ```

3. **Compare Against Baseline**:
   ```bash
   # Generate baseline
   git checkout main
   cargo bench --bench startup_time > baseline.txt

   # Compare changes
   git checkout feature-branch
   cargo bench --bench startup_time > current.txt

   # Analyze difference
   ./scripts/benchmark_comparison.py current.json baseline.json
   ```

## 🔗 Related Documentation

- [Architecture](ARCHITECTURE.md) - System design principles
- [Performance Guide](PERFORMANCE_GUIDE.md) - User performance tuning
- [Build Configuration](../BUILD_CONFIGURATION.md) - Optimization settings
- [Profiling Guide](../PROFILING_GUIDE.md) - Advanced performance analysis