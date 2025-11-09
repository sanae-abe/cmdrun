# CLIリファレンス

cmdrunコマンドラインインターフェースの完全なリファレンスです。

## 目次

- [グローバルオプション](#グローバルオプション)
- [コマンド](#コマンド)
  - [init](#init) - プロジェクト初期化
  - [add](#add) - コマンドを追加
  - [run](#run) - コマンドを実行
  - [retry](#retry) - 失敗コマンド再実行
  - [list](#list) - コマンド一覧
  - [remove](#remove) - コマンドを削除
  - [edit](#edit) - コマンドを編集
  - [info](#info) - コマンド情報を表示
  - [search](#search) - コマンドを検索
  - [graph](#graph) - 依存関係グラフ表示
  - [watch](#watch) - ファイル監視実行
  - [env](#env) - 環境管理
  - [history](#history) - 実行履歴管理
  - [template](#template) - テンプレート管理
  - [plugin](#plugin) - プラグイン管理
  - [open](#open) - 設定ファイルを開く
  - [validate](#validate) - 設定を検証
  - [config](#config) - 設定管理
  - [completion](#completion) - シェル補完スクリプト生成
  - [typo](#typo) - タイポ検出機能
- [終了コード](#終了コード)
- [設定ファイル](#設定ファイル)

---

## グローバルオプション

すべてのコマンドで使用可能なオプション:

### `-c, --config <FILE>`

設定ファイルのパスを指定します（デフォルト: `~/.config/cmdrun/commands.toml`）。

このオプションを使用すると、複数の設定ファイルを用途別に使い分けることができます。
仕事用、個人用、プロジェクト固有、環境別など、異なるコマンドセットを管理できます。

**注意:** これは設定ファイルの**パス指定**です。設定ファイル**の中身**を変更する場合は `cmdrun config` サブコマンドを使用してください。

**使用例:**

```bash
# 仕事用の設定ファイルを使用
cmdrun --config ~/work/commands.toml list
cmdrun -c ~/work/commands.toml run deploy

# 個人用の設定ファイルを使用
cmdrun -c ~/personal/commands.toml run backup

# プロジェクト固有の設定
cd ~/projects/myapp
cmdrun -c ./commands.toml run dev

# 環境別の設定
cmdrun -c ~/.cmdrun/production.toml run deploy
cmdrun -c ~/.cmdrun/staging.toml run deploy
cmdrun -c ~/.cmdrun/development.toml run dev
```

**詳細は[設定リファレンス](CONFIGURATION.md#カスタム設定ファイルの指定)を参照してください。**

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

### `-V, --version`

cmdrunのバージョンを表示します。

**使用例:**

```bash
cmdrun --version
# 出力: cmdrun 1.0.0
```

### `-v, --verbose`

詳細な出力を有効にします。複数回指定するとより詳細になります。

- `-v`: 詳細なログ
- `-vv`: デバッグレベルのログ
- `-vvv`: トレースレベルのログ

**使用例:**

```bash
# 標準出力
cmdrun run build

# 詳細出力
cmdrun -v run build

# デバッグ出力
cmdrun -vv run build

# トレース出力
cmdrun -vvv run build
```

---

## コマンド

### init

プロジェクトの初期化と設定ファイルの作成を行います。

#### 構文

```bash
cmdrun init [OPTIONS]
```

#### 説明

新しいプロジェクトでcmdrunを使い始める際に、設定ファイル（`commands.toml`）を作成します。
テンプレートを指定すると、プロジェクトタイプに応じた事前定義コマンドが含まれます。

#### オプション

- `--template <TEMPLATE>` - 使用するテンプレート（rust, nodejs, python, react等）
- `--language <LANG>` - 言語設定（english/japanese/chinese_simplified/chinese_traditional、デフォルト: english）

#### 使用例

```bash
# デフォルト設定で初期化
cmdrun init

# Rustプロジェクト用テンプレートで初期化
cmdrun init --template rust-cli

# 日本語設定で初期化
cmdrun init --language japanese --template nodejs-web

# 簡体中国語設定で初期化
cmdrun init --language chinese_simplified --template react-app
```

---

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

### retry

最後に失敗したコマンドを再実行します。

#### 構文

```bash
cmdrun retry
```

#### 説明

実行履歴から最後に失敗したコマンドを自動的に検出し、再実行します。
デバッグやテスト修正後の確認に便利です。

#### 使用例

```bash
# 失敗したテストを修正後に再実行
cmdrun run test  # 失敗
# ... コードを修正 ...
cmdrun retry  # 最後に失敗したcmdrunコマンドを再実行
```

**注意**: `cmdrun retry`は、**cmdrunで実行したコマンド**の履歴から最後に失敗したものを再実行します。シェルで直接実行したコマンド（`npm test`等）は対象外です。

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

### graph

コマンドの依存関係グラフを表示します。

#### 構文

```bash
cmdrun graph [OPTIONS] [COMMAND]
```

#### 説明

指定したコマンドの依存関係をツリー形式で表示します。
循環依存の検出や実行順序の確認に使用します。

#### 引数

- `[COMMAND]` - グラフ表示するコマンドのID（省略可）

#### オプション

- `--format <FORMAT>` - 出力形式（tree/dot、デフォルト: tree）

#### 使用例

```bash
# buildコマンドの依存関係を表示
cmdrun graph build

# DOT形式で出力（Graphviz用）
cmdrun graph build --format dot

# 全コマンドの依存関係を表示
cmdrun graph
```

#### 出力例

```
build
├── lint
├── test
│   └── compile
└── docs
```

---

### watch

ファイル変更を監視してコマンドを自動実行します。

#### 構文

```bash
cmdrun watch [OPTIONS] <COMMAND>
```

#### 説明

指定したパターンのファイル変更を監視し、変更があれば自動的にコマンドを実行します。
開発時の自動ビルドやテスト実行に便利です。

#### 引数

- `<COMMAND>` - 実行するコマンドのID（必須）

#### オプション

- `--pattern <PATTERN>` - 監視するファイルパターン（例: `**/*.rs`）
- `--path <PATH>` - 監視するディレクトリ（複数指定可能）
- `--debounce <MS>` - デバウンス時間（ミリ秒、デフォルト: 500）
- `--no-recursive` - 再帰的な監視を無効化

#### 使用例

```bash
# Rustファイルの変更を監視してビルド
cmdrun watch build --pattern "**/*.rs"

# テストを自動実行（デバウンス1秒）
cmdrun watch test --pattern "**/*.rs" --debounce 1000

# 複数のディレクトリを監視
cmdrun watch dev --path src --path lib
```

---

### env

環境管理を行います。

#### 構文

```bash
cmdrun env <SUBCOMMAND>
```

#### 説明

開発・ステージング・本番など異なる環境の作成・切り替え・管理を行います。
各環境ごとに環境変数やコマンド設定を分離できます。

#### サブコマンド

- `create <NAME>` - 新しい環境を作成
- `use <NAME>` - 環境を切り替え
- `current` - 現在の環境を表示
- `list` - 全環境を一覧表示
- `set <KEY> <VALUE>` - 環境変数を設定
- `remove <NAME>` - 環境を削除

#### 使用例

```bash
# 環境を作成
cmdrun env create dev --description "開発環境"
cmdrun env create prod --description "本番環境"

# 環境を切り替え
cmdrun env use dev
cmdrun run start  # 開発環境の設定で起動

# 環境変数を設定
cmdrun env set API_URL https://api.staging.com --env staging

# 現在の環境を確認
cmdrun env current

# 全環境を表示
cmdrun env list
```

**詳細は[環境管理ガイド](../ENVIRONMENT_MANAGEMENT.md)を参照してください。**

---

### history

実行履歴の管理を行います。

#### 構文

```bash
cmdrun history <SUBCOMMAND>
```

#### 説明

コマンド実行履歴の記録・検索・統計表示・エクスポートを行います。
SQLiteベースの永続化ストレージ（最大1000件）を使用します。

#### サブコマンド

- `list` - 履歴を一覧表示
- `search <KEYWORD>` - キーワードで履歴を検索
- `stats` - 統計情報を表示
- `export` - 履歴をエクスポート
- `clear` - 履歴をクリア

#### 使用例

```bash
# 履歴を表示
cmdrun history list

# コマンドを検索
cmdrun history search build

# 統計情報を表示
cmdrun history stats

# JSON形式でエクスポート
cmdrun history export --format json -o history.json

# 履歴をクリア
cmdrun history clear
```

**詳細は[履歴機能ガイド](HISTORY.md)を参照してください。**

---

### template

テンプレート管理を行います。

#### 構文

```bash
cmdrun template <SUBCOMMAND>
```

#### 説明

プロジェクトテンプレートの使用・作成・共有を行います。
ビルトインテンプレート（rust-cli, nodejs-web, python-data, react-app）を利用可能です。

#### サブコマンド

- `list` - 利用可能なテンプレートを一覧表示
- `use <TEMPLATE>` - テンプレートを使用
- `add <NAME>` - カスタムテンプレートを作成
- `export <TEMPLATE> <PATH>` - テンプレートをエクスポート

#### 使用例

```bash
# 利用可能なテンプレートを表示
cmdrun template list

# テンプレートを使用
cmdrun template use rust-cli

# カスタムテンプレートを作成
cmdrun template add my-template

# テンプレートをエクスポート
cmdrun template export rust-cli ./my-template.toml
```

**ビルトインテンプレート:**
- `rust-cli` - Rust CLI開発（cargo build/test/clippy/fmt）
- `nodejs-web` - Node.js Web開発（npm dev/build/test）
- `python-data` - Python データサイエンス（pytest/jupyter）
- `react-app` - React アプリケーション（dev/build/storybook）

**詳細は[テンプレート機能レポート](../../TEMPLATE_FEATURE_REPORT.md)を参照してください。**

---

### plugin

プラグイン管理を行います。

#### 構文

```bash
cmdrun plugin <SUBCOMMAND>
```

#### 説明

外部プラグインによる機能拡張を管理します。
動的プラグインローディング（libloading）により、コンパイル不要で機能を追加できます。

#### サブコマンド

- `list` - プラグインを一覧表示
- `info <NAME>` - プラグインの詳細を表示
- `enable <NAME>` - プラグインを有効化
- `disable <NAME>` - プラグインを無効化

#### 使用例

```bash
# プラグインを一覧表示
cmdrun plugin list

# プラグインの詳細を表示
cmdrun plugin info logger

# プラグインを有効化/無効化
cmdrun plugin enable logger
cmdrun plugin disable logger
```

**詳細は[プラグインシステムレポート](../../PLUGIN_SYSTEM_IMPLEMENTATION_REPORT.md)および[プラグインAPI](../plugins/API.md)を参照してください。**

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

cmdrunの設定を管理します（設定ファイル内の値の表示・変更）。

**注意:** このコマンドは設定ファイル**の中身**を管理します。**どの設定ファイルを使うか**を指定する場合は `--config/-c` グローバルオプションを使用してください。

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

# カスタム設定ファイルの内容を表示
cmdrun --config ~/work/commands.toml config show

# カスタム設定ファイルの値を変更
cmdrun -c ~/work/commands.toml config set language english
```

#### --config オプションとの違い

| 目的 | コマンド | 説明 |
|------|---------|------|
| 設定ファイルの選択 | `cmdrun --config <FILE> ...` | **どの**設定ファイルを使うか |
| 設定値の管理 | `cmdrun config ...` | 設定ファイル**の中身**を表示・変更 |

**実例:**
```bash
# 仕事用設定ファイルの言語設定を変更
cmdrun -c ~/work/commands.toml config set language japanese

# 個人用設定ファイルの内容を確認
cmdrun -c ~/personal/commands.toml config show

# デフォルト設定ファイルの値を取得
cmdrun config get shell
```

#### 出力例

```
現在の設定:

  language: japanese
  shell: bash
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

# Bash補完のインストール (Linux - system-wide)
# 注意: system-wideインストールにはroot権限が必要です
cmdrun completion bash | sudo tee /etc/bash_completion.d/cmdrun

# Bash補完のインストール (Linux - ユーザー固有、sudo不要)
mkdir -p ~/.local/share/bash-completion/completions
cmdrun completion bash > ~/.local/share/bash-completion/completions/cmdrun

# Zsh補完のインストール
cmdrun completion zsh > "${fpath[1]}/_cmdrun"

# Fish補完のインストール
cmdrun completion fish > ~/.config/fish/completions/cmdrun.fish
```

**セキュリティ注意:**
- system-wideインストール（`/etc/bash_completion.d/`）はroot権限が必要です
- ユーザー固有インストールも可能です（sudo不要、上記参照）
- 信頼できないスクリプトに対してsudoを使用しないでください

#### Shell別の機能

cmdrunのShell Completionは、各シェルの特性に応じた最適な補完体験を提供します。

**Zsh:**
- `Tab`を1回押下: 説明付きメニュー選択を表示
- 矢印キーまたは`Tab`/`Shift+Tab`でナビゲート
- 各コマンドの完全な説明を表示

**Bash:**
- `Tab`を2回押下: コマンド名リストを表示
- 説明なし（Bashの制限）

**Fish:**
- `Tab`押下: 説明付きコマンドリストを表示
- 矢印キーでナビゲート
- 入力に応じて自動フィルタリング

#### グローバル設定フォールバック

ローカル`commands.toml`がなくても、グローバル設定ファイル（`~/.config/cmdrun/commands.toml`）からコマンドを補完します。
プロジェクト外でもcmdrunの登録コマンドを利用可能です。

```bash
# ローカル設定がない場合でも動作
cd /tmp
cmdrun run [Tab]  # グローバル設定から補完
```

---

### typo

タイポ検出機能により、コマンド名の誤りを自動検出して修正候補を提示します。

#### 構文

この機能は自動的に動作します。設定で有効/無効を切り替えられます。

#### 説明

cmdrunは入力されたコマンド名の誤りを自動検出し、類似したコマンドを修正候補として提示します。
Levenshtein距離アルゴリズムにより、タイプミスや綴り間違いを検出します。

#### 動作例

```bash
$ cmdrun seach docker
Error: Unknown command 'seach'

もしかして:
  → search (distance: 1)
  → watch (distance: 2)

利用可能なコマンドは 'cmdrun --help' で確認してください。
```

```bash
$ cmdrun run biuld
Unknown command 'biuld'

💡 Did you mean one of these?
  → build (distance: 2)

Run 'cmdrun --help' for available commands.
```

#### 設定

```toml
[config]
typo_detection = true        # タイポ検出を有効化（デフォルト: true）
typo_threshold = 2           # 最大Levenshtein距離（デフォルト: 2）
auto_correct = false         # 自動修正（デフォルト: false）
```

**設定項目:**

- `typo_detection`: タイポ検出機能の有効/無効
- `typo_threshold`: 修正候補を提示する最大編集距離（1-3推奨）
- `auto_correct`: `true`の場合、候補が1つだけならば自動的に実行（注意して使用）

#### 多言語対応

エラーメッセージは設定言語に応じて表示されます:

- **英語**: "Did you mean 'X'?"
- **日本語**: "もしかして: 'X' ですか?"
- **簡体中文**: "您是否想输入 'X'?"
- **繁體中文**: "您是否想輸入 'X'?"

#### 重要な注意事項

**タイポ検出の対象範囲:**

タイポ検出は`cmdrun run <コマンド名>`で実行するコマンド名に対してのみ機能します。

```bash
# タイポ検出が動作する例
cmdrun run biuld    # → "build"を提案

# タイポ検出が動作しない例
cmdrun seach docker # サブコマンド "seach" 自体のタイポは検出しない
```

サブコマンド自体（search, remove, add等）のタイポは検出されません。
`cmdrun --help`で正しいサブコマンドを確認してください。

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

cmdrunは4言語をサポートしています: **英語、日本語、簡体中国語（简体中文）、繁体中国語（繁體中文）**。

設定ファイルで言語を指定できます:

```toml
[config]
language = "japanese"  # english / japanese / chinese_simplified / chinese_traditional (デフォルト: english)
```

**設定方法:**

```bash
# 日本語に設定
cmdrun config set language japanese

# 簡体中国語に設定
cmdrun config set language chinese_simplified

# 繁体中国語に設定
cmdrun config set language chinese_traditional
```

### 環境変数

cmdrunは以下の環境変数を認識します:

- `NO_COLOR` - カラー出力を無効化
- `CMDRUN_LOG` - ログレベルを設定 (error, warn, info, debug, trace)

**使用例:**

```bash
# カラーを無効化
export NO_COLOR=1
cmdrun list

# デバッグログを有効化
export CMDRUN_LOG=debug
cmdrun run test
```

**注意:** 設定ファイルのパス指定には `--config/-c` オプションを使用してください:

```bash
# 推奨: --config オプションを使用
cmdrun --config /path/to/custom/commands.toml list
cmdrun -c ~/work/commands.toml run build

# 非推奨: 環境変数 CMDRUN_CONFIG（サポートされていません）
# export CMDRUN_CONFIG=/path/to/custom/commands.toml
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
