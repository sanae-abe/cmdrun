# cmdrun パフォーマンス最適化戦略

## 目標

- **起動時間**: 50ms以下（Node.js版の1/10）
- **メモリ使用量**: 10MB以下（Node.js版の1/20）
- **並列実行**: CPUコア数まで効率的にスケール
- **大規模プロジェクト**: 1000+コマンド定義でも高速動作

## 1. コンパイル時最適化

### LTO（Link Time Optimization）
```toml
[profile.release]
lto = "fat"              # 完全なLTO有効化
codegen-units = 1        # 単一コード生成ユニット
opt-level = 3            # 最大最適化
```

**効果**: バイナリサイズ30%削減、実行速度10-20%向上

### バイナリサイズ削減
```toml
[profile.release]
strip = true             # デバッグシンボル削除
panic = "abort"          # パニック時即終了（アンワインド不要）
```

**効果**: 2MB → 1.5MB程度に削減

## 2. ランタイム最適化

### 高速ハッシュマップ（ahash）
```rust
use ahash::AHashMap;

// 標準HashMap比で2-3倍高速
let mut env: AHashMap<String, String> = AHashMap::new();
```

**理由**:
- 暗号学的安全性不要（設定ファイル処理）
- SipHash（標準）よりAHashの方が高速

### スタック最適化ベクター（SmallVec）
```rust
use smallvec::{SmallVec, smallvec};

// 少数要素時はヒープアロケーション回避
type Args = SmallVec<[String; 4]>;
```

**効果**: 引数4個以下ならスタック上で処理、アロケーション削減

### LazyStatic/LazyLock（正規表現の事前コンパイル）
```rust
use std::sync::LazyLock;
use regex::Regex;

static VAR_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\$\{([A-Za-z_][A-Za-z0-9_]*)(:[?+\-])?([^}]*)?\}").unwrap()
});
```

**効果**: 初回のみコンパイル、以降は再利用で高速化

## 3. 非同期処理最適化

### Tokio マルチスレッドランタイム
```rust
#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
async fn main() {
    // 並列コマンド実行
}
```

**効果**: CPUバウンド・IOバウンドタスクの効率的並列化

### 並列コマンド実行
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

## 4. メモリ最適化

### String vs &str の適切な使い分け
```rust
// 読み取り専用 → &str
fn process_command(cmd: &str) -> Result<()> { ... }

// 所有権必要 → String
fn store_command(cmd: String) -> Command { ... }
```

### Clone回避（Arc/Rc活用）
```rust
use std::sync::Arc;

// 大きな設定を共有
let config = Arc::new(load_config());
let config_clone = Arc::clone(&config);  // ポインタコピーのみ
```

## 5. IO最適化

### バッファリング（BufReader/BufWriter）
```rust
use tokio::io::{BufReader, AsyncBufReadExt};

let reader = BufReader::new(stdout);
let mut lines = reader.lines();
```

**効果**: システムコール削減、IO性能向上

### 並列ファイル読み込み
```rust
let handles: Vec<_> = files
    .into_iter()
    .map(|file| tokio::spawn(read_file(file)))
    .collect();

let results = futures::future::join_all(handles).await;
```

## 6. ベンチマーク戦略

### Criterion.rs 活用
```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_interpolation(c: &mut Criterion) {
    let ctx = InterpolationContext::new(true);
    c.bench_function("interpolate simple", |b| {
        b.iter(|| ctx.interpolate(black_box("${USER}")))
    });
}

criterion_group!(benches, bench_interpolation);
criterion_main!(benches);
```

**測定項目**:
- 変数展開速度
- コマンドパース速度
- 設定ファイル読み込み速度
- 並列実行オーバーヘッド

### プロファイリング
```bash
# CPU プロファイル
cargo flamegraph --bin cmdrun -- run test

# メモリプロファイル
valgrind --tool=massif target/release/cmdrun run test

# ベンチマーク実行
cargo bench
```

## 7. 実測性能目標

### 起動時間
```
目標: 50ms以下
測定: hyperfine 'cmdrun --version'

結果目標:
  Time (mean ± σ):      45.2 ms ±   2.3 ms
```

### 単純コマンド実行
```
目標: Node.js版比で10倍高速
測定: hyperfine 'cmdrun run dev' 'npm run dev'

結果目標:
  cmdrun:    52.3 ms ±   1.8 ms
  npm:      523.1 ms ±  12.3 ms
  → 10倍高速達成
```

### メモリ使用量
```
目標: 10MB以下
測定: /usr/bin/time -v cmdrun run test

結果目標:
  Maximum resident set size: 8192 KB
```

### 並列実行スケーラビリティ
```
目標: 4コアで3.5倍以上の高速化
測定: 独立した4コマンドの並列実行

結果目標:
  逐次実行: 4.0s
  並列実行: 1.1s
  → 3.6倍高速化
```

## 8. 最適化チェックリスト

- [ ] LTO有効化（Cargo.toml）
- [ ] バイナリストリップ（Cargo.toml）
- [ ] ahashで高速HashMap
- [ ] SmallVecでスタック最適化
- [ ] LazyLockで正規表現事前コンパイル
- [ ] 非同期IO（Tokio）
- [ ] 並列実行（JoinSet）
- [ ] バッファリングIO
- [ ] 不要なClone削減
- [ ] Arc/Rcで共有
- [ ] Criterion.rsでベンチマーク
- [ ] Flamegraphでプロファイリング
- [ ] Hyperfineで実測性能確認

## 9. トレードオフ検討

### コンパイル時間 vs 実行速度
- **LTO**: コンパイル時間+30%、実行速度+15%
- **判断**: リリースビルドのみLTO有効化

### バイナリサイズ vs 機能
- **機能**: JSON出力、カラー対応、トレースログ
- **判断**: feature flagで選択可能に

### 並列度 vs 安定性
- **並列度**: CPUコア数まで
- **判断**: デフォルトは逐次、明示的に並列化

## 10. 継続的最適化

### CI/CDでの性能回帰検出
```yaml
- name: Performance benchmark
  run: |
    cargo bench --bench performance
    # 前回との比較、劣化検出
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
