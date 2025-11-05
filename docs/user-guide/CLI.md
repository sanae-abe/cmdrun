# CLIリファレンス

cmdrunコマンドラインインターフェースの完全なリファレンスです。

## 目次

- [グローバルオプション](#グローバルオプション)
- [コマンド](#コマンド)
  - [add](#add) - コマンドを追加
  - [run](#run) - コマンドを実行
  - [list](#list) - コマンド一覧
  - [remove](#remove) - コマンドを削除
  - [edit](#edit) - コマンドを編集
  - [info](#info) - コマンド情報を表示
  - [search](#search) - コマンドを検索
  - [open](#open) - 設定ファイルを開く
  - [validate](#validate) - 設定を検証
  - [config](#config) - 設定管理
  - [completion](#completion) - シェル補完スクリプト生成
- [終了コード](#終了コード)
- [設定ファイル](#設定ファイル)

---

## グローバルオプション

すべてのコマンドで使用可能なオプション:

### `-h, --help`

cmdrunまたは特定のコマンドのヘルプを表示します。

**使用例:**

```bash
# 一般的なヘルプを表示
cmdrun --help

# 特定のコマンドのヘルプを表示
cmdrun run --help
cmdrun add --help
```

### `--version`

cmdrunのバージョンを表示します。

**使用例:**

```bash
cmdrun --version
# 出力: cmdrun 1.0.0
```

### `-v, --verbose`

詳細な出力を有効にします。

- `-v`: デバッグレベルのログ
- `-vv`: トレースレベルのログ

**使用例:**

```bash
# 標準出力
cmdrun run build

# 詳細出力
cmdrun -v run build

# 非常に詳細な出力
cmdrun -vv run build
```

---

## コマンド

### add

新しいコマンドを設定ファイルに追加します。

#### 構文

```bash
cmdrun add [ID] [COMMAND] [DESCRIPTION]
```

#### 説明

グローバル設定ファイル（`~/.config/cmdrun/commands.toml`）に新しいコマンドを追加します。
引数を省略すると対話モードで入力できます。

#### 引数

- `[ID]` - コマンドの一意な識別子（省略可、対話モードで入力）
- `[COMMAND]` - 実行するコマンド（省略可、対話モードで入力）
- `[DESCRIPTION]` - コマンドの説明（省略可、対話モードで入力）

#### 使用例

```bash
# 対話モードで追加
cmdrun add

# 全ての引数を指定して追加
cmdrun add dev "npm run dev" "開発サーバーを起動"

# よく使うコマンドの例
cmdrun add push "git add . && git commit && git push" "変更をコミット＆プッシュ"
cmdrun add prod-ssh "ssh user@production-server.com" "本番サーバーに接続"
cmdrun add docker-clean "docker system prune -af" "未使用のDockerリソースを削除"
```

#### 対話モード例

```
=== コマンドを追加 ===

コマンドID: build
コマンド: cargo build --release
説明: リリースビルド

プレビュー
  ID: build
  コマンド: cargo build --release
  説明: リリースビルド

どうしますか？
❯ はい、このコマンドを追加します
  いいえ、再編集します
  キャンセル

📝 コマンドを追加中 'build' ...
✓ コマンドを追加しました 'build'
  説明: リリースビルド
  コマンド: cargo build --release
```

---

### run

登録されたコマンドを実行します。

#### 構文

```bash
cmdrun run [OPTIONS] <NAME> [-- ARGS...]
```

#### 説明

設定ファイルに登録されたコマンドを実行します。依存関係があれば正しい順序で実行されます。

#### 引数

- `<NAME>` - 実行するコマンドのID（必須）
- `[ARGS...]` - コマンドに渡す追加引数（省略可）

#### オプション

- `-p, --parallel` - 依存関係を並列実行

#### 使用例

```bash
# シンプルなコマンド実行
cmdrun run dev

# 並列実行で依存関係を解決
cmdrun run build --parallel

# コマンドに追加引数を渡す
cmdrun run dev -- --port 8080

# 詳細出力で実行
cmdrun -v run build
```

---

### list

登録されている全コマンドを一覧表示します。

#### 構文

```bash
cmdrun list [OPTIONS] [KEYWORD]
```

#### 説明

設定ファイルに登録されている全コマンドを表示します。
キーワードを指定すると、そのキーワードを含むコマンドのみ表示します。

#### 引数

- `[KEYWORD]` - 検索キーワード（省略可）

#### オプション

- `-v, --verbose` - 各コマンドの詳細情報を表示

#### 使用例

```bash
# コマンド一覧を表示
cmdrun list

# 詳細情報付きで表示
cmdrun list --verbose

# 特定のキーワードで検索
cmdrun list docker
cmdrun list dev
```

#### 出力例

**標準出力:**

```
利用可能なコマンド:

  dev - 開発サーバーを起動
  push - 変更をコミット＆プッシュ
  prod-ssh - 本番サーバーに接続
  docker-clean - 未使用のDockerリソースを削除
```

**詳細出力:**

```
利用可能なコマンド:

  dev - 開発サーバーを起動
    コマンド:
      npm run dev
    依存関係: なし

  push - 変更をコミット＆プッシュ
    コマンド:
      git add . && git commit && git push
    依存関係: なし
```

---

### remove

コマンドを設定ファイルから削除します。

#### 構文

```bash
cmdrun remove [OPTIONS] <ID>
```

#### 説明

グローバル設定ファイルからコマンドを削除します。削除前にバックアップが作成されます。

#### 引数

- `<ID>` - 削除するコマンドのID（必須）

#### オプション

- `-f, --force` - 確認プロンプトをスキップ

#### 使用例

```bash
# 確認プロンプト付きで削除
cmdrun remove old-command

# 確認なしで削除
cmdrun remove old-command --force
```

#### 出力例

```
削除対象:
  ID: old-command
  説明: 古いビルドスクリプト
  コマンド: make old-build

本当に削除しますか？ (y/N): y

✓ バックアップを作成しました: commands.toml.backup.20231105_143022
✓ コマンドを削除しました 'old-command'
```

---

### edit

既存のコマンドを対話的に編集します。

#### 構文

```bash
cmdrun edit [ID]
```

#### 説明

登録されているコマンドの内容を対話的に編集します。IDを省略すると選択メニューが表示されます。

#### 引数

- `[ID]` - 編集するコマンドのID（省略可）

#### 使用例

```bash
# 特定のコマンドを編集
cmdrun edit dev

# 対話的にコマンド選択
cmdrun edit
```

---

### info

コマンドの詳細情報を表示します。

#### 構文

```bash
cmdrun info [ID]
```

#### 説明

登録されているコマンドの詳細情報を表示します。

#### 引数

- `[ID]` - 情報を表示するコマンドのID（省略可）

#### 使用例

```bash
# 特定のコマンドの情報表示
cmdrun info dev

# 対話的に選択
cmdrun info
```

---

### search

キーワードでコマンドを検索します。

#### 構文

```bash
cmdrun search <KEYWORD>
```

#### 説明

コマンドID、説明、コマンド本体、タグから指定したキーワードを検索します（大文字小文字を区別しません）。

#### 引数

- `<KEYWORD>` - 検索キーワード（必須）

#### 使用例

```bash
# docker関連のコマンドを検索
cmdrun search docker

# dev関連のコマンドを検索
cmdrun search dev

# gitコマンドを検索
cmdrun search git
```

#### 出力例

```
検索キーワード: 'docker'

✓ 2件のコマンドが見つかりました:

  • docker-clean - 未使用のDockerリソースを削除
    一致箇所: id, description

  • docker-logs - Dockerコンテナのログを表示
    一致箇所: id, command

💡 詳細は cmdrun info <コマンド> で確認できます
```

---

### open

設定ファイルをエディタで開きます。

#### 構文

```bash
cmdrun open
```

#### 説明

グローバル設定ファイル（`~/.config/cmdrun/commands.toml`）をデフォルトエディタで開きます。

設定ファイルが存在しない場合は、自動的に作成されます。

#### 使用するエディタ

以下の順序でエディタを試行します:

- **macOS**: `open`, `code`, `vim`
- **Linux**: `xdg-open`, `code`, `vim`, `nano`
- **Windows**: `code`, `notepad`

#### 使用例

```bash
# 設定ファイルを開く
cmdrun open
```

#### 出力例

```
Opening: ~/.config/cmdrun/commands.toml
✓ Opened in code
```

---

### validate

設定ファイルを検証します。

#### 構文

```bash
cmdrun validate [OPTIONS]
```

#### 説明

設定ファイルの構文、必須フィールド、依存関係などを検証します。

#### オプション

- `-v, --verbose` - 詳細な検証レポートを表示
- `--check-cycles` - 循環依存をチェック

#### 使用例

```bash
# 設定ファイルを検証
cmdrun validate

# 詳細出力で検証
cmdrun validate --verbose

# 循環依存をチェック
cmdrun validate --check-cycles

# 完全な検証
cmdrun validate --verbose --check-cycles
```

#### 出力例

**成功時:**

```
設定を検証中...

✓ 設定を読み込みました: commands.toml

情報:
  ℹ 15個のコマンドが定義されています
  ℹ 依存関係グラフを構築しました

✓ 設定は有効です (15個のコマンド)
```

**エラーがある場合:**

```
設定を検証中...

✓ 設定を読み込みました: commands.toml

エラー:
  ✗ 循環依存: build → compile → build

警告:
  ⚠ コマンド 'old-script' に説明がありません

✗ 設定の検証に失敗しました (1個のエラー)
```

---

### config

cmdrunの設定を管理します。

#### 構文

```bash
cmdrun config <SUBCOMMAND>
```

#### サブコマンド

- `show` - 現在の設定を表示
- `set <KEY> <VALUE>` - 設定値を変更
- `get <KEY>` - 特定の設定値を取得

#### 使用例

```bash
# 設定を表示
cmdrun config show

# 言語設定を変更
cmdrun config set language japanese

# 設定値を取得
cmdrun config get language
```

#### 出力例

```
現在の設定:

  language: japanese
  config_path: ~/.config/cmdrun/commands.toml
```

---

### completion

シェル補完スクリプトを生成します。

#### 構文

```bash
cmdrun completion <SHELL>
```

#### 説明

cmdrunコマンドのシェル補完スクリプトを生成します。

#### 引数

- `<SHELL>` - シェルの種類（必須）
  - `bash`
  - `zsh`
  - `fish`
  - `powershell`

#### 使用例

```bash
# Bash補完スクリプト生成
cmdrun completion bash

# Zsh補完スクリプト生成
cmdrun completion zsh

# Fish補完スクリプト生成
cmdrun completion fish

# Bash補完のインストール (Linux)
cmdrun completion bash | sudo tee /etc/bash_completion.d/cmdrun

# Zsh補完のインストール
cmdrun completion zsh > "${fpath[1]}/_cmdrun"

# Fish補完のインストール
cmdrun completion fish > ~/.config/fish/completions/cmdrun.fish
```

---

## 終了コード

cmdrunは以下の終了コードを使用します:

| 終了コード | 意味 | 説明 |
|-----------|------|------|
| `0` | 成功 | コマンドが正常に実行されました |
| `1` | 一般エラー | コマンド実行失敗、設定エラー、検証エラー |
| `2` | 使用方法エラー | 無効なコマンドライン引数またはオプション |
| `130` | 中断 | コマンドが中断されました (Ctrl+C) |

### 使用例

```bash
# 終了コードを確認
cmdrun run test
echo $?  # 出力: 0 (成功) または 1 (失敗)

# スクリプトで使用
if cmdrun validate; then
    echo "設定は有効です"
    cmdrun run build
else
    echo "設定にエラーがあります"
    exit 1
fi

# コマンドを連結
cmdrun run lint && cmdrun run test && cmdrun run build
```

---

## 設定ファイル

### 設定ファイルの場所

cmdrunはグローバル設定ファイルを使用します:

- **Linux/macOS**: `~/.config/cmdrun/commands.toml`
- **Windows**: `%APPDATA%\cmdrun\commands.toml`

### 言語設定

cmdrunは日本語と英語をサポートしています。設定ファイルで言語を指定できます:

```toml
[config]
language = "japanese"  # または "english" (デフォルト)
```

### 環境変数

cmdrunは以下の環境変数を認識します:

- `CMDRUN_CONFIG` - 設定ファイルのパスを上書き
- `CMDRUN_SHELL` - コマンド実行時のシェルを上書き
- `NO_COLOR` - カラー出力を無効化
- `CMDRUN_LOG` - ログレベルを設定 (error, warn, info, debug, trace)

**使用例:**

```bash
# カスタム設定ファイルを使用
export CMDRUN_CONFIG=/path/to/custom/commands.toml
cmdrun list

# 特定のシェルを使用
export CMDRUN_SHELL=/bin/bash
cmdrun run build

# カラーを無効化
export NO_COLOR=1
cmdrun list

# デバッグログを有効化
export CMDRUN_LOG=debug
cmdrun run test
```

---

## 高度な使用方法

### 並列実行

依存関係を並列実行して高速化:

```bash
# 逐次実行 (デフォルト)
cmdrun run build
# 実行順序: lint → test → compile → package (一つずつ)

# 並列実行
cmdrun run build --parallel
# グループ1: lint, test (並列)
# グループ2: compile
# グループ3: package
```

### 引数の渡し方

コマンドに追加の引数を渡す:

```bash
# -- 以降の引数がコマンドに渡される
cmdrun run test -- --verbose --filter integration

# commands.tomlでの定義:
[commands.test]
cmd = "cargo test"
# 実際の実行: cargo test --verbose --filter integration
```

### スクリプトとの統合

```bash
#!/bin/bash
# CI/CDスクリプト例

set -e  # エラーで終了

# 設定を検証
cmdrun validate --check-cycles

# 品質チェックを並列実行
cmdrun run lint --parallel

# テストを実行
cmdrun run test

# すべてのチェックが通ればビルド
cmdrun run build --parallel

echo "ビルドが正常に完了しました!"
```

---

## 関連ドキュメント

- [インストールガイド](INSTALLATION.md)
- [設定リファレンス](CONFIGURATION.md)
- [国際化（i18n）](I18N.md)

---

## ヘルプの取得

問題が発生した場合やヘルプが必要な場合:

1. `cmdrun --help` でクイックリファレンスを確認
2. `cmdrun <コマンド> --help` でコマンド固有のヘルプを確認
3. [GitHub Issues](https://github.com/sanae-abe/cmdrun/issues)を確認

**クイックヘルプコマンド:**

```bash
# 一般的なヘルプ
cmdrun --help

# コマンド固有のヘルプ
cmdrun run --help
cmdrun add --help
cmdrun validate --help

# 設定の全コマンドを一覧表示
cmdrun list --verbose

# 設定の有効性を確認
cmdrun validate --verbose
```
