# インストールガイド

cmdrunのインストール方法を説明します。

## 目次

- [クイックインストール](#クイックインストール)
- [インストール方法](#インストール方法)
  - [Cargoでインストール（推奨）](#cargoでインストール推奨)
  - [ソースからビルド](#ソースからビルド)
- [動作確認](#動作確認)
- [アップデート](#アップデート)
- [アンインストール](#アンインストール)

## クイックインストール

### macOS/Linux
```bash
# Cargo（Rustパッケージマネージャー）を使用
cargo install cmdrun
```

### Windows
```powershell
# Cargoを使用
cargo install cmdrun
```

## インストール方法

### Cargoでインストール（推奨）

最も簡単で確実な方法は、RustのパッケージマネージャーであるCargoを使用することです。

#### 前提条件
- Rustツールチェーン（rustc 1.70.0以降）
- Cargo（通常Rustと一緒にインストールされます）

#### Rustのインストール
Rustがインストールされていない場合：

**macOS/Linux:**
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
```

**Windows:**
[rustup-init.exe](https://rustup.rs/)をダウンロードして実行

#### cmdrunのインストール
```bash
cargo install cmdrun
```

**インストール詳細:**
- バイナリの場所: `~/.cargo/bin/cmdrun` (Unix) または `%USERPROFILE%\.cargo\bin\cmdrun.exe` (Windows)
- Rustインストール時に自動的にPATHに追加されます
- インストール時間: 1-3分程度（システムによる）

#### 特定のバージョンをインストール
```bash
# 最新バージョンをインストール
cargo install cmdrun

# 特定のバージョンをインストール
cargo install cmdrun --version 1.0.0

# 強制的に再インストール
cargo install cmdrun --force
```

### ソースからビルド

開発者向けまたは最新の機能を使いたい場合。

#### 前提条件
- Rustツールチェーン（rustc 1.70.0以降）
- Git

#### クローンとビルド
```bash
# リポジトリをクローン
git clone git@github.com:sanae-abe/cmdrun.git
cd cmdrun

# リリースモードでビルド（最適化）
cargo build --release

# バイナリの場所: ./target/release/cmdrun
./target/release/cmdrun --version

# ~/.cargo/bin/ にインストール
cargo install --path .
```

## 動作確認

インストール後、`cmdrun`が正しく動作することを確認します：

### バージョン確認
```bash
cmdrun --version
# 出力例: cmdrun 1.0.0 (または現在のバージョン)
```

### インストール場所の確認
```bash
# Unix系システム
which cmdrun
# 出力例: /usr/local/bin/cmdrun または ~/.cargo/bin/cmdrun

# Windows
where.exe cmdrun
# 出力例: C:\Users\YourName\.cargo\bin\cmdrun.exe など
```

### 基本機能のテスト
```bash
# ヘルプを表示
cmdrun --help

# コマンド一覧を表示（初回は空）
cmdrun list
```

### テストコマンドの作成
```bash
# テストコマンドを追加
cmdrun add hello "echo 'Hello from cmdrun!'" "テストコマンド"

# コマンドを実行
cmdrun run hello
# 出力例: Hello from cmdrun!
```

---

## トラブルシューティング

### コマンドが見つからない

**問題:** `cmdrun: command not found` または `'cmdrun' is not recognized`

**解決方法:**

1. **PATHを確認:**
   ```bash
   # Unix系
   echo $PATH | grep -o "[^:]*cargo[^:]*"

   # Windows PowerShell
   $env:Path -split ';' | Select-String cargo
   ```

2. **Cargo binをPATHに追加:**
   ```bash
   # Bash/Zsh (~/.bashrc または ~/.zshrc)
   export PATH="$HOME/.cargo/bin:$PATH"

   # Fish (~/.config/fish/config.fish)
   set -gx PATH $HOME/.cargo/bin $PATH

   # Windows PowerShell (管理者として実行)
   $env:Path += ";$env:USERPROFILE\.cargo\bin"
   [Environment]::SetEnvironmentVariable("Path", $env:Path, [EnvironmentVariableTarget]::User)
   ```

3. **シェルを再起動:**
   ```bash
   # Unix系
   exec $SHELL

   # Windows: PowerShell/コマンドプロンプトを閉じて再度開く
   ```

### Cargoインストールが失敗する

**問題:** `cargo install cmdrun` 実行時のコンパイルエラー

**解決方法:**

1. **Rustツールチェーンの更新:**
   ```bash
   rustup update stable
   cargo install cmdrun
   ```

2. **Rustバージョンの確認:**
   ```bash
   rustc --version
   # 1.70.0以降が必要
   ```

3. **Cargoキャッシュのクリア:**
   ```bash
   cargo clean
   rm -rf ~/.cargo/registry/cache
   cargo install cmdrun
   ```

4. **詳細ログ出力でインストール:**
   ```bash
   cargo install cmdrun --verbose
   # エラーメッセージを確認
   ```

### 権限エラー

**問題:** バイナリの移動時に権限エラーが発生

**解決方法:**

1. **sudo使用 (Unix系):**
   ```bash
   sudo mv cmdrun /usr/local/bin/
   ```

2. **ユーザーディレクトリを使用:**
   ```bash
   # ユーザーbinディレクトリを作成
   mkdir -p ~/.local/bin
   mv cmdrun ~/.local/bin/

   # PATHに追加
   export PATH="$HOME/.local/bin:$PATH"
   ```

3. **Windows: 管理者として実行**
   - PowerShellを右クリック → 管理者として実行

### バイナリが実行できない

**問題:** バイナリは存在するが実行できない

**解決方法:**

1. **実行権限の付与 (Unix系):**
   ```bash
   chmod +x cmdrun
   ```

2. **ファイルタイプの確認:**
   ```bash
   file cmdrun
   # 出力例: ELF 64-bit executable (Linux) または Mach-O executable (macOS)
   ```

3. **Windows: ファイルのブロック解除**
   ```powershell
   Unblock-File -Path cmdrun.exe
   ```

### 動作が遅い

**問題:** cmdrunの実行速度が遅い

**解決方法:**

1. **リリースビルドの確認:**
   ```bash
   # ソースからビルドした場合、リリースモードを使用
   cargo build --release
   ```

2. **デバッグシンボルの削除:**
   ```bash
   # デバッグシンボルを削除して高速化
   strip target/release/cmdrun
   ```

3. **最新バージョンへの更新:**
   ```bash
   cargo install cmdrun --force
   ```

---

## アップデート

### Cargoでインストールした場合
```bash
# 最新バージョンへ更新
cargo install cmdrun --force
```

### ソースからビルドした場合
```bash
cd cmdrun  # プロジェクトディレクトリ
git pull
cargo install --path . --force
```

---

## アンインストール

### Cargoでインストールした場合
```bash
cargo uninstall cmdrun
```

### 設定ファイルの削除（任意）
```bash
# cmdrun関連ファイルを削除（注意！）
# Linux/macOS
rm -rf ~/.config/cmdrun

# Windows
Remove-Item "$env:APPDATA\cmdrun" -Recurse
```

---

## 次のステップ

インストール後：

1. **[クイックスタートガイド](../../README.md#クイックスタート)** でコマンドを登録
2. **[設定リファレンス](CONFIGURATION.md)** で高度な機能を確認
3. **[CLIリファレンス](CLI.md)** で利用可能なコマンドを確認

---

## プラットフォーム固有の注意事項

### macOS
- **Gatekeeper:** 初回実行時に「システム環境設定」→「セキュリティとプライバシー」で許可が必要な場合があります

### Linux
- **SELinux:** 必要に応じて `chcon -t bin_t /usr/local/bin/cmdrun` を実行

### Windows
- **ウイルス対策ソフト:** 初回実行時に誤検知される場合があります
- **PATH順序:** Cargo binディレクトリが他のパスより優先されるように設定してください
- **PowerShell実行ポリシー:** `Set-ExecutionPolicy RemoteSigned` が必要な場合があります

---

**困ったときは** [トラブルシューティング](#トラブルシューティング)を参照してください。
