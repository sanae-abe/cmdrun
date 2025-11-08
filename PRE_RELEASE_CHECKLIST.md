# cmdrun v1.0.0 - リリース前動作確認チェックリスト

> **目的**: crates.io公開前の最終動作確認
> **対象**: v1.0.0（Phase 1-2全機能統合版）
> **作成日**: 2025-11-07

---

## 📋 確認手順

各項目を実行し、✅をチェックしてください。

---

## 🔧 1. ビルド・インストール確認

### 1.1 ビルド
```bash
# クリーンビルド
cargo clean
cargo build --release

# 期待: エラーなし、警告最小限
```
- [x] リリースビルド成功
- [x] 警告が許容範囲内（< 10件）

ユーザー確認結果：✅

### 1.2 テスト
```bash
# 全テスト実行
cargo test --workspace

# 期待: 339 passed; 0 failed
```
- [x] 全339テストパス
- [x] 失敗テスト0件

ユーザー確認結果：✅

### 1.3 インストール
```bash
# ローカルインストール
cargo install --path . --force

# バージョン確認
cmdrun --version

# ヘルプ表示
cmdrun --help

# 期待: cmdrun 1.0.0
```
- [x] インストール成功
- [x] バージョン表示正常
- [ ] ヘルプ表示正常

ユーザー確認結果：❌ヘルプにコマンドやオプションが不足している

---

## 🎯 2. コア機能確認

### 2.1 初期化
```bash
# 設定ファイル初期化
cd /tmp/private/cmdrun-test
cmdrun init --template rust

# 期待: commands.toml作成、テンプレート適用
```
- [x] `commands.toml` 作成成功
- [x] テンプレート内容正常

ユーザー確認結果：✅

### 2.2 コマンド管理
```bash
# コマンド追加
cmdrun add test-cmd "echo テスト" "テストコマンド"

# コマンド一覧
cmdrun list

# コマンド実行
cmdrun run test-cmd

# コマンド削除
cmdrun remove test-cmd

# 期待: 各操作が正常に完了
```
- [x] `add` 成功
- [x] `list` 表示正常
- [x] `run` 実行成功
- [x] `remove` 削除成功

ユーザー確認結果：✅

### 2.3 依存関係
```bash
# 依存関係グラフ表示
cmdrun graph build

# 期待: ツリー形式表示、循環依存なし
```
- [ ] グラフ表示正常
- [ ] 依存関係解決正常

ユーザー確認結果：❌ No dependenciesと表示される。
❯ cmdrun graph build
2025-11-08T03:16:58.463391Z  INFO cmdrun::config::loader: Loading global config: /Users/sanae.abe/Library/Application Support/cmdrun/commands.toml
2025-11-08T03:16:58.464328Z  INFO cmdrun::config::loader: Loading local config: /private/tmp/private/cmdrun-test/commands.toml
Dependencies for: build

  No dependencies

### 2.4 Watch Mode
```bash
# Watch Mode起動（別ターミナル）
cmdrun watch test --pattern "**/*.rs"

# ファイル変更してトリガー確認
# Ctrl+C で終了

# 期待: ファイル変更検知、自動実行
```
- [x] Watch Mode起動
- [ ] ファイル変更検知
- [ ] 自動実行成功

