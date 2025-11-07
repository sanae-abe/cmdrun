# Environment Management

cmdrunの環境管理機能により、開発・ステージング・本番など異なる環境を簡単に切り替えることができます。

## 概要

環境管理機能は以下を提供します:

- 環境の作成・切り替え
- 環境別の設定ファイル
- 環境変数のプロファイル管理
- 設定の自動マージ

## コマンド一覧

### 環境の作成

```bash
# 新しい環境を作成
cmdrun env create dev --description "Development environment"
cmdrun env create staging --description "Staging environment"
cmdrun env create prod --description "Production environment"
```

### 環境の切り替え

```bash
# 環境を切り替え
cmdrun env use dev
cmdrun env use staging
cmdrun env use prod

# デフォルト環境に戻る
cmdrun env use default
```

### 現在の環境確認

```bash
# 現在アクティブな環境を表示
cmdrun env current
```

### 環境一覧

```bash
# 利用可能な環境を一覧表示
cmdrun env list
```

### 環境変数の設定

```bash
# 現在の環境に変数を設定
cmdrun env set NODE_ENV development

# 特定の環境に変数を設定
cmdrun env set API_URL https://api.staging.com --env staging
cmdrun env set API_URL https://api.production.com --env prod
```

### 環境の詳細情報

```bash
# 現在の環境の詳細を表示
cmdrun env info

# 特定の環境の詳細を表示
cmdrun env info prod
```

## 設定ファイル構造

### ディレクトリ構成

```
.cmdrun/
├── config.toml              # デフォルト設定
├── config.dev.toml          # 開発環境設定
├── config.staging.toml      # ステージング環境設定
└── config.prod.toml         # 本番環境設定
```

### 設定ファイル例

#### デフォルト設定 (.cmdrun/config.toml)

```toml
[config]
shell = "bash"
timeout = 300

[config.env]
APP_NAME = "MyApp"
LOG_LEVEL = "info"

[environment]
current = "dev"  # 現在の環境

[environment.dev]
description = "Development environment"

[environment.dev.variables]
NODE_ENV = "development"
API_URL = "http://localhost:3000"
DEBUG = "true"

[environment.staging]
description = "Staging environment"

[environment.staging.variables]
NODE_ENV = "staging"
API_URL = "https://api-staging.example.com"
DEBUG = "false"

[environment.prod]
description = "Production environment"

[environment.prod.variables]
NODE_ENV = "production"
API_URL = "https://api.example.com"
DEBUG = "false"

[commands.start]
description = "Start the application"
cmd = "npm start"
env = { PORT = "3000" }

[commands.test]
description = "Run tests"
cmd = "npm test"
```

#### 開発環境設定 (.cmdrun/config.dev.toml)

```toml
[config]
timeout = 600  # 開発環境では長めのタイムアウト

[config.env]
LOG_LEVEL = "debug"

[commands.dev_start]
description = "Start development server with hot reload"
cmd = "npm run dev"

[commands.debug]
description = "Start with debugger"
cmd = "node --inspect src/index.js"
```

#### 本番環境設定 (.cmdrun/config.prod.toml)

```toml
[config]
strict_mode = true
timeout = 60

[config.env]
LOG_LEVEL = "warn"

[commands.deploy]
description = "Deploy to production"
cmd = "npm run deploy"
confirm = true  # 本番デプロイには確認を要求
```

## 設定のマージ動作

環境を切り替えると、以下の順序で設定がマージされます:

1. **ベース設定** (.cmdrun/config.toml) を読み込み
2. **環境固有設定** (.cmdrun/config.{env}.toml) があればマージ
3. 環境固有の設定が優先される

### マージ例

ベース設定:
```toml
[config]
shell = "bash"
timeout = 300

[config.env]
BASE_VAR = "base_value"

[commands.test]
cmd = "npm test"
```

開発環境設定:
```toml
[config]
timeout = 600

[config.env]
BASE_VAR = "dev_override"
DEV_VAR = "dev_value"

[commands.dev_test]
cmd = "npm run test:dev"
```

マージ結果 (dev環境):
```toml
[config]
shell = "bash"      # ベースから
timeout = 600       # dev で上書き

[config.env]
BASE_VAR = "dev_override"  # dev で上書き
DEV_VAR = "dev_value"      # dev で追加

[commands.test]
cmd = "npm test"           # ベースから

[commands.dev_test]
cmd = "npm run test:dev"   # dev で追加
```

## 実用例

### ケース1: Web開発

