# cmdrun セキュリティ設計書

## セキュリティ原則

1. **eval完全排除**: 動的コード実行を一切使用しない
2. **入力検証**: 全ユーザー入力の厳格な検証
3. **最小権限**: 必要最小限の権限で実行
4. **監査可能**: すべての実行を記録・追跡可能
5. **Fail Secure**: エラー時は安全側に倒す

## 1. eval排除戦略

### Node.js版の問題
```javascript
// ❌ 危険: 任意コード実行可能
eval(`const result = ${userInput}`);

// ❌ 危険: シェルインジェクション
exec(`sh -c "${userInput}"`);
```

### Rust版の安全な実装
```rust
// ✅ 安全: 変数展開のみ、コード実行なし
let interpolated = interpolate_variables(input, &env)?;

// ✅ 安全: シェルコマンドは明示的に構築
let mut cmd = Command::new(shell);
cmd.arg("-c").arg(command);
```

**実装詳細**:
- 正規表現ベースの変数展開（`interpolation.rs`）
- ホワイトリスト方式の変数構文
- シェルエスケープは不要（引数として渡す）

## 2. 変数展開のセキュリティ

### 許可する構文（ホワイトリスト）
```
${VAR}              # 基本展開
${VAR:-default}     # デフォルト値
${VAR:?error}       # 必須変数
${VAR:+value}       # 条件置換
```

### 禁止する構文（ブラックリスト）
```
${VAR/pattern/replacement}  # パターン置換
${VAR%pattern}              # サフィックス削除
${VAR#pattern}              # プレフィックス削除
$(command)                  # コマンド置換
`command`                   # コマンド置換（バッククォート）
```

### 実装による保護
```rust
// 正規表現で厳密に制御
static VAR_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\$\{([A-Za-z_][A-Za-z0-9_]*)(:[?+\-])?([^}]*)?\}").unwrap()
});

// マッチしない構文はそのまま（展開しない）
// → コマンド置換等は無効化される
```

## 3. シェルインジェクション対策

### 脆弱な例（Node.js）
```javascript
// ❌ シェルインジェクション可能
exec(`echo ${userInput}`);
// userInput = "; rm -rf /" → 危険
```

### 安全な実装（Rust）
```rust
// ✅ 引数として渡すため安全
let mut cmd = Command::new("bash");
cmd.arg("-c");
cmd.arg(format!("echo {}", user_input));
// user_input に ; があっても解釈されない
```

**追加保護策**:
```rust
// シェル特殊文字のエスケープ（shell-words クレート）
use shell_words::quote;

let safe_arg = quote(user_input);
```

## 4. 設定ファイルの検証

### TOML パース時の検証
```rust
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]  // 未知フィールドでエラー
pub struct CommandsConfig {
    #[serde(default)]
    pub commands: HashMap<String, Command>,
}

// カスタム検証
impl CommandsConfig {
    pub fn validate(&self) -> Result<()> {
        // 循環依存チェック
        self.check_circular_deps()?;

        // コマンド文字列の妥当性チェック
        for (name, cmd) in &self.commands {
            cmd.validate(name)?;
        }

        Ok(())
    }
}
```

### 循環依存検出
```rust
fn check_circular_deps(&self) -> Result<()> {
    let mut visited = HashSet::new();
    let mut rec_stack = HashSet::new();

    for cmd_name in self.commands.keys() {
        if self.has_cycle(cmd_name, &mut visited, &mut rec_stack)? {
            return Err(ConfigError::CircularDependency(
                cmd_name.clone()
            ).into());
        }
    }
    Ok(())
}
```

## 5. 機密情報の保護

### 環境変数の安全な取り扱い
```rust
use secrecy::{Secret, ExposeSecret};

// 機密情報をSecretでラップ
let api_key = Secret::new(env::var("API_KEY")?);

// 使用時のみ公開
let client = ApiClient::new(api_key.expose_secret());

// ログには出力されない
println!("Key: {:?}", api_key);  // "Secret([REDACTED])"
```

### パスワード等のマスク
```rust
// 環境変数名のパターンマッチ
fn is_sensitive(key: &str) -> bool {
    let sensitive_patterns = [
        "PASSWORD", "SECRET", "TOKEN", "KEY", "CREDENTIAL"
    ];

    sensitive_patterns.iter().any(|p| key.contains(p))
}

// ログ出力時にマスク
fn log_env(key: &str, value: &str) {
    if is_sensitive(key) {
        println!("{}=***", key);
    } else {
        println!("{}={}", key, value);
    }
}
```

## 6. ファイルシステムアクセス制御

### 作業ディレクトリの検証
```rust
use std::path::{Path, PathBuf};

fn validate_working_dir(path: &Path) -> Result<PathBuf> {
    // 絶対パス化
    let abs_path = path.canonicalize()
        .map_err(|_| ConfigError::InvalidWorkingDir)?;

    // シンボリックリンク攻撃対策
    if abs_path.is_symlink() {
        return Err(SecurityError::SymlinkNotAllowed.into());
    }

    // ディレクトリトラバーサル対策
    let current_dir = env::current_dir()?;
    if !abs_path.starts_with(&current_dir) {
        return Err(SecurityError::DirectoryTraversal.into());
    }

    Ok(abs_path)
}
```

