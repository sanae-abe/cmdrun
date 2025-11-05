# cmdrun 配布クイックスタートガイド

## 🚀 最速配布手順（15分）

### 前提条件
- ✅ GitHubアカウント
- ✅ crates.ioアカウント（GitHubでログイン可能）
- ✅ プロジェクトが ~/projects/cmdrun にある

---

## Step 1: GitHubリポジトリ作成（5分）

```bash
# 1. GitHubでリポジトリ作成
# https://github.com/new
# Repository name: cmdrun
# Public
# 初期化オプションはすべてOFF（既存プロジェクトがあるため）

# 2. ローカルからプッシュ
cd ~/projects/cmdrun

# リモート追加（既に.gitが存在する場合）
git remote add origin https://github.com/sanae-abe/cmdrun.git
git branch -M main
git push -u origin main

# まだgit initしていない場合
git init
git add .
git commit -m "Initial commit"
git remote add origin https://github.com/sanae-abe/cmdrun.git
git branch -M main
git push -u origin main
```

---

## Step 2: crates.io公開（5分）

```bash
# 1. crates.ioログイン
# https://crates.io にアクセス
# → Log in with GitHub

# 2. APIトークン取得
# https://crates.io/settings/tokens
# → New Token
# → トークン名: "cmdrun-publishing"
# → トークンをコピー

# 3. ログイン
cargo login YOUR_TOKEN_HERE

# 4. 公開前確認
cargo package --list        # パッケージ内容確認
cargo publish --dry-run     # ドライラン

# 5. 本番公開（取り消し不可！）
cargo publish

# 6. 確認
# https://crates.io/crates/cmdrun
# 5-10分後: https://docs.rs/cmdrun
```

---

## Step 3: GitHub Actions設定（3分）

```bash
cd ~/projects/cmdrun

# 1. ワークフローファイル配置
mkdir -p .github/workflows
cp docs/distribution/ci.yml.template .github/workflows/ci.yml
cp docs/distribution/release.yml.template .github/workflows/release.yml

# 2. Homebrewトークン作成（後で使う）
# GitHub Settings → Developer settings → Personal access tokens → Tokens (classic)
# → Generate new token (classic)
# Scopes: repo（full control）
# トークン名: HOMEBREW_TAP_TOKEN
# コピーしておく

# 3. GitHubシークレット登録
# https://github.com/sanae-abe/cmdrun/settings/secrets/actions
# → New repository secret
# Name: HOMEBREW_TAP_TOKEN
# Value: <先ほどのトークン>

# 4. プッシュ
git add .github/workflows/
git commit -m "ci: Add GitHub Actions workflows"
git push
```

---

## Step 4: 初回リリース（2分）

```bash
cd ~/projects/cmdrun

# 1. バージョン確認
grep '^version' Cargo.toml
# version = "1.0.0"

# 2. タグ作成・プッシュ
git tag -a v1.0.0 -m "Release v1.0.0"
git push origin v1.0.0

# 3. GitHub Actionsで自動ビルド開始
# https://github.com/sanae-abe/cmdrun/actions
# 5-10分後にビルド完了・リリース作成

# 4. リリース確認
# https://github.com/sanae-abe/cmdrun/releases
```

---

## Step 5: Homebrew Tap作成（オプション、10分）

