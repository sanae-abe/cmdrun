# cmdrun - TODO & Development Roadmap

> **最終更新**: 2025-11-08 17:00 JST
> **バージョン**: v1.0.0 → v1.2.0開発完了 → crates.io公開準備完了
> **プロジェクト概要**: Rust製高速・安全・クロスプラットフォームコマンドランナー
>
> **現在の状況**: Phase 1-2完全完了 → CI品質チェック完了 → **crates.io公開待ち**（最優先） ✅
>
> **包括的分析レポート**: [docs/private/COMPREHENSIVE_ANALYSIS_2025-11-08.md](docs/private/COMPREHENSIVE_ANALYSIS_2025-11-08.md)
> - 総合評価: A+ (9.2/10)
> - 主要な強み: 業界最高水準セキュリティ、豊富な機能、優れたアーキテクチャ
> - 改善課題: 市場認知度、起動時間ギャップ（6.5ms vs 競合3-5ms）
> - 市場機会: 2025年トレンド（AI統合、コンテキスト認識、セキュリティ重視）

---

## 📊 プロジェクト現状

### ✅ v1.0.0 実装済み機能

#### コア機能
- [x] コマンド管理（add/run/list/search/remove/info/edit）
- [x] 変数展開システム（`${VAR}`, `${1}`, `${VAR:-default}`, `${VAR:?error}`, `${VAR:+value}`）
- [x] 依存関係管理・循環依存検出・グラフ可視化
- [x] Watch Mode（ファイル監視、Globパターン、デバウンス、gitignore統合）
- [x] 並列実行サポート
- [x] 国際化（英語・日本語）
- [x] クロスプラットフォーム（Linux/macOS/Windows/FreeBSD）
- [x] セキュリティ対策（シェルインジェクション防止、機密情報保護）
- [x] パフォーマンス最適化（起動4ms、メモリ10MB以下）

---

## 🎯 開発ロードマップ

### Phase 1: パッケージ公開・安定化（現在） - v1.1.0

#### 🔴 高優先度

##### パッケージ公開（最優先）

**状態**: CI品質チェック完了、公開準備完了

- [x] GitHub Actionsワークフロー設定完了
- [x] マルチプラットフォームビルド対応
- [x] Homebrew Formula作成
- [x] PUBLISHING.mdドキュメント作成
- [x] ライセンス互換性確認完了
- [x] Windows CI テスト修正完了
- [x] **CI品質チェック完了** ✅ **2025-11-08完了**
  - [x] clippy警告0件達成（全ターゲット対応）
  - [x] rustfmt適用完了（セキュリティコード含む）
  - [x] 全339テストパス維持
- [ ] **crates.io公開実行**（次のステップ）
  - [ ] crates.ioアカウント作成とCARGO_TOKEN設定
  - [ ] `cargo publish --dry-run`検証
  - [ ] 初回タグプッシュとリリース実行
- [ ] **配布パッケージ整備**
  - [ ] Homebrew tap リポジトリ作成（オプション）
  - [ ] Scoop/Chocolateyマニフェスト作成（Windows配布）
  - [ ] GitHub Releases自動化確認

##### 中国語対応（i18n拡充）- ✅ **v1.0.0実装済み**

**実装完了**:
- [x] 中国語（簡体字）翻訳完了
- [x] 中国語（繁体字）翻訳完了
- [x] README.zh-CN.md, README.zh-TW.md 作成済み
- [x] 設定ファイルベースの言語切り替え実装
- [x] 4言語対応完了（英/日/簡体中/繁体中）

##### テストカバレッジ向上

- [x] カバレッジ測定（`cargo-tarpaulin`導入） ✅ **v0.31.2導入完了**
- [x] 現状カバレッジ測定 ✅ **41.56% (999/2404 lines)**
- [x] CI統合（Codecov連携、60%閾値） ✅ **完了**
- [x] カバレッジ大幅向上 ✅ **2025-11-07完了: 41.56% → 46.01%**
- [x] 統合テスト拡充 ✅ **完了**
  - [x] Windowsパイプテスト修正（`tests/security/injection.rs` - commit a23cf03で修正済み）
  - [x] 既存テスト確認（253件全パス: 171 unit + 82 integration）
  - [x] 50件の新規テスト追加（303件総数）
