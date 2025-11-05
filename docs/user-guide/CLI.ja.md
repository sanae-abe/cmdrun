# CLIリファレンス

[English](CLI.md) | [日本語](CLI.ja.md)

cmdrunコマンドラインインターフェースのすべてのコマンドとオプションの完全なリファレンスです。

## 目次

- [グローバルオプション](#グローバルオプション)
- [コマンド](#コマンド)
  - [run](#run)
  - [list](#list)
  - [add](#add)
  - [remove](#remove)
  - [edit](#edit)
  - [info](#info)
  - [search](#search)
  - [open](#open)
  - [init](#init)
  - [validate](#validate)
  - [graph](#graph)
  - [completion](#completion)
- [終了コード](#終了コード)
- [設定](#設定)

---

## グローバルオプション

これらのオプションはすべてのコマンドで利用可能です：

### `-h, --help`

cmdrunまたは特定のコマンドのヘルプ情報を表示します。

**例：**

```bash
# 一般的なヘルプを表示
cmdrun --help

# 特定のコマンドのヘルプを表示
cmdrun run --help
cmdrun add --help
```

### `--version`

cmdrunのバージョンを表示します。

**例：**

```bash
cmdrun --version
# 出力: cmdrun 0.1.0
```

### `-v, --verbose`

デバッグや詳細情報のための詳細出力を有効にします。

- `-v`: デバッグレベルのログ
- `-vv`: トレースレベルのログ

**例：**

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

### run

設定ファイルで定義されたコマンドを実行します。

#### 書式

```bash
cmdrun run [OPTIONS] <NAME> [-- ARGS...]
```

#### 説明

`commands.toml`設定ファイルから指定されたコマンドを実行します。コマンドに依存関係がある場合は、正しい順序で最初に実行されます。シーケンシャル実行とパラレル実行の両方のモードをサポートしています。

#### 引数

- `<NAME>` - 実行するコマンドの名前/ID（必須）
- `[ARGS...]` - コマンドに渡す追加の引数（オプション）

#### オプション

- `-p, --parallel` - 可能な場合、依存関係を並列実行します

#### 例

**English:**

```bash
# Run a simple command
cmdrun run test

# Run command with parallel dependency execution
cmdrun run build --parallel

# Pass additional arguments to the command
cmdrun run dev -- --port 8080

# Run with verbose output
cmdrun -v run build
```

**Japanese (日本語):**

```bash
# シンプルなコマンド実行
cmdrun run test

# 並列実行で依存関係を解決
cmdrun run build --parallel

# コマンドに追加引数を渡す
cmdrun run dev -- --port 8080

# 詳細出力で実行
cmdrun -v run build
```

#### 出力例

```
Running: Build the project (with parallel dependencies)
📋 Execution plan: 3 groups
▶ Group 1/3 (2 commands)
  ✓ lint completed in 1.23s
  ✓ test completed in 2.45s
▶ Group 2/3 (1 commands)
  ✓ compile completed in 5.67s
▶ Group 3/3 (1 commands)
  ✓ package completed in 1.89s
✓ All commands completed in 11.24s
```

---

### list

設定ファイルから利用可能なすべてのコマンドを一覧表示します。

#### 書式

```bash
cmdrun list [OPTIONS]
```

#### 説明

`commands.toml`ファイルで定義されたすべてのコマンドを説明と共に表示します。詳細フラグを使用すると、コマンド仕様や依存関係を含む詳細情報が表示されます。

#### オプション

- `-v, --verbose` - 各コマンドの詳細情報を表示します

#### 例

**English:**

```bash
# List all commands
cmdrun list

# List with detailed information
cmdrun list --verbose
```

**Japanese (日本語):**

```bash
# コマンド一覧を表示
cmdrun list

# 詳細情報付きで表示
cmdrun list --verbose
```

#### 出力例

**標準出力：**

```
Available commands:

  build - Build the project
  clean - Clean build artifacts
  dev - Start development server
  test - Run all tests
```

**詳細出力：**

```
Available commands:

  build - Build the project
    Command:
      cargo build --release
    Dependencies: ["lint", "test"]

  dev - Start development server
    Command:
      cargo watch -x run
    [...]
```

---

### add

設定ファイルに新しいコマンドを追加します。

#### 書式

```bash
cmdrun add [OPTIONS] [ID] [COMMAND] [DESCRIPTION]
```

#### 説明

`commands.toml`設定ファイルに新しいコマンドエントリを追加します。インタラクティブモード（引数を省略した場合）または、スクリプト用にすべての引数を指定して使用できます。

インタラクティブモードでは、以下のガイド付きエクスペリエンスを提供します：
- 入力検証
- 保存前のプレビュー
- 戻るナビゲーションのサポート
- 多言語プロンプト

#### 引数

- `[ID]` - ユニークなコマンド識別子（オプション、省略時はプロンプトが表示されます）
- `[COMMAND]` - 実行するコマンド（オプション、省略時はプロンプトが表示されます）
- `[DESCRIPTION]` - コマンドの説明（オプション、省略時はプロンプトが表示されます）

#### オプション

- `-c, --category <CATEGORY>` - コマンドのカテゴリ
- `-t, --tags <TAGS>` - コマンドのタグ（カンマ区切り）

#### 例

**English:**

```bash
# Interactive mode
cmdrun add

# Add with all arguments
cmdrun add build "cargo build --release" "Build release binary"

# Add with category and tags
cmdrun add test "cargo test" "Run tests" \
  --category testing \
  --tags rust,ci

# Quick one-liner
cmdrun add lint "cargo clippy" "Lint code"
```

**Japanese (日本語):**

```bash
# 対話モードで追加
cmdrun add

# 全ての引数を指定して追加
cmdrun add build "cargo build --release" "リリースビルド"

# カテゴリとタグを指定
cmdrun add test "cargo test" "テスト実行" \
  --category testing \
  --tags rust,ci

# ワンライナーで追加
cmdrun add lint "cargo clippy" "リンター実行"
```

#### インタラクティブモードの例

```
=== Add New Command ===

Command ID: build
Command: cargo build --release
Description: Build release binary

Preview
  ID: build
  Command: cargo build --release
  Description: Build release binary

What would you like to do?
❯ Yes, add this command
  No, edit again
  Cancel

📝 Adding command 'build' to commands.toml
✓ Command added successfully 'build'
  Description: Build release binary
  Command: cargo build --release
```

---

### remove

設定ファイルからコマンドを削除します。

#### 書式

```bash
cmdrun remove [OPTIONS] <ID>
```

#### 説明

`commands.toml`設定ファイルからコマンドエントリを削除します。安全のため、変更前にバックアップを作成します。`--force`フラグを使用しない限り、確認が必要です。

#### 引数

- `<ID>` - 削除するコマンドID（必須）

#### オプション

- `-f, --force` - 確認プロンプトをスキップします
- `-c, --config <PATH>` - 設定ファイルのパス（デフォルト: 自動検出）

#### 例

**English:**

```bash
# Remove with confirmation
cmdrun remove old-command

# Remove without confirmation
cmdrun remove old-command --force

# Remove from specific config file
cmdrun remove build --config ./custom-commands.toml
```

**Japanese (日本語):**

```bash
# 確認プロンプト付きで削除
cmdrun remove old-command

# 確認なしで削除
cmdrun remove old-command --force

# 指定した設定ファイルから削除
cmdrun remove build --config ./custom-commands.toml
```

#### 出力例

```
Removal target:
  ID: old-command
  Description: Obsolete build script
  Command: make old-build

Are you sure? (y/N): y

✓ Backup created: commands.toml.backup.20231105_143022
✓ Command removed successfully 'old-command'
```

---

### edit

既存のコマンドをインタラクティブに編集します。

#### 書式

```bash
cmdrun edit [ID]
```

#### 説明

既存のコマンドのプロパティ（説明、コマンド文字列、タグ、実行設定など）を変更するためのインタラクティブエディタを開きます。コマンドIDが提供されない場合は、選択メニューが表示されます。

#### 引数

- `[ID]` - 編集するコマンドID（オプション、省略時はプロンプトが表示されます）

#### 例

**English:**

```bash
# Edit specific command
cmdrun edit build

# Interactive command selection
cmdrun edit
```

**Japanese (日本語):**

```bash
# 特定のコマンドを編集
cmdrun edit build

# 対話的にコマンド選択
cmdrun edit
```

#### 出力例

```
Current settings
  ID: build
  Description: Build the project
  Command: cargo build
  Tags: []
  Parallel: false
  Confirm: false

Description (Build the project): Build release binary
Command (cargo build): cargo build --release
Tags (comma-separated) (): rust,build
Parallel execution (false): false
Confirm before execution (false): false

✓ Command updated successfully 'build'
```

---

### info

コマンドの詳細情報を表示します。

#### 書式

```bash
cmdrun info [ID]
```

#### 説明

特定のコマンドに関する包括的な情報を表示します：
- 説明
- コマンド仕様（単一、複数、またはプラットフォーム固有）
- 依存関係
- タグ
- 作業ディレクトリ
- 環境変数
- 実行設定
- プラットフォームサポート

#### 引数

- `[ID]` - 情報を表示するコマンドID（オプション、省略時はプロンプトが表示されます）

#### 例

**English:**

```bash
# Show info for specific command
cmdrun info build

# Interactive selection
cmdrun info
```

**Japanese (日本語):**

```bash
# 特定のコマンドの情報表示
cmdrun info build

# 対話的に選択
cmdrun info
```

#### 出力例

```
Command details: build
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Description: Build the project

Command:
  cargo build --release

Dependencies:
  → lint
  → test

Tags: rust, build, ci

Execution settings:
  Parallel: yes
  Confirm: no
  Timeout: 300s

Platforms: Unix, Linux, macOS
```

---

### search

キーワードでコマンドを検索します。

#### 書式

```bash
cmdrun search <KEYWORD>
```

#### 説明

指定されたキーワード（大文字小文字を区別しない）ですべてのコマンドを検索します。以下を検索対象とします：
- コマンドID
- 説明
- コマンドテキスト
- タグ

結果には、キーワードがどこでマッチしたか（id、description、command、またはtags）が表示されます。

#### 引数

- `<KEYWORD>` - 検索するキーワード（必須）

#### 例

**English:**

```bash
# Search for test-related commands
cmdrun search test

# Search for build commands
cmdrun search build

# Search by tag
cmdrun search rust
```

**Japanese (日本語):**

```bash
# テスト関連のコマンドを検索
cmdrun search test

# ビルド系コマンドを検索
cmdrun search build

# タグで検索
cmdrun search rust
```

#### 出力例

```
Searching for: 'test'

✓ Found 3 matching command(s):

  • integration-test - Run integration tests
    Matched in: id, description

  • test - Run all tests
    Matched in: id, description, tags

  • test-watch - Run tests in watch mode
    Matched in: id, command

💡 Use cmdrun info <command> to see details
```

---

### open

設定ファイルをデフォルトエディタで開きます。

#### 書式

```bash
cmdrun open
```

#### 説明

`commands.toml`設定ファイルをシステムのデフォルトエディタまたは適切なテキストエディタで開きます。設定ファイルは以下の順序で検索されます：
1. カレントディレクトリ（`./commands.toml`）
2. 親ディレクトリ（上方向に検索）
3. グローバルディレクトリ（`~/.cmdrun/commands.toml`）

以下の順序でエディタの使用を試みます：
- **macOS**: `open`、`code`、`vim`
- **Linux**: `xdg-open`、`code`、`vim`、`nano`
- **Windows**: `code`、`notepad`

#### 例

**English:**

```bash
# Open configuration file
cmdrun open
```

**Japanese (日本語):**

```bash
# 設定ファイルを開く
cmdrun open
```

#### 出力例

```
Opening: /path/to/project/commands.toml
✓ Opened in code
```

---

### init

新しいcommands.toml設定ファイルを初期化します。

#### 書式

```bash
cmdrun init [OPTIONS]
```

#### 説明

テンプレートから新しい`commands.toml`設定ファイルを作成します。異なる開発環境に最適化された複数のプロジェクト固有のテンプレートを提供します。

利用可能なテンプレート：
- **default** - 汎用コマンドランナー設定
- **web** - Web開発（HTML/CSS/JS）
- **rust** - cargoコマンドを使用したRustプロジェクト
- **node** - npm/yarnコマンドを使用したNode.jsプロジェクト
- **python** - 一般的なツールを使用したPythonプロジェクト

#### オプション

- `-t, --template <TEMPLATE>` - 使用するテンプレート（web、rust、node、python）
- `-i, --interactive` - インタラクティブモードでテンプレートを選択します
- `-o, --output <PATH>` - 出力パス（デフォルト: `commands.toml`）

#### 例

**English:**

```bash
# Create with default template
cmdrun init

# Create with specific template
cmdrun init --template rust

# Create with interactive selection
cmdrun init --interactive

# Create at custom location
cmdrun init --output ./custom/path/commands.toml

# Create for Node.js project
cmdrun init -t node
```

**Japanese (日本語):**

```bash
# デフォルトテンプレートで作成
cmdrun init

# 特定のテンプレートで作成
cmdrun init --template rust

# 対話モードで選択
cmdrun init --interactive

# カスタムパスに作成
cmdrun init --output ./custom/path/commands.toml

# Node.jsプロジェクト用に作成
cmdrun init -t node
```

#### 出力例

```
✓ Created commands.toml using Rust project template

Next steps:
  1. Edit commands.toml to define your commands
  2. Run cmdrun list to list available commands
  3. Run cmdrun run <name> to execute a command
```

---

### validate

設定ファイルを検証します。

#### 書式

```bash
cmdrun validate [OPTIONS]
```

#### 説明

`commands.toml`設定ファイルを以下の点で検証します：
- 構文エラー
- 必須フィールドの欠落
- 無効なコマンド参照
- 循環依存（`--check-cycles`が有効な場合）
- 壊れたエイリアス参照
- プラットフォーム固有のコマンドの有効性

#### オプション

- `-p, --path <PATH>` - 設定ファイルのパス（デフォルト: 自動検出）
- `-v, --verbose` - 詳細な検証レポートを表示します
- `--check-cycles` - 循環依存をチェックします

#### 例

**English:**

```bash
# Validate configuration
cmdrun validate

# Validate with detailed output
cmdrun validate --verbose

# Check for circular dependencies
cmdrun validate --check-cycles

# Validate specific file
cmdrun validate --path ./custom-commands.toml

# Full validation
cmdrun validate --verbose --check-cycles
```

**Japanese (日本語):**

```bash
# 設定ファイルを検証
cmdrun validate

# 詳細出力で検証
cmdrun validate --verbose

# 循環依存をチェック
cmdrun validate --check-cycles

# 特定のファイルを検証
cmdrun validate --path ./custom-commands.toml

# 完全な検証
cmdrun validate --verbose --check-cycles
```

#### 出力例

**成功時：**

```
Validating configuration...

✓ Loaded configuration from commands.toml

Information:
  ℹ 15 commands defined
  ℹ 3 aliases defined
  ℹ Dependency graph built successfully

✓ Configuration is valid (15 commands, 3 aliases)
```

**エラーがある場合：**

```
Validating configuration...

✓ Loaded configuration from commands.toml

Errors:
  ✗ Alias 'quick-test' points to non-existent command 'test-fast'
  ✗ Circular dependency in 'build': build → compile → build

Warnings:
  ⚠ Command 'old-script' has no description

✗ Configuration validation failed with 2 error(s)
```

---

### graph

依存関係グラフを表示します。

#### 書式

```bash
cmdrun graph [COMMAND]
```

#### 説明

コマンドの依存関係をツリー構造で可視化します。どのコマンドがどれに依存しているかを表示し、実行順序を理解するのに役立ちます。

#### 引数

- `[COMMAND]` - 依存関係を表示する特定のコマンド（オプション）

#### 例

**English:**

```bash
# Show all dependencies
cmdrun graph

# Show dependencies for specific command
cmdrun graph build
```

**Japanese (日本語):**

```bash
# すべての依存関係を表示
cmdrun graph

# 特定のコマンドの依存関係を表示
cmdrun graph build
```

#### 出力例

**単一コマンド：**

```
Dependencies for: build
  → lint
  → test
  → compile
```

**すべてのコマンド：**

```
Dependency graph:

build
  → lint
  → test
  → compile

deploy
  → build
  → validate

test
  → format
```

---

### completion

シェル補完スクリプトを生成します。

#### 書式

```bash
cmdrun completion <SHELL>
```

#### 説明

cmdrunコマンドのシェル補完スクリプトを生成します。bash、zsh、fish、PowerShell、elvishを含む主要なシェルをサポートしています。

#### 引数

- `<SHELL>` - 補完を生成するシェル（必須）
  - `bash`
  - `zsh`
  - `fish`
  - `powershell`
  - `elvish`

#### 例

**English:**

```bash
# Generate bash completion
cmdrun completion bash

# Generate zsh completion
cmdrun completion zsh

# Generate fish completion
cmdrun completion fish

# Install bash completion (Linux)
cmdrun completion bash | sudo tee /etc/bash_completion.d/cmdrun

# Install zsh completion
cmdrun completion zsh > "${fpath[1]}/_cmdrun"

# Install fish completion
cmdrun completion fish > ~/.config/fish/completions/cmdrun.fish
```

**Japanese (日本語):**

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

#### 出力例

```
→ Generating bash completion script...

# Bash completion script output...

Installation instructions:

  Add to your ~/.bashrc:
    eval "$(cmdrun completion bash)"

  Or save to completion directory:
    cmdrun completion bash > /etc/bash_completion.d/cmdrun

Note: After installation, restart your shell or source the config file.
```

---

## 終了コード

cmdrunは実行ステータスを示すために標準の終了コードを使用します：

| 終了コード | 意味 | 説明 |
|-----------|------|------|
| `0` | 成功 | コマンドが正常に実行されました |
| `1` | 一般的なエラー | コマンドが失敗、設定エラー、または検証エラー |
| `2` | 無効な使用法 | 無効なコマンドライン引数またはオプション |
| `130` | 中断 | コマンドが中断されました（Ctrl+C） |

### 例

**English:**

```bash
# Check exit code
cmdrun run test
echo $?  # Prints: 0 (success) or 1 (failure)

# Use in scripts
if cmdrun validate; then
    echo "Configuration is valid"
    cmdrun run build
else
    echo "Configuration has errors"
    exit 1
fi

# Chain commands
cmdrun run lint && cmdrun run test && cmdrun run build
```

**Japanese (日本語):**

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

## 設定

cmdrunは以下の順序で設定ファイルを検索します：

1. **プロジェクトローカル**: カレントディレクトリと親ディレクトリ
   - `./commands.toml`
   - `./.cmdrun.toml`
   - `./cmdrun.toml`

2. **グローバル**: ユーザーのホームディレクトリ
   - `~/.cmdrun/commands.toml`
   - `~/.cmdrun/.cmdrun.toml`
   - `~/.cmdrun/cmdrun.toml`

### 言語設定

cmdrunは英語と日本語の国際化（i18n）をサポートしています。設定ファイルで言語を設定してください：

```toml
[config]
language = "Japanese"  # または "English" (デフォルト)
```

### 環境変数

cmdrunは以下の環境変数を尊重します：

- `CMDRUN_CONFIG` - 設定ファイルのパスを上書きします
- `CMDRUN_SHELL` - コマンド実行のシェルを上書きします
- `NO_COLOR` - カラー出力を無効にします
- `CMDRUN_LOG` - ログレベルを設定します（error、warn、info、debug、trace）

**例：**

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

## 高度な使用法

### 並列実行

より高速なビルドのためにコマンドの依存関係を並列実行します：

```bash
# シーケンシャル（デフォルト）
cmdrun run build
# 実行順序: lint → test → compile → package (1つずつ)

# 並列
cmdrun run build --parallel
# グループ1: lint, test (並列)
# グループ2: compile
# グループ3: package
```

### 引数の渡し方

コマンドに追加の引数を渡します：

```bash
# -- の後の引数はコマンドに渡されます
cmdrun run test -- --verbose --filter integration

# commands.tomlの場合:
[commands.test]
cmd = "cargo test"
# 実際の実行: cargo test --verbose --filter integration
```

### 複数の設定ファイルの操作

```bash
# 特定の設定を検証
cmdrun validate --path ./configs/production.toml

# 特定の設定から削除
cmdrun remove old-cmd --config ./configs/dev.toml

# 特定の場所に初期化
cmdrun init --output ./configs/new-project.toml
```

### スクリプト統合

```bash
#!/bin/bash
# CI/CDスクリプトの例

set -e  # エラーで終了

# 設定を検証
cmdrun validate --check-cycles

# 品質チェックを並列実行
cmdrun run lint --parallel

# テストを実行
cmdrun run test

# すべてのチェックが通ればビルド
cmdrun run build --parallel

echo "ビルドが正常に完了しました！"
```

---

## 関連項目

- [スタートガイド](./getting-started.md)
- [設定リファレンス](../technical/configuration.md)
- [並列実行ガイド](./parallel-execution.md)
- [サンプル](./examples.md)

---

## ヘルプの取得

問題が発生した場合やヘルプが必要な場合：

1. クイックリファレンスは`cmdrun --help`を実行してください
2. コマンド固有のヘルプは`cmdrun <command> --help`を実行してください
3. [GitHub Issues](https://github.com/sanae-abe/cmdrun/issues)を確認してください
4. [完全なドキュメント](https://github.com/sanae-abe/cmdrun/docs)を読んでください

**クイックヘルプコマンド：**

```bash
# 一般的なヘルプ
cmdrun --help

# コマンド固有のヘルプ
cmdrun run --help
cmdrun add --help
cmdrun validate --help

# 設定内のすべてのコマンドを一覧表示
cmdrun list --verbose

# 設定の有効性を確認
cmdrun validate --verbose
```
