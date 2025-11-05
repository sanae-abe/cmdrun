# cmdrun クロスプラットフォーム対応設計

## サポート対象プラットフォーム

### Tier 1（完全サポート）
- **Linux x86_64**: Ubuntu 20.04+, Debian 11+, RHEL 8+
- **macOS**: macOS 11+ (Intel & Apple Silicon)
- **Windows**: Windows 10/11 (x86_64)

### Tier 2（ベストエフォート）
- **Linux ARM64**: Raspberry Pi 4+
- **FreeBSD**: 13.0+

## 1. プラットフォーム検出

### コンパイル時検出
```rust
// src/platform/mod.rs

/// 現在のプラットフォーム
pub enum Platform {
    Linux,
    MacOS,
    Windows,
    FreeBSD,
    Unknown,
}

impl Platform {
    /// コンパイル時プラットフォーム検出
    pub const fn current() -> Self {
        #[cfg(target_os = "linux")]
        return Platform::Linux;

        #[cfg(target_os = "macos")]
        return Platform::MacOS;

        #[cfg(target_os = "windows")]
        return Platform::Windows;

        #[cfg(target_os = "freebsd")]
        return Platform::FreeBSD;

        #[cfg(not(any(
            target_os = "linux",
            target_os = "macos",
            target_os = "windows",
            target_os = "freebsd"
        )))]
        return Platform::Unknown;
    }

    /// プラットフォーム名
    pub const fn name(&self) -> &'static str {
        match self {
            Platform::Linux => "linux",
            Platform::MacOS => "macos",
            Platform::Windows => "windows",
            Platform::FreeBSD => "freebsd",
            Platform::Unknown => "unknown",
        }
    }

    /// Unix系か
    pub const fn is_unix(&self) -> bool {
        matches!(self, Platform::Linux | Platform::MacOS | Platform::FreeBSD)
    }

    /// Windows か
    pub const fn is_windows(&self) -> bool {
        matches!(self, Platform::Windows)
    }
}
```

### ランタイム詳細情報
```rust
use os_info;

pub fn get_os_info() -> OsInfo {
    let info = os_info::get();

    OsInfo {
        os_type: info.os_type(),
        version: info.version().to_string(),
        bitness: info.bitness(),
    }
}
```

## 2. シェル検出と対応

### Unix系シェル
```rust
// src/platform/unix.rs

pub fn detect_unix_shell() -> String {
    // 環境変数SHELLを優先
    if let Ok(shell) = env::var("SHELL") {
        if let Some(shell_name) = shell.split('/').last() {
            return shell_name.to_string();
        }
    }

    // フォールバック: 一般的なシェルを検索
    let shells = ["bash", "zsh", "fish", "sh"];

    for shell in &shells {
        if which::which(shell).is_ok() {
            return shell.to_string();
        }
    }

    // 最終フォールバック
    "sh".to_string()
}

pub fn get_shell_flags(shell: &str) -> &'static [&'static str] {
    match shell {
        "bash" | "zsh" | "sh" => &["-c"],
        "fish" => &["-c"],
        _ => &["-c"],
    }
}
```

### Windows シェル
```rust
// src/platform/windows.rs

pub fn detect_windows_shell() -> String {
    // PowerShell Core (pwsh) を優先
    if which::which("pwsh").is_ok() {
        return "pwsh".to_string();
    }

    // Windows PowerShell
    if which::which("powershell").is_ok() {
        return "powershell".to_string();
    }

    // cmd.exe（最終フォールバック）
    "cmd".to_string()
}

pub fn get_shell_flags(shell: &str) -> &'static [&'static str] {
    if shell.contains("pwsh") || shell.contains("powershell") {
        &["-Command"]
    } else {
        // cmd.exe
        &["/C"]
    }
}
```

## 3. パス処理

### パス区切り文字
```rust
use std::path::{Path, PathBuf, MAIN_SEPARATOR};

pub fn normalize_path(path: &str) -> PathBuf {
    // プラットフォーム固有の区切り文字に変換
    let normalized = if cfg!(windows) {
        path.replace('/', "\\")
    } else {
        path.replace('\\', "/")
    };

    PathBuf::from(normalized)
}

pub fn join_paths(base: &Path, relative: &str) -> PathBuf {
    let mut path = base.to_path_buf();

    for component in relative.split(&['/', '\\'][..]) {
        if !component.is_empty() && component != "." {
            path.push(component);
        }
    }

    path
}
```

