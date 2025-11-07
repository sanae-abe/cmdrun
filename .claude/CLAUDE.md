# cmdrun - プロジェクト固有設定

> **Rust製の高速・安全・クロスプラットフォームコマンドランナー**
>
> パフォーマンス（起動4ms）、セキュリティ（eval()ゼロ）、開発者体験を重視したCLIツール

## 🎯 プロジェクト概要

- **言語**: Rust 1.75+ (MSRV)
- **プロジェクトタイプ**: CLIツール、システムユーティリティ
- **主要技術**: tokio (async runtime), clap (CLI), TOML設定、ファイル監視
- **セキュリティ重視**: シェルインジェクション対策、機密情報保護、入力検証
- **パフォーマンス目標**: 起動時間4ms、メモリ10MB以下

## 🤖 推奨Subagents（プロジェクト特化）

このプロジェクトでは以下のsubagentsを**積極的に活用**してください：

### 🔴 最優先Agents（常時活用）

#### 1. **rust-engineer** - Rust専門家
```yaml
活用シーン:
  - Cargo.toml最適化（LTO、codegen設定）
  - async/await パターンの最適化
  - unsafe コードレビュー
  - 生涯パラメータ・所有権の設計
  - パフォーマンスクリティカルなコード最適化

優先タスク:
  - src/command/executor.rs: tokio非同期処理の最適化
  - src/watch/: ファイル監視の効率化
  - Cargo.toml: profile設定のRust 1.75+最適化
```

#### 2. **security-auditor** - セキュリティ監査
```yaml
活用シーン:
  - src/security/: シェルインジェクション対策レビュー
  - src/command/interpolation.rs: 変数展開の安全性監査
  - OWASP Top 10準拠確認
  - 依存関係の脆弱性スキャン

優先タスク:
  - 変数展開（${VAR}）のインジェクション脆弱性テスト
  - shell-wordsクレートの使用方法検証
  - secrecyクレートによる機密情報保護の強化
```

#### 3. **cli-developer** - CLI UX専門家
```yaml
活用シーン:
  - clap設定の最適化
  - dialoguer対話プロンプトのUX改善
  - エラーメッセージの分かりやすさ向上
  - シェル補完機能の強化

優先タスク:
  - src/cli.rs: コマンド引数設計の改善
  - src/commands/: サブコマンドのUX統一
  - エラーメッセージの多言語対応（英語・日本語）
```

### 🟡 高優先Agents（定期活用）

#### 4. **performance-engineer** - パフォーマンス最適化
```yaml
活用シーン:
  - 起動時間4msの検証・維持
  - メモリフットプリント10MB目標達成
  - criterion ベンチマーク設計
  - プロファイリング・ボトルネック特定

優先タスク:
  - 起動時間のベンチマーク自動化
  - ahash/smallvec最適化の効果測定
  - TOML パース速度の改善
```

#### 5. **test-automator** - テスト自動化
```yaml
活用シーン:
  - proptest を使ったproperty-based testing
  - 統合テストのカバレッジ拡大
  - セキュリティテストの強化
  - CI/CDパイプラインの最適化

優先タスク:
  - tests/security/injection.rs: テストケース追加
  - tests/integration/: カバレッジ向上
  - proptestによるfuzzing強化
```

#### 6. **workflow-orchestrator** - ワークフロー最適化
```yaml
活用シーン:
  - src/command/dependency.rs: 依存関係グラフ最適化
  - 並列実行アルゴリズムの改善
  - 複雑なタスク依存関係の設計

優先タスク:
  - 依存関係解決アルゴリズムの高度化
  - 並列実行の効率化
  - デッドロック検出機能の実装
```

### 🟢 状況依存Agents（特定タスク時）

#### 7. **compliance-auditor** - ライセンス・コンプライアンス監査
```yaml
活用シーン: 依存関係追加時、リリース前
タスク: MITライセンス準拠確認、依存クレートのライセンス監査
```

#### 8. **microservices-architect** - 分散システム設計
```yaml
活用シーン: 将来的なサーバーモード実装時
タスク: クライアント・サーバーアーキテクチャ設計
```

#### 9. **embedded-systems** - 組み込み最適化
```yaml
活用シーン: no_std対応、極限の軽量化
タスク: メモリ使用量の最小化、バイナリサイズ削減
```

#### 10. **sre-engineer** - 本番運用
```yaml
活用シーン: リリース後の監視・信頼性向上
タスク: SLI/SLO設定、モニタリング戦略、障害対応
```

#### 11. **multi-agent-coordinator** - 複数エージェント協調
```yaml
活用シーン: 大規模リファクタリング、総合レビュー
タスク: rust-engineer + security-auditor + performance-engineer 並列実行
```

#### 12. **llm-architect** - AI機能実装
```yaml
活用シーン: 将来機能（AI駆動コマンド提案）
タスク: 自然言語によるコマンド生成、スマート補完
```