### ファイル読み込みの制限
```rust
// 設定ファイルサイズ上限（DoS対策）
const MAX_CONFIG_SIZE: u64 = 10 * 1024 * 1024;  // 10MB

fn read_config_file(path: &Path) -> Result<String> {
    let metadata = fs::metadata(path)?;

    if metadata.len() > MAX_CONFIG_SIZE {
        return Err(SecurityError::ConfigTooLarge.into());
    }

    fs::read_to_string(path)
}
```

## 7. プロセス実行の安全性

### タイムアウト（DoS対策）
```rust
use tokio::time::timeout;

// 必ずタイムアウトを設定
let timeout_duration = Duration::from_secs(config.timeout.unwrap_or(300));

match timeout(timeout_duration, child.wait()).await {
    Ok(status) => status,
    Err(_) => {
        child.kill().await?;
        return Err(ExecutionError::Timeout);
    }
}
```

### リソース制限
```rust
// Unix系: rlimit でリソース制限
#[cfg(unix)]
fn set_resource_limits(cmd: &mut Command) {
    use std::os::unix::process::CommandExt;

    unsafe {
        cmd.pre_exec(|| {
            // CPU時間制限
            libc::setrlimit(libc::RLIMIT_CPU, &libc::rlimit {
                rlim_cur: 300,  // 300秒
                rlim_max: 300,
            });

            // メモリ制限
            libc::setrlimit(libc::RLIMIT_AS, &libc::rlimit {
                rlim_cur: 1024 * 1024 * 1024,  // 1GB
                rlim_max: 1024 * 1024 * 1024,
            });

            Ok(())
        });
    }
}
```

## 8. 監査ログ

### 実行ログの記録
```rust
use tracing::{info, warn, error};

// コマンド実行前
info!(
    command = %cmd_name,
    user = %current_user,
    working_dir = %working_dir,
    "Executing command"
);

// 実行結果
info!(
    command = %cmd_name,
    exit_code = %result.exit_code,
    duration_ms = %result.duration.as_millis(),
    "Command completed"
);

// エラー時
error!(
    command = %cmd_name,
    error = %err,
    "Command failed"
);
```

### 構造化ログ（JSON出力）
```rust
use tracing_subscriber::fmt::format::FmtSpan;

tracing_subscriber::fmt()
    .json()
    .with_span_events(FmtSpan::FULL)
    .with_current_span(false)
    .init();
```

**出力例**:
```json
{
  "timestamp": "2025-11-05T10:30:00Z",
  "level": "INFO",
  "message": "Executing command",
  "command": "build",
  "user": "alice",
  "working_dir": "/home/alice/project"
}
```

## 9. 依存関係のセキュリティ

### cargo-audit による脆弱性検出
```bash
# CI/CDに組み込み
cargo audit

# 自動修正
cargo audit fix
```

### cargo-deny による依存関係管理
```toml
# deny.toml
[advisories]
vulnerability = "deny"
unmaintained = "warn"
unsound = "warn"

[licenses]
unlicensed = "deny"
allow = ["MIT", "Apache-2.0", "BSD-3-Clause"]
```

```bash
# ライセンス・脆弱性チェック
cargo deny check
```

## 10. セキュリティチェックリスト

### 設計時
- [ ] eval/動的コード実行を使用しない
- [ ] 入力検証を実装
- [ ] 最小権限の原則を適用
- [ ] タイムアウトを設定
- [ ] エラー処理を適切に実装

### 実装時
- [ ] 変数展開はホワイトリスト方式
- [ ] シェルインジェクション対策
- [ ] ディレクトリトラバーサル対策
- [ ] 機密情報のマスク
- [ ] リソース制限の実装

### テスト時
- [ ] 悪意ある入力でのテスト
- [ ] 循環依存の検出テスト
- [ ] タイムアウトのテスト
- [ ] 権限エスカレーションのテスト

### 運用時
- [ ] 監査ログの有効化
- [ ] 定期的な依存関係の更新
- [ ] cargo-audit の実行
- [ ] セキュリティアドバイザリの確認

## 11. 脅威モデル

### 想定される脅威

1. **悪意ある設定ファイル**
   - 対策: TOML パース時の厳格な検証
   - 対策: 循環依存の検出

2. **シェルインジェクション**
   - 対策: 引数として渡す（シェル解釈を回避）
   - 対策: shell-words でエスケープ

3. **ディレクトリトラバーサル**
   - 対策: 絶対パス化と検証
   - 対策: 親ディレクトリへのアクセス制限

4. **DoS攻撃（リソース枯渇）**
   - 対策: タイムアウトの強制
   - 対策: リソース制限（rlimit）
   - 対策: 設定ファイルサイズ上限

5. **機密情報の漏洩**
   - 対策: secrecy クレートで保護
   - 対策: ログ出力時のマスク

## 12. セキュアコーディングガイドライン

### Rustのメモリ安全性を活用
```rust
// ✅ コンパイラが保証
// - Use-after-free なし
// - バッファオーバーフローなし
// - データ競合なし
```

### unsafeコードの最小化
```rust
// unsafeは必要最小限に
#[cfg(unix)]
unsafe {
    // リソース制限設定のみ
    libc::setrlimit(...);
}

// 安全性不変条件を文書化
/// # Safety
/// This function must only be called after fork()
unsafe fn set_limits() { ... }
```

### エラーハンドリングの徹底
```rust
// Result型を活用
fn execute_command(cmd: &str) -> Result<ExecutionResult> {
    // エラーは必ず処理
}

// パニックは回避
let value = map.get(key).ok_or(ConfigError::KeyNotFound)?;
```
