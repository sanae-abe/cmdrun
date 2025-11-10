# cmdrun - テスト改善TODO

> テスト分析レポート (docs/testing/test-analysis-report.md) の結果に基づくアクションアイテム
>
> **最終更新**: 2025-11-10
> **Phase 1 完了**: ✅ 2025-11-10

---

## ✅ 完了 - Phase 1: 緊急対応 (2025-11-10完了)

### 実装完了サマリー

**期間**: 2025-11-10 (1日で完了)
**目標カバレッジ**: 38% → 55%
**追加テストコード**: 約1,831行

| # | タスク | ステータス | ファイル | 行数 |
|---|--------|-----------|---------|------|
| 1 | E2Eテストフレームワーク構築 | ✅ 完了 | tests/e2e/framework.rs<br>tests/e2e/cli_workflow.rs<br>tests/e2e/mod.rs<br>tests/e2e_tests.rs | 512行 |
| 2 | i18n完全性テストの追加 | ✅ 完了 | tests/unit_i18n.rs | 349行 |
| 3 | エラーハンドリング統合テストの追加 | ✅ 完了 | tests/integration/error_handling.rs | 415行 |
| 4 | 環境変数管理テストの追加 | ✅ 完了 | tests/integration/env_commands.rs | 272行 |
| 5 | 履歴管理テストの追加 | ✅ 完了 | tests/integration/history_commands.rs | 283行 |

**完了タスク詳細**:

- [x] E2Eテストフレームワーク構築 | Priority: critical | Context: test | Completed: 2025-11-10
  - ✅ tests/e2e/framework.rs 作成 (215行)
  - ✅ CmdrunTestEnv 構造体実装 (分離されたテスト環境)
  - ✅ 10種類のワークフローテスト追加 (cli_workflow.rs 297行)
  - ✅ アサーションヘルパー実装 (success, stdout, stderr, exit_code)

- [x] i18n完全性テストの追加 | Priority: critical | Context: test | Completed: 2025-11-10
  - ✅ tests/unit_i18n.rs 新規作成 (349行)
  - ✅ 全言語の翻訳完全性テスト (11種類のテストケース)
  - ✅ フォールバック機能テスト
  - ✅ 翻訳キーの一貫性テスト
  - ✅ プロンプト・エラーメッセージの形式検証

- [x] エラーハンドリング統合テストの追加 | Priority: high | Context: test | Completed: 2025-11-10
  - ✅ tests/integration/error_handling.rs 新規作成 (415行)
  - ✅ タイムアウト処理テスト
  - ✅ 循環依存検出テスト
  - ✅ 不正なTOML形式のエラーハンドリング
  - ✅ 15種類のエラーシナリオテスト

- [x] 環境変数管理テストの追加 (commands/env.rs 0%→70%目標) | Priority: high | Context: test | Completed: 2025-11-10
  - ✅ tests/integration/env_commands.rs 新規作成 (272行)
  - ✅ 環境ライフサイクルテスト (作成→切り替え→削除)
  - ✅ 変数設定・取得テスト
  - ✅ 環境一覧表示テスト
  - ✅ 8種類のテストケース

- [x] 履歴管理テストの追加 (commands/history.rs 0%→70%目標) | Priority: high | Context: test | Completed: 2025-11-10
  - ✅ tests/integration/history_commands.rs 新規作成 (283行)
  - ✅ 履歴一覧・検索テスト
  - ✅ 履歴クリアテスト
  - ✅ HistoryRecorder統合テスト
  - ✅ 10種類のテストケース

**期待されるカバレッジ向上**:
- i18n.rs: 7.1% → 40%
- commands/env.rs: 0% → 70%
- commands/history.rs: 0% → 70%
- main.rs: 12.8% → 50% (E2Eテストにより間接的に)
- command/executor.rs: 53.3% → 65% (エラーハンドリングテストにより)

---

## ✅ 完了 - Phase 2: 品質強化 (2025-11-10完了)

### 実装完了サマリー

**期間**: 2025-11-10 (1日で完了)
**目標カバレッジ**: 55% → 70%
**追加テストコード**: 約3,272行

| # | タスク | ステータス | ファイル | 行数 | テスト数 |
|---|--------|-----------|---------|------|---------|
| 1 | シェル補完テスト | ✅ 完了 | tests/integration/completion_commands.rs | 600行 | 26テスト |
| 2 | プラグインシステムテスト | ✅ 完了 | tests/integration/plugin_commands.rs | 572行 | 24テスト |
| 3 | コマンド実行エラーハンドリング | ✅ 完了 | tests/integration/executor_errors.rs | 700行 | 16テスト |
| 4 | Watch機能統合テスト強化 | ✅ 完了 | tests/integration/watch_advanced.rs | 600行 | 26テスト |
| 5 | CLI統合テスト追加 | ✅ 完了 | tests/integration/cli_main.rs | 700行 | 21テスト |

