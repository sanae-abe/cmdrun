# cmdrun

[![Version](https://img.shields.io/badge/version-1.3.0-blue.svg)](https://github.com/sanae-abe/cmdrun)
[![Rust](https://img.shields.io/badge/rust-1.75%2B-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](LICENSE)

[English](README.md) | [日本語](README.ja.md) | [简体中文](README.zh-CN.md) | [繁體中文](README.zh-TW.md)

> **個人全域命令管理器，管理您的常用命令**
>
> 一次註冊命令，隨處執行。快速、安全、跨平台。

## 目錄

- [為什麼選擇 cmdrun？](#為什麼選擇-cmdrun)
- [安裝](#安裝)
- [基本用法](#基本用法)
- [功能特性](#功能特性)
- [配置範例](#配置範例)
- [文件](#文件)
- [授權](#授權)

## 為什麼選擇 cmdrun？

### 🚀 效能

- **啟動速度快約29倍** 相較於基於Node.js的工作執行器
- **4ms啟動時間** 對比npm/yarn的115ms+
- **10MB記憶體佔用** 對比Node.js的200MB+

### 🔒 安全性

- **零`eval()`** - 無動態程式碼執行
- **安全的變數展開** - 無shell注入漏洞
- **相依性稽核** - 內建安全掃描

### 🌍 跨平台

- **支援的作業系統**：Linux、macOS、Windows、FreeBSD
- **Shell偵測**：自動偵測bash/zsh/fish/pwsh
- **原生二進位檔**：無執行時相依性

### 💎 開發者體驗

- **TOML配置** - 型別安全，易於閱讀
- **強大功能** - 相依關係、並行執行、鉤子、監視模式
- **優秀的錯誤提示** - 詳細的情境錯誤訊息

### 🎯 獨特優勢

**vs just (24.5k stars):**
- ✅ 互動式TUI模式（just: 僅CLI）
- ✅ 執行歷史和統計（just: 無）
- ✅ 外掛程式系統（just: 無）
- ✅ 環境管理（just: 無）

**vs task (13.2k stars):**
- ✅ 進階安全性（無eval、fuzzing）
- ✅ 多語言支援（task: 僅英語）
- ✅ 範本系統（task: 無）
- ✅ Rust建置（task: Go）

**vs cargo-make (2.5k stars):**
- ✅ 啟動快2.3倍（6.5ms vs 15ms）
- ✅ 語言無關（cargo-make: 專注Rust）
- ✅ 現代化UX（TUI、拼寫檢測）
- ✅ 互動模式

**僅cmdrun擁有的全部功能:**
- 🔒 零eval安全性與fuzzing（373,423測試，0漏洞）
- 🌍 4語言支援（英/日/簡體中/繁體中）
- 🎨 帶模糊搜尋的互動式TUI
- 📊 基於SQLite的執行歷史
- 🔌 動態外掛程式系統
- 🎯 智慧拼寫檢測

## 安裝

#### 系統需求

- **作業系統**：Linux、macOS、Windows、FreeBSD
- **Rust**：1.75+（MSRV）

#### 安裝 Rust 工具鏈

```bash
# 1. 下載並執行 Rustup（Rust 安裝程式）
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 2. 載入環境變數
source ~/.cargo/env

# 3. 驗證安裝
rustc --version
cargo --version
```

#### 建置和安裝 cmdrun

```bash
# 1. 複製儲存庫
git clone git@github.com:sanae-abe/cmdrun.git
cd cmdrun

# 2. 建置並安裝
cargo install --path .

# 3. 驗證安裝
cmdrun --version
cmdrun --help
```

### 更新

```bash
# 如果從原始碼安裝
cd cmdrun  # 進入專案目錄
git pull

# 重新建置並安裝
cargo install --path . --force
```

### 解除安裝

```bash
# 1. 刪除二進位檔案
cargo uninstall cmdrun

# 2. 刪除配置檔案（選擇性）
# Linux/macOS
rm -rf ~/.config/cmdrun

# Windows（在 PowerShell 中執行）
# Remove-Item -Recurse -Force "$env:APPDATA\cmdrun"

# 3. 刪除專案目錄（選擇性）
# cd ..
# rm -rf cmdrun
```

**注意：**
- `cargo uninstall cmdrun` 僅刪除可執行檔
- 配置檔案（commands.toml等）需要手動刪除
- 如果想保留設定，請跳過步驟2

## 基本用法

cmdrun 是一個**個人全域命令管理器**，讓您可以註冊並從系統的任何位置執行常用命令。

#### 註冊常用命令

```bash
# 互動式新增命令
cmdrun add

# 或直接使用參數新增
cmdrun add dev "npm run dev" "啟動開發伺服器"
cmdrun add push "git add . && git commit && git push" "提交並推送變更"
cmdrun add prod-ssh "ssh user@production-server.com" "連線到生產伺服器"
cmdrun add docker-clean "docker system prune -af" "清理未使用的Docker資源"
cmdrun add db-backup "pg_dump mydb > backup_$(date +%Y%m%d).sql" "備份資料庫"
```

#### 執行和管理命令

```bash
# 執行已註冊的命令
cmdrun run dev

# 列出所有已註冊的命令
cmdrun list

# 搜尋命令
cmdrun search docker

# 刪除命令
cmdrun remove dev
```

#### 配置管理

```bash
# 顯示所有設定
cmdrun config show

# 更改語言
cmdrun config set language chinese-traditional

# 使用自訂配置檔案
cmdrun --config ~/work/commands.toml list
cmdrun -c ~/.cmdrun/personal.toml run dev

# 顯示說明
cmdrun --help
```

**配置檔案位置：**
- Linux/macOS：`~/.config/cmdrun/commands.toml`
- Windows：`%APPDATA%\cmdrun\commands.toml`
- 自訂路徑：使用 `--config/-c` 選項指定任何路徑

## 功能特性

### 變數展開

```toml
[commands.deploy]
cmd = "scp dist/ ${DEPLOY_USER:?DEPLOY_USER未設定}@${DEPLOY_HOST:?DEPLOY_HOST未設定}:${DEPLOY_PATH:-/var/www}"
```

支援的語法：
- `${VAR}` - 基本展開
- `${1}`、`${2}`、... - 位置參數
- `${VAR:-default}` - 預設值
- `${VAR:?error}` - 必需變數
- `${VAR:+value}` - 條件替換

**位置參數範例：**

```toml
[commands.convert]
description = "轉換圖片格式"
cmd = "sharp -i ${1} -f ${2:-webp} -q ${3:-80} -o ${4:-output.webp}"
```

```bash
# 使用參數
cmdrun run convert input.png webp 90 output.webp
# 展開為：sharp -i input.png -f webp -q 90 -o output.webp

# 使用預設值
cmdrun run convert input.png
# 展開為：sharp -i input.png -f webp -q 80 -o output.webp
```

### 相依關係

```toml
[commands.test]
cmd = "cargo test"
deps = ["build"]  # 在 'test' 之前執行 'build'

[commands.build]
cmd = "cargo build --release"
```

### 並行執行

```toml
[commands.check]
parallel = true
cmd = [
    "cargo fmt -- --check",
    "cargo clippy",
]
```

### 平台特定命令

```toml
[commands."open:browser"]
cmd.unix = "open http://localhost:3000"
cmd.windows = "start http://localhost:3000"
cmd.linux = "xdg-open http://localhost:3000"
```

### 鉤子

```toml
[hooks]
pre_run = "echo '正在啟動...'"
post_run = "echo '完成！'"

[hooks.commands.deploy]
pre_run = "git diff --exit-code"  # 確保沒有未提交的變更
post_run = "echo '部署時間 $(date)' >> deploy.log"
```

### 環境變數

```toml
[config.env]
NODE_ENV = "development"
RUST_BACKTRACE = "1"

[commands.dev]
cmd = "npm run dev"
env = { PORT = "3000" }  # 命令特定環境變數
```

### 環境管理

輕鬆在開發、預發布和生產環境之間切換。

```bash
# 建立環境
cmdrun env create dev --description "開發環境"
cmdrun env create prod --description "生產環境"

# 切換環境
cmdrun env use dev
cmdrun run start  # 使用開發設定啟動

cmdrun env use prod
cmdrun run deploy  # 使用生產設定部署

# 設定環境變數
cmdrun env set API_URL https://api.staging.com --env staging
```

詳見 [環境管理指南](docs/ENVIRONMENT_MANAGEMENT.md)。

### 歷史記錄和日誌

記錄、搜尋和重播命令執行歷史。

```bash
# 顯示歷史記錄
cmdrun history list

# 搜尋命令
cmdrun history search build

# 顯示統計資訊
cmdrun history stats

# 重試上次失敗的命令
cmdrun retry

# 匯出歷史記錄
cmdrun history export --format json -o history.json
```

詳見 [歷史記錄指南](docs/user-guide/HISTORY.md)。

### 範本系統

使用、建立和分享專案範本。

```bash
# 列出可用範本
cmdrun template list

# 使用範本
cmdrun template use rust-cli

# 建立自訂範本
cmdrun template add my-template

# 匯出範本
cmdrun template export rust-cli ./my-template.toml
```

**內建範本：**
- `rust-cli` - Rust CLI開發（cargo build/test/clippy/fmt）
- `nodejs-web` - Node.js Web開發（npm dev/build/test）
- `python-data` - Python資料科學（pytest/jupyter）
- `react-app` - React應用程式（dev/build/storybook）

詳見 [範本功能報告](TEMPLATE_FEATURE_REPORT.md)。

### 外掛程式系統

透過外部外掛程式擴充功能。

```toml
# commands.toml
[plugins]
enabled = ["hello", "logger"]

[plugins.logger]
path = "plugins/logger_plugin.so"
log_file = "cmdrun.log"
level = "info"
```

```bash
# 列出外掛程式
cmdrun plugin list

# 顯示外掛程式詳情
cmdrun plugin info logger

# 啟用/停用外掛程式
cmdrun plugin enable logger
cmdrun plugin disable logger
```

詳見 [外掛程式系統報告](PLUGIN_SYSTEM_IMPLEMENTATION_REPORT.md) 和 [外掛程式 API](docs/plugins/API.md)。

### 監視模式 - 檔案監視

```toml
# 在 commands.toml 中正常定義命令
[commands.dev]
cmd = "cargo build"

[commands.test]
cmd = "cargo test"
```

```bash
# 從命令列使用監視模式執行
# 監視 Rust 檔案並在變更時建置
cmdrun watch dev --pattern "**/*.rs"

# 自動執行測試（1秒防抖）
cmdrun watch test --pattern "**/*.rs" --debounce 1000

# 監視多個目錄
cmdrun watch dev --path src --path lib
```

**監視模式主要功能：**
- **Glob模式**：檔案過濾（例如 `**/*.rs`、`**/*.ts`）
- **排除模式**：排除不需要的檔案/目錄（預設排除 `node_modules`、`target` 等）
- **防抖**：防止頻繁變更時不必要的執行（預設500ms）
- **遞迴監視**：自動監視子目錄（可使用 `--no-recursive` 停用）
- **gitignore整合**：自動遵守 `.gitignore` 模式

詳見 [監視模式指南](docs/user-guide/WATCH_MODE.md)。

### 互動模式（TUI）

啟動帶模糊搜尋的互動式終端UI。

```bash
# 啟動互動模式
cmdrun interactive
# 或
cmdrun -i
```

**功能：**
- 🔍 **模糊搜尋**：對所有命令進行增量搜尋
- ⚡ **快速執行**：按Enter鍵執行命令
- 📊 **即時預覽**：檢視命令詳情、相依關係和執行歷史
- ⌨️ **鍵盤導航**：
  - `↑`/`↓` 或 `j`/`k`：導航命令
  - `Enter`：執行選定命令
  - `Ctrl+U`：清除搜尋輸入
  - `Ctrl+W`：向後刪除單字
  - `Esc` 或 `q`：退出

**預覽面板：**
- 命令描述和實際命令字串
- 環境變數展開預覽
- 執行統計（執行次數、最後執行時間）

詳見 [TUI實作摘要](docs/TUI_IMPLEMENTATION_SUMMARY.md)。

### 拼寫檢測

cmdrun 自動檢測命令名稱中的拼寫錯誤並提供糾正建議。

**範例：**
```bash
$ cmdrun seach docker
Error: Unknown command 'seach'

您是否想輸入：
  → search (distance: 1)
  → watch (distance: 2)

執行 'cmdrun --help' 檢視可用命令。
```

**配置：**
```toml
[config]
typo_detection = true
typo_threshold = 2        # 最大Levenshtein距離
auto_correct = false      # 設為true自動糾正
```

**多語言錯誤訊息：**
- 英語: "Did you mean 'X'?"
- 日語: "もしかして: 'X' ですか？"
- 簡體中文: "您是否想输入 'X'？"
- 繁體中文: "您是否想輸入 'X'？"

### 語言設定（i18n）

cmdrun 支援4種語言：**英語、日語、簡體中文（简体中文）、繁體中文（繁體中文）**

**自動語言偵測：**
- 讀取 `LANG` 環境變數
- 支援：`en`、`ja`、`zh_CN`、`zh_TW`、`zh_HK`

**本地化命令（9個）：**
- `cmdrun add`、`search`、`init`、`remove`、`info`
- `cmdrun config`、`watch`、`validate`、`edit`
- 拼寫建議的多語言錯誤訊息

**配置：**
```toml
[config]
language = "chinese-traditional"  # 或 "english"、"japanese"、"chinese-simplified"
```

**範例（繁體中文）：**
```bash
$ cmdrun add test "echo 測試" "測試命令"
📝 正在新增命令 'test' ...
✓ 成功新增命令 'test'
  描述: 測試命令
  命令: echo 測試
```

**文件：**
- English: [README.md](README.md)
- 日本語: [README.ja.md](README.ja.md)
- 简体中文: [README.zh-CN.md](README.zh-CN.md)
- 繁體中文: [README.zh-TW.md](README.zh-TW.md)

詳見 [I18N指南](docs/user-guide/I18N.md)。

### 自訂配置檔案

您可以使用 `--config/-c` 選項在多個配置檔案之間切換。

**使用範例：**

```bash
# 工作相關命令
cmdrun --config ~/work/commands.toml list
cmdrun -c ~/work/commands.toml run deploy

# 個人命令
cmdrun -c ~/personal/commands.toml run backup

# 專案特定命令
cd ~/projects/myapp
cmdrun -c ./commands.toml run dev
```

**使用情境：**

1. **特定環境配置**
   ```bash
   # 生產環境
   cmdrun -c ~/.cmdrun/production.toml run deploy

   # 預發布環境
   cmdrun -c ~/.cmdrun/staging.toml run deploy

   # 開發環境
   cmdrun -c ~/.cmdrun/development.toml run dev
   ```

2. **多專案管理**
   ```bash
   # 專案 A
   cmdrun -c ~/projects/project-a/commands.toml run test

   # 專案 B
   cmdrun -c ~/projects/project-b/commands.toml run test
   ```

3. **基於角色的命令集**
   ```bash
   # 系統管理
   cmdrun -c ~/.cmdrun/admin.toml run server-check

   # 開發工作
   cmdrun -c ~/.cmdrun/dev.toml run code-review
   ```

**詳情請參閱 [配置參考](docs/user-guide/CONFIGURATION.md#custom-configuration-file-specification)。**

## 配置範例

您可以直接編輯配置檔案（`~/.config/cmdrun/commands.toml`）以使用進階功能：

```toml
# 帶相依關係的命令
[commands.deploy]
description = "部署到生產環境"
cmd = "ssh user@server 'cd /app && git pull && npm install && pm2 restart app'"
deps = ["test"]  # 只在測試通過後部署
confirm = true   # 執行前要求確認

[commands.test]
description = "執行測試"
cmd = "npm test"

# 使用環境變數
[commands.backup]
description = "建立備份"
cmd = "rsync -avz ~/projects/ ${BACKUP_PATH:?BACKUP_PATH未設定}"

# 平台特定命令
[commands.open]
description = "開啟瀏覽器"
cmd.unix = "open http://localhost:3000"
cmd.windows = "start http://localhost:3000"
cmd.linux = "xdg-open http://localhost:3000"
```

## 文件

### 使用者指南
- [CLI參考](docs/user-guide/CLI.md)
- [配置參考](docs/user-guide/CONFIGURATION.md)
- [國際化（i18n）](docs/user-guide/I18N.md)
- [監視模式](docs/user-guide/WATCH_MODE.md)
- [歷史記錄](docs/user-guide/HISTORY.md)
- [常見問題](docs/user-guide/FAQ.md)
- [技巧集錦](docs/user-guide/RECIPES.md)
- [故障排除](docs/user-guide/TROUBLESHOOTING.md)

### 功能指南
- [環境管理](docs/ENVIRONMENT_MANAGEMENT.md)
- [範本系統](TEMPLATE_FEATURE_REPORT.md)
- [外掛程式系統](PLUGIN_SYSTEM_IMPLEMENTATION_REPORT.md)

### 外掛程式開發
- [外掛程式 API 規範](docs/plugins/API.md)
- [外掛程式開發指南](docs/plugins/DEVELOPMENT_GUIDE.md)
- [範例外掛程式](examples/plugins/README.md)

### 技術文件
- [架構](docs/technical/ARCHITECTURE.md)
- [效能](docs/technical/PERFORMANCE.md)
- [效能指南](docs/technical/PERFORMANCE_GUIDE.md)
- [安全性](docs/technical/SECURITY.md)
- [跨平台支援](docs/technical/CROSS_PLATFORM.md)
- [散布](docs/technical/DISTRIBUTION.md)

## 授權

本專案採用 [MIT 授權](LICENSE)。

---
**開發者**：sanae.a.sunny@gmail.com
