# Publishing Guide

cmdrunをcrates.ioとHomebrewで公開するためのガイド。

## 前提条件

### crates.io公開
1. crates.ioアカウント作成: https://crates.io/
2. APIトークン取得: https://crates.io/me
3. GitHubシークレット設定:
   - `CARGO_TOKEN`: crates.io APIトークン

### Homebrew公開
1. Homebrew tapリポジトリ作成（推奨）:
   ```bash
   # 例: https://github.com/sanae-abe/homebrew-tap
   ```

## 公開手順

### 1. バージョン更新

```bash
# Cargo.tomlのバージョンを更新
vim Cargo.toml  # version = "1.0.1"

# 変更をコミット
git add Cargo.toml
git commit -m "chore: bump version to 1.0.1"
git push
```

### 2. タグ作成・プッシュ

```bash
# タグを作成
git tag -a v1.0.1 -m "Release v1.0.1"

# タグをプッシュ（GitHub Actionsが自動実行される）
git push origin v1.0.1
```

### 3. GitHub Actionsの自動実行

タグプッシュ後、以下が自動実行されます:

1. **ビルド**: 複数プラットフォームのバイナリ作成
   - Linux x86_64 (GNU)
   - Linux x86_64 (musl)
   - macOS x86_64
   - macOS ARM64
   - Windows x86_64

2. **GitHub Release作成**: バイナリを添付

3. **crates.io公開**: 自動publish

### 4. Homebrew Formula更新

#### 方法A: Tapリポジトリを使用（推奨）

1. Tapリポジトリを作成:
   ```bash
   # 新規リポジトリ: homebrew-tap
   # 場所: https://github.com/sanae-abe/homebrew-tap
   ```

2. Formulaをコピー:
   ```bash
   cp Formula/cmdrun.rb /path/to/homebrew-tap/Formula/
   ```

3. SHA256を計算:
   ```bash
   # リリース後のtarballをダウンロード
   curl -LO https://github.com/sanae-abe/cmdrun/archive/refs/tags/v1.0.1.tar.gz

   # SHA256を計算
   shasum -a 256 v1.0.1.tar.gz
   # 結果をFormula/cmdrun.rbのsha256に設定
   ```

4. Formulaを更新してコミット:
   ```bash
   cd /path/to/homebrew-tap
   vim Formula/cmdrun.rb  # URLとsha256を更新
   git add Formula/cmdrun.rb
   git commit -m "cmdrun 1.0.1"
   git push
   ```

#### 方法B: Homebrew本体に提出

```bash
brew create https://github.com/sanae-abe/cmdrun/archive/refs/tags/v1.0.1.tar.gz
# 編集後、PRを提出
```

## インストール方法（ユーザー向け）

### crates.io経由
```bash
cargo install cmdrun
```

### Homebrew経由（Tap使用時）
```bash
brew tap sanae-abe/tap
brew install cmdrun
```

### Homebrew経由（本体登録時）
```bash
brew install cmdrun
```

### GitHub Releases経由
```bash
# Linux/macOS
curl -LO https://github.com/sanae-abe/cmdrun/releases/latest/download/cmdrun-linux-x86_64.tar.gz
tar xzf cmdrun-linux-x86_64.tar.gz
sudo mv cmdrun /usr/local/bin/

# Windows
# cmdrun-windows-x86_64.zip をダウンロードして展開
```

## トラブルシューティング

### crates.io公開失敗
- `CARGO_TOKEN`が正しく設定されているか確認
- Cargo.tomlの必須フィールドが全て記入されているか確認
- クレート名が既に使用されていないか確認

### Homebrew Formula失敗
- SHA256が正しく計算されているか確認
- URLが正しいか確認（v1.0.0 → v1.0.1など）
- `brew install --build-from-source cmdrun`でテスト

## 参考リンク

- [crates.io Publishing Guide](https://doc.rust-lang.org/cargo/reference/publishing.html)
- [Homebrew Formula Cookbook](https://docs.brew.sh/Formula-Cookbook)
- [GitHub Actions for Rust](https://github.com/actions-rs)
