# cmdrun - 高速、安全、クロスプラットフォームなコマンドランナー

[English](README.md) | [日本語](README.ja.md)

> Rustで書かれた、`package.json`スクリプトやMakefileのモダンな代替ツールです。

[![Crates.io](https://img.shields.io/crates/v/cmdrun.svg)](https://crates.io/crates/cmdrun)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE)
[![Build Status](https://github.com/sanae-abe/cmdrun/workflows/CI/badge.svg)](https://github.com/sanae-abe/cmdrun/actions)

## 目次

- [なぜcmdrunなのか？](#なぜcmdrunなのか)
- [クイックスタート](#クイックスタート)
- [機能](#機能)
- [ドキュメント](#ドキュメント)
- [比較](#比較)
- [パフォーマンスベンチマーク](#パフォーマンスベンチマーク)
- [使用例](#使用例)
- [コントリビューション](#コントリビューション)
- [ライセンス](#ライセンス)

## なぜcmdrunなのか？

### 🚀 パフォーマンス
- **起動時間が約29倍高速** - Node.jsベースのタスクランナーと比較
- **起動時間4ms** - npm/yarnの115ms以上と比較
- **メモリフットプリント10MB** - Node.jsの200MB以上と比較

### 🔒 セキュリティ
- **`eval()`ゼロ** - 動的コード実行なし
- **安全な変数展開** - シェルインジェクション脆弱性なし
- **依存関係監査** - ビルトインセキュリティスキャン

### 🌍 クロスプラットフォーム
- **あらゆる環境で動作**: Linux、macOS、Windows、FreeBSD
- **シェル検出**: bash/zsh/fish/pwshを自動検出
- **ネイティブバイナリ**: ランタイム依存なし

### 💎 開発者体験
- **TOML設定** - 型安全で読みやすい
- **強力な機能** - 依存関係、並列実行、フック
- **優れたエラー表示** - コンテキスト付き詳細エラーメッセージ

## クイックスタート

### システム要件

- **オペレーティングシステム**: Linux、macOS、Windows、FreeBSD
- **ソースからのビルド用**: Rust 1.70以上（MSRV）

### インストール

#### Rustツールチェーンのインストール（未インストールの場合）

```bash
# 1. Rustup（Rustインストーラー）をダウンロード・実行
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 2. 環境変数を読み込み
source ~/.cargo/env

# 新しいターミナルを開くか、以下を実行
# bash使用時
source ~/.bashrc

# zsh使用時（macOS標準）
source ~/.zshrc

# 3. インストール確認
rustc --version
cargo --version
```

#### cmdrunのインストール

**方法1: ソースからインストール（開発推奨）**

```bash
# 1. リポジトリをクローン
git clone ssh://git@rendezvous.m3.com:3789/sanae-abe/cmdrun.git
cd cmdrun

# 2. ビルド&インストール
cargo install --path .

# 3. 動作確認
cmdrun --version
cmdrun --help
```

**方法2: crates.ioからインストール**

```bash
cargo install cmdrun
```

#### アップデート

```bash
# ソースからインストールした場合
cd cmdrun  # プロジェクトディレクトリ
git pull

# 再ビルド&インストール
cargo install --path . --force
```

<!-- 将来のインストール方法（利用可能になるまでコメントアウト）
#### Homebrew (macOS/Linux)
```bash
brew install sanae-abe/tap/cmdrun
```

#### Scoop (Windows)
```bash
scoop bucket add cmdrun https://github.com/sanae-abe/scoop-bucket
scoop install cmdrun
```
-->

### 基本的な使い方

1. プロジェクトに`commands.toml`を作成します：

```toml
[config]
language = "japanese"  # オプション: "english"（デフォルト）または "japanese"

[commands.dev]
description = "開発サーバーを起動"
cmd = "npm run dev"

[commands.build]
description = "本番用ビルド"
cmd = [
    "npm run type-check",
    "npm run lint",
    "npm run build",
]

[commands.test]
description = "テストを実行"
cmd = "cargo test --all-features"
```

2. コマンドを実行します：

```bash
# コマンドを実行
cmdrun run dev

# 利用可能なコマンドをリスト表示
cmdrun list

# 設定管理
cmdrun config show              # 全設定を表示
cmdrun config get language      # 特定の設定を取得
cmdrun config set language japanese  # 言語を日本語に変更

# ヘルプを表示
cmdrun --help
```

## 機能

### 変数展開

```toml
[commands.deploy]
cmd = "scp dist/ ${DEPLOY_USER:?DEPLOY_USERが設定されていません}@${DEPLOY_HOST:?DEPLOY_HOSTが設定されていません}:${DEPLOY_PATH:-/var/www}"
```

サポートされる構文：
- `${VAR}` - 基本展開
- `${1}`, `${2}`, ... - 位置引数
- `${VAR:-default}` - デフォルト値
- `${VAR:?error}` - 必須変数
- `${VAR:+value}` - 条件付き置換

**位置引数の例:**

```toml
[commands.convert]
description = "画像フォーマット変換"
cmd = "sharp -i ${1} -f ${2:-webp} -q ${3:-80} -o ${4:-output.webp}"
```

```bash
# 引数を指定して実行
cmdrun run convert input.png webp 90 output.webp
# 展開結果: sharp -i input.png -f webp -q 90 -o output.webp

# デフォルト値を使用
cmdrun run convert input.png
# 展開結果: sharp -i input.png -f webp -q 80 -o output.webp
```

### 依存関係

```toml
[commands.test]
cmd = "cargo test"
deps = ["build"]  # 'test'の前に'build'を実行

[commands.build]
cmd = "cargo build --release"
```

### 並列実行

```toml
[commands.check]
parallel = true
cmd = [
    "cargo fmt -- --check",
    "cargo clippy",
]
```

### プラットフォーム固有のコマンド

```toml
[commands."open:browser"]
cmd.unix = "open http://localhost:3000"
cmd.windows = "start http://localhost:3000"
cmd.linux = "xdg-open http://localhost:3000"
```

### フック

```toml
[hooks]
pre_run = "echo '開始中...'"
post_run = "echo '完了!'"

[hooks.commands.deploy]
pre_run = "git diff --exit-code"  # コミットされていない変更がないことを確認
post_run = "echo '$(date)にデプロイ' >> deploy.log"
```

### 環境変数

```toml
[config.env]
NODE_ENV = "development"
RUST_BACKTRACE = "1"

[commands.dev]
cmd = "npm run dev"
env = { PORT = "3000" }  # コマンド固有の環境変数
```

### 言語設定（i18n）

cmdrunは英語と日本語の国際化をサポートしています。`commands.toml`で言語を設定できます：

```toml
[config]
language = "japanese"  # または "english"（デフォルト）
```

**サポートされるメッセージ：**
- コマンド実行（実行中、完了、エラー）
- 対話的プロンプト（コマンドID、説明など）
- 成功/エラーメッセージ（コマンドが追加されました、コマンドが見つかりませんなど）
- バリデーションエラー（空の入力、重複コマンドなど）

**例（日本語）：**
```bash
$ cmdrun add test-ja "echo テスト" "日本語テストコマンド"
📝 コマンドを追加中 'test-ja' ...
✓ コマンドを追加しました 'test-ja'
  説明: 日本語テストコマンド
  コマンド: echo テスト
```

**例（英語）：**
```bash
$ cmdrun add test-en "echo test" "English test command"
📝 Adding command 'test-en' ...
✓ Command added successfully 'test-en'
  Description: English test command
  Command: echo test
```

**現在サポートされているコマンド：**
- `cmdrun add` - 完全にローカライズ済み（プロンプト、メッセージ、エラー）
- より多くのコマンドが将来のリリースでローカライズされます

## ドキュメント

### ユーザーガイド
- [インストールガイド](docs/user-guide/INSTALLATION.md)
- [CLIリファレンス](docs/user-guide/CLI.md)
- [設定リファレンス](docs/user-guide/CONFIGURATION.md)
- [国際化（i18n）](docs/user-guide/I18N.md)

### 技術ドキュメント
- [パフォーマンス](docs/technical/PERFORMANCE.md)
- [セキュリティ](docs/technical/SECURITY.md)
- [クロスプラットフォームサポート](docs/technical/CROSS_PLATFORM.md)
- [配布](docs/technical/DISTRIBUTION.md)

### 開発
- [コントリビューティング](CONTRIBUTING.md)
- [ロードマップ](docs/development/ROADMAP.md)

## 比較

### vs npm scripts

```json
// package.json (Node.js)
{
  "scripts": {
    "build": "tsc && webpack",
    "test": "jest",
    "deploy": "npm run build && scp -r dist/ user@host:/path"
  }
}
```

対比

```toml
# commands.toml (cmdrun)
[commands.build]
cmd = ["tsc", "webpack"]

[commands.test]
cmd = "jest"

[commands.deploy]
cmd = "scp -r dist/ ${DEPLOY_USER}@${DEPLOY_HOST}:${DEPLOY_PATH}"
deps = ["build"]
```

**メリット**：
- ✅ 起動時間が約29倍高速
- ✅ 型安全な設定
- ✅ 依存関係管理
- ✅ 変数展開
- ✅ プラットフォーム固有のコマンド

### vs Makefile

```makefile
# Makefile
.PHONY: build test

build:
	cargo build --release

test: build
	cargo test
```

対比

```toml
# commands.toml
[commands.build]
cmd = "cargo build --release"

[commands.test]
cmd = "cargo test"
deps = ["build"]
```

**メリット**：
- ✅ より簡単な構文（TOMLとMakeのタブ依存性の比較）
- ✅ クロスプラットフォーム（GNU Make不要）
- ✅ より良いエラーメッセージ
- ✅ 変数展開
- ✅ 並列実行

## パフォーマンスベンチマーク

```bash
# 起動時間の比較（hyperfineで測定）
$ hyperfine --shell=none './target/release/cmdrun --version' 'npm --version' --warmup 5

Benchmark 1: ./target/release/cmdrun --version
  Time (mean ± σ):       4.0 ms ±   0.3 ms    [User: 1.3 ms, System: 1.3 ms]
  Range (min … max):     3.5 ms …   4.6 ms    30 runs

Benchmark 2: npm --version
  Time (mean ± σ):     115.4 ms ±  13.0 ms    [User: 59.7 ms, System: 18.9 ms]
  Range (min … max):   104.5 ms … 158.4 ms    30 runs

Summary
  ./target/release/cmdrun --version ran
    28.88 ± 3.79 times faster than npm --version
```

**主要パフォーマンス指標：**
- **起動時間**: 平均4ms（目標の100ms以下を十分下回る）
- **速度向上**: npmより約29倍高速（測定値28.88 ± 3.79倍）
- **メモリフットプリント**: 約10MB対Node.jsの200MB以上
- **バイナリサイズ**: LTOとstripで最適化

## 使用例

<details>
<summary>📱 Web開発</summary>

```toml
[config]
shell = "bash"

[commands.dev]
description = "開発サーバーを起動"
cmd = "npm run dev"
env = { PORT = "3000", NODE_ENV = "development" }

[commands.build]
description = "本番用ビルド"
cmd = [
    "npm run type-check",
    "npm run lint",
    "npm run build",
]

[commands.deploy]
description = "本番環境へデプロイ"
cmd = "npm run build && firebase deploy"
deps = ["build"]
confirm = true
```

**使い方：**
```bash
# 開発サーバーを起動
cmdrun run dev

# 本番用ビルド（type-check、lint、buildを順次実行）
cmdrun run build

# デプロイ（確認を求め、最初にbuildを実行）
cmdrun run deploy
```
</details>

<details>
<summary>🦀 Rustプロジェクト</summary>

```toml
[commands.dev]
cmd = "cargo watch -x run"

[commands.test]
cmd = "cargo test --all-features"

[commands.bench]
cmd = "cargo bench"

[commands.release]
cmd = [
    "cargo test --all-features",
    "cargo build --release",
    "cargo package",
]
confirm = true
```

**使い方：**
```bash
# 開発用ウォッチモード
cmdrun run dev

# すべてのテストを実行
cmdrun run test

# リリースを作成（確認付き）
cmdrun run release
```
</details>

<details>
<summary>⚡ 高度な機能</summary>

#### 依存関係管理
```toml
[commands.e2e]
description = "E2Eテストを実行"
cmd = "playwright test"
deps = ["build"]  # 'e2e'の前に自動的に'build'を実行

[commands.ci]
description = "完全なCIパイプライン"
deps = ["test", "lint", "build"]  # すべてのチェックを実行
```

#### プラットフォーム固有のコマンド
```toml
[commands.open-browser]
description = "ブラウザを開く"
cmd.unix = "open http://localhost:3000"
cmd.windows = "start http://localhost:3000"
cmd.linux = "xdg-open http://localhost:3000"
```

#### 並列実行
```toml
[commands.lint-all]
description = "すべてのリンターを並列実行"
parallel = true
cmd = [
    "eslint src/",
    "stylelint src/**/*.css",
    "tsc --noEmit",
]
```
</details>

## コントリビューション

コントリビューションを歓迎します！詳細は[CONTRIBUTING.md](CONTRIBUTING.md)をご覧ください。

### 開発環境のセットアップ

```bash
# リポジトリをクローン
git clone https://github.com/sanae-abe/cmdrun
cd cmdrun

# ビルド
cargo build

# テストを実行
cargo test

# ベンチマークを実行
cargo bench

# コードをフォーマット
cargo fmt

# リント
cargo clippy
```

---
**開発者**: sanae-abe@m3.com
