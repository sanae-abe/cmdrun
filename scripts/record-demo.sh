#!/bin/bash
# cmdrun デモGIF作成スクリプト

set -e

DEMO_DIR="docs/demos"
CAST_FILE="$DEMO_DIR/cmdrun-demo.cast"
GIF_FILE="$DEMO_DIR/cmdrun-demo.gif"

mkdir -p "$DEMO_DIR"

echo "🎬 cmdrun デモ録画スクリプト"
echo ""
echo "使い方:"
echo "  1. このスクリプトを実行"
echo "  2. 録画が開始されたら、cmdrun コマンドを実演"
echo "  3. 'exit' で録画終了"
echo "  4. 自動的にGIFが生成されます"
echo ""
echo "📝 推奨デモ実演内容:"
echo "  cmdrun --help              # ヘルプ表示"
echo "  cmdrun list                # コマンド一覧"
echo "  cmdrun run dev             # dev コマンド実行"
echo "  cmdrun run test            # test コマンド実行"
echo ""
read -p "録画を開始しますか? (y/n) " -n 1 -r
echo

if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    echo "キャンセルしました"
    exit 0
fi

# 録画開始
echo "🔴 録画開始... (終了するには 'exit' を入力)"
asciinema rec "$CAST_FILE"

# GIF生成
echo ""
echo "🎨 GIF生成中..."
agg \
  --fps 15 \
  --speed 1.0 \
  --theme monokai \
  --font-size 14 \
  "$CAST_FILE" \
  "$GIF_FILE"

echo ""
echo "✅ デモGIF作成完了！"
echo "📁 保存場所: $GIF_FILE"
echo ""
echo "📊 ファイルサイズ:"
ls -lh "$GIF_FILE" | awk '{print $5}'
echo ""
echo "💡 次のステップ:"
echo "  README.mdに以下を追加:"
echo ""
echo "## Demo"
echo ""
echo "![cmdrun demo](./docs/demos/cmdrun-demo.gif)"
echo ""
echo "*Basic usage demonstration of cmdrun*"
