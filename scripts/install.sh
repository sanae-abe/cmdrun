#!/bin/bash
# cmdrun インストールスクリプト
#
# 使用方法:
#   curl -sSL https://raw.githubusercontent.com/yourusername/cmdrun/main/scripts/install.sh | bash
#   または
#   ./scripts/install.sh [--version VERSION] [--prefix PATH]

set -euo pipefail

# 色定義
readonly RED='\033[0;31m'
readonly GREEN='\033[0;32m'
readonly YELLOW='\033[1;33m'
readonly BLUE='\033[1;34m'
readonly NC='\033[0m'

# デフォルト設定
REPO="sanae-abe/cmdrun"
VERSION="${1:-latest}"
INSTALL_PREFIX="${INSTALL_PREFIX:-$HOME/.local/bin}"

# フラグ
FORCE=false

# 引数パース
while [[ $# -gt 0 ]]; do
    case $1 in
        --version)
            VERSION="$2"
            shift 2
            ;;
        --prefix)
            INSTALL_PREFIX="$2"
            shift 2
            ;;
        --force)
            FORCE=true
            shift
            ;;
        -h|--help)
            cat <<EOF
🚀 cmdrun インストールスクリプト

使用方法:
  $0 [オプション]

オプション:
  --version VERSION  インストールするバージョン（デフォルト: latest）
  --prefix PATH      インストール先ディレクトリ（デフォルト: ~/.local/bin）
  --force            既存ファイルを上書き
  -h, --help         このヘルプを表示

例:
  # 最新版をインストール
  $0

  # 特定バージョンをインストール
  $0 --version v2.0.0

  # カスタムディレクトリにインストール
  $0 --prefix /usr/local/bin

  # ワンライナーインストール
  curl -sSL https://raw.githubusercontent.com/$REPO/main/scripts/install.sh | bash

EOF
            exit 0
            ;;
        *)
            shift
            ;;
    esac
done

# ログ関数
info() {
    echo -e "${BLUE}ℹ️  $1${NC}"
}

success() {
    echo -e "${GREEN}✅ $1${NC}"
}

warning() {
    echo -e "${YELLOW}⚠️  $1${NC}"
}

error() {
    echo -e "${RED}❌ $1${NC}"
    exit 1
}

# プラットフォーム検出
detect_platform() {
    local os=$(uname -s | tr '[:upper:]' '[:lower:]')
    local arch=$(uname -m)

    case "$arch" in
        x86_64|amd64)
            arch="x86_64"
            ;;
        aarch64|arm64)
            arch="aarch64"
            ;;
        *)
            error "未対応アーキテクチャ: $arch"
            ;;
    esac

    case "$os" in
        darwin)
            os="apple-darwin"
            ;;
        linux)
            os="unknown-linux-gnu"
            ;;
        mingw*|msys*|cygwin*)
            os="pc-windows-msvc"
            ;;
        *)
            error "未対応OS: $os"
            ;;
    esac

    echo "${arch}-${os}"
}

# バージョン解決
resolve_version() {
    if [[ "$VERSION" == "latest" ]]; then
        info "最新バージョンを取得中..."
        # GitHub API使用（レート制限考慮）
        VERSION=$(curl -sSL "https://api.github.com/repos/$REPO/releases/latest" | grep '"tag_name"' | sed -E 's/.*"([^"]+)".*/\1/')

        if [[ -z "$VERSION" ]]; then
            error "最新バージョンの取得に失敗しました"
        fi

        info "最新バージョン: $VERSION"
    fi
}

# ダウンロード
download_binary() {
    local platform=$(detect_platform)
    local archive_name="cmdrun-${VERSION#v}-${platform}.tar.gz"
    local download_url="https://github.com/$REPO/releases/download/$VERSION/$archive_name"
    local temp_dir=$(mktemp -d)
    local temp_archive="$temp_dir/cmdrun.tar.gz"

    info "ダウンロード中: $download_url"

    if ! curl -sSL -f "$download_url" -o "$temp_archive"; then
        error "ダウンロードに失敗しました: $download_url"
    fi

    info "展開中..."
    tar xzf "$temp_archive" -C "$temp_dir"

    local binary_path="$temp_dir/cmdrun"
    if [[ ! -f "$binary_path" ]]; then
        error "バイナリが見つかりません: $binary_path"
    fi

    chmod +x "$binary_path"
    echo "$binary_path"
}

# インストール
install_binary() {
    local temp_file="$1"
    local install_path="$INSTALL_PREFIX/cmdrun"

    # ディレクトリ作成
    mkdir -p "$INSTALL_PREFIX"

    # 既存確認
    if [[ -f "$install_path" ]] && ! $FORCE; then
        warning "既にインストールされています: $install_path"

        read -p "上書きしますか？ [y/N]: " -n 1 -r
        echo

        if [[ ! $REPLY =~ ^[Yy]$ ]]; then
            info "インストールをキャンセルしました"
            rm -f "$temp_file"
            exit 0
        fi
    fi

    # インストール
    mv "$temp_file" "$install_path"
    success "インストール完了: $install_path"
}

# PATH確認
check_path() {
    if [[ ":$PATH:" != *":$INSTALL_PREFIX:"* ]]; then
        warning "PATHに含まれていません: $INSTALL_PREFIX"
        echo
        echo "以下を ~/.zshrc または ~/.bashrc に追加してください:"
        echo "  export PATH=\"\$PATH:$INSTALL_PREFIX\""
    fi
}

# シェル補完インストール
install_completions() {
    info "シェル補完をインストール中..."

    local shell_name
    if [[ -n "${ZSH_VERSION:-}" ]]; then
        shell_name="zsh"
    elif [[ -n "${BASH_VERSION:-}" ]]; then
        shell_name="bash"
    else
        warning "シェル補完のインストールをスキップ（未対応シェル）"
        return
    fi

    local comp_dir
    case "$shell_name" in
        zsh)
            comp_dir="$HOME/.zsh/completions"
            mkdir -p "$comp_dir"
            "$INSTALL_PREFIX/cmdrun" completion zsh > "$comp_dir/_cmdrun" 2>/dev/null || true
            success "Zsh補完をインストール: $comp_dir/_cmdrun"
            ;;
        bash)
            comp_dir="$HOME/.local/share/bash-completion/completions"
            mkdir -p "$comp_dir"
            "$INSTALL_PREFIX/cmdrun" completion bash > "$comp_dir/cmdrun" 2>/dev/null || true
            success "Bash補完をインストール: $comp_dir/cmdrun"
            ;;
    esac
}

# メイン処理
main() {
    echo "🚀 cmdrun インストーラー"
    echo

    # バージョン解決
    resolve_version

    # ダウンロード
    local temp_file=$(download_binary)

    # インストール
    install_binary "$temp_file"

    # PATH確認
    check_path

    # シェル補完
    install_completions

    echo
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    success "インストール完了！"
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo
    echo "📋 次のステップ:"
    echo "  1. バージョン確認:"
    echo "     cmdrun --version"
    echo
    echo "  2. 初期設定ファイル作成:"
    echo "     cmdrun init"
    echo
    echo "  3. コマンド一覧表示:"
    echo "     cmdrun list"
    echo
    echo "📖 ドキュメント: https://github.com/$REPO"
}

# スクリプト実行
main "$@"
