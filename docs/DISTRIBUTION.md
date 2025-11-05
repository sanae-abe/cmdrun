# cmdrun パッケージ配布戦略

## 配布チャネル概要

### 優先度別配布方法

1. **Tier 1（最優先）**
   - cargo install（crates.io）
   - GitHub Releases（バイナリ）
   - Homebrew（macOS/Linux）

2. **Tier 2（高優先）**
   - Scoop（Windows）
   - apt/yum リポジトリ（Linux）
   - Docker Hub

3. **Tier 3（将来対応）**
   - Snap（Linux）
   - Chocolatey（Windows）
   - MacPorts（macOS）

## 1. Cargo / crates.io

### パッケージング準備
```toml
# Cargo.toml
[package]
name = "cmdrun"
version = "2.0.0"
edition = "2021"
rust-version = "1.75"
authors = ["Your Name <email@example.com>"]
license = "MIT OR Apache-2.0"
description = "A fast, secure, and cross-platform command runner"
repository = "https://github.com/yourusername/cmdrun"
readme = "README.md"
keywords = ["cli", "command", "runner", "toml", "task"]
categories = ["command-line-utilities"]

# 配布から除外するファイル
exclude = [
    "tests/fixtures/*",
    ".github/*",
    "scripts/*",
    "benches/*",
]
```

### 公開手順
```bash
# パッケージ検証
cargo package --allow-dirty

# dry-run（実際には公開しない）
cargo publish --dry-run

# 公開
cargo publish
```

### インストール
```bash
# ユーザーが実行
cargo install cmdrun

# 特定バージョン
cargo install cmdrun --version 2.0.0

# Git から直接
cargo install --git https://github.com/yourusername/cmdrun
```

## 2. GitHub Releases

### リリース自動化（GitHub Actions）
```yaml
# .github/workflows/release.yml
name: Release

on:
  push:
    tags:
      - 'v*.*.*'

jobs:
  build:
    name: Build for ${{ matrix.os }}
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        include:
          - os: ubuntu-latest
            target: x86_64-unknown-linux-gnu
            artifact_name: cmdrun
            asset_name: cmdrun-linux-amd64
          - os: ubuntu-latest
            target: aarch64-unknown-linux-gnu
            artifact_name: cmdrun
            asset_name: cmdrun-linux-arm64
          - os: macos-latest
            target: x86_64-apple-darwin
            artifact_name: cmdrun
            asset_name: cmdrun-macos-amd64
          - os: macos-latest
            target: aarch64-apple-darwin
            artifact_name: cmdrun
            asset_name: cmdrun-macos-arm64
          - os: windows-latest
            target: x86_64-pc-windows-msvc
            artifact_name: cmdrun.exe
            asset_name: cmdrun-windows-amd64.exe

    steps:
      - uses: actions/checkout@v4

      - name: Install Rust
        uses: dtolnay/rust-toolchain@stable
        with:
          targets: ${{ matrix.target }}

      - name: Install cross (Linux ARM64)
        if: matrix.target == 'aarch64-unknown-linux-gnu'
        run: cargo install cross --git https://github.com/cross-rs/cross

      - name: Build
        run: |
          if [[ "${{ matrix.target }}" == "aarch64-unknown-linux-gnu" ]]; then
            cross build --release --target ${{ matrix.target }}
          else
            cargo build --release --target ${{ matrix.target }}
          fi

      - name: Strip binary (Unix)
        if: matrix.os != 'windows-latest'
        run: strip target/${{ matrix.target }}/release/${{ matrix.artifact_name }}

      - name: Create archive
        run: |
          mkdir -p dist
          if [[ "${{ matrix.os }}" == "windows-latest" ]]; then
            cp target/${{ matrix.target }}/release/${{ matrix.artifact_name }} dist/${{ matrix.asset_name }}
            cd dist
            7z a ${{ matrix.asset_name }}.zip ${{ matrix.asset_name }}
          else
            cp target/${{ matrix.target }}/release/${{ matrix.artifact_name }} dist/${{ matrix.asset_name }}
            cd dist
            tar czf ${{ matrix.asset_name }}.tar.gz ${{ matrix.asset_name }}
          fi

      - name: Upload artifact
        uses: actions/upload-artifact@v4
        with:
          name: ${{ matrix.asset_name }}
          path: dist/*

  release:
    name: Create Release
    needs: build
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Download artifacts
        uses: actions/download-artifact@v4
        with:
          path: artifacts

      - name: Create Release
        uses: softprops/action-gh-release@v1
        with:
          draft: false
          prerelease: false
          generate_release_notes: true
          files: artifacts/**/*
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
```

### リリースノート自動生成
```yaml
# .github/release.yml
changelog:
  categories:
    - title: 🚀 Features
      labels:
        - enhancement
        - feature
    - title: 🐛 Bug Fixes
      labels:
        - bug
        - fix
    - title: 📚 Documentation
      labels:
        - documentation
    - title: 🔧 Maintenance
      labels:
        - maintenance
        - refactor
```

