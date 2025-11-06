# TODO

## リリース・配布

### マルチプラットフォームバイナリ対応

**優先度**: 中
**期限**: 次回メジャーリリース前

現在、ビルド済みバイナリはmacOSのみ対応。他のプラットフォームへの対応が必要。

#### 対応が必要なプラットフォーム

- [ ] Linux (x86_64-unknown-linux-gnu)
- [ ] Linux (x86_64-unknown-linux-musl) - Alpine Linux等
- [ ] Windows (x86_64-pc-windows-msvc)
- [ ] Windows (x86_64-pc-windows-gnu)

#### 実装方法

**推奨**: GitLab CI/CDパイプラインでの自動ビルド

```yaml
# .gitlab-ci.yml に追加
build:linux:
  stage: build
  image: rust:latest
  script:
    - cargo build --release --target x86_64-unknown-linux-gnu

build:windows:
  stage: build
  image: rust:latest
  script:
    - rustup target add x86_64-pc-windows-gnu
    - cargo build --release --target x86_64-pc-windows-gnu
```

#### 参考リソース

- [rust-cross](https://github.com/cross-rs/cross) - クロスコンパイルツール
- [cargo-zigbuild](https://github.com/rust-cross/cargo-zigbuild) - Zig使用のクロスコンパイル
- GitLab CI/CD Multi-platform builds

#### 関連タスク

- [ ] CI/CD パイプライン設定
- [ ] クロスコンパイル環境の構築
- [ ] 各プラットフォームでのテスト
- [ ] README更新（全プラットフォームのインストール手順追加）

---

## その他のTODO

### ドキュメント

- [ ] コントリビューションガイドの作成
- [ ] アーキテクチャドキュメントの整備

### 機能追加

- [ ] Remote command execution
- [ ] Plugin system for extensibility
- [ ] Interactive mode for command selection
- [ ] Performance profiling and optimization tools

### テスト

- [ ] Integration tests implementation
- [ ] CI/CD でのテスト自動実行
