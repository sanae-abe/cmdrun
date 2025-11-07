#!/bin/bash
# Build all example plugins

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

echo "Building cmdrun example plugins..."
echo ""

# Color output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Build hello_plugin
echo -e "${BLUE}Building hello_plugin...${NC}"
cd hello_plugin
cargo build --release
echo -e "${GREEN}✓ hello_plugin built successfully${NC}"
echo ""
cd ..

# Build logger_plugin
echo -e "${BLUE}Building logger_plugin...${NC}"
cd logger_plugin
cargo build --release
echo -e "${GREEN}✓ logger_plugin built successfully${NC}"
echo ""
cd ..

# Print summary
echo -e "${GREEN}All plugins built successfully!${NC}"
echo ""
echo "Plugin locations:"

# Detect OS and show appropriate paths
if [[ "$OSTYPE" == "linux-gnu"* ]]; then
    EXT="so"
elif [[ "$OSTYPE" == "darwin"* ]]; then
    EXT="dylib"
elif [[ "$OSTYPE" == "msys" || "$OSTYPE" == "cygwin" ]]; then
    EXT="dll"
else
    EXT="so"
fi

echo "  - hello_plugin/target/release/libhello_plugin.$EXT"
echo "  - logger_plugin/target/release/liblogger_plugin.$EXT"
echo ""

echo "Example configuration:"
cat << EOF
[plugins]
enabled = ["hello", "logger"]

[plugins.hello]
path = "examples/plugins/hello_plugin/target/release/libhello_plugin.$EXT"

[plugins.logger]
path = "examples/plugins/logger_plugin/target/release/liblogger_plugin.$EXT"
log_file = "cmdrun.log"
level = "info"
EOF