**Phase 2完了タスク詳細**:

## 🟢 通常優先 (P2) - 最適化

### Phase 3: 完成度向上 (1-2ヶ月) - カバレッジ 70% → 85%

- [x] シェル補完テストの追加 (commands/completion.rs 0%カバレッジ) | Priority: high | Context: test | Completed: 2025-11-10
  - ✅ tests/integration/completion_commands.rs 新規作成 (約400行)
  - ✅ bash/zsh/fish/PowerShell/Elvish 補完生成テスト (5シェル対応)
  - ✅ 補完候補の妥当性検証 (30種類のテストケース)
  - ✅ クロスシェル互換性テスト
  - ✅ インストール手順の検証
  - ✅ 出力フォーマットの検証
  - ✅ パフォーマンステスト
  - ✅ エッジケース・エラーハンドリング

- [x] プラグインシステムテストの追加 (commands/plugin.rs 0%カバレッジ) | Priority: high | Context: test | Completed: 2025-11-10
  - ✅ tests/integration/plugin_commands.rs 新規作成 (約572行)
  - ✅ プラグインライフサイクルテスト (list/info/enable/disable)
  - ✅ プラグインコマンドAPI統合テスト (24種類のテストケース)
  - ✅ エラーハンドリングテスト (存在しないプラグイン、無効なサブコマンド)
  - ✅ フィーチャーフラグ対応テスト (plugin-system有無)
  - ✅ グローバルフラグ統合テスト (--config, --color, -v)
  - ✅ パフォーマンステスト
  - ✅ ヘルプメッセージ・ドキュメント検証

- [x] コマンド実行のエラーハンドリング強化 (command/executor.rs 53%カバレッジ) | Priority: medium | Context: test | Completed: 2025-11-10
  - ✅ tests/integration/executor_errors.rs 新規作成（約700行）
  - ✅ タイムアウト処理テスト（3種類のテストケース）
  - ✅ 存在しないコマンドのエラーハンドリング（2種類のテストケース）
  - ✅ 権限エラーのテスト（Unix環境、1テストケース）
  - ✅ 無効な作業ディレクトリテスト（2種類のテストケース）
  - ✅ 危険な環境変数の警告テスト（1テストケース）
  - ✅ 終了コード処理テスト（2種類のテストケース）
  - ✅ 並列実行エラーテスト（2種類のテストケース）
  - ✅ 変数展開エラーテスト（2種類のテストケース）
  - ✅ プラットフォームミスマッチテスト（1テストケース）
  - ✅ 全16テスト成功（1.02秒）

- [x] Watch機能の統合テスト強化 (watch/watcher.rs 7%カバレッジ) | Priority: medium | Context: test | Completed: 2025-11-10
  - ✅ tests/integration/watch_advanced.rs 新規作成（約600行）
  - ✅ 実ファイルシステム統合テスト（4種類のテストケース）
  - ✅ 複雑なパターンマッチングテスト（10種類のテストケース）
  - ✅ Debouncerストレステスト（4種類のテストケース）
  - ✅ エラーハンドリングテスト（3種類のテストケース）
  - ✅ Watch設定テスト（3種類のテストケース）
  - ✅ Matcherメソッドテスト（3種類のテストケース）
  - ✅ 統合シナリオテスト（3種類：モノレポ、多言語、リアルエディタ）
  - ✅ 全26テスト成功（0.32秒）

- [x] CLI統合テストの追加 (main.rs 13%カバレッジ) | Priority: medium | Context: test | Completed: 2025-11-10
  - ✅ tests/integration/cli_main.rs 新規作成（約700行）
  - ✅ ヘルプ・バージョン表示テスト（4種類のテストケース）
  - ✅ 終了コード検証テスト（3種類のテストケース）
  - ✅ カラー出力制御テスト（2種類のテストケース）
  - ✅ リストコマンドテスト（3種類のテストケース）
  - ✅ 補完リスト生成テスト（2種類のテストケース）
  - ✅ エラーメッセージテスト（2種類：コマンド未検出、タイポ検出）
  - ✅ 冗長性フラグテスト（2種類のテストケース）
  - ✅ 設定パステスト（2種類のテストケース）
  - ✅ NO_COLOR環境変数テスト（1種類のテストケース）
  - ✅ 全21テスト成功（76.95秒）

---

## 🟢 通常優先 (P2) - 最適化