## 📋 Agent活用戦略

### 🎯 開発フェーズ別の推奨Agent

```yaml
新機能実装:
  1. rust-engineer: 設計レビュー
  2. cli-developer: UX評価
  3. test-automator: テスト設計
  4. security-auditor: セキュリティレビュー

パフォーマンス改善:
  1. performance-engineer: ボトルネック特定
  2. rust-engineer: 最適化実装
  3. test-automator: ベンチマーク作成

バグ修正:
  1. debugger: 根本原因特定
  2. rust-engineer: 修正実装
  3. test-automator: 回帰テスト追加

リリース準備:
  1. compliance-auditor: ライセンス監査
  2. security-auditor: 脆弱性スキャン
  3. test-automator: 統合テスト
  4. documentation-engineer: ドキュメント更新
```

### 🚀 Agent活用の具体例

```bash
# 新機能実装時の典型的なワークフロー
Task(rust-engineer, "Watch Mode機能の設計レビュー - src/watch/配下")
Task(cli-developer, "Watch Modeコマンドのクラップ設計とUX評価")
Task(test-automator, "Watch Mode統合テスト設計 - tests/integration/watch.rs")
Task(security-auditor, "ファイルパス操作のセキュリティレビュー")

# パフォーマンス最適化時
Task(performance-engineer, "起動時間の現状分析とボトルネック特定")
Task(rust-engineer, "TOML パース最適化の実装")

# 総合レビュー時（複数Agent協調）
Task(multi-agent-coordinator, "rust-engineer、security-auditor、performance-engineerを使ってcmdrun全体を包括的にレビュー")
```

## 🔧 プロジェクト固有の開発ガイドライン

### Rustコーディング規約

```rust
// 推奨パターン
- async/awaitを積極活用（tokio runtime）
- エラーハンドリング: anyhow::Result<T> for applications, thiserror for libraries
- 型安全: newtype pattern、強い型付け
- パフォーマンス: ahash, smallvec活用
- セキュリティ: shell-words、secrecy使用

// 避けるパターン
- unwrap() の多用（適切なエラー処理を）
- unsafeの無根拠な使用
- 不必要な clone()
- 動的文字列評価（eval相当）
```

### セキュリティ基準

```yaml
必須チェック項目:
  - シェルインジェクション対策: shell-words使用
  - パストラバーサル対策: 入力パス検証
  - 環境変数展開: 安全な変数展開実装
  - 機密情報: secrecyクレート使用
  - 依存関係: cargo-audit定期実行
```

### パフォーマンス目標

```yaml
起動時間: 4ms以下
メモリ使用量: 10MB以下（アイドル時）
バイナリサイズ: 5MB以下（strip後）
TOML パース: 1ms以下（標準的な設定ファイル）
```

## 📁 重要ディレクトリ構造

```
src/
├── cli.rs              # CLI定義（clap） → cli-developer
├── command/            # コマンド実行 → rust-engineer
│   ├── executor.rs     # 非同期実行 → performance-engineer
│   ├── dependency.rs   # 依存関係解決 → workflow-orchestrator
│   └── interpolation.rs # 変数展開 → security-auditor
├── security/           # セキュリティ → security-auditor
│   ├── validation.rs   # 入力検証
│   └── secrets.rs      # 機密情報保護
├── watch/              # ファイル監視 → rust-engineer
│   ├── watcher.rs      # notify使用
│   └── debouncer.rs    # イベント制御
└── config/             # 設定管理 → rust-engineer
    └── schema.rs       # TOML スキーマ

tests/
├── integration/        # 統合テスト → test-automator
└── security/           # セキュリティテスト → security-auditor
```

## 🎯 今後の開発方向性

### Phase 1: 安定化（現在）
- セキュリティ強化 → **security-auditor**
- パフォーマンス最適化 → **performance-engineer**
- テストカバレッジ向上 → **test-automator**

### Phase 2: 機能拡張
- サーバーモード実装 → **microservices-architect**
- プラグインシステム → **rust-engineer**
- AI駆動コマンド提案 → **llm-architect**

### Phase 3: エコシステム
- Kubernetes統合 → **kubernetes-specialist**
- インフラ自動化 → **terraform-engineer**
- SRE実践 → **sre-engineer**

## 📖 関連ドキュメント

- [README.md](../README.md): プロジェクト概要
- [docs/user-guide/](../docs/user-guide/): ユーザーガイド
- [docs/technical/](../docs/technical/): 技術文書
- [Cargo.toml](../Cargo.toml): 依存関係・ビルド設定

---

**💡 開発時のヒント**:
- 新しいタスクを始める際は、まず関連するsubagentに相談してください
- 複雑なタスクは `multi-agent-coordinator` で複数agentを協調させると効率的です
- セキュリティ・パフォーマンスは常に最優先事項です
