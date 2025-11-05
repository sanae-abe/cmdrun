#!/bin/bash
# cmdrun移行スクリプト（Bash版cmd → Rust版cmdrun）
#
# 使用方法:
#   ./scripts/migrate-from-bash.sh [--dry-run] [--force]

set -euo pipefail

# 色定義
readonly RED='\033[0;31m'
readonly GREEN='\033[0;32m'
readonly YELLOW='\033[1;33m'
readonly BLUE='\033[1;34m'
readonly NC='\033[0m'

# パス定義
readonly BASH_CMD="$HOME/.local/bin/cmd"
readonly CMDRUN_BIN="$HOME/.local/bin/cmdrun"
readonly JSON_CONFIG="$HOME/Scripts/commands.json"
readonly TOML_CONFIG="$HOME/.cmdrun/commands.toml"
readonly BACKUP_DIR="$HOME/.config/cmdrun/backups/migration-$(date +%Y%m%d_%H%M%S)"

# フラグ
DRY_RUN=false
FORCE=false

# 引数パース
while [[ $# -gt 0 ]]; do
    case $1 in
        --dry-run)
            DRY_RUN=true
            shift
            ;;
        --force)
            FORCE=true
            shift
            ;;
        -h|--help)
            cat <<EOF
🔄 cmdrun 移行スクリプト（Bash → Rust）

使用方法:
  $0 [オプション]

オプション:
  --dry-run    実際の変更を行わず、実行内容のみ表示
  --force      確認なしで実行
  -h, --help   このヘルプを表示

実行内容:
  1. 既存データのバックアップ作成
  2. commands.json → commands.toml 変換
  3. cmdrunバイナリのインストール確認
  4. シェル補完の更新
  5. Bash版cmdの無効化（オプション）

EOF
            exit 0
            ;;
        *)
            echo -e "${RED}❌ 不明なオプション: $1${NC}"
            echo "ヘルプ: $0 --help"
            exit 1
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
}

# ドライランメッセージ
dry_run_msg() {
    if $DRY_RUN; then
        echo -e "${YELLOW}[DRY RUN] $1${NC}"
    else
        "$@"
    fi
}

# 確認プロンプト
confirm() {
    if $FORCE; then
        return 0
    fi

    local prompt="$1"
    read -p "$prompt [y/N]: " -n 1 -r
    echo
    [[ $REPLY =~ ^[Yy]$ ]]
}