- [x] エラーケース・エッジケーステスト追加 ✅ **完了**
- [x] property-based testing拡充（`proptest`） ✅ **20件追加**
- [ ] 60%目標達成（残課題: main.rs、i18n.rs、対話的コマンド）

##### セキュリティ強化

- [x] 依存関係監査自動化（`cargo-audit` CI統合） ✅ **完了**
- [x] 週次スケジュール設定（毎週月曜9:00 UTC） ✅ **完了**
- [x] 脆弱性報告プロセス確立（SECURITY.md作成） ✅ **完了**
- [x] fuzzing導入完了（`cargo-fuzz`） ✅ **2025-11-07完了**
  - [x] 4つのfuzz target作成（interpolation、validation、toml_config、command_parts）
  - [x] CI週次実行設定（毎週日曜2:00 UTC、5分/target）
  - [x] 初期テスト実施（373,423実行、0クラッシュ✅）
  - [x] ドキュメント整備（fuzz/README.md、FUZZING_REPORT.md）

#### 🟡 中優先度

##### ドキュメント整備

**既存ドキュメント**:
- ✅ CONTRIBUTING.md実装済み
- ✅ 基本的なドキュメント（README、ユーザーガイド、技術ドキュメント）整備済み

**追加項目**:
- [x] アーキテクチャドキュメント（コードベース構造、設計思想） ✅ **ARCHITECTURE.md 40KB完成**
- [x] パフォーマンスガイド（大規模プロジェクトでの最適化手法） ✅ **PERFORMANCE_GUIDE.md 15KB完成**
- [x] セキュリティガイド（ベストプラクティス詳細） ✅ **SECURITY.md完成**
- [x] トラブルシューティングガイド ✅ **TROUBLESHOOTING.md 17KB完成**
- [x] よくある質問（FAQ） ✅ **FAQ.md 19KB完成**
- [x] レシピ集（実用例） ✅ **RECIPES.md 23KB完成**
- [x] ドキュメントインデックス更新 ✅ **docs/README.md更新**

##### i18n拡充

**既存実装（英語・日本語）**:
- [x] i18n基盤整備（149個の翻訳キー準備） ✅ **完了**
- [x] `cmdrun add`完全対応 ✅ **既存**
- [x] `cmdrun search`完全対応 ✅ **完了**
- [x] `cmdrun init`言語選択機能 ✅ **完了**
- [x] 6コマンドの多言語対応完了 ✅ **2025-11-07完了**
  - [x] `cmdrun remove`, `info`, `config`, `watch`, `validate`, `edit`
  - [x] 967行追加、150行削除
  - [x] テスト全パス（4件）
  - 対応率: 90% (9/10コマンド、completionは英語のまま適切)

**中国語対応（Phase 1高優先度へ移動）**:
- 詳細は上記「中国語対応（i18n拡充）- リリース前実装」セクション参照

**残タスク**:
- [ ] エラーメッセージの完全ローカライズ（全言語）

##### パフォーマンス検証

- [x] ベンチマーク実装 ✅ **10カテゴリ完成**
  - [x] `benches/command_execution.rs`（5カテゴリ） ✅
  - [x] `benches/toml_parsing.rs`（5カテゴリ） ✅
- [x] `Cargo.toml:156-162`のベンチマーク有効化 ✅ **完了**
- [x] CI統合（パフォーマンス回帰検出、150%閾値） ✅ **完了**
- [x] プロファイリング定期実行体制構築 ✅ **2025-11-07完了**
  - [x] `scripts/profile.sh` - プロファイリング自動化スクリプト
  - [x] `scripts/benchmark.sh` - ベンチマーク自動化スクリプト
  - [x] `docs/technical/PROFILING.md` - 完全ガイド（830行）
  - [x] `PERFORMANCE_BASELINE.md` - ベースライン測定値
