# cmdrun Performance Guide

A comprehensive guide to optimizing cmdrun performance for large-scale projects.

## Table of Contents

- [Performance Overview](#performance-overview)
- [Benchmarking](#benchmarking)
- [Profiling](#profiling)
- [Optimization Techniques](#optimization-techniques)
- [Large-Scale Projects](#large-scale-projects)
- [Platform-Specific Optimizations](#platform-specific-optimizations)
- [Troubleshooting Performance Issues](#troubleshooting-performance-issues)

---

## Performance Overview

### Design Goals

cmdrun is designed for **high performance** with the following targets:

| Metric | Target | Typical Result |
|--------|--------|----------------|
| Startup Time | < 5ms | ~4ms |
| Memory Footprint (Idle) | < 15MB | ~10MB |
| Config Parse Time | < 1ms | ~0.5ms (100 commands) |
| Command Execution Overhead | < 1ms | ~0.3ms |
| Binary Size | < 5MB | ~3MB (stripped) |

### Why cmdrun is Fast

**1. Native Binary (Rust):**
- No runtime overhead (unlike Node.js/Python)
- Ahead-of-time compilation
- Zero-cost abstractions

**2. Optimized Build:**
```toml
[profile.release]
opt-level = 3          # Maximum optimization
lto = "fat"            # Link-time optimization
codegen-units = 1      # Single codegen unit
strip = true           # Strip symbols
panic = "abort"        # Smaller binary
```

**3. Lazy Initialization:**
- Regex patterns compiled once (`once_cell::Lazy`)
- Config loaded only when needed
- Minimal static data

**4. Efficient Data Structures:**
- `ahash` for faster hashing
- `smallvec` for stack allocation
- Minimal allocations

**5. Async I/O:**
- Non-blocking file operations
- Efficient parallel execution
- Tokio multi-threaded runtime

---

## Benchmarking

### Measuring Startup Time

**Using `time` command:**
```bash
# macOS/Linux
time cmdrun --version

# Typical output:
# real    0m0.004s  # 4ms total
# user    0m0.001s
# sys     0m0.002s
```

**Using `hyperfine` (recommended):**
```bash
# Install hyperfine
cargo install hyperfine

# Benchmark startup
hyperfine 'cmdrun --version'

# Compare with other tools
hyperfine \
  'cmdrun --version' \
  'npm --version' \
  'make --version'
```

**Example results:**
```
Benchmark 1: cmdrun --version
  Time (mean ± σ):       4.2 ms ±   0.3 ms    [User: 1.1 ms, System: 2.8 ms]
  Range (min … max):     3.8 ms …   5.1 ms    500 runs

Benchmark 2: npm --version
  Time (mean ± σ):     121.4 ms ±   8.2 ms    [User: 89.3 ms, System: 28.1 ms]
  Range (min … max):   112.3 ms … 145.7 ms    25 runs

Summary
  'cmdrun --version' ran
   28.90 ± 2.14 times faster than 'npm --version'
```

---

### Measuring Config Parse Time

```bash
# Create large config
for i in {1..1000}; do
  echo "[commands.cmd$i]" >> commands.toml
  echo 'cmd = "echo test"' >> commands.toml
done

# Benchmark list command (parses config)
hyperfine 'cmdrun -c commands.toml list'
```

**Expected results:**
- 100 commands: ~0.5ms parse time
- 500 commands: ~2ms parse time
- 1000 commands: ~4ms parse time

---

### Measuring Command Execution Overhead

```bash
# Measure overhead
hyperfine 'cmdrun run echo' 'echo test'

# Example output:
# cmdrun run echo:  5.3 ms
# echo test:        1.2 ms
# Overhead:         4.1 ms (mostly startup)
```

**Breaking down the overhead:**
- Binary startup: ~4ms
- Config parsing: ~0.5ms
- Variable interpolation: ~0.1ms
- Process spawn: ~0.3ms
- **Total:** ~5ms

---

### Memory Profiling

**Using `time -v` (Linux):**
```bash
/usr/bin/time -v cmdrun run echo

# Look for:
# Maximum resident set size: 10240 kbytes  # ~10MB
```

**Using macOS Activity Monitor:**
1. Run `cmdrun watch build --pattern "**/*.rs"`
2. Open Activity Monitor
3. Find cmdrun process
4. Check Memory column

**Expected memory usage:**
- Idle (watching): 10-15MB
- Parsing large config (1000 commands): +5MB
- Running parallel commands: varies by commands

---

## Profiling

### Using perf (Linux)

**1. Install perf:**
```bash
sudo apt install linux-tools-common linux-tools-generic
```

**2. Build with debug symbols:**
```bash
cargo build --profile release-with-debug
```

**3. Profile:**
```bash
# Record profile
perf record --call-graph=dwarf ./target/release-with-debug/cmdrun run build

# Analyze
perf report
```

**4. Look for:**
- Hot functions (high % samples)
- Unexpected allocations
- Regex compilation in hot path

---

### Using Instruments (macOS)

**1. Build with debug symbols:**
```bash
cargo build --profile release-with-debug
```

**2. Profile with Instruments:**
```bash
# Time Profiler
instruments -t "Time Profiler" ./target/release-with-debug/cmdrun run build

# Allocations
instruments -t "Allocations" ./target/release-with-debug/cmdrun run build
```

**3. Analyze in Instruments GUI:**
```bash
open /Applications/Xcode.app/Contents/Applications/Instruments.app
```

---

### Using flamegraph

**1. Install flamegraph:**
```bash
cargo install flamegraph
```

**2. Generate flamegraph:**
```bash
# Requires root/sudo on Linux
sudo cargo flamegraph -- run build

# Opens flamegraph.svg in browser
```

**3. Interpret:**
- Width = time spent
- Height = call stack depth
- Look for wide bars (hot paths)

---

### Using cargo-flamegraph

```bash
# Profile specific code
cargo build --release
sudo flamegraph ./target/release/cmdrun run build

# With custom flags
sudo flamegraph -o my-flamegraph.svg -- \
  ./target/release/cmdrun run build
```

---

## Optimization Techniques

### 1. Config File Optimization

**Problem:** Large config file (5000 lines) slows down parsing.

**Solutions:**

**A. Split config files:**
```bash
# Instead of one large file
~/.config/cmdrun/commands.toml (5000 lines)

# Split by category
~/.config/cmdrun/docker.toml (500 lines)
~/.config/cmdrun/k8s.toml (500 lines)
~/.config/cmdrun/dev.toml (500 lines)

# Use different configs
cmdrun -c ~/.config/cmdrun/docker.toml list
cmdrun -c ~/.config/cmdrun/k8s.toml run deploy
```

**B. Remove unused commands:**
```bash
# Find commands never used
# (requires logging - future feature)
cmdrun analyze-usage

# Remove manually
cmdrun remove old-command
```

**C. Optimize TOML structure:**
```toml
# ❌ Inefficient (redundant data)
[commands.test-unit]
cmd = "cargo test --lib"
description = "Run unit tests"
deps = []
parallel = false
timeout = 300

[commands.test-integration]
cmd = "cargo test --test '*'"
description = "Run integration tests"
deps = []
parallel = false
timeout = 300

# ✅ Efficient (minimal data, use defaults)
[commands.test-unit]
cmd = "cargo test --lib"

[commands.test-integration]
cmd = "cargo test --test '*'"
```

---

### 2. Dependency Graph Optimization

**Problem:** Deep dependency chains slow down resolution.

**Example Problem:**
```toml
[commands.deploy]
deps = ["build"]

[commands.build]
deps = ["test"]

[commands.test]
deps = ["lint"]

[commands.lint]
deps = ["format"]

[commands.format]
cmd = "cargo fmt"

# Depth: 5 levels
# Resolution time: ~0.5ms per level = 2.5ms
```

**Solution:** Flatten where possible:
```toml
[commands.deploy]
deps = ["build", "test", "lint"]  # Parallel resolution

[commands.build]
cmd = "cargo build"

[commands.test]
cmd = "cargo test"

[commands.lint]
cmd = "cargo clippy"

# Depth: 2 levels
# Resolution time: ~1ms
```

---

### 3. Parallel Execution

**Problem:** Sequential execution is slow.

**Sequential (slow):**
```toml
[commands.check-all]
cmd = [
    "cargo fmt -- --check",
    "cargo clippy",
    "cargo test"
]
# Total time: 15s (5s + 5s + 5s)
```

**Parallel (fast):**
```toml
[commands.check-all]
parallel = true
cmd = [
    "cargo fmt -- --check",
    "cargo clippy",
    "cargo test"
]
# Total time: 5s (max of all)
```

**Benchmark:**
```bash
# Sequential
hyperfine 'cmdrun run check-all-seq'
# Result: 15.2s

# Parallel
hyperfine 'cmdrun run check-all-par'
# Result: 5.3s

# Speedup: 2.87x
```

---

### 4. Watch Mode Optimization

**Problem:** Watch mode uses too much CPU/memory.

**Solutions:**

**A. Narrow watch patterns:**
```bash
# ❌ Too broad (watches everything)
cmdrun watch build --pattern "**/*"

# ✅ Specific (watches only relevant files)
cmdrun watch build --pattern "**/*.rs"
```

**B. Increase debounce:**
```bash
# ❌ Too aggressive (500ms default)
cmdrun watch build --pattern "**/*.rs"

# ✅ Less frequent (1000ms)
cmdrun watch build --pattern "**/*.rs" --debounce 1000
```

**C. Use ignore patterns:**
```bash
# Exclude large directories
cmdrun watch build \
  --pattern "**/*.rs" \
  --ignore "**/target/**" \
  --ignore "**/node_modules/**"
```

**D. Respect .gitignore (default):**
```gitignore
# .gitignore
target/
*.tmp
*.swp
```

---

### 5. Variable Interpolation Optimization

**Problem:** Complex variable expansion slows down execution.

**Slow:**
```toml
[commands.complex]
cmd = "echo ${A} ${B} ${C} ${D} ${E} ${F} ${G} ${H} ${I} ${J}"
```

**Optimized:**
```toml
# Use fewer variables
[commands.simple]
cmd = "echo ${MESSAGE}"

# Or use config.env
[config.env]
MESSAGE = "Hello World"
```

**Benchmark:**
```bash
# 10 variables: ~0.3ms interpolation
# 2 variables:  ~0.1ms interpolation
```

---

## Large-Scale Projects

### Handling 1000+ Commands

**1. Use namespaces:**
```toml
[commands.project-a:build]
[commands.project-a:test]
[commands.project-a:deploy]

[commands.project-b:build]
[commands.project-b:test]
[commands.project-b:deploy]
```

**2. Use multiple config files:**
```bash
# Per-project configs
~/projects/project-a/commands.toml
~/projects/project-b/commands.toml

# Switch configs
cd ~/projects/project-a
cmdrun -c ./commands.toml run build
```

**3. Use shell aliases:**
```bash
# ~/.bashrc
alias cmdrun-a='cmdrun -c ~/projects/project-a/commands.toml'
alias cmdrun-b='cmdrun -c ~/projects/project-b/commands.toml'

# Usage
cmdrun-a run build
cmdrun-b run deploy
```

---

### Monorepo Support

**Strategy 1: Root config with working_dir:**
```toml
# Root commands.toml
[commands.frontend:build]
cmd = "npm run build"
working_dir = "./frontend"

[commands.backend:build]
cmd = "cargo build"
working_dir = "./backend"

[commands.build-all]
parallel = true
deps = ["frontend:build", "backend:build"]
```

**Strategy 2: Multiple configs:**
```bash
# Project structure
monorepo/
├── commands.toml          # Root commands
├── frontend/
│   └── commands.toml      # Frontend commands
└── backend/
    └── commands.toml      # Backend commands

# Usage
cmdrun run build-all                     # Root
cmdrun -c frontend/commands.toml run dev # Frontend
cmdrun -c backend/commands.toml run dev  # Backend
```

---

### CI/CD Optimization

**Problem:** CI runs slow with cmdrun.

**Solutions:**

**1. Cache binary:**
```yaml
# .github/workflows/ci.yml
- name: Cache cmdrun
  uses: actions/cache@v3
  with:
    path: ~/.cargo/bin/cmdrun
    key: cmdrun-${{ runner.os }}-v1.0.0

- name: Install cmdrun
  if: steps.cache.outputs.cache-hit != 'true'
  run: cargo install cmdrun
```

**2. Use pre-built binaries (when available):**
```yaml
- name: Download cmdrun
  run: |
    wget https://github.com/sanae-abe/cmdrun/releases/download/v1.0.0/cmdrun-linux-x86_64
    chmod +x cmdrun-linux-x86_64
    sudo mv cmdrun-linux-x86_64 /usr/local/bin/cmdrun
```

**3. Optimize parallel jobs:**
```yaml
jobs:
  test:
    strategy:
      matrix:
        os: [ubuntu-latest, macos-latest, windows-latest]
    steps:
      - run: cmdrun run test  # Parallel across OSes
```

---

## Platform-Specific Optimizations

### Linux

**1. Use release build:**
```bash
cargo build --release
# vs
cargo build  # Debug build is 10x slower
```

**2. Use musl for smaller binaries:**
```bash
rustup target add x86_64-unknown-linux-musl
cargo build --release --target x86_64-unknown-linux-musl

# Result: 3MB vs 5MB (glibc)
```

**3. Optimize for CPU:**
```bash
# Build for current CPU
RUSTFLAGS="-C target-cpu=native" cargo build --release
```

---

### macOS

**1. Universal binary (Intel + Apple Silicon):**
```bash
# Build for both architectures
cargo build --release --target x86_64-apple-darwin
cargo build --release --target aarch64-apple-darwin

# Create universal binary
lipo -create \
  target/x86_64-apple-darwin/release/cmdrun \
  target/aarch64-apple-darwin/release/cmdrun \
  -output cmdrun-universal
```

**2. Optimize for Apple Silicon:**
```bash
# Native build on M1/M2
cargo build --release --target aarch64-apple-darwin
```

---

### Windows

**1. Use `--target` for consistent builds:**
```powershell
cargo build --release --target x86_64-pc-windows-msvc
```

**2. Strip binary:**
```powershell
# Install strip tool
cargo install cargo-strip

# Strip binary
cargo strip --release
```

---

## Troubleshooting Performance Issues

### Slow Startup

**Symptom:** `cmdrun --version` takes >100ms

**Diagnosis:**
```bash
# Measure
time cmdrun --version

# Check binary size
ls -lh ~/.cargo/bin/cmdrun

# Check if debug build
file ~/.cargo/bin/cmdrun
```

**Solutions:**
```bash
# Rebuild with optimizations
cd cmdrun
cargo clean
cargo build --release
cargo install --path . --force

# Verify
time cmdrun --version  # Should be <10ms
```

---

### Slow Config Parsing

**Symptom:** `cmdrun list` takes >100ms

**Diagnosis:**
```bash
# Measure
hyperfine 'cmdrun list'

# Check config size
wc -l ~/.config/cmdrun/commands.toml

# Profile
cargo build --profile release-with-debug
perf record ./target/release-with-debug/cmdrun list
perf report
```

**Solutions:**
- Split large config files
- Remove unused commands
- Simplify TOML structure

---

### High Memory Usage

**Symptom:** cmdrun uses >100MB

**Diagnosis:**
```bash
# Linux
/usr/bin/time -v cmdrun run build

# macOS
/usr/bin/time -l cmdrun run build
```

**Solutions:**
- Check for memory leaks (run valgrind)
- Reduce parallel command count
- Increase system limits if needed

---

### Slow Watch Mode

**Symptom:** High CPU usage in watch mode

**Diagnosis:**
```bash
# Monitor CPU
top -p $(pgrep cmdrun)

# Check watch pattern
cmdrun watch build --pattern "**/*" --verbose
```

**Solutions:**
```bash
# Narrow pattern
cmdrun watch build --pattern "**/*.rs"

# Increase debounce
cmdrun watch build --pattern "**/*.rs" --debounce 1000

# Reduce watch scope
cmdrun watch build --path src --pattern "**/*.rs"
```

---

## Performance Checklist

### Build-Time Optimizations
- [ ] Use `--release` build
- [ ] Enable LTO (`lto = "fat"`)
- [ ] Use single codegen unit (`codegen-units = 1`)
- [ ] Strip symbols (`strip = true`)
- [ ] Consider PGO (Profile-Guided Optimization)

### Config Optimizations
- [ ] Keep config files under 1000 commands
- [ ] Use multiple config files for large projects
- [ ] Remove unused commands
- [ ] Flatten deep dependency trees
- [ ] Use minimal TOML (rely on defaults)

### Runtime Optimizations
- [ ] Use parallel execution where possible
- [ ] Optimize variable interpolation
- [ ] Use efficient watch patterns
- [ ] Set appropriate debounce times
- [ ] Leverage .gitignore for watch mode

### Platform Optimizations
- [ ] Use target-specific builds
- [ ] Enable CPU-specific optimizations
- [ ] Use musl on Linux (smaller binary)
- [ ] Create universal binaries on macOS
- [ ] Cache binaries in CI/CD

---

## Future Optimizations

### Planned Features
- [ ] Config caching (parse once, cache)
- [ ] Command usage analytics (remove unused)
- [ ] Incremental parsing (only parse changed sections)
- [ ] JIT compilation for hot paths
- [ ] Binary compression

### Research Areas
- Profile-Guided Optimization (PGO)
- SIMD for string operations
- Alternative TOML parsers
- Lazy loading of command definitions

---

## Japanese Optimization Guide (日本語最適化ガイド)

### パフォーマンス目標

- **起動時間**: 50ms以下（Node.js版の1/10）
- **メモリ使用量**: 10MB以下（Node.js版の1/20）
- **並列実行**: CPUコア数まで効率的にスケール
- **大規模プロジェクト**: 1000+コマンド定義でも高速動作

### コンパイル時最適化

#### LTO（Link Time Optimization）
```toml
[profile.release]
lto = "fat"              # 完全なLTO有効化
codegen-units = 1        # 単一コード生成ユニット
opt-level = 3            # 最大最適化
```

**効果**: バイナリサイズ30%削減、実行速度10-20%向上

#### バイナリサイズ削減
```toml
[profile.release]
strip = true             # デバッグシンボル削除
panic = "abort"          # パニック時即終了（アンワインド不要）
```

**効果**: 2MB → 1.5MB程度に削減

### ランタイム最適化

#### 高速ハッシュマップ（ahash）
```rust
use ahash::AHashMap;

// 標準HashMap比で2-3倍高速
let mut env: AHashMap<String, String> = AHashMap::new();
```

**理由**: 暗号学的安全性不要（設定ファイル処理）、SipHash（標準）よりAHashの方が高速

#### スタック最適化ベクター（SmallVec）
```rust
use smallvec::{SmallVec, smallvec};

// 少数要素時はヒープアロケーション回避
type Args = SmallVec<[String; 4]>;
```

**効果**: 引数4個以下ならスタック上で処理、アロケーション削減

#### LazyStatic/LazyLock（正規表現の事前コンパイル）
```rust
use std::sync::LazyLock;
use regex::Regex;

static VAR_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\$\{([A-Za-z_][A-Za-z0-9_]*)(:[?+\-])?([^}]*)?\}").unwrap()
});
```

**効果**: 初回のみコンパイル、以降は再利用で高速化

### 非同期処理最適化

#### Tokio マルチスレッドランタイム
```rust
#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
async fn main() {
    // 並列コマンド実行
}
```

**効果**: CPUバウンド・IOバウンドタスクの効率的並列化

#### 並列コマンド実行
```rust
use tokio::task::JoinSet;

let mut set = JoinSet::new();
for cmd in parallel_commands {
    set.spawn(execute_command(cmd));
}

while let Some(result) = set.join_next().await {
    // 結果処理
}
```

**効果**: 独立コマンドの完全並列実行

### メモリ最適化

#### String vs &str の適切な使い分け
```rust
// 読み取り専用 → &str
fn process_command(cmd: &str) -> Result<()> { ... }

// 所有権必要 → String
fn store_command(cmd: String) -> Command { ... }
```

#### Clone回避（Arc/Rc活用）
```rust
use std::sync::Arc;

// 大きな設定を共有
let config = Arc::new(load_config());
let config_clone = Arc::clone(&config);  // ポインタコピーのみ
```

### プロファイル駆動最適化（PGO）
```bash
# プロファイルデータ収集
RUSTFLAGS="-Cprofile-generate=/tmp/pgo-data" cargo build --release
./target/release/cmdrun run test

# PGOビルド
RUSTFLAGS="-Cprofile-use=/tmp/pgo-data/merged.profdata" cargo build --release
```

**効果**: 10-20%の追加高速化

---

## Related Documentation

- [Architecture](ARCHITECTURE.md) - System design and internals
- [Performance Benchmarks](PERFORMANCE_BENCHMARKS.md) - Benchmark results
- [User Guide](../user-guide/) - Usage documentation

---

**Last Updated:** 2025-11-15
**Version:** 1.0.0