```bash
# 開発環境を作成
cmdrun env create dev --description "Local development"
cmdrun env set NODE_ENV development --env dev
cmdrun env set API_URL http://localhost:3000 --env dev

# ステージング環境を作成
cmdrun env create staging --description "Staging server"
cmdrun env set NODE_ENV staging --env staging
cmdrun env set API_URL https://api-staging.example.com --env staging

# 本番環境を作成
cmdrun env create prod --description "Production server"
cmdrun env set NODE_ENV production --env prod
cmdrun env set API_URL https://api.example.com --env prod

# 開発開始
cmdrun env use dev
cmdrun run start  # 開発環境の設定で起動

# ステージングテスト
cmdrun env use staging
cmdrun run test  # ステージング環境の設定でテスト

# 本番デプロイ
cmdrun env use prod
cmdrun run deploy  # 本番環境の設定でデプロイ
```

### ケース2: マイクロサービス開発

```bash
# サービスごとの環境を作成
cmdrun env create auth-service
cmdrun env set SERVICE_NAME auth --env auth-service
cmdrun env set PORT 3001 --env auth-service

cmdrun env create api-gateway
cmdrun env set SERVICE_NAME gateway --env api-gateway
cmdrun env set PORT 3000 --env api-gateway

# サービスを切り替えて作業
cmdrun env use auth-service
cmdrun run dev  # 認証サービス起動

cmdrun env use api-gateway
cmdrun run dev  # APIゲートウェイ起動
```

### ケース3: データベース環境

```bash
# データベース環境を作成
cmdrun env create db-dev
cmdrun env set DB_HOST localhost --env db-dev
cmdrun env set DB_PORT 5432 --env db-dev
cmdrun env set DB_NAME myapp_dev --env db-dev

cmdrun env create db-prod
cmdrun env set DB_HOST prod-db.example.com --env db-prod
cmdrun env set DB_PORT 5432 --env db-prod
cmdrun env set DB_NAME myapp_prod --env db-prod

# 開発データベースでマイグレーション
cmdrun env use db-dev
cmdrun run migrate

# 本番データベースでマイグレーション（確認あり）
cmdrun env use db-prod
cmdrun run migrate  # confirm = true で確認プロンプト表示
```

## セキュリティ考慮事項

### 機密情報の保護

環境変数に機密情報（パスワード、APIキーなど）を含める場合:

1. `.cmdrun/` ディレクトリを `.gitignore` に追加
2. 環境変数は環境ごとに異なる値を設定
3. 本番環境の設定は特に厳重に管理

```bash
# .gitignore に追加
echo ".cmdrun/config.*.toml" >> .gitignore
```

### 本番環境の安全性

```toml
# config.prod.toml
[config]
strict_mode = true  # 厳格モード有効化

[commands.deploy]
description = "Deploy to production"
cmd = "./deploy.sh"
confirm = true  # デプロイ前に確認を要求
timeout = 300
```

## トラブルシューティング

### 環境が見つからない

```bash
$ cmdrun env use nonexistent
Error: Environment 'nonexistent' not found

# 解決: 環境を作成
$ cmdrun env create nonexistent
```

### 設定ファイルの確認

```bash
# 環境の詳細情報を表示
$ cmdrun env info dev

# 設定ファイルの場所を確認
Environment: dev
  Description: Development environment
  Environment variables:
    NODE_ENV = development
    API_URL = http://localhost:3000
  Config file: .cmdrun/config.dev.toml
```

### 設定のリセット

デフォルト環境に戻すには:

```bash
cmdrun env use default
```

## API（プログラム利用）

Rust APIとして環境管理機能を利用:

```rust
use cmdrun::config::environment::EnvironmentManager;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let manager = EnvironmentManager::default_instance()?;

    // 環境作成
    manager.create_environment(
        "dev".to_string(),
        "Development environment".to_string()
    ).await?;

    // 環境切り替え
    manager.switch_environment("dev").await?;

    // 変数設定
    manager.set_variable(
        "dev",
        "API_URL".to_string(),
        "http://localhost:3000".to_string()
    ).await?;

    // 現在の環境取得
    let current = manager.get_current_environment().await?;
    println!("Current environment: {}", current);

    Ok(())
}
```

## まとめ

環境管理機能により:

- ✅ 環境ごとの設定を簡単に切り替え
- ✅ 環境変数のプロファイル管理
- ✅ 設定ファイルの自動マージ
- ✅ 安全な本番環境運用

を実現できます。