```bash
# 1. GitHubでリポジトリ作成
# https://github.com/new
# Repository name: homebrew-cmdrun （この名前が必須）
# Public

# 2. ローカルでセットアップ
mkdir -p ~/Projects/homebrew-cmdrun
cd ~/Projects/homebrew-cmdrun
git init
mkdir Formula

# 3. Formulaファイル作成
cp ~/projects/cmdrun/docs/distribution/cmdrun.rb.template Formula/cmdrun.rb

# 4. SHA256取得（GitHub Releasesのビルド完了後）
VERSION="1.0.0"
ARM64_URL="https://github.com/sanae-abe/cmdrun/releases/download/v${VERSION}/cmdrun-v${VERSION}-aarch64-apple-darwin.tar.gz"
X86_64_URL="https://github.com/sanae-abe/cmdrun/releases/download/v${VERSION}/cmdrun-v${VERSION}-x86_64-apple-darwin.tar.gz"

ARM64_SHA=$(curl -sL "$ARM64_URL" | shasum -a 256 | cut -d' ' -f1)
X86_64_SHA=$(curl -sL "$X86_64_URL" | shasum -a 256 | cut -d' ' -f1)

echo "ARM64 SHA256: $ARM64_SHA"
echo "x86_64 SHA256: $X86_64_SHA"

# 5. FormulaのSHA256を置換
sed -i '' "s/REPLACE_WITH_ARM64_SHA256_AFTER_RELEASE/${ARM64_SHA}/" Formula/cmdrun.rb
sed -i '' "s/REPLACE_WITH_X86_64_SHA256_AFTER_RELEASE/${X86_64_SHA}/" Formula/cmdrun.rb

# 6. プッシュ
git add Formula/cmdrun.rb
git commit -m "Initial cmdrun formula v${VERSION}"
git remote add origin https://github.com/sanae-abe/homebrew-cmdrun.git
git push -u origin main

# 7. インストールテスト
brew tap sanae-abe/cmdrun
brew install cmdrun
cmdrun --version
```

---

## ✅ 完了！

以下の方法でインストール可能になりました：

### Rustユーザー向け
```bash
cargo install cmdrun
```

### macOSユーザー向け
```bash
brew tap sanae-abe/cmdrun
brew install cmdrun
```

### 全プラットフォーム向け
```bash
# GitHub Releasesから直接ダウンロード
# https://github.com/sanae-abe/cmdrun/releases/latest
```

---

## 次のステップ

### すぐにやるべきこと
- [ ] README.md を英語・日本語で充実
- [ ] ドキュメント追加（チュートリアル、ユースケース）
- [ ] GitHub Issuesテンプレート作成

### 徐々にやること
- [ ] コントリビューションガイド作成
- [ ] GitHub Discussions有効化
- [ ] ブログ・SNSで宣伝
- [ ] ユーザーフィードバック収集

### v2.0.0以降
- [ ] 公式Homebrew（homebrew/core）への登録申請
- [ ] Windowsパッケージマネージャー対応（Scoop/Chocolatey）
- [ ] Linuxディストリビューション対応（AUR/Snapcraft）

---

## 更新版リリース手順

```bash
# 1. バージョンアップ
# Cargo.toml: version = "1.0.1"
# CHANGELOG.md: 変更内容記載

# 2. コミット・タグ
git add Cargo.toml CHANGELOG.md
git commit -m "Bump version to 1.0.1"
git tag v1.0.1
git push && git push origin v1.0.1

# 3. crates.io公開
cargo publish

# 4. GitHub Releasesは自動生成
# GitHub Actions が自動実行

# 5. Homebrewも自動更新
# release.ymlのupdate-homebrewジョブが自動実行
```

---

## トラブルシューティング

### Q: cargo publish でエラーが出る

```bash
# エラー内容確認
cargo publish --dry-run 2>&1 | less

# よくあるエラー
# - missing field 'license' → Cargo.tomlにlicense追加
# - missing README.md → readme = "README.md" 追加
# - invalid token → cargo login やり直し
```

### Q: GitHub Actions が失敗する

```bash
# ログ確認
# https://github.com/sanae-abe/cmdrun/actions
# → Failed job → ログ詳細確認

# ローカルでビルドテスト
cargo build --release
cargo test
cargo clippy
```

### Q: Homebrew Formula の SHA256 が合わない

```bash
# SHA256再取得
curl -sL "URL" | shasum -a 256

# Formula更新
cd ~/Projects/homebrew-cmdrun
vim Formula/cmdrun.rb
# sha256行を更新
git add Formula/cmdrun.rb
git commit -m "Fix SHA256"
git push
```

---

## 参考資料

- 📘 [配布方法詳細ガイド](DISTRIBUTION_GUIDE.md)
- 📋 [リリースチェックリスト](RELEASE_CHECKLIST.md)
- 🔧 [CI設定テンプレート](ci.yml.template)
- 🚀 [リリース設定テンプレート](release.yml.template)
- 🍺 [Homebrew Formulaテンプレート](cmdrun.rb.template)
