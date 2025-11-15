# Git Hooks

このディレクトリにはプロジェクトのGit hooksが含まれています。

## セットアップ

クローン後、以下のコマンドでhooksを有効化してください：

```bash
git config core.hooksPath .githooks
```

## Hooks

### pre-commit
- **実行内容**: `cargo fmt` による自動フォーマット
- **所要時間**: 1-2秒
- **目的**: コミット前にコードを自動整形

### pre-push
- **実行内容**:
  1. `cargo clippy --all-features -- -D warnings` - 静的解析
  2. `cargo test --all-features --lib --bins` - 単体テスト（統合テスト除外）
- **所要時間**: 10-30秒
- **目的**: Push前の高速品質チェック（Fail Fast）

## 設計方針

- **Clippy優先**: 型エラーを数秒で検出
- **統合テスト除外**: proptest（237秒）を除外し高速化
- **CI完全チェック**: GitHub Actionsで全テスト実行

## Hooksを一時的に無効化

緊急時は以下のオプションでhooksをスキップできます：

```bash
# pre-commitをスキップ
git commit --no-verify

# pre-pushをスキップ
git push --no-verify
```

**注意**: CIで全チェックが実行されるため、ローカルでスキップしてもCIで失敗する可能性があります。
