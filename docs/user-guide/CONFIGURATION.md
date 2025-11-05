# 設定リファレンス

cmdrunのTOML設定ファイルの完全なリファレンスです。

## 目次

- [設定ファイル](#設定ファイル)
- [グローバル設定](#グローバル設定)
- [コマンド定義](#コマンド定義)
- [変数展開](#変数展開)
- [プラットフォーム固有のコマンド](#プラットフォーム固有のコマンド)
- [環境変数](#環境変数)
- [フック](#フック)
- [完全な例](#完全な例)

---

## 設定ファイル

### ファイルの場所

cmdrunはデフォルトでグローバル設定ファイルを使用します:

- **Linux/macOS**: `~/.config/cmdrun/commands.toml`
- **Windows**: `%APPDATA%\cmdrun\commands.toml`

### カスタム設定ファイルの指定

`--config`（短縮形: `-c`）オプションで任意の設定ファイルを指定できます:

```bash
# カスタム設定ファイルを使用
cmdrun --config ~/work/commands.toml list
cmdrun -c ~/.cmdrun/personal.toml run dev

# プロジェクト固有の設定を使用
cmdrun -c ./project-commands.toml run build

# 環境別の設定を切り替え
cmdrun -c ~/.cmdrun/production.toml run deploy
cmdrun -c ~/.cmdrun/staging.toml run deploy
```

**使用例**:
- **仕事用と個人用の分離**: `~/work/commands.toml`と`~/personal/commands.toml`
- **プロジェクト固有のコマンド**: プロジェクトディレクトリに`commands.toml`を配置
- **環境別の設定**: 本番・ステージング・開発環境ごとに異なる設定ファイル

### 設定ファイルの作成

初回実行時に自動的に作成されます。手動で作成する場合:

```bash
# デフォルトの場所に作成
# Linux/macOS
mkdir -p ~/.config/cmdrun
touch ~/.config/cmdrun/commands.toml

# Windows PowerShell
New-Item -ItemType Directory -Force -Path "$env:APPDATA\cmdrun"
New-Item -ItemType File -Force -Path "$env:APPDATA\cmdrun\commands.toml"

# カスタムの場所に作成
mkdir -p ~/work
cmdrun init --output ~/work/commands.toml
```

---

## グローバル設定

`[config]`セクションで全コマンドに適用される設定を定義します。

### 基本構造

```toml
[config]
shell = "bash"              # デフォルトシェル
strict_mode = true          # 厳格な変数展開
parallel = false            # デフォルト並列実行
timeout = 300               # デフォルトタイムアウト（秒）
working_dir = "."           # デフォルト作業ディレクトリ
language = "japanese"       # UI言語（english/japanese）
```

### 設定項目

#### `shell`

**型**: 文字列
**デフォルト**:
- Unix/Linux/macOS: `"bash"`
- Windows: `"pwsh"`

**説明**: コマンド実行時のデフォルトシェル

**サポートされているシェル**:
- `bash` - Bourne Again SHell
- `zsh` - Z Shell
- `fish` - Friendly Interactive SHell
- `pwsh` - PowerShell
- `sh` - POSIX シェル
- `cmd` - Windows コマンドプロンプト（Windowsのみ）

**例**:
```toml
[config]
shell = "zsh"
```

#### `language`

**型**: 文字列
**デフォルト**: `"english"`

**説明**: UI言語の設定

**サポートされている言語**:
- `"english"` - 英語
- `"japanese"` - 日本語

**例**:
```toml
[config]
language = "japanese"
```

#### `strict_mode`

**型**: 真偽値
**デフォルト**: `true`

**説明**: 変数展開の動作を制御

- `true`: 未定義の変数はエラーになる
- `false`: 未定義の変数は空文字列に展開される

**例**:
```toml
[config]
strict_mode = false  # 未定義変数を許可
```

#### `timeout`

**型**: 整数
**デフォルト**: なし（タイムアウトなし）

**説明**: コマンドのデフォルトタイムアウト（秒単位）

**例**:
```toml
[config]
timeout = 300  # 5分でタイムアウト
```

---

## コマンド定義

`[commands.<ID>]`セクションで個別のコマンドを定義します。

### 基本構造

```toml
[commands.コマンドID]
description = "コマンドの説明"
cmd = "実行するコマンド"
```

### シンプルなコマンド

```toml
[commands.dev]
description = "開発サーバーを起動"
cmd = "npm run dev"

[commands.push]
description = "変更をコミット＆プッシュ"
cmd = "git add . && git commit && git push"
```

### 複数のコマンド

複数のコマンドを順次実行:

```toml
[commands.deploy]
description = "本番環境へデプロイ"
cmd = [
    "npm run build",
    "npm run test",
    "scp -r dist/ user@server:/var/www"
]
```

### 依存関係

他のコマンドを事前に実行:

```toml
[commands.test]
description = "テストを実行"
cmd = "npm test"

[commands.build]
description = "ビルド実行"
cmd = "npm run build"
deps = ["test"]  # テストが成功してからビルド
```

### 並列実行

複数のコマンドを並列実行:

```toml
[commands.check]
description = "品質チェック"
parallel = true
cmd = [
    "npm run lint",
    "npm run type-check",
    "npm test"
]
```

### 確認プロンプト

実行前に確認:

```toml
[commands.deploy]
description = "本番環境へデプロイ"
cmd = "ssh user@prod 'cd /app && git pull && npm install && pm2 restart app'"
confirm = true  # 実行前に確認
```

### 作業ディレクトリ

特定のディレクトリで実行:

```toml
[commands.frontend-build]
description = "フロントエンドビルド"
cmd = "npm run build"
working_dir = "./frontend"
```

### タイムアウト

個別のタイムアウト設定:

```toml
[commands.long-task]
description = "長時間かかるタスク"
cmd = "npm run heavy-process"
timeout = 600  # 10分でタイムアウト
```

---

## 変数展開

コマンド内で変数を使用できます。

### 基本的な変数展開

```toml
[commands.deploy]
cmd = "scp dist/ ${USER}@${HOST}:${PATH}"
```

実行時:
```bash
export USER="admin"
export HOST="production-server.com"
export PATH="/var/www"
cmdrun run deploy
# 実行: scp dist/ admin@production-server.com:/var/www
```

### 位置引数

コマンド実行時の引数を使用:

```toml
[commands.convert]
description = "画像フォーマット変換"
cmd = "sharp -i ${1} -f ${2:-webp} -q ${3:-80} -o ${4:-output.webp}"
```

実行:
```bash
# 全引数を指定
cmdrun run convert input.png jpeg 90 output.jpg
# 実行: sharp -i input.png -f jpeg -q 90 -o output.jpg

# デフォルト値を使用
cmdrun run convert input.png
# 実行: sharp -i input.png -f webp -q 80 -o output.webp
```

### デフォルト値

変数が未定義の場合のデフォルト値:

```toml
[commands.backup]
cmd = "rsync -avz ~/projects/ ${BACKUP_PATH:-/tmp/backup}"
```

`BACKUP_PATH`が未定義の場合は`/tmp/backup`が使用されます。

### 必須変数

変数が未定義の場合はエラー:

```toml
[commands.deploy]
cmd = "ssh ${DEPLOY_USER:?DEPLOY_USER not set}@${DEPLOY_HOST:?DEPLOY_HOST not set}"
```

環境変数が設定されていない場合、エラーメッセージを表示します。

### 条件付き置換

変数が定義されている場合のみ値を使用:

```toml
[commands.build]
cmd = "npm run build ${NODE_ENV:+--mode production}"
```

`NODE_ENV`が定義されている場合のみ`--mode production`が追加されます。

### サポートされる構文

| 構文 | 説明 | 例 |
|------|------|-----|
| `${VAR}` | 基本展開 | `${USER}` |
| `${1}`, `${2}`, ... | 位置引数 | `${1}`, `${2}` |
| `${VAR:-default}` | デフォルト値 | `${PORT:-3000}` |
| `${VAR:?error}` | 必須変数 | `${API_KEY:?not set}` |
| `${VAR:+value}` | 条件付き置換 | `${DEBUG:+--verbose}` |

---

## プラットフォーム固有のコマンド

OS別に異なるコマンドを定義:

### 基本的な使い方

```toml
[commands.open-browser]
description = "ブラウザを開く"
cmd.unix = "open http://localhost:3000"
cmd.windows = "start http://localhost:3000"
cmd.linux = "xdg-open http://localhost:3000"
```

### プラットフォーム指定

- `cmd.unix` - macOS、Linux、FreeBSD
- `cmd.windows` - Windows
- `cmd.macos` - macOSのみ
- `cmd.linux` - Linuxのみ

### フォールバック

プラットフォーム固有のコマンドが定義されていない場合、`cmd`が使用されます:

```toml
[commands.build]
description = "ビルド実行"
cmd = "npm run build"  # すべてのプラットフォームで使用
cmd.windows = "npm.cmd run build"  # Windowsのみ上書き
```

---

## 環境変数

コマンド実行時の環境変数を設定できます。

### グローバル環境変数

すべてのコマンドに適用:

```toml
[config.env]
NODE_ENV = "development"
RUST_BACKTRACE = "1"
PATH = "/usr/local/bin:$PATH"
```

### コマンド固有の環境変数

特定のコマンドのみ:

```toml
[commands.dev]
description = "開発サーバーを起動"
cmd = "npm run dev"
env = { PORT = "3000", DEBUG = "true" }

[commands.test]
description = "テストを実行"
cmd = "npm test"
env = { NODE_ENV = "test", CI = "true" }
```

### 環境変数の優先順位

1. コマンド固有の環境変数（`env`）
2. グローバル環境変数（`[config.env]`）
3. システムの環境変数

---

## フック

コマンドの前後に処理を実行できます。

### グローバルフック

すべてのコマンドに適用:

```toml
[hooks]
pre_run = "echo '開始中...'"
post_run = "echo '完了!'"
```

### コマンド固有のフック

特定のコマンドのみ:

```toml
[hooks.commands.deploy]
pre_run = "git diff --exit-code"  # コミットされていない変更がないことを確認
post_run = "echo '$(date)にデプロイ' >> deploy.log"

[hooks.commands.build]
pre_run = "npm run lint"
post_run = "npm run test"
```

### 実行順序

1. グローバル`pre_run`
2. コマンド固有`pre_run`
3. メインコマンド
4. コマンド固有`post_run`
5. グローバル`post_run`

---

## 完全な例

### 個人向けコマンド集

```toml
# グローバル設定
[config]
language = "japanese"
shell = "bash"

# 開発関連
[commands.dev]
description = "開発サーバーを起動"
cmd = "npm run dev"
env = { PORT = "3000" }

[commands.push]
description = "変更をコミット＆プッシュ"
cmd = "git add . && git commit && git push"

# サーバー接続
[commands.prod-ssh]
description = "本番サーバーに接続"
cmd = "ssh ${PROD_USER:?not set}@${PROD_HOST:?not set}"

[commands.staging-ssh]
description = "ステージングサーバーに接続"
cmd = "ssh staging@staging-server.com"

# Docker関連
[commands.docker-clean]
description = "未使用のDockerリソースを削除"
cmd = "docker system prune -af"
confirm = true

[commands.docker-logs]
description = "Dockerコンテナのログを表示"
cmd = "docker logs ${1:?Container name required} -f"

# データベース
[commands.db-backup]
description = "データベースをバックアップ"
cmd = "pg_dump mydb > backup_$(date +%Y%m%d).sql"

[commands.db-restore]
description = "データベースを復元"
cmd = "psql mydb < ${1:?Backup file required}"
confirm = true

# その他
[commands.weather]
description = "天気を確認"
cmd = "curl wttr.in/Tokyo?lang=ja"

[commands.ip]
description = "外部IPアドレスを確認"
cmd = "curl -s https://ipinfo.io/ip"
```

### 開発プロジェクト用

```toml
[config]
language = "japanese"
working_dir = "."

[config.env]
NODE_ENV = "development"

# 開発
[commands.dev]
description = "開発サーバーを起動"
cmd = "npm run dev"
env = { PORT = "3000" }

[commands.build]
description = "プロダクションビルド"
cmd = "npm run build"
deps = ["lint", "test"]

[commands.test]
description = "テストを実行"
cmd = "npm test"

[commands.lint]
description = "リンターを実行"
cmd = "npm run lint"

# デプロイ
[commands.deploy]
description = "本番環境へデプロイ"
cmd = [
    "npm run build",
    "rsync -avz dist/ ${DEPLOY_USER}@${DEPLOY_HOST}:${DEPLOY_PATH}"
]
deps = ["build"]
confirm = true

# フック
[hooks]
pre_run = "echo '実行中: $CMDRUN_COMMAND'"
post_run = "echo '完了しました'"

[hooks.commands.deploy]
pre_run = "git diff --exit-code"
post_run = "echo '$(date)にデプロイ' >> deploy.log"
```

---

## 関連ドキュメント

- [インストールガイド](INSTALLATION.md)
- [CLIリファレンス](CLI.md)
- [国際化（i18n）](I18N.md)

---

## トラブルシューティング

### 設定ファイルが見つからない

```bash
# 設定ファイルの場所を確認
cmdrun config show

# 設定ファイルを開いて作成
cmdrun open
```

### 変数が展開されない

```toml
# strict_modeを無効化
[config]
strict_mode = false

# またはデフォルト値を指定
[commands.example]
cmd = "echo ${VAR:-default}"
```

### コマンドが実行されない

```bash
# 詳細ログで確認
cmdrun -v run your-command

# 設定を検証
cmdrun validate --verbose
```

### シェルの問題

```toml
# シェルを明示的に指定
[config]
shell = "bash"

# またはコマンドごとに指定
[commands.example]
cmd = "your-command"
shell = "zsh"
```
