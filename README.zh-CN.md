# cmdrun

[![Version](https://img.shields.io/badge/version-1.0.0-blue.svg)](https://github.com/sanae-abe/cmdrun)
[![Rust](https://img.shields.io/badge/rust-1.75%2B-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](LICENSE)

[English](README.md) | [日本語](README.ja.md) | [简体中文](README.zh-CN.md) | [繁體中文](README.zh-TW.md)

> **个人全局命令管理器，管理您的常用命令**
>
> 一次注册命令，随处运行。快速、安全、跨平台。

## 目录

- [为什么选择 cmdrun？](#为什么选择-cmdrun)
- [安装](#安装)
- [基本用法](#基本用法)
- [功能特性](#功能特性)
- [配置示例](#配置示例)
- [文档](#文档)
- [许可证](#许可证)

## 为什么选择 cmdrun？

### 🚀 性能

- **启动速度快约29倍** 相比基于Node.js的任务运行器
- **4ms启动时间** 对比npm/yarn的115ms+
- **10MB内存占用** 对比Node.js的200MB+

### 🔒 安全性

- **零`eval()`** - 无动态代码执行
- **安全的变量展开** - 无shell注入漏洞
- **依赖审计** - 内置安全扫描

### 🌍 跨平台

- **支持的操作系统**：Linux、macOS、Windows、FreeBSD
- **Shell检测**：自动检测bash/zsh/fish/pwsh
- **原生二进制**：无运行时依赖

### 💎 开发者体验

- **TOML配置** - 类型安全，易于阅读
- **强大功能** - 依赖关系、并行执行、钩子、监视模式
- **优秀的错误提示** - 详细的上下文错误消息

### 🎯 cmdrun的特点

**独特功能组合:**
- 🔒 零eval安全性与fuzzing（373,423测试，0漏洞）
- 🌍 4语言支持（英/日/简体中/繁体中）
- 🎨 带模糊搜索的交互式TUI
- 📊 基于SQLite的执行历史
- 🔌 动态插件系统
- 🎯 智能拼写检测

## 安装

#### 系统要求

- **操作系统**：Linux、macOS、Windows、FreeBSD
- **Rust**：1.75+（MSRV）

#### 安装 Rust 工具链

```bash
# 1. 下载并运行 Rustup（Rust 安装器）
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 2. 加载环境变量
source ~/.cargo/env

# 3. 验证安装
rustc --version
cargo --version
```

#### 构建和安装 cmdrun

```bash
# 1. 克隆仓库
git clone git@github.com:sanae-abe/cmdrun.git
cd cmdrun

# 2. 构建并安装
cargo install --path .

# 3. 验证安装
cmdrun --version
cmdrun --help
```

### 更新

```bash
# 如果从源码安装
cd cmdrun  # 进入项目目录
git pull

# 重新构建并安装
cargo install --path . --force
```

### 卸载

```bash
# 1. 删除二进制文件
cargo uninstall cmdrun

# 2. 删除配置文件（可选）
# Linux/macOS
rm -rf ~/.config/cmdrun

# Windows（在 PowerShell 中运行）
# Remove-Item -Recurse -Force "$env:APPDATA\cmdrun"

# 3. 删除项目目录（可选）
# cd ..
# rm -rf cmdrun
```

**注意：**
- `cargo uninstall cmdrun` 仅删除可执行文件
- 配置文件（commands.toml等）需要手动删除
- 如果想保留设置，请跳过步骤2

## 基本用法

cmdrun 是一个**个人全局命令管理器**，让您可以注册并从系统的任何位置运行常用命令。

#### 注册常用命令

```bash
# 交互式添加命令
cmdrun add

# 或直接使用参数添加
cmdrun add dev "npm run dev" "启动开发服务器"
cmdrun add push "git add . && git commit && git push" "提交并推送更改"
cmdrun add prod-ssh "ssh user@production-server.com" "连接到生产服务器"
cmdrun add docker-clean "docker system prune -af" "清理未使用的Docker资源"
cmdrun add db-backup "pg_dump mydb > backup_$(date +%Y%m%d).sql" "备份数据库"
```

#### 运行和管理命令

```bash
# 运行已注册的命令
cmdrun run dev

# 列出所有已注册的命令
cmdrun list

# 搜索命令
cmdrun search docker

# 删除命令
cmdrun remove dev
```

#### 配置管理

```bash
# 显示所有设置
cmdrun config show

# 更改语言
cmdrun config set language chinese-simplified

# 使用自定义配置文件
cmdrun --config ~/work/commands.toml list
cmdrun -c ~/.cmdrun/personal.toml run dev

# 显示帮助
cmdrun --help
```

**配置文件位置：**
- Linux/macOS：`~/.config/cmdrun/commands.toml`
- Windows：`%APPDATA%\cmdrun\commands.toml`
- 自定义路径：使用 `--config/-c` 选项指定任何路径

## 功能特性

### 变量展开

```toml
[commands.deploy]
cmd = "scp dist/ ${DEPLOY_USER:?DEPLOY_USER未设置}@${DEPLOY_HOST:?DEPLOY_HOST未设置}:${DEPLOY_PATH:-/var/www}"
```

支持的语法：
- `${VAR}` - 基本展开
- `${1}`、`${2}`、... - 位置参数
- `${VAR:-default}` - 默认值
- `${VAR:?error}` - 必需变量
- `${VAR:+value}` - 条件替换

**位置参数示例：**

```toml
[commands.convert]
description = "转换图片格式"
cmd = "sharp -i ${1} -f ${2:-webp} -q ${3:-80} -o ${4:-output.webp}"
```

```bash
# 使用参数
cmdrun run convert input.png webp 90 output.webp
# 展开为：sharp -i input.png -f webp -q 90 -o output.webp

# 使用默认值
cmdrun run convert input.png
# 展开为：sharp -i input.png -f webp -q 80 -o output.webp
```

### 依赖关系

```toml
[commands.test]
cmd = "cargo test"
deps = ["build"]  # 在 'test' 之前运行 'build'

[commands.build]
cmd = "cargo build --release"
```

### 并行执行

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

### 钩子

```toml
[hooks]
pre_run = "echo '正在启动...'"
post_run = "echo '完成！'"

[hooks.commands.deploy]
pre_run = "git diff --exit-code"  # 确保没有未提交的更改
post_run = "echo '部署时间 $(date)' >> deploy.log"
```

### 环境变量

```toml
[config.env]
NODE_ENV = "development"
RUST_BACKTRACE = "1"

[commands.dev]
cmd = "npm run dev"
env = { PORT = "3000" }  # 命令特定环境变量
```

### 环境管理

轻松在开发、预发布和生产环境之间切换。

```bash
# 创建环境
cmdrun env create dev --description "开发环境"
cmdrun env create prod --description "生产环境"

# 切换环境
cmdrun env use dev
cmdrun run start  # 使用开发设置启动

cmdrun env use prod
cmdrun run deploy  # 使用生产设置部署

# 设置环境变量
cmdrun env set API_URL https://api.staging.com --env staging
```

详见 [环境管理指南](docs/ENVIRONMENT_MANAGEMENT.md)。

### 历史记录和日志

记录、搜索和重放命令执行历史。

```bash
# 显示历史记录
cmdrun history list

# 搜索命令
cmdrun history search build

# 显示统计信息
cmdrun history stats

# 重试上次失败的命令
cmdrun retry

# 导出历史记录
cmdrun history export --format json -o history.json
```

详见 [历史记录指南](docs/user-guide/HISTORY.md)。

### 模板系统

使用、创建和共享项目模板。

```bash
# 列出可用模板
cmdrun template list

# 使用模板
cmdrun template use rust-cli

# 创建自定义模板
cmdrun template add my-template

# 导出模板
cmdrun template export rust-cli ./my-template.toml
```

**内置模板：**
- `rust-cli` - Rust CLI开发（cargo build/test/clippy/fmt）
- `nodejs-web` - Node.js Web开发（npm dev/build/test）
- `python-data` - Python数据科学（pytest/jupyter）
- `react-app` - React应用（dev/build/storybook）

详见 [模板功能报告](TEMPLATE_FEATURE_REPORT.md)。

### 插件系统

通过外部插件扩展功能。

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
# 列出插件
cmdrun plugin list

# 显示插件详情
cmdrun plugin info logger

# 启用/禁用插件
cmdrun plugin enable logger
cmdrun plugin disable logger
```

详见 [插件系统报告](PLUGIN_SYSTEM_IMPLEMENTATION_REPORT.md) 和 [插件 API](docs/plugins/API.md)。

### 监视模式 - 文件监视

```toml
# 在 commands.toml 中正常定义命令
[commands.dev]
cmd = "cargo build"

[commands.test]
cmd = "cargo test"
```

```bash
# 从命令行使用监视模式运行
# 监视 Rust 文件并在更改时构建
cmdrun watch dev --pattern "**/*.rs"

# 自动运行测试（1秒防抖）
cmdrun watch test --pattern "**/*.rs" --debounce 1000

# 监视多个目录
cmdrun watch dev --path src --path lib
```

**监视模式主要功能：**
- **Glob模式**：文件过滤（例如 `**/*.rs`、`**/*.ts`）
- **排除模式**：排除不需要的文件/目录（默认排除 `node_modules`、`target` 等）
- **防抖**：防止频繁更改时不必要的执行（默认500ms）
- **递归监视**：自动监视子目录（可使用 `--no-recursive` 禁用）
- **gitignore集成**：自动遵守 `.gitignore` 模式

详见 [监视模式指南](docs/user-guide/WATCH_MODE.md)。

### 交互模式（TUI）

启动带模糊搜索的交互式终端UI。

```bash
# 启动交互模式
cmdrun interactive
# 或
cmdrun -i
```

**功能：**
- 🔍 **模糊搜索**：对所有命令进行增量搜索
- ⚡ **快速执行**：按Enter键执行命令
- 📊 **实时预览**：查看命令详情、依赖关系和执行历史
- ⌨️ **键盘导航**：
  - `↑`/`↓` 或 `j`/`k`：导航命令
  - `Enter`：执行选定命令
  - `Ctrl+U`：清除搜索输入
  - `Ctrl+W`：向后删除单词
  - `Esc` 或 `q`：退出

**预览面板：**
- 命令描述和实际命令字符串
- 环境变量展开预览
- 执行统计（运行次数、最后执行时间）

详见 [TUI实现摘要](docs/TUI_IMPLEMENTATION_SUMMARY.md)。

### 拼写检测

cmdrun 自动检测命令名称中的拼写错误并提供纠正建议。

**示例：**
```bash
$ cmdrun seach docker
Error: Unknown command 'seach'

您是否想输入：
  → search (distance: 1)
  → watch (distance: 2)

运行 'cmdrun --help' 查看可用命令。
```

**配置：**
```toml
[config]
typo_detection = true
typo_threshold = 2        # 最大Levenshtein距离
auto_correct = false      # 设为true自动纠正
```

**多语言错误消息：**
- 英语: "Did you mean 'X'?"
- 日语: "もしかして: 'X' ですか？"
- 简体中文: "您是否想输入 'X'？"
- 繁体中文: "您是否想輸入 'X'？"

### 语言设置（i18n）

cmdrun 支持4种语言：**英语、日语、简体中文（简体中文）、繁体中文（繁體中文）**

**自动语言检测：**
- 读取 `LANG` 环境变量
- 支持：`en`、`ja`、`zh_CN`、`zh_TW`、`zh_HK`

**本地化命令（9个）：**
- `cmdrun add`、`search`、`init`、`remove`、`info`
- `cmdrun config`、`watch`、`validate`、`edit`
- 拼写建议的多语言错误消息

**配置：**
```toml
[config]
language = "chinese-simplified"  # 或 "english"、"japanese"、"chinese-traditional"
```

**示例（简体中文）：**
```bash
$ cmdrun add test "echo 测试" "测试命令"
📝 正在添加命令 'test' ...
✓ 成功添加命令 'test'
  说明: 测试命令
  命令: echo 测试
```

**文档：**
- English: [README.md](README.md)
- 日本語: [README.ja.md](README.ja.md)
- 简体中文: [README.zh-CN.md](README.zh-CN.md)
- 繁體中文: [README.zh-TW.md](README.zh-TW.md)

详见 [I18N指南](docs/user-guide/I18N.md)。

### 自定义配置文件

您可以使用 `--config/-c` 选项在多个配置文件之间切换。

**使用示例：**

```bash
# 工作相关命令
cmdrun --config ~/work/commands.toml list
cmdrun -c ~/work/commands.toml run deploy

# 个人命令
cmdrun -c ~/personal/commands.toml run backup

# 项目特定命令
cd ~/projects/myapp
cmdrun -c ./commands.toml run dev
```

**使用场景：**

1. **特定环境配置**
   ```bash
   # 生产环境
   cmdrun -c ~/.cmdrun/production.toml run deploy

   # 预发布环境
   cmdrun -c ~/.cmdrun/staging.toml run deploy

   # 开发环境
   cmdrun -c ~/.cmdrun/development.toml run dev
   ```

2. **多项目管理**
   ```bash
   # 项目 A
   cmdrun -c ~/projects/project-a/commands.toml run test

   # 项目 B
   cmdrun -c ~/projects/project-b/commands.toml run test
   ```

3. **基于角色的命令集**
   ```bash
   # 系统管理
   cmdrun -c ~/.cmdrun/admin.toml run server-check

   # 开发任务
   cmdrun -c ~/.cmdrun/dev.toml run code-review
   ```

**详情请参阅 [配置参考](docs/user-guide/CONFIGURATION.md#custom-configuration-file-specification)。**

## 配置示例

您可以直接编辑配置文件（`~/.config/cmdrun/commands.toml`）以使用高级功能：

```toml
# 带依赖关系的命令
[commands.deploy]
description = "部署到生产环境"
cmd = "ssh user@server 'cd /app && git pull && npm install && pm2 restart app'"
deps = ["test"]  # 只在测试通过后部署
confirm = true   # 运行前要求确认

[commands.test]
description = "运行测试"
cmd = "npm test"

# 使用环境变量
[commands.backup]
description = "创建备份"
cmd = "rsync -avz ~/projects/ ${BACKUP_PATH:?BACKUP_PATH未设置}"

# 平台特定命令
[commands.open]
description = "打开浏览器"
cmd.unix = "open http://localhost:3000"
cmd.windows = "start http://localhost:3000"
cmd.linux = "xdg-open http://localhost:3000"
```

## 文档

### 用户指南
- [CLI参考](docs/user-guide/CLI.md)
- [配置参考](docs/user-guide/CONFIGURATION.md)
- [国际化（i18n）](docs/user-guide/I18N.md)
- [监视模式](docs/user-guide/WATCH_MODE.md)
- [历史记录](docs/user-guide/HISTORY.md)
- [常见问题](docs/user-guide/FAQ.md)
- [技巧集锦](docs/user-guide/RECIPES.md)
- [故障排除](docs/user-guide/TROUBLESHOOTING.md)

### 功能指南
- [环境管理](docs/ENVIRONMENT_MANAGEMENT.md)
- [模板系统](TEMPLATE_FEATURE_REPORT.md)
- [插件系统](PLUGIN_SYSTEM_IMPLEMENTATION_REPORT.md)

### 插件开发
- [插件 API 规范](docs/plugins/API.md)
- [插件开发指南](docs/plugins/DEVELOPMENT_GUIDE.md)
- [示例插件](examples/plugins/README.md)

### 技术文档
- [架构](docs/technical/ARCHITECTURE.md)
- [性能](docs/technical/PERFORMANCE.md)
- [性能指南](docs/technical/PERFORMANCE_GUIDE.md)
- [安全性](docs/technical/SECURITY.md)
- [跨平台支持](docs/technical/CROSS_PLATFORM.md)
- [分发](docs/technical/DISTRIBUTION.md)

## 许可证

本项目采用 [MIT 许可证](LICENSE)。

---
**开发者**：sanae.a.sunny@gmail.com
