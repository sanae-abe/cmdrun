# cmdrun 配布方法ガイド

## 目次
1. [配布方法の比較](#配布方法の比較)
2. [crates.io での配布](#cratesio-での配布)
3. [Homebrew での配布](#homebrew-での配布)
4. [GitHub Releases での配布](#github-releases-での配布)
5. [推奨配布戦略](#推奨配布戦略)

---

## 配布方法の比較

| 項目 | crates.io | Homebrew | GitHub Releases | バイナリ直接配布 |
|------|-----------|----------|-----------------|------------------|
| **対象ユーザー** | Rust開発者 | macOS/Linuxユーザー | 全プラットフォーム | 技術者 |
| **インストール方法** | `cargo install cmdrun` | `brew install cmdrun` | 手動ダウンロード | 手動配置 |
| **ビルド時間** | ユーザー環境で5-10分 | 事前ビルド、数秒 | 事前ビルド、数秒 | 即座 |
| **依存関係** | Rust toolchain必須 | なし | なし | なし |
| **自動更新** | `cargo install -f` | `brew upgrade` | 手動 | 手動 |
| **配布の手間** | 低（cargo publish一発） | 中（Formula保守） | 中（リリース作成） | 高（全環境ビルド） |
| **信頼性** | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐ |
| **到達範囲** | Rustコミュニティ | macOS/Linux広範 | 最広範 | 限定的 |

### 推奨度

1. **🥇 crates.io** - 最優先（Rustユーザー向け標準）
2. **🥈 Homebrew** - macOSユーザー向け最適
3. **🥉 GitHub Releases** - 全プラットフォーム対応
4. バイナリ直接配布 - 補助的

---

## crates.io での配布

### メリット
- Rust公式パッケージレジストリ
- `cargo install cmdrun` だけで簡単インストール
- バージョン管理・依存関係解決が自動
- ドキュメント自動生成（docs.rs）
- Rustコミュニティへの露出

### デメリット
- ユーザー環境でビルドが必要（時間がかかる）
- Rust toolchainのインストールが必須
- 非Rustユーザーには敷居が高い

### 配布手順

#### 1. 事前準備（初回のみ）

```bash
# crates.ioアカウント作成
# https://crates.io でGitHubアカウントでログイン

# APIトークン取得
# https://crates.io/me → API Tokens → New Token

# トークン登録
cargo login YOUR_API_TOKEN_HERE
```

#### 2. Cargo.toml の確認

```toml
[package]
name = "cmdrun"
version = "1.0.0"
edition = "2021"
rust-version = "1.75"  # MSRV（Minimum Supported Rust Version）
authors = ["Sanae Abe <sanae.abe@example.com>"]
license = "MIT OR Apache-2.0"  # デュアルライセンス推奨
description = "A fast, secure, and cross-platform command runner with TOML configuration"
repository = "https://github.com/sanae-abe/cmdrun"
homepage = "https://github.com/sanae-abe/cmdrun"
documentation = "https://docs.rs/cmdrun"  # 自動生成
readme = "README.md"
keywords = ["cli", "command", "runner", "toml", "task"]  # 最大5個
categories = ["command-line-utilities", "development-tools"]
exclude = [
    "tests/fixtures/*",
    ".github/*",
    "scripts/*",
    "*.toml.backup.*",
]
```

#### 3. ドライランテスト

```bash
cd ~/projects/cmdrun

# パッケージ内容確認
cargo package --list

# ドライラン（実際には公開しない）
cargo publish --dry-run

# 警告・エラーがないか確認
# - README.mdが存在するか
# - LICENSEファイルが存在するか
# - 不要なファイルが含まれていないか
```

#### 4. 公開

```bash
# 本番公開（取り消し不可！）
cargo publish

# 公開後確認
# https://crates.io/crates/cmdrun
# https://docs.rs/cmdrun （5-10分後にドキュメント生成）
```

#### 5. 更新版の公開

```bash
# 1. Cargo.tomlのバージョン更新
# version = "1.0.1"

# 2. CHANGELOG.md更新

# 3. Git commitしてタグ作成
git add Cargo.toml CHANGELOG.md
git commit -m "Bump version to 1.0.1"
git tag v1.0.1
git push && git push --tags

# 4. crates.ioに公開
cargo publish
```

### crates.io 公開チェックリスト

- [ ] `Cargo.toml` の必須フィールド入力済み
  - [ ] name, version, authors
  - [ ] license（MIT OR Apache-2.0推奨）
  - [ ] description（短く明確に）
  - [ ] repository, homepage
  - [ ] keywords（最大5個）
  - [ ] categories（適切なカテゴリ選択）
- [ ] `README.md` 存在（crates.ioページに表示）
- [ ] `LICENSE` or `LICENSE-MIT`/`LICENSE-APACHE` 存在
- [ ] `cargo package --list` で不要ファイル除外確認
- [ ] `cargo publish --dry-run` 成功
- [ ] ビルドテスト `cargo build --release` 成功
- [ ] テスト `cargo test` 全パス
- [ ] ドキュメント `cargo doc` 生成成功

### crates.io 公開後の注意

- **削除・アンパブリッシュ不可**: 一度公開したバージョンは削除できない（yank可能だが非推奨）
- **バージョン重複不可**: 同じバージョン番号で再公開できない
- **72時間以内のyank**: 公開後72時間以内なら `cargo yank --vers 1.0.0` で非推奨化可能
- **ドキュメント再生成**: docs.rsでビルド失敗したら再ビルドリクエスト可能

---

## Homebrew での配布

### メリット
- macOS/Linuxユーザーに最適
- 事前ビルド済みバイナリで高速インストール
- 依存関係なし（Rust不要）
- 自動更新対応（`brew upgrade`）
- 信頼性が高い

### デメリット
- macOS/Linux限定（Windowsは非対応）
- Formula保守が必要
- 複数プラットフォームビルドが必要

### 配布手順

#### 1. 個人Tap作成（推奨）

```bash
# GitHubで新規リポジトリ作成
# リポジトリ名: homebrew-cmdrun（必須命名規則）
# 公開設定: Public

# ローカルでセットアップ
mkdir -p ~/Projects/homebrew-cmdrun
cd ~/Projects/homebrew-cmdrun
git init
mkdir Formula

# Formula作成（/tmp/homebrew-cmdrun.rb参照）
cp /tmp/homebrew-cmdrun.rb Formula/cmdrun.rb

# 初回コミット
git add Formula/cmdrun.rb
git commit -m "Initial cmdrun formula"
git remote add origin https://github.com/sanae-abe/homebrew-cmdrun.git
git push -u origin main
```

#### 2. ユーザーのインストール方法

```bash
# Tap追加
brew tap sanae-abe/cmdrun

# インストール
brew install cmdrun

# アップデート
brew upgrade cmdrun

# アンインストール
brew uninstall cmdrun
brew untap sanae-abe/cmdrun
```

#### 3. Formula更新（新バージョン公開時）

```bash
cd ~/Projects/homebrew-cmdrun

# バージョン更新（手動 or GitHub Actionsで自動）
# Formula/cmdrun.rb を編集
# - version行を更新
# - URLを新バージョンに更新
# - sha256を新しい値に更新

# SHA256取得方法
VERSION="1.0.1"
ARM64_SHA=$(curl -sL "https://github.com/sanae-abe/cmdrun/releases/download/v${VERSION}/cmdrun-v${VERSION}-aarch64-apple-darwin.tar.gz" | shasum -a 256 | cut -d' ' -f1)
X86_64_SHA=$(curl -sL "https://github.com/sanae-abe/cmdrun/releases/download/v${VERSION}/cmdrun-v${VERSION}-x86_64-apple-darwin.tar.gz" | shasum -a 256 | cut -d' ' -f1)

echo "ARM64 SHA256: $ARM64_SHA"
echo "x86_64 SHA256: $X86_64_SHA"

# Formulaにコミット
git add Formula/cmdrun.rb
git commit -m "Update cmdrun to ${VERSION}"
git push
```

#### 4. 公式Homebrew/coreへの登録（オプション）

より広く配布したい場合は公式リポジトリに申請可能（審査厳格）

```bash
# 1. Formula作成
brew create https://github.com/sanae-abe/cmdrun/archive/refs/tags/v1.0.0.tar.gz

# 2. Formulaテスト
brew install --build-from-source cmdrun
brew test cmdrun
brew audit --strict cmdrun

# 3. homebrew/core にPR送信
# https://github.com/Homebrew/homebrew-core
```

---

## GitHub Releases での配布

### メリット
- 全プラットフォーム対応（macOS/Linux/Windows）
- バイナリ直接配布で最も柔軟
- GitHubが配信インフラを提供
- バージョン管理が容易

### デメリット
- 手動ダウンロード・インストールが必要
- 自動更新機構なし
- 複数プラットフォームビルドが必要

### 配布手順（GitHub Actions自動化）

#### 1. GitHub Actions設定

```bash
cd ~/projects/cmdrun

# ワークフローファイル配置
mkdir -p .github/workflows
cp /tmp/github-actions-ci.yml .github/workflows/ci.yml
cp /tmp/github-actions-release.yml .github/workflows/release.yml

git add .github/workflows/
git commit -m "ci: Add GitHub Actions workflows"
git push
```

#### 2. リリース作成

```bash
# タグ作成・プッシュで自動ビルド開始
VERSION="1.0.0"
git tag -a "v${VERSION}" -m "Release v${VERSION}"
git push origin "v${VERSION}"

# GitHub Actionsが自動実行:
# 1. 複数プラットフォームでビルド
# 2. tarball/zip作成
# 3. SHA256生成
# 4. GitHub Releasesに自動公開
```

#### 3. ユーザーのインストール方法

```bash
# macOS (Apple Silicon)
curl -L https://github.com/sanae-abe/cmdrun/releases/download/v1.0.0/cmdrun-v1.0.0-aarch64-apple-darwin.tar.gz | tar xz
sudo mv cmdrun /usr/local/bin/

# macOS (Intel)
curl -L https://github.com/sanae-abe/cmdrun/releases/download/v1.0.0/cmdrun-v1.0.0-x86_64-apple-darwin.tar.gz | tar xz
sudo mv cmdrun /usr/local/bin/

# Linux
curl -L https://github.com/sanae-abe/cmdrun/releases/download/v1.0.0/cmdrun-v1.0.0-x86_64-unknown-linux-gnu.tar.gz | tar xz
sudo mv cmdrun /usr/local/bin/

# Windows (PowerShell)
Invoke-WebRequest -Uri "https://github.com/sanae-abe/cmdrun/releases/download/v1.0.0/cmdrun-v1.0.0-x86_64-pc-windows-msvc.zip" -OutFile "cmdrun.zip"
Expand-Archive -Path "cmdrun.zip" -DestinationPath "."
Move-Item cmdrun.exe C:\Windows\System32\
```

---

## 推奨配布戦略

### 段階的ロールアウト

#### Phase 1: 初期リリース（v1.0.0）

1. **crates.io 公開** - Rustコミュニティへの露出
   ```bash
   cargo publish
   ```

2. **GitHub Releases** - 全プラットフォーム対応
   ```bash
   git tag v1.0.0
   git push origin v1.0.0
   # GitHub Actions自動実行
   ```

3. **個人Homebrew Tap** - macOSユーザー向け
   ```bash
   # homebrew-cmdrun リポジトリ作成・Formula追加
   ```

#### Phase 2: 成長期（v1.1.0〜）

4. **ドキュメント充実**
   - README.md 多言語対応（日本語・英語）
   - チュートリアル・ユースケース追加
   - docs.rs ドキュメント拡充

5. **コミュニティ構築**
   - GitHub Discussions 有効化
   - Issue/PR テンプレート整備
   - CONTRIBUTING.md 作成

#### Phase 3: 成熟期（v2.0.0〜）

6. **公式Homebrew登録申請**
   - homebrew/core にPR
   - 厳格な審査対応

7. **その他パッケージマネージャー対応**
   - Scoop（Windows）
   - Chocolatey（Windows）
   - Snapcraft（Linux）
   - AUR（Arch Linux）

### 優先順位付け

| 配布方法 | 優先度 | 実装時期 | 対象ユーザー |
|---------|-------|---------|-------------|
| crates.io | ⭐⭐⭐⭐⭐ | v1.0.0 | Rust開発者 |
| GitHub Releases | ⭐⭐⭐⭐⭐ | v1.0.0 | 全ユーザー |
| 個人Homebrew Tap | ⭐⭐⭐⭐ | v1.0.0 | macOSユーザー |
| 公式Homebrew | ⭐⭐⭐ | v2.0.0+ | 広範なmacOSユーザー |
| Scoop/Chocolatey | ⭐⭐ | v1.5.0+ | Windowsユーザー |

---

## まとめ：推奨アクションプラン

### すぐにやること（v1.0.0リリース時）

```bash
# 1. GitHubリポジトリ作成・プッシュ
cd ~/projects/cmdrun
git remote add origin https://github.com/sanae-abe/cmdrun.git
git push -u origin main

# 2. GitHub Actions設定
cp /tmp/github-actions-*.yml .github/workflows/
git add .github/workflows/
git commit -m "ci: Add CI/CD workflows"
git push

# 3. crates.io公開
cargo login YOUR_TOKEN
cargo publish --dry-run  # テスト
cargo publish            # 本番

# 4. GitHub Release作成
git tag v1.0.0
git push origin v1.0.0
# → GitHub Actionsが自動ビルド・リリース

# 5. Homebrew Tap作成
# homebrew-cmdrun リポジトリ作成
# Formula追加（GitHub Actions自動更新設定）
```

### 後でやること

- [ ] README.md 英語版作成
- [ ] ドキュメントサイト構築（GitHub Pages）
- [ ] コミュニティガイドライン整備
- [ ] 公式Homebrew登録検討（v2.0.0+）

---

## トラブルシューティング

### crates.io公開エラー

**エラー**: `error: missing field 'license'`
```toml
# Cargo.tomlに追加
license = "MIT OR Apache-2.0"
```

**エラー**: `error: package contains README.md but it is not included in the package`
```toml
# Cargo.tomlに追加
readme = "README.md"
```

### Homebrew Formula SHA256不一致

```bash
# SHA256を再取得
curl -sL "URL" | shasum -a 256

# Formula更新
vim Formula/cmdrun.rb
# sha256行を新しい値に更新
```

### GitHub Actions ビルド失敗

```bash
# ローカルでクロスコンパイルテスト
rustup target add aarch64-apple-darwin
cargo build --release --target aarch64-apple-darwin

# エラーログ確認
# GitHub Actions → Failed job → ログ確認
```

---

## 参考リンク

- [crates.io Publishing Guide](https://doc.rust-lang.org/cargo/reference/publishing.html)
- [Homebrew Formula Cookbook](https://docs.brew.sh/Formula-Cookbook)
- [GitHub Actions Documentation](https://docs.github.com/en/actions)
- [Rust Package Layout](https://doc.rust-lang.org/cargo/guide/project-layout.html)