ユーザー確認結果：❌ エラーが表示される
❯ cmdrun watch test --pattern "**/*.css"
2025-11-08T03:25:39.788699Z  INFO cmdrun::config::loader: Loading global config: /Users/sanae.abe/Library/Application Support/cmdrun/commands.toml
2025-11-08T03:25:39.790903Z  INFO cmdrun::config::loader: Loading local config: /Users/sanae.abe/homebrew/var/www/wordpress/wp-content/themes/go100/commands.toml
Watch Configuration
════════════════════════════════════════════════════════════
  Command: test
  Watching: .
  Patterns: **/*.css
  Debounce: 500ms
════════════════════════════════════════════════════════════
2025-11-08T03:25:39.794844Z  INFO cmdrun::commands::watch: Watch mode started. Press Ctrl+C to stop.

2025-11-08T03:25:39.794857Z  INFO cmdrun::watch::watcher: Starting watch mode paths=["/Users/sanae.abe/homebrew/var/www/wordpress/wp-content/themes/go100"] command=test
2025-11-08T03:25:39.796593Z  INFO cmdrun::watch::watcher: Watching path path=/Users/sanae.abe/homebrew/var/www/wordpress/wp-content/themes/go100 recursive=true
2025-11-08T03:25:39.796605Z  INFO cmdrun::watch::watcher: Watch mode started. Press Ctrl+C to stop.
2025-11-08T03:25:55.098720Z  INFO cmdrun::watch::executor: Executing command due to file change command=test path=/Users/sanae.abe/homebrew/var/www/wordpress/wp-content/themes/go100/assets/src/css/pages/single-seminar.css
2025-11-08T03:25:55.139294Z ERROR cmdrun::watch::executor: Command failed exit_code=exit status: 1
2025-11-08T03:25:55.139326Z  INFO cmdrun::watch::watcher: Command executed successfully path=/Users/sanae.abe/homebrew/var/www/wordpress/wp-content/themes/go100/assets/src/css/pages/single-seminar.css
2025-11-08T03:26:01.629878Z  INFO cmdrun::watch::executor: Executing command due to file change command=test path=/Users/sanae.abe/homebrew/var/www/wordpress/wp-content/themes/go100/assets/src/css/pages/single-seminar.css
2025-11-08T03:26:01.648685Z ERROR cmdrun::watch::executor: Command failed exit_code=exit status: 1
2025-11-08T03:26:01.648700Z  INFO cmdrun::watch::watcher: Command executed successfully path=/Users/sanae.abe/homebrew/var/www/wordpress/wp-content/themes/go100/assets/src/css/pages/single-seminar.css
2025-11-08T03:26:04.077376Z  INFO cmdrun::watch::executor: Executing command due to file change command=test path=/Users/sanae.abe/homebrew/var/www/wordpress/wp-content/themes/go100/assets/src/css/pages/single-seminar.css
2025-11-08T03:26:04.094556Z ERROR cmdrun::watch::executor: Command failed exit_code=exit status: 1
2025-11-08T03:26:04.094586Z  INFO cmdrun::watch::watcher: Command executed successfully path=/Users/sanae.abe/homebrew/var/www/wordpress/wp-content/themes/go100/assets/src/css/pages/single-seminar.css

---

## ✨ 3. v1.2.0新機能確認

### 3.1 環境管理
```bash
# 環境作成
cmdrun env create dev --description "開発環境"
cmdrun env create prod --description "本番環境"

# 環境一覧
cmdrun env list

# 環境切り替え
cmdrun env use dev
cmdrun env current

# 環境変数設定
cmdrun env set API_URL https://api.dev.com

# 期待: 各操作正常、環境分離動作
```
- [x] 環境作成成功
- [x] 環境一覧表示
- [x] 環境切り替え動作
- [x] 変数設定成功

ユーザー確認結果：❌ 環境ごとにコマンドを切り替える機能がない


### 3.2 履歴・ログ
```bash
# コマンド実行（履歴記録）
cmdrun run test

# 履歴表示
cmdrun history list

# 履歴検索
cmdrun history search test

# 統計表示
cmdrun history stats

# 履歴エクスポート
cmdrun history export --format json -o /tmp/history.json

# 失敗コマンド再実行
cmdrun retry

# 期待: 履歴記録・検索・統計・エクスポート正常
```
- [ ] 履歴記録動作
- [ ] 履歴表示正常
- [ ] 検索機能動作
- [ ] 統計表示正常
- [ ] エクスポート成功
- [ ] retry動作正常

ユーザー確認結果：
  - ❌履歴記録動作でエラーが出る
❯ cmdrun run test
2025-11-08T03:36:15.909806Z  INFO cmdrun::config::loader: Loading global config: /Users/sanae.abe/Library/Application Support/cmdrun/commands.toml
2025-11-08T03:36:15.911335Z  INFO cmdrun::config::loader: Loading local config: /Users/sanae.abe/homebrew/var/www/wordpress/wp-content/themes/go100/commands.toml
Running: Run tests
→ npm test
npm error Missing script: "test"
npm error
npm error To see a list of scripts, run:
npm error   npm run
npm error A complete log of this run can be found in: /Users/sanae.abe/.npm/_logs/2025-11-08T03_36_16_050Z-debug-0.log
Error: Command execution error: Command failed with exit code 1: npm test
  - ❌履歴表示が出ない
	  ❯ cmdrun history list
	No history entries found

### 3.3 テンプレート
```bash
# テンプレート一覧
cmdrun template list

# テンプレート使用
cmdrun template use nodejs-web -o /tmp/test-nodejs.toml

# カスタムテンプレート作成
cmdrun template add my-template

# テンプレートエクスポート
cmdrun template export rust-cli /tmp/rust-cli.toml

# 期待: 4種組み込みテンプレート、カスタム作成・エクスポート動作
```
- [x] 組み込みテンプレート4種確認
- [x] テンプレート使用成功
- [x] カスタム作成動作
- [x] エクスポート成功

ユーザー確認結果：✅

### 3.4 プラグイン（基本）
```bash
# プラグイン一覧
cmdrun plugin list

# プラグイン情報（サンプルプラグインがある場合）
cmdrun plugin info hello

# 期待: プラグイン管理機能動作
```
- [ ] プラグイン一覧表示
- [ ] プラグイン情報表示

ユーザー確認結果：❌エラーが出る

 via  v8.4.14 on   sanae-abe@m3.com
❯ cmdrun plugin list
error: unrecognized subcommand 'plugin'

Usage: cmdrun [OPTIONS] <COMMAND>

For more information, try '--help'.

go100 on  style/form-dark-mode [!?] is 󰏗 v1.0.0 via  v25.0.0 via  v8.4.14 on   sanae-abe@m3.com
❯ cmdrun plugin info hello
error: unrecognized subcommand 'plugin'

Usage: cmdrun [OPTIONS] <COMMAND>

For more information, try '--help'.

---

## 🌐 4. グローバル設定確認

### 4.1 グローバル設定作成
```bash
# プラットフォーム別グローバル設定ディレクトリ
# Linux: ~/.config/cmdrun/
# macOS: ~/Library/Application Support/cmdrun/
# Windows: %APPDATA%/cmdrun/

# グローバル設定作成（macOS例）
mkdir -p ~/Library/Application\ Support/cmdrun
cat ~/Library/Application\ Support/cmdrun/commands.toml
[commands.global-cmd]
description = """Global command"
cmd = "echo Global command works"

# 期待: グローバル設定ファイル作成成功
```
- [x] グローバル設定ディレクトリ作成
- [x] グローバル設定ファイル作成

ユーザー確認結果：✅

### 4.2 グローバル+ローカルマージ
```bash
# ローカル設定
cd /tmp/test-global
cmdrun init

# グローバルコマンド表示確認
cmdrun list

# 期待: グローバルとローカルのコマンド両方表示
```
- [x] グローバルコマンド一覧表示
- [x] ローカルコマンド一覧表示
- [x] マージ動作正常

### 4.3 優先順位確認
```bash
# ローカル設定が優先されることを確認
cmdrun info <command>

# 期待: ローカル設定がグローバルを上書き
```
- [x] ローカル優先順位正常

ユーザー確認結果：✅

---

## 🌍 5. クロスプラットフォーム確認

### 10.1 シェル検出
```bash
# 現在のシェル検出テスト
cmdrun run test

# 期待: bash/zsh/fish/pwsh自動検出
```
- [ ] シェル自動検出動作

ユーザー確認結果：❓確認手順が不明

### 10.2 パス処理
```bash
# プラットフォーム固有パス
cmdrun info test

# 期待: 正しいパス表示（Linux/macOS: /、Windows: \）
```
- [ ] パス処理正常

ユーザー確認結果：❌ パスが表示されない

---

## 📚 6. ドキュメント確認

### 10.1 README
```bash
# README確認
cat README.md | head -50
cat README.ja.md | head -50

# 期待: v1.2.0機能記載、英語（README.md）・日本語（README.ja.md）
```
- [ ] README.md更新済み（英語版）
- [ ] README.ja.md更新済み（日本語版）
- [ ] 4大機能記載確認

ユーザー確認結果：ℹ️「4大機能」とは何か知らないので確認できない。Caude Codeが確認して。

### 10.2 CHANGELOG
```bash
# CHANGELOG確認
cat CHANGELOG.md | head -100

# 期待: v1.0.0セクション、4大機能記載
```
- [x] CHANGELOG.md更新済み
- [x] v1.0.0に全機能統合

ユーザー確認結果：ℹ️「4大機能」とは何か知らないので確認できない。Caude Codeが確認して。

### 6.3 ユーザーガイド
```bash
# ドキュメント一覧
ls docs/user-guide/

# 期待: HISTORY.md, FAQ.md等の新規ドキュメント
```
- [ ] ユーザーガイド充実
- [ ] 技術ドキュメント整備

ユーザー確認結果：ℹ️内容を把握していないので確認できない。Caude Codeが確認して。

---

## ⚡ 7. パフォーマンス確認

### 10.1 起動時間
```bash
# 起動時間測定（hyperfineインストール済みの場合）
hyperfine --warmup 5 --min-runs 20 'cmdrun --version'

# 期待: < 10ms（目標4ms）
```
- [x] 起動時間10ms以下

ユーザー確認結果：✅

### 10.2 メモリ使用量
```bash
# メモリ使用量確認（macOS）
/usr/bin/time -l cmdrun --version 2>&1 | grep "maximum resident set size"

# 期待: < 15MB
```
- [x] メモリ使用量15MB以下

ユーザー確認結果：✅

---

## 🔒 8. セキュリティ確認

### 10.1 依存関係監査
```bash
# 脆弱性スキャン
cargo audit

# 期待: 0 vulnerabilities found
```
- [x] 既知脆弱性なし

ユーザー確認結果：✅

### 10.2 シェルインジェクション対策
```bash
# 危険なコマンド検証（失敗すべき）
cmdrun add dangerous "echo test; rm -rf /" "Dangerous"

# 期待: バリデーションエラー
```
- [ ] 危険コマンド拒否

ユーザー確認結果：❌ 成功する

❯ cmdrun add dangerous "echo test; rm -rf /" "Dangerous"
2025-11-08T04:00:51.586064Z  INFO cmdrun::config::loader: Loading global config: /Users/sanae.abe/Library/Application Support/cmdrun/commands.toml
📝 Adding command 'dangerous' /Users/sanae.abe/.cmdrun/commands.toml
✓ Command added successfully 'dangerous'
  Description: Dangerous
  Command: echo test; rm -rf /

---

## 📦 9. パッケージング確認

### 10.1 Cargo.toml
```bash
# メタデータ確認
grep -A 10 "\[package\]" Cargo.toml

# 期待: version = "1.0.0", 正しいメタデータ
```
- [x] バージョン1.0.0
- [ ] メタデータ完全

ユーザー確認結果：ℹ️完全なメタデータが分からない。Caude Codeが確認して

### 10.2 dry-run
```bash
# 公開テスト（実際には公開しない）
cargo publish --dry-run

# 期待: エラーなし、警告最小限
```
- [ ] dry-run成功
- [ ] パッケージサイズ適切

ユーザー確認結果：❌ エラーが出る

❯ cargo publish --dry-run
    Updating crates.io index
error: 1 files in the working directory contain changes that were not yet committed into git:

PRE_RELEASE_CHECKLIST.md

to proceed despite this and include the uncommitted changes, pass the `--allow-dirty` flag

---

## ✅ 10. 最終確認

### 10.1 総合チェック
- [ ] 全227テストパス
- [ ] コンパイル警告0件（許容範囲内）
- [ ] ドキュメント更新完了
- [ ] CHANGELOG.md完全
- [ ] セキュリティ監査クリア
- [ ] パフォーマンス目標達成

### 10.2 Git状態
```bash
git status
git log --oneline -5

# 期待: クリーンな状態、適切なコミット履歴
```
- [ ] 作業ツリークリーン
- [ ] コミット履歴適切

---

## 🚀 公開準備完了条件

**すべての項目に✅がついたら、crates.io公開可能です。**

### 公開コマンド
```bash
# 1. タグ作成
git tag -a v1.0.0 -m "Release v1.0.0"
git push origin v1.0.0

# 2. crates.io公開
cargo publish

# 3. GitHub Release作成
gh release create v1.0.0 --title "v1.0.0" --notes-file CHANGELOG.md
```

---

## 📝 メモ欄

**問題が見つかった場合**:
- 問題内容:
- 修正内容:
- 再確認日:

---

**確認完了日**: _____ / _____ / _____
**確認者**: _____________________