- [x] 起動時間・メモリ使用量の継続監視 ✅ **ベースライン確立**
  - 起動時間: 6.5ms（平均）、4.6ms（最小）
  - メモリ: 4.5MB（目標15MB以下達成✅）
  - TOMLパース: 0.215ms（目標1ms以下達成✅）
  - 評価: 4指標中3指標で目標大幅達成

---

### Phase 2: 機能拡張 - v1.2.0 ✅ **完了**

#### ✅ 環境管理機能完了（2025-11-07）

- [x] 環境切り替え（`cmdrun env use dev/staging/prod`） ✅
  - [x] 6コマンド実装（use, current, list, set, create, info）
  - [x] 設定マージ機能（base + 環境固有）
  - [x] 環境変数プロファイル管理
  - [x] 統合テスト6件全パス
  - [x] ドキュメント完備（docs/ENVIRONMENT_MANAGEMENT.md）

##### 履歴・ログ機能完了（2025-11-07）

- [x] コマンド実行履歴（`cmdrun history`） ✅
  - [x] SQLiteストレージ（自動記録、最大1000件）
  - [x] 6コマンド実装（list, search, clear, export, stats, retry）
  - [x] 実行時間記録、終了コード、統計
  - [x] エクスポート（JSON/CSV）
  - [x] 機密情報フィルタリング
  - [x] 統合テスト7件全パス
  - [x] ドキュメント完備（docs/user-guide/HISTORY.md）

##### テンプレート機能完了（2025-11-07）

- [x] コマンドテンプレート（`cmdrun template add/use`） ✅
  - [x] 6コマンド実装（add, use, list, remove, export, import）
  - [x] 組み込み4種（rust-cli, nodejs-web, python-data, react-app）
  - [x] テンプレート検証、共有機能
  - [x] テスト45件全パス
  - [x] ドキュメント完備（TEMPLATE_FEATURE_REPORT.md）

##### プラグインシステム基盤完了（2025-11-07）

- [x] プラグインAPI設計 ✅
  - [x] Plugin trait定義（フック、カスタムコマンド）
  - [x] 動的ローダー（libloading）
  - [x] レジストリ、マネージャー実装
  - [x] サンプル2種（hello_plugin, logger_plugin）
  - [x] 4コマンド実装（list, info, enable, disable）
  - [x] ドキュメント850行（API仕様、開発ガイド）

##### グローバル設定機能完了（2025-11-08）

- [x] グローバル設定ファイルサポート ✅
  - [x] プラットフォーム別パス自動検出
    - Linux: `~/.config/cmdrun/commands.toml`
    - macOS: `~/Library/Application Support/cmdrun/commands.toml`
    - Windows: `%APPDATA%/cmdrun/commands.toml`
  - [x] 自動マージ機能（ローカル > グローバル）
  - [x] `merge_with()` メソッド実装
  - [x] 統合テスト追加
  - [x] 全339テストパス

#### 🟢 低優先度

##### Watch Mode拡張

- [ ] 複数コマンドの連鎖実行
- [ ] 変更ファイルを引数として渡す機能
- [ ] ホットリロード機能強化

##### 依存関係グラフ高度化

- [ ] Graphviz形式エクスポート
- [ ] Mermaid形式エクスポート
- [ ] 依存関係の最適化提案

---

### Phase 3: エコシステム構築 - v2.0.0

#### 🟢 低優先度（将来構想）

##### サーバーモード

- [ ] デーモンプロセス（長時間実行タスク管理）
- [ ] クライアント・サーバーアーキテクチャ
- [ ] リモートコマンド実行（SSH経由）
- [ ] マルチユーザー対応

##### AI駆動機能

- [ ] 自然言語によるコマンド生成
- [ ] スマート補完（コンテキスト考慮）
- [ ] コマンド提案（使用パターン分析）

##### Kubernetes統合

- [ ] kubectl連携
- [ ] マニフェスト自動生成
- [ ] CI/CD統合強化

##### プラグインエコシステム

- [ ] プラグインレジストリ
- [ ] プラグインマーケットプレイス
- [ ] コミュニティプラグイン支援