### バージョンタグ作成
```bash
# バージョン更新
vim Cargo.toml  # version = "2.0.0"

# Git タグ作成
git tag -a v2.0.0 -m "Release v2.0.0"
git push origin v2.0.0

# GitHub Actions が自動実行
```

## 3. Homebrew

### Formula 作成
```ruby
# Formula/cmdrun.rb
class Cmdrun < Formula
  desc "Fast, secure, and cross-platform command runner"
  homepage "https://github.com/yourusername/cmdrun"
  url "https://github.com/yourusername/cmdrun/archive/v2.0.0.tar.gz"
  sha256 "0123456789abcdef..." # tar.gz の SHA256
  license "MIT"

  depends_on "rust" => :build

  def install
    system "cargo", "install", *std_cargo_args
  end

  test do
    system "#{bin}/cmdrun", "--version"
  end
end
```

### Tap リポジトリ
```bash
# homebrew-cmdrun リポジトリ作成
# https://github.com/yourusername/homebrew-cmdrun

# インストール
brew tap yourusername/cmdrun
brew install cmdrun

# または直接
brew install yourusername/cmdrun/cmdrun
```

### バイナリ配布版（高速）
```ruby
class Cmdrun < Formula
  desc "Fast, secure, and cross-platform command runner"
  homepage "https://github.com/yourusername/cmdrun"
  version "2.0.0"
  license "MIT"

  on_macos do
    if Hardware::CPU.intel?
      url "https://github.com/yourusername/cmdrun/releases/download/v2.0.0/cmdrun-macos-amd64.tar.gz"
      sha256 "..."
    else
      url "https://github.com/yourusername/cmdrun/releases/download/v2.0.0/cmdrun-macos-arm64.tar.gz"
      sha256 "..."
    end
  end

  on_linux do
    if Hardware::CPU.intel?
      url "https://github.com/yourusername/cmdrun/releases/download/v2.0.0/cmdrun-linux-amd64.tar.gz"
      sha256 "..."
    else
      url "https://github.com/yourusername/cmdrun/releases/download/v2.0.0/cmdrun-linux-arm64.tar.gz"
      sha256 "..."
    end
  end

  def install
    bin.install "cmdrun"
  end

  test do
    assert_match "cmdrun 2.0.0", shell_output("#{bin}/cmdrun --version")
  end
end
```

## 4. Scoop（Windows）

### Manifest 作成
```json
{
  "version": "2.0.0",
  "description": "Fast, secure, and cross-platform command runner",
  "homepage": "https://github.com/yourusername/cmdrun",
  "license": "MIT",
  "architecture": {
    "64bit": {
      "url": "https://github.com/yourusername/cmdrun/releases/download/v2.0.0/cmdrun-windows-amd64.exe.zip",
      "hash": "sha256:...",
      "bin": "cmdrun.exe"
    }
  },
  "checkver": {
    "github": "https://github.com/yourusername/cmdrun"
  },
  "autoupdate": {
    "architecture": {
      "64bit": {
        "url": "https://github.com/yourusername/cmdrun/releases/download/v$version/cmdrun-windows-amd64.exe.zip"
      }
    }
  }
}
```

### Bucket 公開
```bash
# scoop-bucket リポジトリ作成
# https://github.com/yourusername/scoop-bucket

# インストール
scoop bucket add cmdrun https://github.com/yourusername/scoop-bucket
scoop install cmdrun
```

## 5. Linux パッケージ

### Debian/Ubuntu (apt)

#### パッケージビルド
```bash
# scripts/package-deb.sh
#!/bin/bash
set -e

VERSION="2.0.0"
ARCH="amd64"

# ビルド
cargo build --release --target x86_64-unknown-linux-gnu

# パッケージディレクトリ作成
mkdir -p cmdrun_${VERSION}_${ARCH}/DEBIAN
mkdir -p cmdrun_${VERSION}_${ARCH}/usr/bin
mkdir -p cmdrun_${VERSION}_${ARCH}/usr/share/doc/cmdrun

# バイナリコピー
cp target/x86_64-unknown-linux-gnu/release/cmdrun cmdrun_${VERSION}_${ARCH}/usr/bin/
strip cmdrun_${VERSION}_${ARCH}/usr/bin/cmdrun

# control ファイル作成
cat > cmdrun_${VERSION}_${ARCH}/DEBIAN/control <<EOF
Package: cmdrun
Version: ${VERSION}
Architecture: ${ARCH}
Maintainer: Your Name <email@example.com>
Description: Fast, secure, and cross-platform command runner
 A modern replacement for package.json scripts and Makefiles
Section: utils
Priority: optional
Homepage: https://github.com/yourusername/cmdrun
EOF

# ドキュメント
cp README.md cmdrun_${VERSION}_${ARCH}/usr/share/doc/cmdrun/
cp LICENSE cmdrun_${VERSION}_${ARCH}/usr/share/doc/cmdrun/

# パッケージ作成
dpkg-deb --build cmdrun_${VERSION}_${ARCH}
```