# メイン処理
main() {
    echo "🔄 cmdrun 移行スクリプト（Bash → Rust）"
    echo

    # Step 1: バックアップ
    info "📦 既存データをバックアップ中..."

    if $DRY_RUN; then
        dry_run_msg "mkdir -p \"$BACKUP_DIR\""
        if [[ -f "$JSON_CONFIG" ]]; then
            dry_run_msg "cp \"$JSON_CONFIG\" \"$BACKUP_DIR/\""
        fi
        if [[ -f "$HOME/Scripts/cmd.log" ]]; then
            dry_run_msg "cp \"$HOME/Scripts/cmd.log\" \"$BACKUP_DIR/\""
        fi
    else
        mkdir -p "$BACKUP_DIR"

        if [[ -f "$JSON_CONFIG" ]]; then
            cp "$JSON_CONFIG" "$BACKUP_DIR/"
            success "commands.json をバックアップ"
        fi

        if [[ -f "$HOME/Scripts/cmd.log" ]]; then
            cp "$HOME/Scripts/cmd.log" "$BACKUP_DIR/" 2>/dev/null || true
            success "cmd.log をバックアップ"
        fi
    fi

    echo

    # Step 2: JSON → TOML変換
    info "🔧 設定ファイルを変換中（JSON → TOML）..."

    if [[ ! -f "$JSON_CONFIG" ]]; then
        warning "commands.json が見つかりません。スキップします。"
    elif [[ -f "$TOML_CONFIG" ]]; then
        warning "commands.toml は既に存在します。"

        if confirm "上書きしますか？"; then
            if $DRY_RUN; then
                dry_run_msg "python3で変換実行"
            else
                # 変換スクリプト実行（既に作成済み）
                if [[ -f "/tmp/robust-json-to-toml.py" ]]; then
                    python3 /tmp/robust-json-to-toml.py
                    success "TOML変換完了"
                else
                    error "変換スクリプトが見つかりません"
                    exit 1
                fi
            fi
        fi
    else
        if $DRY_RUN; then
            dry_run_msg "python3で変換実行"
        else
            mkdir -p "$HOME/.cmdrun"
            if [[ -f "/tmp/robust-json-to-toml.py" ]]; then
                python3 /tmp/robust-json-to-toml.py
                success "TOML変換完了"
            else
                error "変換スクリプトが見つかりません"
                exit 1
            fi
        fi
    fi

    echo

    # Step 3: cmdrunインストール確認
    info "🧪 cmdrunインストール確認..."

    if command -v cmdrun &> /dev/null; then
        success "cmdrun $(cmdrun --version 2>&1 | head -1) インストール済み"
    elif [[ -f "$CMDRUN_BIN" ]]; then
        success "cmdrun バイナリ存在: $CMDRUN_BIN"
    else
        error "cmdrun がインストールされていません"
        echo
        info "インストール方法:"
        echo "  1. リリースビルド: cd ~/projects/cmdrun && cargo build --release"
        echo "  2. インストール: cp target/release/cmdrun ~/.local/bin/"
        echo "  3. または: cargo install --path ~/projects/cmdrun"
        exit 1
    fi

    echo

    # Step 4: シェル補完更新
    info "🔧 シェル補完を更新中..."

    if $DRY_RUN; then
        dry_run_msg "mkdir -p ~/.zsh/completions"
        dry_run_msg "cmdrun completion zsh > ~/.zsh/completions/_cmdrun"
    else
        if command -v cmdrun &> /dev/null; then
            mkdir -p "$HOME/.zsh/completions"
            cmdrun completion zsh > "$HOME/.zsh/completions/_cmdrun" 2>/dev/null || true
            success "Zsh補完を更新"
        fi
    fi

    echo

    # Step 5: Bash版cmd無効化（オプション）
    if [[ -f "$BASH_CMD" ]]; then
        echo "📋 Bash版cmdの処理:"
        echo "  現在: $BASH_CMD"
        echo
        echo "オプション:"
        echo "  1. 無効化（リネーム）- 推奨"
        echo "  2. 削除"
        echo "  3. 保持（両方共存）"
        echo

        if $FORCE; then
            choice=1
        else
            read -p "選択 [1-3]: " choice
        fi

        case $choice in
            1)
                if $DRY_RUN; then
                    dry_run_msg "mv \"$BASH_CMD\" \"$BASH_CMD.backup\""
                else
                    mv "$BASH_CMD" "$BASH_CMD.backup"
                    success "Bash版cmdを無効化（.backup追加）"
                fi
                ;;
            2)
                if confirm "本当に削除しますか？"; then
                    if $DRY_RUN; then
                        dry_run_msg "rm \"$BASH_CMD\""
                    else
                        rm "$BASH_CMD"
                        success "Bash版cmdを削除"
                    fi
                fi
                ;;
            3)
                info "Bash版cmdを保持します"
                ;;
            *)
                warning "無効な選択。Bash版cmdを保持します"
                ;;
        esac
    fi

    echo
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    success "移行完了！"
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo
    echo "📋 次のステップ:"
    echo "  1. 新しいシェルを開くか、シェル設定を再読み込み:"
    echo "     source ~/.zshrc"
    echo
    echo "  2. cmdrun動作確認:"
    echo "     cmdrun --version"
    echo "     cmdrun list"
    echo
    echo "  3. バックアップ場所:"
    echo "     $BACKUP_DIR"
    echo
    echo "📖 詳細: https://github.com/yourusername/cmdrun/blob/main/docs/MIGRATION.md"
}

# スクリプト実行
main "$@"
