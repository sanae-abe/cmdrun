# cmdrun 実装開始ガイド

このドキュメントでは、cmdrun Rust+TOML版の実装を開始するための手順を説明します。

## 📋 前提条件

### 必須ツール
```bash
# Rust 1.75+ インストール
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup update stable

# バージョン確認
rustc --version  # 1.75以上

# 開発ツール
cargo install cargo-watch    # ホットリロード
cargo install cargo-audit    # セキュリティ監査
cargo install cargo-deny     # 依存関係管理
cargo install cargo-flamegraph  # プロファイリング
```

### 推奨ツール
```bash
# ベンチマーク
cargo install cargo-criterion

# クロスコンパイル
cargo install cross

# コードカバレッジ
cargo install cargo-tarpaulin

# 実測性能
brew install hyperfine  # macOS
# or
apt install hyperfine   # Ubuntu
```

## 🚀 実装手順

### Step 1: プロジェクト初期化（Day 1）

```bash
# このディレクトリで作業開始
cd /Users/sanae.abe/Scripts/cmdrun-rust-design

# ビルド確認
cargo build

# テスト実行
cargo test

# フォーマット
cargo fmt

# Lint
cargo clippy
```

### Step 2: 基盤実装（Day 1-2）

#### 2.1 エラー型実装
```bash
# src/error.rs は完成
cargo test --lib error
```

#### 2.2 設定スキーマ実装
```bash
# src/config/schema.rs は完成
# 追加タスク: src/config/mod.rs 作成
```

```rust
// src/config/mod.rs
pub mod schema;
pub mod loader;
pub mod validation;

pub use schema::{CommandsConfig, Command, Platform};
pub use loader::ConfigLoader;
```

#### 2.3 loader.rs 実装
```rust
// src/config/loader.rs
use crate::config::schema::CommandsConfig;
use crate::error::Result;
use std::path::{Path, PathBuf};

pub struct ConfigLoader {
    search_paths: Vec<PathBuf>,
}

impl ConfigLoader {
    pub fn new() -> Self {
        Self {
            search_paths: vec![
                PathBuf::from(".cmdrun.toml"),
                PathBuf::from("cmdrun.toml"),
                PathBuf::from("commands.toml"),
            ],
        }
    }

    pub async fn load(&self) -> Result<CommandsConfig> {
        for path in &self.search_paths {
            if path.exists() {
                let content = tokio::fs::read_to_string(path).await?;
                let config: CommandsConfig = toml::from_str(&content)?;
                return Ok(config);
            }
        }

        Err(crate::error::ConfigError::FileNotFound(
            PathBuf::from("commands.toml")
        ).into())
    }
}
```

### Step 3: コマンド実行実装（Day 3-7）

#### 3.1 変数展開テスト
```bash
cargo test --lib command::interpolation
```

#### 3.2 executor.rs テスト
```bash
# 修正箇所: main.rs の依存関係追加
cargo add clap_complete  # シェル補完用

cargo test --lib command::executor
```

#### 3.3 統合テスト作成
```rust
// tests/integration/basic.rs
use cmdrun::config::schema::{Command, CommandSpec};
use cmdrun::command::executor::{CommandExecutor, ExecutionContext};

#[tokio::test]
async fn test_simple_echo() {
    let ctx = ExecutionContext::default();
    let executor = CommandExecutor::new(ctx);

    let cmd = Command {
        description: "Test".to_string(),
        cmd: CommandSpec::Single("echo hello".to_string()),
        // ... 他のフィールド
    };

    let result = executor.execute(&cmd).await.unwrap();
    assert!(result.success);
}
```

### Step 4: CLI実装（Day 8-9）

```bash
# main.rs の修正
# - テンプレートファイル追加
mkdir -p templates
cp examples/commands.toml templates/commands.toml

# ビルド＆実行
cargo run -- --help
cargo run -- list
```

### Step 5: モジュール補完（Day 10-14）

#### 必要なファイル作成

