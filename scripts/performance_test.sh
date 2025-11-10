#!/bin/bash

# Performance testing script for cmdrun
# Measures startup time, memory usage, and binary size

set -euo pipefail

# Configuration
BINARY_PATH="${1:-./target/release/cmdrun}"
ITERATIONS=10
WARMUP_ITERATIONS=3

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Helper functions
log_info() {
    echo -e "${BLUE}ℹ️  $1${NC}"
}

log_success() {
    echo -e "${GREEN}✅ $1${NC}"
}

log_warning() {
    echo -e "${YELLOW}⚠️  $1${NC}"
}

log_error() {
    echo -e "${RED}❌ $1${NC}"
}

# Check if binary exists
if [[ ! -f "$BINARY_PATH" ]]; then
    log_error "Binary not found: $BINARY_PATH"
    log_info "Build it first: cargo build --release"
    exit 1
fi

log_info "Testing performance of: $BINARY_PATH"
log_info "Binary version: $($BINARY_PATH --version)"

# Test 1: Binary Size
log_info "📏 Measuring binary size..."
BINARY_SIZE=$(stat -c%s "$BINARY_PATH" 2>/dev/null || stat -f%z "$BINARY_PATH")
BINARY_SIZE_MB=$(echo "scale=2; $BINARY_SIZE / 1024 / 1024" | bc)
echo "Binary size: ${BINARY_SIZE} bytes (${BINARY_SIZE_MB} MB)"

TARGET_SIZE=5242880  # 5MB
if [ "$BINARY_SIZE" -le $TARGET_SIZE ]; then
    log_success "Binary size within target (≤ 5MB)"
else
    log_warning "Binary size exceeds target of 5MB"
fi

# Test 2: Startup Time
log_info "⏱️  Measuring startup time..."

# Warm up
log_info "Warming up ($WARMUP_ITERATIONS iterations)..."
for i in $(seq 1 $WARMUP_ITERATIONS); do
    "$BINARY_PATH" --version > /dev/null 2>&1
done

# Measure startup time for --version
log_info "Measuring --version startup time ($ITERATIONS iterations)..."
TOTAL_TIME_NS=0
MIN_TIME_NS=999999999999
MAX_TIME_NS=0

for i in $(seq 1 $ITERATIONS); do
    START_NS=$(date +%s%N)
    "$BINARY_PATH" --version > /dev/null 2>&1
    END_NS=$(date +%s%N)

    TIME_NS=$((END_NS - START_NS))
    TOTAL_TIME_NS=$((TOTAL_TIME_NS + TIME_NS))

    if [ $TIME_NS -lt $MIN_TIME_NS ]; then
        MIN_TIME_NS=$TIME_NS
    fi

    if [ $TIME_NS -gt $MAX_TIME_NS ]; then
        MAX_TIME_NS=$TIME_NS
    fi

    TIME_MS=$(echo "scale=2; $TIME_NS / 1000000" | bc)
    echo "Run $i: ${TIME_MS}ms"
done

# Calculate statistics
AVG_TIME_NS=$((TOTAL_TIME_NS / ITERATIONS))
AVG_TIME_MS=$(echo "scale=2; $AVG_TIME_NS / 1000000" | bc)
MIN_TIME_MS=$(echo "scale=2; $MIN_TIME_NS / 1000000" | bc)
MAX_TIME_MS=$(echo "scale=2; $MAX_TIME_NS / 1000000" | bc)

echo
echo "Startup time statistics:"
echo "  Average: ${AVG_TIME_MS}ms"
echo "  Minimum: ${MIN_TIME_MS}ms"
echo "  Maximum: ${MAX_TIME_MS}ms"

TARGET_MS="4.0"
if (( $(echo "$AVG_TIME_MS <= $TARGET_MS" | bc -l) )); then
    log_success "Average startup time within target (≤ ${TARGET_MS}ms)"
else
    log_warning "Average startup time exceeds target of ${TARGET_MS}ms"
fi

# Test 3: Memory Usage
log_info "💾 Measuring memory usage..."

# Create a test config file
TEST_CONFIG="/tmp/cmdrun_test_config.toml"
cat > "$TEST_CONFIG" << 'EOF'
[commands.test]
description = "Test command"
cmd = "echo hello"

[commands.build]
description = "Build command"
cmd = "cargo check"
EOF

# Measure memory usage (requires GNU time)
if command -v /usr/bin/time >/dev/null 2>&1; then
    MEMORY_KB=$((/usr/bin/time -f "%M" "$BINARY_PATH" --config "$TEST_CONFIG" list > /dev/null) 2>&1 | tail -1)
    MEMORY_MB=$(echo "scale=2; $MEMORY_KB / 1024" | bc)
    echo "Peak memory usage: ${MEMORY_MB}MB"

    TARGET_MEMORY_MB="10.0"
    if (( $(echo "$MEMORY_MB <= $TARGET_MEMORY_MB" | bc -l) )); then
        log_success "Memory usage within target (≤ ${TARGET_MEMORY_MB}MB)"
    else
        log_warning "Memory usage exceeds target of ${TARGET_MEMORY_MB}MB"
    fi
else
    log_warning "GNU time not available, skipping memory measurement"
fi

# Clean up
rm -f "$TEST_CONFIG"

# Test 4: Config Loading Performance
log_info "📄 Testing config loading performance..."

# Create configs of different sizes
for NUM_COMMANDS in 10 50 100; do
    CONFIG_FILE="/tmp/cmdrun_config_${NUM_COMMANDS}.toml"

    # Generate config
    echo "[config]" > "$CONFIG_FILE"
    for i in $(seq 0 $((NUM_COMMANDS - 1))); do
        cat >> "$CONFIG_FILE" << EOF

[commands.cmd$i]
description = "Command $i"
cmd = "echo 'Command $i'"
EOF
    done

    # Measure loading time
    START_NS=$(date +%s%N)
    "$BINARY_PATH" --config "$CONFIG_FILE" list > /dev/null 2>&1
    END_NS=$(date +%s%N)

    TIME_NS=$((END_NS - START_NS))
    TIME_MS=$(echo "scale=2; $TIME_NS / 1000000" | bc)

    echo "Config with $NUM_COMMANDS commands: ${TIME_MS}ms"

    # Clean up
    rm -f "$CONFIG_FILE"
done

# Summary
echo
echo "🎯 Performance Summary:"
echo "  Binary size: ${BINARY_SIZE_MB}MB (target: ≤ 5MB)"
echo "  Startup time: ${AVG_TIME_MS}ms (target: ≤ 4ms)"
if command -v /usr/bin/time >/dev/null 2>&1; then
    echo "  Memory usage: ${MEMORY_MB}MB (target: ≤ 10MB)"
fi

# Overall status
ALL_PASSED=true

if [ "$BINARY_SIZE" -gt 5242880 ]; then
    ALL_PASSED=false
fi

if ! (( $(echo "$AVG_TIME_MS <= 4.0" | bc -l) )); then
    ALL_PASSED=false
fi

if command -v /usr/bin/time >/dev/null 2>&1; then
    if ! (( $(echo "$MEMORY_MB <= 10.0" | bc -l) )); then
        ALL_PASSED=false
    fi
fi

echo
if [ "$ALL_PASSED" = true ]; then
    log_success "All performance targets met! 🎉"
    exit 0
else
    log_warning "Some performance targets were not met"
    exit 1
fi