### ホームディレクトリ
```rust
use dirs;

pub fn home_dir() -> Option<PathBuf> {
    dirs::home_dir()
}

pub fn config_dir() -> PathBuf {
    #[cfg(unix)]
    return dirs::config_dir()
        .unwrap_or_else(|| home_dir().unwrap().join(".config"))
        .join("cmdrun");

    #[cfg(windows)]
    return dirs::config_dir()
        .unwrap_or_else(|| home_dir().unwrap().join("AppData\\Roaming"))
        .join("cmdrun");
}
```

## 4. 環境変数の違い

### パス区切り
```rust
pub fn path_separator() -> char {
    #[cfg(unix)]
    return ':';

    #[cfg(windows)]
    return ';';
}

pub fn split_path_env(path_env: &str) -> Vec<PathBuf> {
    path_env
        .split(path_separator())
        .map(PathBuf::from)
        .collect()
}
```

### 標準環境変数
```rust
pub struct EnvVars {
    pub home: PathBuf,
    pub user: String,
    pub path: Vec<PathBuf>,
}

impl EnvVars {
    pub fn load() -> Self {
        #[cfg(unix)]
        let user = env::var("USER").unwrap_or_else(|_| "unknown".to_string());

        #[cfg(windows)]
        let user = env::var("USERNAME").unwrap_or_else(|_| "unknown".to_string());

        let home = home_dir().unwrap_or_else(|| PathBuf::from("."));
        let path = env::var("PATH")
            .map(|p| split_path_env(&p))
            .unwrap_or_default();

        Self { home, user, path }
    }
}
```

## 5. ファイルシステムの違い

### 実行権限
```rust
#[cfg(unix)]
pub fn is_executable(path: &Path) -> bool {
    use std::os::unix::fs::PermissionsExt;

    if let Ok(metadata) = path.metadata() {
        let permissions = metadata.permissions();
        permissions.mode() & 0o111 != 0
    } else {
        false
    }
}

#[cfg(windows)]
pub fn is_executable(path: &Path) -> bool {
    // Windows: 拡張子で判定
    if let Some(ext) = path.extension() {
        matches!(
            ext.to_str().unwrap_or("").to_lowercase().as_str(),
            "exe" | "bat" | "cmd" | "com"
        )
    } else {
        false
    }
}
```

### シンボリックリンク
```rust
pub fn is_symlink(path: &Path) -> bool {
    #[cfg(unix)]
    return path.symlink_metadata()
        .map(|m| m.file_type().is_symlink())
        .unwrap_or(false);

    #[cfg(windows)]
    return path.symlink_metadata()
        .map(|m| m.file_type().is_symlink())
        .unwrap_or(false);
}
```

## 6. プロセス実行の違い

### シグナル処理
```rust
#[cfg(unix)]
pub async fn setup_signal_handlers() {
    use tokio::signal::unix::{signal, SignalKind};

    let mut sigint = signal(SignalKind::interrupt()).unwrap();
    let mut sigterm = signal(SignalKind::terminate()).unwrap();

    tokio::select! {
        _ = sigint.recv() => {
            eprintln!("Received SIGINT, terminating...");
        }
        _ = sigterm.recv() => {
            eprintln!("Received SIGTERM, terminating...");
        }
    }
}

#[cfg(windows)]
pub async fn setup_signal_handlers() {
    use tokio::signal::windows;

    let mut ctrl_c = windows::ctrl_c().unwrap();
    let mut ctrl_break = windows::ctrl_break().unwrap();

    tokio::select! {
        _ = ctrl_c.recv() => {
            eprintln!("Received Ctrl+C, terminating...");
        }
        _ = ctrl_break.recv() => {
            eprintln!("Received Ctrl+Break, terminating...");
        }
    }
}
```