##### GUI版

- [ ] TUI版（`ratatui`使用）
- [ ] Web UI版（WASM対応）
- [ ] デスクトップアプリ版（Tauri）

---

---

## 📊 市場分析サマリー（2025-11-08） 🆕

### 競合ポジショニング

| ツール | 起動時間 | 特徴 | ターゲット | GitHub Stars |
|--------|----------|------|-----------|--------------|
| **cmdrun** | 6.5ms | 高機能＋セキュリティ | エンタープライズ | 0（新規） |
| just | 5ms | シンプル・高速 | ミニマリスト | 24.5k |
| task | 3ms | バランス型 | 一般開発者 | 13.2k |
| cargo-make | 15ms | Rust特化 | Rustプロジェクト | 2.5k |

### 市場トレンド（2025年）

1. **AI統合** - GitHub Copilot CLI、Qodo、Warp等のAI駆動ツール主流化
2. **コンテキスト認識** - プロジェクト固有のインテリジェンス、ワークフロー理解
3. **Rust製高速ツール** - fd、ripgrep、batのデファクト化
4. **セキュリティ重視** - Supply chain攻撃対策、ゼロトラスト原則

### cmdrunの差別化ポイント

**強み**:
- ✅ eval完全排除（業界最高水準セキュリティ）
- ✅ 環境管理+履歴+テンプレート（業界唯一の統合実装）
- ✅ プラグインシステム（拡張性）
- ✅ 多言語対応（日本市場独占可能）

**弱み**:
- ⚠️ 起動時間ギャップ（task比2倍遅、just比30%遅）
- ⚠️ 市場認知度ゼロ（後発の不利）
- ⚠️ バイナリサイズ5MB（justの2.5倍）

### 改善優先度（分析結果より）

**即座（今週）**:
1. crates.io公開 → 市場露出開始
2. スペルチェック実装 → UX向上（3-4日）
3. 技術記事3本 → SEO・認知度向上

**短期（2-3週間）**:
1. TUI実装 → 差別化強化（just/task未対応、1-2週間）
2. 中国語対応 → 市場拡大（14億人、4-5日）
3. パフォーマンス最適化 → 起動4.5ms目標

**中期（1-2ヶ月）**:
1. Feature flags導入 → 最小バイナリ2.5MB
2. AI統合実験 → トレンド先取り
3. 中国市場展開 → Gitee/知乎/CSDN

---

## 🐛 既知の問題

### 高優先度（Phase 1完了分）

- [x] **i18n拡充完了** ✅ **2025-11-07完了**
  - 実績: 9コマンド対応（対応率90%）
  - 詳細: `cmdrun add`, `search`, `init`, `remove`, `info`, `config`, `watch`, `validate`, `edit`
  - 967行追加、150行削除、テスト全パス

- [x] **テストカバレッジ大幅向上** ✅ **2025-11-07完了**
  - 実績: 41.56% → 46.01% (+4.45%, 149行カバレッジ追加)
  - テスト数: 253件 → 303件 (+50件)
  - 新規テストファイル: 10ファイル（統合テスト、proptest、ユニットテスト）
  - 主要改善: search 66.7%, config 37.7%, interpolation 89.7%, executor 65.0%

- [x] **パフォーマンス検証体制完全構築** ✅ **2025-11-07完了**
  - プロファイリング・ベンチマークスクリプト作成
  - PROFILING.md（830行）、PERFORMANCE_BASELINE.md作成
  - ベースライン確立: 起動6.5ms、メモリ4.5MB、TOMLパース0.215ms

- [x] **セキュリティ強化完全完了** ✅ **2025-11-07完了**
  - cargo-fuzz導入、4つのfuzz target実装
  - CI週次実行設定（毎週日曜2:00 UTC）
  - 初期テスト: 373,423実行、0脆弱性✅
  - ドキュメント: fuzz/README.md、FUZZING_REPORT.md

### 中優先度

なし（Phase 1完了）

### 低優先度