```bash
# プラットフォーム対応
cat > src/platform/mod.rs <<'EOF'
pub mod shell;
pub mod unix;
pub mod windows;

pub use shell::detect_shell;
EOF

cat > src/platform/shell.rs <<'EOF'
pub fn detect_shell() -> String {
    if cfg!(windows) {
        if which::which("pwsh").is_ok() {
            "pwsh".to_string()
        } else {
            "cmd".to_string()
        }
    } else {
        std::env::var("SHELL")
            .ok()
            .and_then(|s| s.split('/').last().map(String::from))
            .unwrap_or_else(|| "bash".to_string())
    }
}
EOF

# 出力モジュール
mkdir -p src/output
cat > src/output/mod.rs <<'EOF'
pub mod formatter;
pub mod logger;
EOF

# ユーティリティ
cat > src/utils.rs <<'EOF'
// ユーティリティ関数
EOF

# コマンドモジュール
cat > src/command/mod.rs <<'EOF'
pub mod executor;
pub mod interpolation;

pub use executor::{CommandExecutor, ExecutionContext};
pub use interpolation::{InterpolationContext, interpolate};
EOF
```

## 🧪 テスト実行

### 単体テスト
```bash
cargo test --lib
```

### 統合テスト
```bash
cargo test --test integration
```

### すべてのテスト
```bash
cargo test
```

## 📊 パフォーマンス測定

### ベンチマーク
```bash
# benches/performance.rs 作成後
cargo bench
```

### プロファイリング
```bash
cargo flamegraph --bin cmdrun -- run test
```

### 実測性能
```bash
# リリースビルド
cargo build --release

# 起動時間測定
hyperfine './target/release/cmdrun --version'
```

## 🔧 開発ワークフロー

### 日次ルーチン
```bash
# 1. コード更新
git pull

# 2. テスト実行
cargo test

# 3. 機能実装
# ... コーディング ...

# 4. フォーマット＆Lint
cargo fmt
cargo clippy --fix

# 5. コミット
git add .
git commit -m "feat: implement feature X"
git push
```

### CI/CD確認
```bash
# .github/workflows/ci.yml 作成
# - cargo test
# - cargo clippy
# - cargo build --release
```

## 📖 ドキュメント参照

### 実装時の参照順序

1. **DESIGN_SUMMARY.md** - 全体像把握
2. **docs/ARCHITECTURE.md** - アーキテクチャ理解（作成予定）
3. **examples/commands.toml** - TOML設定例
4. **src/config/schema.rs** - スキーマ定義
5. **src/command/interpolation.rs** - 変数展開ロジック
6. **src/command/executor.rs** - 実行エンジン
7. **docs/PERFORMANCE.md** - 最適化戦略
8. **docs/SECURITY.md** - セキュリティ設計

## 🎯 マイルストーン

### Week 1 完了条件
- [x] プロジェクトビルド成功
- [x] エラー型定義完了
- [x] 設定スキーマ定義完了
- [ ] TOML読み込み実装
- [ ] 変数展開動作
- [ ] 単一コマンド実行成功

### Week 2 完了条件
- [ ] CLI完全動作
- [ ] 複数コマンド実行
- [ ] 依存関係解決
- [ ] プラットフォーム対応
- [ ] カラー出力

### MVP完成条件
- [ ] 全単体テスト通過
- [ ] 統合テスト5件以上
- [ ] `cmdrun run <command>` 動作
- [ ] `cmdrun list` 動作
- [ ] README完成
- [ ] 起動時間 < 100ms

## 🐛 トラブルシューティング

### ビルドエラー
```bash
# 依存関係更新
cargo update

# クリーンビルド
cargo clean && cargo build
```

### テスト失敗
```bash
# 詳細出力
cargo test -- --nocapture

# 特定テストのみ
cargo test test_name -- --nocapture
```

### パフォーマンス問題
```bash
# リリースビルド確認
cargo build --release
./target/release/cmdrun --version

# プロファイル
cargo flamegraph --bin cmdrun
```

## 📞 サポート

### 質問・議論
- GitHub Discussions（作成予定）
- Issue Tracker

### コントリビューション
- CONTRIBUTING.md 参照（作成予定）
- コードレビュー歓迎

## 次のステップ

1. **今すぐ開始**: `cargo build` を実行
2. **テスト作成**: 統合テストから着手
3. **機能実装**: MVP完成を目指す
4. **フィードバック**: 早期ユーザーテスト

Happy Coding! 🦀
