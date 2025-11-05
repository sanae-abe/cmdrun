# Watch Mode

Watch Modeは、ファイルの変更を監視し、変更検出時に自動的にコマンドを実行する機能です。開発中の自動再コンパイル、テスト実行、ホットリロードなどに便利です。

## 目次

- [基本的な使い方](#基本的な使い方)
- [オプション](#オプション)
- [実用例](#実用例)
- [パターンマッチング](#パターンマッチング)
- [除外パターン](#除外パターン)
- [デバウンス](#デバウンス)
- [トラブルシューティング](#トラブルシューティング)

## 基本的な使い方

最もシンプルな使い方は、監視したいコマンドを指定するだけです：

```bash
# カレントディレクトリ配下のすべてのファイルを監視
cmdrun watch dev

# 変更検出時にメッセージが表示され、コマンドが自動実行されます
```

## オプション

### パス指定 (`--path`, `-p`)

監視する特定のパスを指定できます：

```bash
# src ディレクトリを監視
cmdrun watch dev --path src

# 複数のパスを監視
cmdrun watch dev --path src --path tests
```

デフォルト: カレントディレクトリ

### パターン指定 (`--pattern`, `-w`)

Globパターンで監視するファイルを制限できます：

```bash
# Rustファイルのみ監視
cmdrun watch build --pattern "**/*.rs"

# 複数のパターン
cmdrun watch test --pattern "**/*.rs" --pattern "**/*.toml"

# TypeScriptとJSONファイル
cmdrun watch dev --pattern "**/*.ts" --pattern "**/*.json"
```

**サポートされるGlobパターン:**
- `**/*.rs` - すべてのRustファイル（再帰的）
- `*.js` - カレントディレクトリのJavaScriptファイルのみ
- `src/**/*` - srcディレクトリ以下のすべてのファイル
- `{*.js,*.ts}` - JavaScriptまたはTypeScriptファイル

デフォルト: `**/*` (すべてのファイル)

### 除外パターン (`--exclude`, `-e`)

特定のファイルやディレクトリを除外できます：

```bash
# testディレクトリを除外
cmdrun watch build --exclude "**/test/**"

# 複数の除外パターン
cmdrun watch dev --exclude "**/test/**" --exclude "**/*.tmp"

# ログファイルを除外
cmdrun watch build --exclude "**/*.log" --exclude "**/*.out"
```

**デフォルトで除外されるパターン:**
- `**/node_modules/**`
- `**/target/**`
- `**/.git/**`
- `**/dist/**`
- `**/build/**`
- `**/__pycache__/**`
- `**/.cache/**`

これらは `.gitignore` の尊重が有効な場合（デフォルト）に自動的に除外されます。

### デバウンス遅延 (`--debounce`, `-d`)

変更検出後、コマンド実行までの待機時間をミリ秒単位で指定します：

```bash
# 1秒待機
cmdrun watch build --debounce 1000

# 100ms待機（素早いフィードバック）
cmdrun watch test --debounce 100

# 2秒待機（大きなファイル変更時）
cmdrun watch deploy --debounce 2000
```

デフォルト: 500ms

**推奨値:**
- **開発サーバー**: 300-500ms（素早い再起動）
- **ビルド**: 500-1000ms（ファイル保存の完了を待つ）
- **テスト**: 500-1000ms（複数ファイル変更の統合）
- **デプロイ**: 1000-2000ms（不要な実行を防ぐ）

### gitignore無視 (`--ignore-gitignore`)

`.gitignore` ファイルを無視し、すべてのファイルを監視します：

```bash
# .gitignoreを無視（デバッグ時に有用）
cmdrun watch build --ignore-gitignore
```

デフォルト: false（`.gitignore`を尊重）

### 非再帰監視 (`--no-recursive`)

サブディレクトリを監視せず、指定されたディレクトリのみ監視します：

```bash
# カレントディレクトリのみ監視
cmdrun watch build --no-recursive

# src直下のファイルのみ
cmdrun watch test --path src --no-recursive
```

デフォルト: false（再帰的に監視）

## 実用例

### 1. Rust開発

```bash
# Rustファイルの変更を監視してビルド
cmdrun watch build --pattern "**/*.rs"

# テストの自動実行
cmdrun watch test --pattern "**/*.rs" --debounce 1000

# Cargo.tomlの変更も含める
cmdrun watch build --pattern "**/*.rs" --pattern "**/Cargo.toml"
```

### 2. TypeScript/Node.js開発

```bash
# TypeScriptファイルの監視
cmdrun watch dev --pattern "**/*.ts" --pattern "**/*.tsx"

# package.jsonの変更も監視
cmdrun watch dev --pattern "**/*.ts" --pattern "package.json"

# テストディレクトリを除外
cmdrun watch build --pattern "**/*.ts" --exclude "**/test/**"
```

### 3. Web開発

```bash
# HTML/CSS/JSファイルの監視
cmdrun watch dev --pattern "**/*.html" --pattern "**/*.css" --pattern "**/*.js"

# srcディレクトリのみ監視
cmdrun watch dev --path src

# 素早いフィードバック
cmdrun watch dev --debounce 300
```

### 4. Python開発

```bash
# Pythonファイルの監視
cmdrun watch test --pattern "**/*.py"

# __pycache__を除外（デフォルトで除外済み）
cmdrun watch test --pattern "**/*.py"

# 特定のディレクトリのみ
cmdrun watch test --path app --path tests
```

### 5. ドキュメント執筆

```bash
# Markdownファイルの監視
cmdrun watch preview --pattern "**/*.md"

# 長めのデバウンス（保存完了を待つ）
cmdrun watch build-docs --pattern "**/*.md" --debounce 1000
```

### 6. 複数パス・複雑なパターン

```bash
# 複数のソースディレクトリを監視
cmdrun watch build --path src --path lib --path api

# 特定の拡張子のみ（複数）
cmdrun watch lint --pattern "**/*.{js,ts,jsx,tsx}"

# 多くの除外パターン
cmdrun watch build \
  --exclude "**/test/**" \
  --exclude "**/mock/**" \
  --exclude "**/*.spec.ts"
```

## パターンマッチング

Watch Modeは[globパターン](https://en.wikipedia.org/wiki/Glob_(programming))を使用してファイルをマッチングします。

### 基本パターン

| パターン | 説明 | 例 |
|---------|------|-----|
| `*` | 任意の文字列（スラッシュ以外） | `*.rs` → `main.rs`, `lib.rs` |
| `**` | 任意のパス（ディレクトリを含む） | `**/*.rs` → `src/main.rs`, `src/lib/mod.rs` |
| `?` | 任意の1文字 | `test?.rs` → `test1.rs`, `testA.rs` |
| `[abc]` | 文字セット | `test[123].rs` → `test1.rs`, `test2.rs` |
| `{a,b}` | 選択肢 | `*.{js,ts}` → `main.js`, `app.ts` |

### パターン例

```bash
# すべてのRustファイル（再帰的）
--pattern "**/*.rs"

# srcディレクトリ内のすべてのファイル
--pattern "src/**/*"

# JavaScriptまたはTypeScript
--pattern "**/*.{js,ts}"

# test1.rs, test2.rs など
--pattern "**/test[0-9].rs"

# カレントディレクトリの.tomlファイルのみ
--pattern "*.toml"

# 複数レベルのワイルドカード
--pattern "src/**/component/*.tsx"
```

## 除外パターン

除外パターンは監視から特定のファイル/ディレクトリを除外します。

### デフォルト除外

`.gitignore`を尊重する場合（デフォルト）、以下が自動的に除外されます：

- `**/node_modules/**` - Node.jsの依存関係
- `**/target/**` - Rustのビルド成果物
- `**/.git/**` - Gitディレクトリ
- `**/dist/**` - 配布ディレクトリ
- `**/build/**` - ビルド成果物
- `**/__pycache__/**` - Pythonキャッシュ
- `**/.cache/**` - 各種キャッシュ

### カスタム除外

```bash
# テストファイルを除外
--exclude "**/test/**"

# 一時ファイルを除外
--exclude "**/*.tmp" --exclude "**/*.bak"

# 特定のディレクトリ
--exclude "**/vendor/**" --exclude "**/third_party/**"

# ログファイル
--exclude "**/*.log" --exclude "**/*.out"
```

### 除外の優先順位

1. `--exclude`で明示的に指定された除外パターン
2. `.gitignore`の除外（`--ignore-gitignore`が指定されていない場合）
3. デフォルトの除外パターン

## デバウンス

デバウンスは、ファイル変更検出後、コマンド実行前の待機時間です。

### デバウンスが重要な理由

- **複数ファイル保存**: エディタが複数ファイルを素早く保存する場合、1回だけコマンドを実行
- **ファイル書き込み完了**: 大きなファイルの書き込み完了を待つ
- **不要な実行防止**: 頻繁な変更時の無駄な実行を防ぐ

### デバウンス値の選択

```bash
# 素早いフィードバック（100-300ms）
cmdrun watch dev --debounce 300

# バランス型（500-1000ms）- デフォルト推奨
cmdrun watch build --debounce 500

# 慎重型（1000-2000ms）
cmdrun watch deploy --debounce 2000
```

### デバウンスの動作

```
時刻 0ms:    ファイルA変更 → タイマー開始（500ms）
時刻 200ms:  ファイルB変更 → タイマーリセット（500ms）
時刻 300ms:  ファイルC変更 → タイマーリセット（500ms）
時刻 800ms:  （変更なし）→ コマンド実行
```

## トラブルシューティング

### 変更が検出されない

**問題**: ファイルを変更してもコマンドが実行されない

**解決策**:

1. パターンを確認:
   ```bash
   # デバッグ出力を有効にして確認
   cmdrun watch dev -vvv --pattern "**/*.rs"
   ```

2. 除外パターンをチェック:
   ```bash
   # デフォルト除外を表示（起動時に表示されます）
   cmdrun watch dev
   ```

3. `.gitignore`を確認:
   ```bash
   # .gitignoreを一時的に無視
   cmdrun watch dev --ignore-gitignore
   ```

### コマンドが頻繁に実行される

**問題**: 1回の変更で複数回コマンドが実行される

**解決策**:

```bash
# デバウンス時間を増やす
cmdrun watch build --debounce 1000

# または2000ms
cmdrun watch build --debounce 2000
```

### パフォーマンスが遅い

**問題**: 大きなプロジェクトで監視が遅い

**解決策**:

1. 監視パスを限定:
   ```bash
   # プロジェクト全体ではなく、srcのみ
   cmdrun watch dev --path src
   ```

2. パターンを限定:
   ```bash
   # すべてのファイルではなく、特定の拡張子のみ
   cmdrun watch dev --pattern "**/*.rs"
   ```

3. 不要なディレクトリを除外:
   ```bash
   cmdrun watch dev --exclude "**/vendor/**" --exclude "**/third_party/**"
   ```

### 停止できない

**問題**: `Ctrl+C`を押しても停止しない

**解決策**:

- `Ctrl+C`を1秒程度長押し
- または別のターミナルから強制終了:
  ```bash
  pkill -f "cmdrun watch"
  ```

### エディタとの競合

**問題**: エディタの自動保存で意図しない実行

**解決策**:

1. デバウンスを増やす:
   ```bash
   cmdrun watch dev --debounce 1000
   ```

2. 一時ファイルを除外:
   ```bash
   cmdrun watch dev --exclude "**/*.swp" --exclude "**/*~"
   ```

## 高度な使い方

### 複数コマンドの監視

異なるコマンドで異なるパターンを監視したい場合は、複数のターミナルを使用:

```bash
# ターミナル1: ビルド監視
cmdrun watch build --pattern "**/*.rs"

# ターミナル2: テスト監視
cmdrun watch test --pattern "**/*.rs"

# ターミナル3: ドキュメント監視
cmdrun watch docs --pattern "**/*.md"
```

### シェルスクリプトとの組み合わせ

```bash
# シェルスクリプトでWatch Modeを起動
#!/bin/bash
if [ "$WATCH" = "true" ]; then
    cmdrun watch dev --pattern "**/*.rs" --debounce 500
else
    cmdrun run dev
fi
```

### CI/CD環境での使用

Watch ModeはCI/CD環境での使用は推奨されません（無限ループになるため）。開発環境でのみ使用してください。

## まとめ

Watch Modeは開発効率を大幅に向上させる強力な機能です：

- **自動化**: ファイル保存時に自動実行
- **柔軟性**: パターン、除外、デバウンスで細かく制御
- **パフォーマンス**: 効率的なファイル監視
- **使いやすさ**: シンプルなコマンドラインインターフェース

開発ワークフローに合わせて、オプションを調整してください。

## 関連ドキュメント

- [CLI リファレンス](CLI.md)
- [設定リファレンス](CONFIGURATION.md)
- [コマンド実行](CLI.md#run-command)
