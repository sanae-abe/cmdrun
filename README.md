# cmdrun

[English](README.md) | [日本語](README.ja.md)

> **頻繁に使うコマンドを管理する個人向けグローバルコマンド管理ツール**
>
> コマンドを一度登録すれば、どこからでも実行可能。高速・安全・クロスプラットフォーム対応。

[![Crates.io](https://img.shields.io/crates/v/cmdrun.svg)](https://crates.io/crates/cmdrun)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE)
[![Build Status](https://github.com/sanae-abe/cmdrun/workflows/CI/badge.svg)](https://github.com/sanae-abe/cmdrun/actions)

## 目次

- [cmdrunの特徴](#cmdrunの特徴)
- [クイックスタート](#クイックスタート)
- [機能](#機能)
- [ドキュメント](#ドキュメント)
- [ライセンス](#ライセンス)

## cmdrunの特徴

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

#### Rustツールチェーンのインストール

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

#### アップデート

```bash
# ソースからインストールした場合
cd cmdrun  # プロジェクトディレクトリ
git pull

# 再ビルド&インストール
cargo install --path . --force
```

### 基本的な使い方

cmdrunは**個人向けグローバルコマンド管理ツール**です。頻繁に使うコマンドを登録し、システムのどこからでも実行できます。

#### よく使うコマンドを登録

```bash
# 対話的にコマンドを追加
cmdrun add

# または、直接パラメータを指定して追加
cmdrun add dev "npm run dev" "開発サーバーを起動"
cmdrun add push "git add . && git commit && git push" "変更をコミット＆プッシュ"
cmdrun add prod-ssh "ssh user@production-server.com" "本番サーバーに接続"
cmdrun add docker-clean "docker system prune -af" "未使用のDockerリソースを削除"
cmdrun add db-backup "pg_dump mydb > backup_$(date +%Y%m%d).sql" "データベースをバックアップ"
```

#### コマンドを実行・管理

```bash
# 登録したコマンドを実行
cmdrun run dev

# 登録されている全コマンドを表示
cmdrun list

# コマンドを検索
cmdrun list docker

# コマンドを削除
cmdrun remove dev
```

#### 設定管理

```bash
# 設定を表示
cmdrun config show

# 言語設定を変更
cmdrun config set language japanese

# ヘルプを表示
cmdrun --help
```

**設定ファイルの場所:**
- Linux/macOS: `~/.config/cmdrun/commands.toml`
- Windows: `%APPDATA%\cmdrun\commands.toml`

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

### 高度な設定例

設定ファイル（`~/.config/cmdrun/commands.toml`）を直接編集することで、より高度な機能を使用できます：

```toml
# 依存関係を持つコマンド
[commands.deploy]
description = "本番環境へデプロイ"
cmd = "ssh user@server 'cd /app && git pull && npm install && pm2 restart app'"
deps = ["test"]  # テストが成功してからデプロイ
confirm = true   # 実行前に確認

[commands.test]
description = "テストを実行"
cmd = "npm test"

# 環境変数を使用
[commands.backup]
description = "バックアップを作成"
cmd = "rsync -avz ~/projects/ ${BACKUP_PATH:?BACKUP_PATH not set}"

# プラットフォーム別のコマンド
[commands.open]
description = "ブラウザを開く"
cmd.unix = "open http://localhost:3000"
cmd.windows = "start http://localhost:3000"
cmd.linux = "xdg-open http://localhost:3000"
```

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

---
**開発者**: sanae-abe@m3.com