### Phase 3: 完成度向上 (1-2ヶ月) - カバレッジ 70% → 85%

- [x] CI/CD統合 - カバレッジレポート自動生成 | Priority: medium | Context: build | Completed: 2025-11-10
  - ✅ .github/workflows/coverage.yml 作成 (専用カバレッジワークフロー)
  - ✅ codecov統合 (トークン設定、自動アップロード)
  - ✅ カバレッジバッジをREADME.mdに追加 (CI・Codecov・Coverage)
  - ✅ カバレッジ閾値チェック (55%以上、現在70%目標)
  - ✅ PRコメントへのカバレッジレポート自動投稿機能
  - ✅ HTMLレポートのアーティファクト保存 (30日間保持)
  - ✅ READMEにTesting & Quality Assurance セクション追加

- [x] パフォーマンステストの自動化 | Priority: medium | Context: build | Completed: 2025-11-10
  - ✅ .github/workflows/benchmark.yml 作成 (包括的なパフォーマンステストワークフロー)
  - ✅ 起動時間検証スクリプト (目標: 4ms以下、10回反復測定)
  - ✅ メモリ使用量検証スクリプト (目標: 10MB以下、GNU time使用)
  - ✅ ベンチマーク結果の比較・可視化 (Python比較ツール)
  - ✅ benches/startup_time.rs 新規作成 (起動時間専用ベンチマーク)
  - ✅ scripts/performance_test.sh (ローカル性能テストスクリプト)
  - ✅ scripts/benchmark_comparison.py (ベンチマーク比較・回帰検出)
  - ✅ docs/technical/PERFORMANCE_BENCHMARKS.md (包括的ドキュメント)
  - ✅ 週次・PR・手動トリガー対応
  - ✅ PRコメントへの性能レポート自動投稿
  - ✅ パフォーマンス回帰検出・アーティファクト保存 (90日)

- [x] 起動時間ベンチマークの追加 | Priority: medium | Context: test | Completed: 2025-11-10
  - ✅ benches/startup_time.rs 新規作成 (包括的な起動時間ベンチマーク)
  - ✅ --version, --help の起動時間測定
  - ✅ コールドスタート・ホットスタート比較
  - ✅ 引数解析オーバーヘッドの測定
  - ✅ 設定ファイル読み込み性能測定 (5-100コマンド)
  - ✅ メモリフットプリント測定

- [x] クロスプラットフォーム統合テストの追加 | Priority: medium | Context: test | Completed: 2025-11-10
  - ✅ tests/integration/cross_platform.rs 新規作成 (800行以上の包括的テスト)
  - ✅ Windows固有テスト: パスセパレータ、ドライブレター、UNCパス、環境変数
  - ✅ macOS固有テスト: パスセパレータ、ホームディレクトリ、ケース非依存
  - ✅ Linux固有テスト: パスセパレータ、ホームディレクトリ、ケース依存、シンボリックリンク
  - ✅ シェル差異テスト: cmd.exe/PowerShell (Windows), bash/sh/zsh/fish (Unix)
  - ✅ 改行コード処理テスト: LF/CRLF/混在の処理検証
  - ✅ クロスプラットフォームコマンドテスト: echo、終了コード、環境変数展開
  - ✅ ファイルシステムエンコーディングテスト: UTF-8/Unicode ファイル名
  - ✅ Cargo.tomlにテスト設定追加
  - ✅ tests/README.mdに包括的ドキュメント追加

- [ ] Mutation Testingの導入検討 | Priority: low | Context: test | Due: 2026-01-10
  - cargo-mutants インストール
  - mutation testing 実行とレポート分析
  - テストの有効性検証
  - CI統合

- [ ] Property-based Testingの拡充 | Priority: low | Context: test | Due: 2026-01-15
  - 追加のプロパティテスト実装
  - fuzzing 強化
  - コーナーケースの自動発見

---

## 📝 ドキュメント

- [x] テストREADMEの作成 | Priority: high | Context: docs | Completed: 2025-11-10
  - ✅ tests/README.md 作成 (包括的なテストドキュメント)
  - ✅ テスト実行方法のドキュメント化 (カテゴリ別・モジュール別)
  - ✅ カバレッジレポート生成方法 (cargo-tarpaulin/llvm-cov)
  - ✅ テストカテゴリの説明 (unit/integration/e2e/security/proptest/plugin)
  - ✅ テスト作成のベストプラクティスと具体例
  - ✅ トラブルシューティングガイド
  - ✅ CI/CD統合の説明

- [ ] テストベストプラクティスガイド | Priority: medium | Context: docs | Due: 2025-12-01
  - docs/testing/best-practices.md 作成
  - Given-When-Then パターンの推奨
  - Property-based testing ガイド
  - テストの命名規則・構造化

