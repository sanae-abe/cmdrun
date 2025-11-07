# Command History Feature

cmdrunは包括的なコマンド実行履歴機能を提供します。すべてのコマンド実行が自動的に記録され、検索・分析・再実行が可能です。

## 機能概要

- **自動記録**: すべてのコマンド実行を自動追跡
- **詳細な履歴**: コマンド名、引数、実行時間、終了コード、作業ディレクトリを記録
- **セキュリティ**: 機密情報（API_KEY、PASSWORDなど）の自動フィルタリング
- **高速検索**: SQLiteベースの効率的なクエリ
- **エクスポート**: JSON/CSV形式での履歴エクスポート
- **統計情報**: 成功率、平均実行時間などの統計表示
- **簡単な再実行**: 失敗したコマンドの即座の再実行

## 基本的な使い方

### 履歴一覧の表示

```bash
# 最新50件を表示（デフォルト）
cmdrun history list

# 最新20件を表示
cmdrun history list --limit 20

# 失敗したコマンドのみ表示
cmdrun history list --failed

# 統計情報を表示
cmdrun history list --stats
```

### 履歴検索

```bash
# "build"を含むコマンドを検索
cmdrun history search build

# "test"を含むコマンドを最大10件検索
cmdrun history search test --limit 10
```

### 統計情報の表示

```bash
# 詳細な統計情報を表示
cmdrun history stats
```

出力例:
```
History Statistics

  Total commands: 156
  Successful: 142
  Failed: 14
  Success rate: 91.0%
  Avg duration: 2.34s
```

### コマンドの再実行

```bash
# 最後に失敗したコマンドを再実行
cmdrun retry

# 特定のIDのコマンドを再実行
cmdrun retry 42
```

### 履歴のエクスポート

```bash
# JSON形式でエクスポート
cmdrun history export --format json -o history.json

# CSV形式でエクスポート
cmdrun history export --format csv -o history.csv

# 最新100件のみエクスポート
cmdrun history export --format json --limit 100
```

### 履歴のクリア

```bash
# 確認プロンプト付きでクリア
cmdrun history clear

# 確認なしでクリア
cmdrun history clear --force
```

## 履歴データの保存場所

履歴データはSQLiteデータベースに保存されます:

- **Linux/macOS**: `~/.local/share/cmdrun/history.db`
- **Windows**: `%APPDATA%\cmdrun\history.db`

## 記録される情報

各コマンド実行について以下の情報が記録されます:

- **コマンド名**: 実行されたコマンド
- **引数**: コマンドに渡された引数（JSON形式）
- **開始時刻**: コマンド実行の開始時刻
- **実行時間**: コマンドの実行時間（ミリ秒）
- **終了コード**: プロセスの終了コード
- **成功/失敗**: コマンドが成功したか失敗したか
- **作業ディレクトリ**: コマンド実行時の作業ディレクトリ
- **環境変数**: コマンド実行時の環境変数（機密情報は除外）

## セキュリティ機能

### 機密情報の自動フィルタリング

以下のパターンを含む環境変数は自動的に履歴から除外されます:

- `KEY` (例: `API_KEY`, `SECRET_KEY`)
- `SECRET` (例: `SECRET_TOKEN`, `AWS_SECRET`)
- `TOKEN` (例: `ACCESS_TOKEN`, `AUTH_TOKEN`)
- `PASSWORD` (例: `DB_PASSWORD`, `USER_PASSWORD`)
- `PASS` (例: `DB_PASS`)
- `API` (例: `API_SECRET`)
- `AUTH` (例: `AUTH_KEY`)
- `CREDENTIAL` (例: `AWS_CREDENTIALS`)

### 履歴サイズ制限

デフォルトでは最大1000件の履歴が保持されます。この制限を超えると、古いエントリから自動的に削除されます。

## 実用例

### 1. デプロイコマンドの失敗を調査

```bash
# 失敗したコマンドを一覧表示
cmdrun history list --failed

# 特定のデプロイコマンドを検索
cmdrun history search deploy

# 最後に失敗したコマンドの詳細を確認してから再実行
cmdrun retry
```

### 2. パフォーマンス分析

```bash
# 統計情報で平均実行時間を確認
cmdrun history stats

# ビルドコマンドの履歴を検索
cmdrun history search build

# エクスポートしてさらに詳細な分析
cmdrun history export --format json -o build_history.json
```

### 3. チーム共有のためのエクスポート

```bash
# 最新100件をCSV形式でエクスポート
cmdrun history export --format csv --limit 100 -o team_history.csv
```

### 4. 定期的なクリーンアップ

```bash
# 古い履歴をクリア
cmdrun history clear --force

# または、エクスポート後にクリア
cmdrun history export --format json -o backup.json
cmdrun history clear --force
```

## JSON出力例

```json
[
  {
    "id": 42,
    "command": "build",
    "args": "[\"--release\"]",
    "start_time": 1699876543210,
    "duration_ms": 2500,
    "exit_code": 0,
    "success": true,
    "working_dir": "/home/user/project",
    "environment": "{\"PATH\":\"/usr/bin\",\"HOME\":\"/home/user\"}"
  }
]
```

## CSV出力例

```csv
id,command,args,start_time,duration_ms,exit_code,success,working_dir
42,build,"[""--release""]",2024-11-07T12:34:56Z,2500,0,true,/home/user/project
43,test,[],2024-11-07T12:35:10Z,1200,0,true,/home/user/project
44,deploy,"[""production""]",2024-11-07T12:36:00Z,5000,1,false,/home/user/project
```

## トラブルシューティング

### 履歴が記録されない

データベースファイルへの書き込み権限を確認してください:
```bash
ls -la ~/.local/share/cmdrun/history.db
```

### データベースが大きくなりすぎた

履歴をクリアするか、制限を調整してください:
```bash
cmdrun history clear
```

### エクスポートに時間がかかる

`--limit`オプションでエクスポート件数を制限してください:
```bash
cmdrun history export --format json --limit 100 -o history.json
```

## パフォーマンス

- **起動時間**: 履歴機能は起動時間に影響しません（遅延読み込み）
- **メモリ使用量**: SQLiteベースで効率的、10MB以下
- **検索速度**: インデックス付きで高速（1000件で < 10ms）
- **ストレージ**: 1件あたり約1KB、1000件で約1MB

## 今後の機能予定

- [ ] 履歴の自動バックアップ
- [ ] より高度なフィルタリング（日付範囲、実行時間など）
- [ ] グラフィカルな統計表示
- [ ] 履歴に基づくコマンド推奨機能
- [ ] リモートバックアップ機能