- [ ] **テストカバレッジ60%達成**
  - 現状: 46.01%
  - 残課題: main.rs 0%, i18n.rs 17%, 対話的コマンド（技術的制約）
  - 推定工数: 大（サブプロセステスト、対話的UI自動化必要）

---

## 📝 メンテナンスタスク

### 定期実行（毎週）

- [ ] 依存関係更新確認（`cargo outdated`）
- [ ] セキュリティ監査（`cargo audit`）
- [ ] テスト全実行（`cargo test --all-features`）
- [ ] Clippy警告確認（`cargo clippy --all-targets`）

### 定期実行（毎月）

- [ ] パフォーマンスベンチマーク
- [ ] ドキュメント更新確認
- [ ] GitHub Issues/PRレビュー
- [ ] コミュニティフィードバック確認

### リリース時

- [ ] CHANGELOG.md更新
- [ ] バージョン番号更新（`Cargo.toml`, `README.md`）
- [ ] ドキュメント更新
- [ ] タグ作成・GitHub Release
- [ ] crates.io公開
- [ ] 配布パッケージ更新（Homebrew/Scoop/Chocolatey）

---

## 🎨 改善アイデア（優先度未定）

### ユーザビリティ

**Phase 1高優先度へ移動**:
- ✅ スペルチェック・typo修正提案（上記「リリース前実装」セクション参照）

**中優先度**:
- [ ] コマンド実行時間の表示強化（`Executed in 2.5s`）
- [ ] プログレスバー（長時間実行コマンド用）
- [ ] 実行結果の統計情報（成功/失敗率、平均実行時間など）

### 開発者体験

- [ ] `cmdrun doctor`コマンド（設定診断・環境チェック）
- [ ] `cmdrun import`コマンド（他ツールからの移行支援）
  - npm scripts
  - Makefile
  - package.json scripts
- [ ] `cmdrun export`コマンド（他形式へのエクスポート）

### インテグレーション

- [ ] VSCode拡張機能
- [ ] Vim/Neovimプラグイン
- [ ] シェル統合（zsh/bash completion強化）
- [ ] GitHub Actions統合
- [ ] GitLab CI統合

---

## ✅ 完了済み項目

### v1.0.0（初回リリース）
- [x] **統合テスト実装**: `tests/integration/`に実装済み
- [x] **CI/CDでのテスト自動実行**: GitHub Actionsで実装済み
- [x] **コントリビューションガイド**: `CONTRIBUTING.md`作成済み
- [x] **基本的なドキュメント**: README、ユーザーガイド、技術ドキュメント整備済み
- [x] **セキュリティ検証**: 精密なコマンド検証実装済み
- [x] **v1.0.0リリース**: 初回安定版リリース完了
- [x] **GitHubリポジトリ移行**: GitLabからGitHubへ移行完了
- [x] **マルチプラットフォームビルド**: Linux/macOS/Windows対応完了
- [x] **依存ライブラリライセンス確認**: MIT互換性確認済み
- [x] **Watch Mode実装**: ファイル監視機能実装済み
- [x] **国際化（i18n）基盤**: 英語・日本語サポート（`cmdrun add`完全対応）
- [x] **依存関係管理**: グラフ可視化、循環依存検出
- [x] **変数展開システム**: 高度な変数展開機能実装
- [x] **セキュリティ強化**: シェルインジェクション対策、機密情報保護
- [x] **パフォーマンス最適化**: 起動4ms、メモリ10MB以下達成

### v1.1.0 Phase 1完了（2025-11-07）
- [x] **テストカバレッジ大幅向上**: 41.56% → 46.01% (+4.45%)
- [x] **i18n拡充完了**: 9コマンド対応（対応率90%）
- [x] **パフォーマンス検証体制完全構築**: ベースライン確立
- [x] **セキュリティ強化完全完了**: fuzzing導入（4 targets、373,423実行、0脆弱性）
- [x] **包括的ドキュメント整備**: 6ドキュメント新規作成