### プロセスキル
```rust
use std::process::Child;

#[cfg(unix)]
pub fn kill_process(child: &mut Child) -> std::io::Result<()> {
    use nix::sys::signal::{kill, Signal};
    use nix::unistd::Pid;

    if let Some(pid) = child.id() {
        kill(Pid::from_raw(pid as i32), Signal::SIGTERM)?;
    }

    Ok(())
}

#[cfg(windows)]
pub fn kill_process(child: &mut Child) -> std::io::Result<()> {
    child.kill()
}
```

## 7. テスト戦略

### プラットフォーム別テスト
```rust
#[cfg(unix)]
#[test]
fn test_unix_specific() {
    let shell = detect_unix_shell();
    assert!(shell == "bash" || shell == "zsh" || shell == "sh");
}

#[cfg(windows)]
#[test]
fn test_windows_specific() {
    let shell = detect_windows_shell();
    assert!(
        shell.contains("pwsh")
            || shell.contains("powershell")
            || shell == "cmd"
    );
}

// 全プラットフォーム共通テスト
#[test]
fn test_cross_platform() {
    let platform = Platform::current();
    assert_ne!(platform.name(), "unknown");
}
```

### CI/CDマトリックス
```yaml
# .github/workflows/ci.yml
strategy:
  matrix:
    os: [ubuntu-latest, macos-latest, windows-latest]
    rust: [stable, beta]
    include:
      - os: ubuntu-latest
        target: x86_64-unknown-linux-gnu
      - os: macos-latest
        target: x86_64-apple-darwin
      - os: macos-latest
        target: aarch64-apple-darwin
      - os: windows-latest
        target: x86_64-pc-windows-msvc
```

## 8. ビルド設定

### クロスコンパイル
```toml
# Cross.toml
[target.x86_64-unknown-linux-gnu]
image = "ghcr.io/cross-rs/x86_64-unknown-linux-gnu:edge"

[target.aarch64-unknown-linux-gnu]
image = "ghcr.io/cross-rs/aarch64-unknown-linux-gnu:edge"

[target.x86_64-pc-windows-gnu]
image = "ghcr.io/cross-rs/x86_64-pc-windows-gnu:edge"
```

```bash
# クロスコンパイル実行
cross build --release --target x86_64-unknown-linux-gnu
cross build --release --target aarch64-apple-darwin
cross build --release --target x86_64-pc-windows-msvc
```

### ターゲット別最適化
```toml
[profile.release]
opt-level = 3
lto = "fat"

# プラットフォーム別最適化
[target.'cfg(target_os = "linux")'.dependencies]
# Linux固有の依存関係

[target.'cfg(target_os = "macos")'.dependencies]
# macOS固有の依存関係

[target.'cfg(target_os = "windows")'.dependencies]
# Windows固有の依存関係
```

## 9. プラットフォーム固有機能

### カラー出力
```rust
use colored::*;

pub fn supports_color() -> bool {
    #[cfg(unix)]
    {
        // TERM環境変数チェック
        env::var("TERM")
            .map(|term| term != "dumb")
            .unwrap_or(false)
    }

    #[cfg(windows)]
    {
        // Windows 10以降はANSIカラー対応
        use os_info;
        let info = os_info::get();
        info.version() >= &os_info::Version::Semantic(10, 0, 0)
    }
}

pub fn init_colors() {
    if !supports_color() {
        colored::control::set_override(false);
    }
}
```

### ターミナルサイズ
```rust
use console::Term;

pub fn terminal_size() -> (u16, u16) {
    let term = Term::stdout();
    term.size()
}
```

## 10. パッケージング

### バイナリ配布
```
cmdrun-2.0.0/
├── cmdrun-2.0.0-x86_64-unknown-linux-gnu.tar.gz
├── cmdrun-2.0.0-x86_64-apple-darwin.tar.gz
├── cmdrun-2.0.0-aarch64-apple-darwin.tar.gz
└── cmdrun-2.0.0-x86_64-pc-windows-msvc.zip
```

### インストーラー
```bash
# Unix系: install.sh
curl -sSL https://example.com/install.sh | sh

# Homebrew
brew install cmdrun

# Windows: Scoop
scoop install cmdrun

# Cargo
cargo install cmdrun
```