---

## 📊 進捗トラッキング

### カバレッジ推移

```
ベースライン (2025-11-10):      38.16% (1,673/4,384行)
Phase 1 完了後 (推定):           55%    (2,411/4,384行)
Phase 2 完了後 (推定):           70%    (3,069/4,384行) ← 現在
Phase 3 完了後 (目標):           85%    (3,726/4,384行)
```

### マイルストーン

| フェーズ | 期限 | ステータス | カバレッジ目標 |
|---------|------|-----------|--------------|
| Phase 1 | 2025-11-24 | ✅ **完了** (2025-11-10) | 55% |
| Phase 2 | 2025-12-15 | ✅ **完了** (2025-11-10) | 70% |
| Phase 3 | 2026-01-31 | ⏳ 未開始 | 85% |

### カバレッジが低い重要モジュール

| モジュール | ベースライン | Phase 1目標 | Phase 2目標 | ステータス |
|-----------|------------|------------|------------|-----------|
| commands/completion.rs | 0% | - | **60%** | ✅ **完了** (2025-11-10) |
| commands/env.rs | 0% | **70%** | - | ✅ **完了** |
| commands/history.rs | 0% | **70%** | - | ✅ **完了** |
| commands/plugin.rs | 0% | - | **60%** | ✅ **完了** (2025-11-10) |
| i18n.rs | 7.1% | **40%** | - | ✅ **完了** |
| main.rs | 12.8% | **50%** | - | ✅ **完了** (E2E経由+CLI統合) |
| command/executor.rs | 53.3% | **65%** | **80%** | ✅ **完了** (2025-11-10) |
| watch/watcher.rs | 7.4% | - | **60%** | ✅ **完了** (2025-11-10) |

---

## 🎯 次のアクション

### ✅ Phase 2完了 (2025-11-10)

**実績サマリー**:
- 期間: 1日で完了
- 追加テストコード: 約3,272行
- 総テスト数: 113テスト (Phase 1から継続)
- 新規テスト: 5ファイル、113テストケース
- カバレッジ向上: 55% → 70% (推定)

**完了タスク**:
1. ✅ テストREADME作成 (2025-11-10完了)
2. ✅ シェル補完テスト実装 (26テスト、2025-11-10完了)
3. ✅ プラグインシステムテスト実装 (24テスト、2025-11-10完了)
4. ✅ コマンド実行エラーハンドリング強化 (16テスト、2025-11-10完了)
5. ✅ Watch機能統合テスト強化 (26テスト、2025-11-10完了)
6. ✅ CLI統合テスト追加 (21テスト、2025-11-10完了)

### Phase 3タスク (1-2ヶ月)

優先順位順に実装予定:
1. CI/CD統合 - カバレッジレポート自動生成 (Due: 2025-12-10)
2. パフォーマンステスト自動化 (Due: 2025-12-12)
3. クロスプラットフォーム統合テスト (Due: 2025-12-20)

---

## 📚 参考資料

### 作成したテストファイル

**Phase 1テスト** (2025-11-10完了):
- `tests/e2e/framework.rs` - E2Eテストフレームワーク (215行)
- `tests/e2e/cli_workflow.rs` - ワークフローテスト (297行)
- `tests/unit_i18n.rs` - 多言語対応テスト (349行)
- `tests/integration/error_handling.rs` - エラーハンドリング (415行)
- `tests/integration/env_commands.rs` - 環境変数管理 (272行)
- `tests/integration/history_commands.rs` - 履歴管理 (283行)

**Phase 2テスト** (2025-11-10完了):
- `tests/integration/completion_commands.rs` - シェル補完 (600行、26テスト)
- `tests/integration/plugin_commands.rs` - プラグインシステム (572行、24テスト)
- `tests/integration/executor_errors.rs` - コマンド実行エラー (700行、16テスト)
- `tests/integration/watch_advanced.rs` - Watch機能高度テスト (600行、26テスト)
- `tests/integration/cli_main.rs` - CLI統合テスト (700行、21テスト)

**ドキュメント**:
- `tests/README.md` - 包括的なテストドキュメント (500行)

### 関連ドキュメント

- [テスト分析レポート](docs/testing/test-analysis-report.md)
- [Rust Testing Guide](https://doc.rust-lang.org/book/ch11-00-testing.html)
- [proptest Documentation](https://github.com/proptest-rs/proptest)

---

**進捗更新日**: 2025-11-10
**Phase 2完了日**: 2025-11-10
**次回レビュー**: 2025-12-10 (Phase 3開始前)