### v1.2.0 Phase 2完了（2025-11-07実装完了分）
- [x] **環境管理機能**: 6コマンド実装、統合テスト6件、ドキュメント完備
  - [x] `cmdrun env use/current/list/set/create/info`
  - [x] 設定マージ機能（base + 環境固有）
  - [x] 環境変数プロファイル管理
- [x] **履歴・ログ機能**: SQLiteストレージ、統合テスト7件
  - [x] `cmdrun history list/search/clear/export/stats`, `cmdrun retry`
  - [x] 機密情報フィルタリング、JSON/CSV エクスポート
- [x] **テンプレート機能**: 組み込み4種、テスト45件
  - [x] `cmdrun template add/use/list/remove/export/import`
  - [x] rust-cli, nodejs-web, python-data, react-app
- [x] **プラグインシステム基盤**: 動的ローダー、サンプル2種、ドキュメント850行
  - [x] Plugin trait、フック、カスタムコマンド対応
  - [x] `cmdrun plugin list/info/enable/disable`
  - [x] hello_plugin, logger_plugin サンプル
- [x] **README更新**: 日英両言語でPhase 2機能追加（+232行）
- [x] **合計追加**: 3,021+行、実装ファイル30+、テスト58件全パス

### crates.io公開準備完了（2025-11-08）
- [x] **CI品質チェック完全完了**
  - [x] clippy警告0件達成（cargo clippy --all-features --quiet）
  - [x] rustfmt適用完了（セキュリティコード含む全ファイル）
  - [x] 全339テストパス維持（374/374 doctests含む）
  - [x] GitHub Actions CI全グリーン
- [x] **グローバル設定機能完了**（Phase 2追加機能）
  - [x] プラットフォーム別パス自動検出（Linux/macOS/Windows）
  - [x] 自動マージ機能（ローカル > グローバル優先度）
  - [x] 統合テスト追加（全339テストパス）
- [x] **v1.0.0ドキュメント更新完了**（2025-11-08追加）
  - [x] README 4言語版更新（EN/JA/ZH-CN/ZH-TW）
  - [x] バージョンバッジ1.0.0統一
  - [x] Interactive Mode (TUI) ドキュメント追加
  - [x] Typo Detection ドキュメント追加
  - [x] 競合優位性セクション追加（vs just/task/cargo-make）
  - [x] i18n対応明記（4言語対応、9コマンド対応）
- [x] **コード品質改善**（2025-11-08追加）
  - [x] clippy警告5件修正→0件達成維持
  - [x] examples/typo_demo.rs ビルドエラー修正
  - [x] 全241テストパス確認

---

## 📚 参考資料

### 技術ドキュメント

- [パフォーマンス](docs/technical/PERFORMANCE.md)
- [セキュリティ](docs/technical/SECURITY.md)
- [クロスプラットフォームサポート](docs/technical/CROSS_PLATFORM.md)
- [配布](docs/technical/DISTRIBUTION.md)

### ユーザーガイド

- [CLIリファレンス](docs/user-guide/CLI.md)
- [設定リファレンス](docs/user-guide/CONFIGURATION.md)
- [Watch Mode](docs/user-guide/WATCH_MODE.md)
- [国際化（i18n）](docs/user-guide/I18N.md)

### 外部リソース

- [Rust Performance Book](https://nnethercote.github.io/perf-book/)
- [Command Line Interface Guidelines](https://clig.dev/)
- [OWASP Top 10](https://owasp.org/www-project-top-ten/)

---

## 🤝 コントリビューション

コントリビューションを歓迎します！詳細は[CONTRIBUTING.md](CONTRIBUTING.md)を参照してください。

**基本的な流れ**:

1. **Issue作成**
   - バグ報告: 再現手順・環境情報を明記
   - 機能提案: ユースケース・実装案を記載

2. **Pull Request**
   - `main`ブランチへのPR
   - テストの追加・更新
   - ドキュメント更新
   - コミットメッセージ規約準拠

3. **コミュニケーション**
   - 英語または日本語でOK
   - GitHubで議論してください

---

**開発者**: sanae.a.sunny@gmail.com
**リポジトリ**: https://github.com/sanae-abe/cmdrun
**ライセンス**: MIT
