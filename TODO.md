# TODO

> **注**: v1.0.0リリース完了。このファイルは将来の機能追加・改善項目を記録しています。

## 優先度: 高

### マルチプラットフォームバイナリ対応

**状態**: 未実施
**現状**: ビルド済みバイナリはmacOSのみ対応

#### 対応が必要なプラットフォーム

- [ ] Linux (x86_64-unknown-linux-gnu)
- [ ] Linux (x86_64-unknown-linux-musl) - Alpine Linux等
- [ ] Windows (x86_64-pc-windows-msvc)
- [ ] Windows (aarch64-pc-windows-msvc) - ARM64

#### 実装方法

GitLab CI/CDパイプラインでクロスコンパイル:

```yaml
# .gitlab-ci.yml に追加
build:linux:
  stage: build
  image: rust:latest
  script:
    - rustup target add x86_64-unknown-linux-gnu
    - cargo build --release --target x86_64-unknown-linux-gnu
    - tar czf cmdrun-linux-x86_64.tar.gz -C target/x86_64-unknown-linux-gnu/release cmdrun

build:windows:
  stage: build
  image: rust:latest
  script:
    - apt-get update && apt-get install -y mingw-w64
    - rustup target add x86_64-pc-windows-gnu
    - cargo build --release --target x86_64-pc-windows-gnu
```

#### 参考リソース

- [cross](https://github.com/cross-rs/cross) - Rustクロスコンパイルツール
- [cargo-zigbuild](https://github.com/rust-cross/cargo-zigbuild) - Zig使用のクロスコンパイル
- [GitLab CI/CD Multi-platform builds](https://docs.gitlab.com/ee/ci/yaml/)

#### 関連タスク

- [ ] GitLab CI/CDパイプライン設定
- [ ] クロスコンパイル環境の構築・テスト
- [ ] 各プラットフォームでの動作確認
- [ ] README更新（全プラットフォームのインストール手順追加）
- [ ] GitLab Releasesへの自動アップロード

---

## 優先度: 中

### ドキュメント整備

- [ ] **アーキテクチャドキュメント**: コードベース構造、設計思想、主要コンポーネントの詳細
- [ ] **パフォーマンスガイド**: 大規模プロジェクトでの最適化手法
- [ ] **セキュリティガイド**: セキュリティベストプラクティス詳細

**備考**:
- ✅ CONTRIBUTING.md: 実装済み
- ✅ 基本的なドキュメント: README.md、docs/配下に整備済み

---

## 優先度: 低（将来の機能追加）

### 新機能アイデア

- [ ] **リモートコマンド実行**: SSH経由でのコマンド実行サポート
- [ ] **プラグインシステム**: 拡張可能なプラグインアーキテクチャ
- [ ] **インタラクティブモード**: fuzzy finderでのコマンド選択UI
- [ ] **パフォーマンスプロファイリング**: コマンド実行時間の計測・分析ツール
- [ ] **コマンド実行履歴**: 実行履歴の記録・検索機能

---

## ✅ 完了済み項目

- [x] **統合テスト実装**: `tests/integration/`に実装済み
- [x] **CI/CDでのテスト自動実行**: `.gitlab-ci.yml`で実装済み
- [x] **コントリビューションガイド**: `CONTRIBUTING.md`作成済み
- [x] **基本的なドキュメント**: README、ユーザーガイド、技術ドキュメント整備済み
- [x] **セキュリティ検証**: 精密なコマンド検証実装済み
- [x] **v1.0.0リリース**: 初回安定版リリース完了
