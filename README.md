# cmdrun

[日本語](README.md) | [English](README.en.md)

> **頻繁に使うコマンドを管理する個人向けグローバルコマンド管理ツール**
>
> コマンドを一度登録すれば、どこからでも実行可能。高速・安全・クロスプラットフォーム対応。

## 目次

- [cmdrunの特徴](#cmdrunの特徴)
- [インストール](#インストール)
- [基本的な使い方](#基本的な使い方)
- [機能](#機能)
- [設定例](#設定例)
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
- **対応OS**: Linux、macOS、Windows、FreeBSD
- **ビルド済みバイナリ**: 現在macOSのみ（他のOSはソースからビルド）
- **シェル検出**: bash/zsh/fish/pwshを自動検出
- **ネイティブバイナリ**: ランタイム依存なし

### 💎 開発者体験
- **TOML設定** - 型安全で読みやすい
- **強力な機能** - 依存関係、並列実行、フック、Watch Mode
- **優れたエラー表示** - コンテキスト付き詳細エラーメッセージ

## インストール

### 方法1: ビルド済みバイナリ

最も簡単で高速な方法です。Rustのインストール不要。

**現在の対応状況**: macOSのみ（Linux/Windows/FreeBSDは準備中）

#### macOS (Intel/Apple Silicon) ✅

```bash
# 1. バイナリをダウンロード
curl -LO "https://rendezvous.m3.com/api/v4/projects/sanae-abe%2Fcmdrun/packages/generic/cmdrun/1.0.0/cmdrun-v1.0.0-x86_64-apple-darwin.tar.gz"

# 2. 解凍
tar -xzf cmdrun-v1.0.0-x86_64-apple-darwin.tar.gz

# 3. バイナリを適切な場所に移動
sudo mv cmdrun /usr/local/bin/

# 4. 動作確認
cmdrun --version
```

または、[リリースページ](https://rendezvous.m3.com/sanae-abe/cmdrun/-/releases/v1.0.0)から直接ダウンロード。

### 方法2: ソースからビルド

#### システム要件

- **オペレーティングシステム**: Linux、macOS、Windows、FreeBSD
- **Rust**: 1.70以上（MSRV）

#### Rustツールチェーンのインストール

```bash
# 1. Rustup（Rustインストーラー）をダウンロード・実行
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 2. 環境変数を読み込み
source ~/.cargo/env

# 3. インストール確認
rustc --version
cargo --version
```

#### cmdrunのビルド&インストール

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

### アップデート

```bash
# ソースからインストールした場合
cd cmdrun  # プロジェクトディレクトリ
git pull

# 再ビルド&インストール
cargo install --path . --force
```

### アンインストール

```bash
# 1. バイナリの削除
cargo uninstall cmdrun

# 2. 設定ファイルの削除（任意）
# Linux/macOS
rm -rf ~/.config/cmdrun

# Windows（PowerShellで実行）
# Remove-Item -Recurse -Force "$env:APPDATA\cmdrun"

# 3. プロジェクトディレクトリの削除（任意）
# cd ..
# rm -rf cmdrun
```

**注意事項:**
- `cargo uninstall cmdrun`は実行ファイルのみを削除します
- 設定ファイル（commands.toml等）は手動で削除する必要があります
- 設定を保持したい場合は、ステップ2をスキップしてください

## 基本的な使い方

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

<img src="docs/screenshots/add.webp" alt="コマンド追加" width="600">

#### コマンドを実行・管理

```bash
# 登録したコマンドを実行
cmdrun run dev

# 登録されている全コマンドを表示
cmdrun list

# コマンドを検索
cmdrun search docker

# コマンドを削除
cmdrun remove dev
```

<img src="docs/screenshots/run.webp" alt="コマンド実行" width="600">

<img src="docs/screenshots/list.webp" alt="コマンド一覧" width="600">

#### 設定管理

```bash
# 設定を表示
cmdrun config show

# 言語設定を変更
cmdrun config set language japanese

# カスタム設定ファイルを使用
cmdrun --config ~/work/commands.toml list
cmdrun -c ~/.cmdrun/personal.toml run dev

# ヘルプを表示
cmdrun --help
```

**設定ファイルの場所:**
- Linux/macOS: `~/.config/cmdrun/commands.toml`
- Windows: `%APPDATA%\cmdrun\commands.toml`
- カスタムパス: `--config/-c` オプションで任意のパスを指定可能

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

### Watch Mode - ファイル監視

```toml
# commands.tomlで通常通りコマンドを定義
[commands.dev]
cmd = "cargo build"

[commands.test]
cmd = "cargo test"
```

```bash
# コマンドラインからWatch Modeで実行
# Rustファイルの変更を監視してビルド
cmdrun watch dev --pattern "**/*.rs"

# テストの自動実行（デバウンス1秒）
cmdrun watch test --pattern "**/*.rs" --debounce 1000

# 複数のディレクトリを監視
cmdrun watch dev --path src --path lib
```

**Watch Modeの主な機能:**
- **Globパターン**: ファイルフィルタリング（例: `**/*.rs`, `**/*.ts`）
- **除外パターン**: 不要なファイル/ディレクトリを除外（デフォルトで`node_modules`, `target`等を除外）
- **デバウンス**: 頻繁な変更時の不要な実行を防止（デフォルト500ms）
- **再帰監視**: サブディレクトリも自動監視（`--no-recursive`で無効化可能）
- **gitignore統合**: `.gitignore`のパターンを自動尊重

詳細は[Watch Modeガイド](docs/user-guide/WATCH_MODE.md)を参照してください。

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

### カスタム設定ファイル

`--config/-c`オプションで複数の設定ファイルを使い分けることができます。

**使用例：**

```bash
# 仕事用のコマンド
cmdrun --config ~/work/commands.toml list
cmdrun -c ~/work/commands.toml run deploy

# 個人用のコマンド
cmdrun -c ~/personal/commands.toml run backup

# プロジェクト固有のコマンド
cd ~/projects/myapp
cmdrun -c ./commands.toml run dev
```

**ユースケース：**

1. **環境別の設定**
   ```bash
   # 本番環境用
   cmdrun -c ~/.cmdrun/production.toml run deploy

   # ステージング環境用
   cmdrun -c ~/.cmdrun/staging.toml run deploy

   # 開発環境用
   cmdrun -c ~/.cmdrun/development.toml run dev
   ```

2. **複数のプロジェクト管理**
   ```bash
   # プロジェクトA
   cmdrun -c ~/projects/project-a/commands.toml run test

   # プロジェクトB
   cmdrun -c ~/projects/project-b/commands.toml run test
   ```

3. **役割別のコマンド**
   ```bash
   # システム管理用
   cmdrun -c ~/.cmdrun/admin.toml run server-check

   # 開発用
   cmdrun -c ~/.cmdrun/dev.toml run code-review
   ```

**詳細は[設定リファレンス](docs/user-guide/CONFIGURATION.md#カスタム設定ファイルの指定)を参照してください。**

## 設定例

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
- [CLIリファレンス](docs/user-guide/CLI.md)
- [設定リファレンス](docs/user-guide/CONFIGURATION.md)
- [国際化（i18n）](docs/user-guide/I18N.md)
- [Watch Mode](docs/user-guide/WATCH_MODE.md)

### 技術ドキュメント
- [パフォーマンス](docs/technical/PERFORMANCE.md)
- [セキュリティ](docs/technical/SECURITY.md)
- [クロスプラットフォームサポート](docs/technical/CROSS_PLATFORM.md)
- [配布](docs/technical/DISTRIBUTION.md)

## ライセンス

このプロジェクトは[MIT License](LICENSE)の下でライセンスされています。

---
**開発者**: sanae-abe@m3.com
