# リリースチェックリスト

## リリース前の準備

### 1. コード品質確認
- [ ] `cargo fmt` - コード整形
- [ ] `cargo clippy -- -D warnings` - Lintエラー0件
- [ ] `cargo test` - 全テストパス
- [ ] `cargo doc --no-deps` - ドキュメント生成成功
- [ ] `cargo audit` - セキュリティ監査パス

### 2. バージョン更新
- [ ] `Cargo.toml` のバージョン更新
- [ ] `CHANGELOG.md` に変更内容記載
- [ ] `README.md` のバージョン表記更新（必要な場合）

### 3. ローカルビルドテスト
```bash
# リリースビルド
cargo build --release

# バージョン確認
./target/release/cmdrun --version

# 基本動作確認
./target/release/cmdrun init
./target/release/cmdrun list
```

### 4. ドキュメント確認
- [ ] README.md が最新
- [ ] CHANGELOG.md に新バージョン追加
- [ ] ドキュメントのリンク切れなし

## リリース手順

### 1. GitHubリポジトリ作成（初回のみ）
```bash
# GitHubで新規リポジトリ作成: sanae-abe/cmdrun

cd ~/projects/cmdrun
git remote add origin https://github.com/sanae-abe/cmdrun.git
git push -u origin main
```

### 2. GitHub Actions設定（初回のみ）
```bash
# ワークフローファイル配置
mkdir -p .github/workflows
cp /tmp/github-actions-ci.yml .github/workflows/ci.yml
cp /tmp/github-actions-release.yml .github/workflows/release.yml

git add .github/workflows/
git commit -m "ci: Add GitHub Actions workflows"
git push
```

### 3. リリースタグ作成
```bash
# バージョン確認
VERSION="1.0.0"

# タグ作成
git tag -a "v${VERSION}" -m "Release v${VERSION}"

# タグをプッシュ（これでリリースビルドが自動開始）
git push origin "v${VERSION}"
```

### 4. GitHub Releasesでリリース作成
- GitHub Actions が自動的にビルド・リリース作成
- リリースノートを編集（必要に応じて）
- バイナリとSHA256が自動アップロード

### 5. Homebrew Tap作成（初回のみ）
```bash
# GitHubで新規リポジトリ作成: sanae-abe/homebrew-cmdrun

mkdir -p ~/Projects/homebrew-cmdrun/Formula
cd ~/Projects/homebrew-cmdrun

# Formula配置
cp /tmp/homebrew-cmdrun.rb Formula/cmdrun.rb

# SHA256を実際の値に置換（リリース後に取得）
ARM64_SHA=$(curl -sL "https://github.com/sanae-abe/cmdrun/releases/download/v${VERSION}/cmdrun-v${VERSION}-aarch64-apple-darwin.tar.gz" | shasum -a 256 | cut -d' ' -f1)
X86_64_SHA=$(curl -sL "https://github.com/sanae-abe/cmdrun/releases/download/v${VERSION}/cmdrun-v${VERSION}-x86_64-apple-darwin.tar.gz" | shasum -a 256 | cut -d' ' -f1)

# SHA256を手動更新（またはGitHub Actionsで自動更新）
sed -i '' "s/REPLACE_WITH_ARM64_SHA256_AFTER_RELEASE/${ARM64_SHA}/" Formula/cmdrun.rb
sed -i '' "s/REPLACE_WITH_X86_64_SHA256_AFTER_RELEASE/${X86_64_SHA}/" Formula/cmdrun.rb

# GitHubにプッシュ
git init
git add Formula/cmdrun.rb
git commit -m "Initial cmdrun formula v${VERSION}"
git remote add origin https://github.com/sanae-abe/homebrew-cmdrun.git
git push -u origin main
```

### 6. Homebrewインストールテスト
```bash
# Tap追加
brew tap sanae-abe/cmdrun

# インストール
brew install cmdrun

# 動作確認
cmdrun --version
cmdrun init
cmdrun list

# クリーンアップ（テスト後）
brew uninstall cmdrun
brew untap sanae-abe/cmdrun
```

## リリース後の作業

### 1. リリースノート確認
- [ ] GitHubリリースページで内容確認
- [ ] ダウンロードリンクが正常動作

### 2. ドキュメント更新
- [ ] README.md のインストール手順更新
- [ ] ブログ・SNSで告知（必要に応じて）

### 3. 次期バージョン準備
- [ ] `Cargo.toml` のバージョンをdev版に更新（例: 1.1.0-dev）
- [ ] `CHANGELOG.md` に Unreleased セクション追加

## トラブルシューティング

### GitHub Actions が失敗する場合
```bash
# ローカルでビルドテスト
cargo build --release --target aarch64-apple-darwin
cargo build --release --target x86_64-apple-darwin

# クロスコンパイルテスト（Linux）
# cross コマンドが必要
cargo install cross
cross build --release --target aarch64-unknown-linux-gnu
```

### Homebrew Formula の SHA256 不一致
```bash
# SHA256を再取得
curl -sL "URL" | shasum -a 256

# Formula更新
brew edit cmdrun
```

### リリースロールバック
```bash
# タグ削除
git tag -d "v${VERSION}"
git push origin :"v${VERSION}"

# GitHubリリース削除（Web UIから）
```
