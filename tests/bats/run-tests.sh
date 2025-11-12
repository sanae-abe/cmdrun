#!/usr/bin/env bash
#
# BATS Security Test Runner for cmdrun
#
# Prerequisites:
#   - BATS installed (brew install bats-core)
#   - cmdrun built in release mode (cargo build --release)
#
# Usage:
#   ./tests/bats/run-tests.sh
#

set -euo pipefail

# Color codes for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Project root directory
PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$PROJECT_ROOT"

echo -e "${YELLOW}=== cmdrun BATS Security Test Runner ===${NC}\n"

# Check if BATS is installed
if ! command -v bats &> /dev/null; then
    echo -e "${RED}Error: BATS is not installed${NC}"
    echo "Please install BATS using: brew install bats-core"
    exit 1
fi

# Check if cmdrun binary exists
if [ ! -f "./target/release/cmdrun" ]; then
    echo -e "${YELLOW}Warning: cmdrun binary not found at ./target/release/cmdrun${NC}"
    echo "Building cmdrun in release mode..."
    cargo build --release
    echo -e "${GREEN}Build completed${NC}\n"
fi

# Run BATS tests
echo -e "${YELLOW}Running security tests...${NC}\n"

if bats tests/bats/security.bats; then
    echo -e "\n${GREEN}✅ All security tests passed!${NC}"
    exit 0
else
    echo -e "\n${RED}❌ Some security tests failed${NC}"
    exit 1
fi