#### リポジトリ公開
```bash
# APT リポジトリセットアップ（GitHub Pages等）
# https://assafmo.github.io/2019/05/02/ppa-repo-hosted-on-github.html
```

### RHEL/CentOS (yum/dnf)
```bash
# RPM パッケージビルド
cargo install cargo-rpm
cargo rpm build
```

## 6. Docker

### Dockerfile
```dockerfile
# Dockerfile
FROM rust:1.75 AS builder

WORKDIR /app
COPY . .

RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/cmdrun /usr/local/bin/cmdrun

ENTRYPOINT ["cmdrun"]
CMD ["--help"]
```

### Docker Hub 公開
```bash
# ビルド＆プッシュ
docker build -t yourusername/cmdrun:2.0.0 .
docker push yourusername/cmdrun:2.0.0
docker tag yourusername/cmdrun:2.0.0 yourusername/cmdrun:latest
docker push yourusername/cmdrun:latest
```

### 使用例
```bash
# プロジェクトディレクトリで実行
docker run --rm -v $(pwd):/workspace -w /workspace yourusername/cmdrun run build
```

## 7. インストールスクリプト

### ワンライナーインストール
```bash
# scripts/install.sh
#!/bin/bash
set -e

REPO="yourusername/cmdrun"
BINARY="cmdrun"

# プラットフォーム検出
OS="$(uname -s)"
ARCH="$(uname -m)"

case "$OS" in
  Linux)
    case "$ARCH" in
      x86_64) TARGET="linux-amd64" ;;
      aarch64) TARGET="linux-arm64" ;;
      *) echo "Unsupported architecture: $ARCH"; exit 1 ;;
    esac
    ;;
  Darwin)
    case "$ARCH" in
      x86_64) TARGET="macos-amd64" ;;
      arm64) TARGET="macos-arm64" ;;
      *) echo "Unsupported architecture: $ARCH"; exit 1 ;;
    esac
    ;;
  *)
    echo "Unsupported OS: $OS"
    exit 1
    ;;
esac

# 最新バージョン取得
VERSION=$(curl -s "https://api.github.com/repos/$REPO/releases/latest" | grep '"tag_name"' | sed -E 's/.*"v([^"]+)".*/\1/')

echo "Installing cmdrun v$VERSION for $TARGET..."

# ダウンロード
DOWNLOAD_URL="https://github.com/$REPO/releases/download/v$VERSION/cmdrun-$TARGET.tar.gz"
curl -sL "$DOWNLOAD_URL" | tar xz

# インストール
sudo mv cmdrun /usr/local/bin/
sudo chmod +x /usr/local/bin/cmdrun

echo "cmdrun installed successfully!"
cmdrun --version
```

### 使用方法
```bash
curl -sSL https://raw.githubusercontent.com/yourusername/cmdrun/main/scripts/install.sh | bash
```

## 8. バージョン管理戦略

### セマンティックバージョニング
```
MAJOR.MINOR.PATCH

2.0.0 → 2.0.1 (パッチ: バグ修正)
2.0.1 → 2.1.0 (マイナー: 機能追加、後方互換性あり)
2.1.0 → 3.0.0 (メジャー: 破壊的変更)
```

### リリースサイクル
- **パッチ**: 2週間ごと（緊急時は随時）
- **マイナー**: 2ヶ月ごと
- **メジャー**: 年1回（破壊的変更必要時）

### Changelog 管理
```markdown
# CHANGELOG.md

## [2.0.0] - 2025-11-05

### Added
- TOML設定ファイルサポート
- 並列コマンド実行
- プラットフォーム別コマンド定義

### Changed
- Rust 完全書き換え（Node.js → Rust）
- 起動時間 10倍高速化

### Removed
- レガシーJSON設定サポート
```

## 9. 配布チェックリスト

### リリース前
- [ ] Cargo.toml バージョン更新
- [ ] CHANGELOG.md 更新
- [ ] README.md 更新
- [ ] 全プラットフォームでビルド確認
- [ ] 全テスト通過確認
- [ ] セキュリティ監査実行

### リリース実行
- [ ] Git タグ作成・プッシュ
- [ ] GitHub Actions 成功確認
- [ ] GitHub Releases 公開確認
- [ ] crates.io 公開
- [ ] Homebrew Formula 更新
- [ ] Scoop Manifest 更新
- [ ] Docker Hub プッシュ

### リリース後
- [ ] リリースノート公開
- [ ] Twitter/SNS 告知
- [ ] ドキュメントサイト更新
- [ ] ユーザーフィードバック収集
