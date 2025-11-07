#!/usr/bin/env bash
# cmdrun Performance Profiling Script
# Usage: ./scripts/profile.sh [startup|memory|flamegraph|all]

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Directories
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
PROFILE_DIR="$PROJECT_ROOT/target/profiling"
RESULTS_DIR="$PROFILE_DIR/results"

# Create directories
mkdir -p "$RESULTS_DIR"

# Timestamp for report files
TIMESTAMP=$(date +%Y%m%d_%H%M%S)

# Helper functions
print_header() {
    echo -e "${BLUE}========================================${NC}"
    echo -e "${BLUE}$1${NC}"
    echo -e "${BLUE}========================================${NC}"
}

print_success() {
    echo -e "${GREEN}✓ $1${NC}"
}

print_warning() {
    echo -e "${YELLOW}⚠ $1${NC}"
}

print_error() {
    echo -e "${RED}✗ $1${NC}"
}

check_tool() {
    local tool=$1
    local install_cmd=$2

    if ! command -v "$tool" &> /dev/null; then
        print_warning "$tool not found"
        if [ -n "$install_cmd" ]; then
            echo "  Install: $install_cmd"
        fi
        return 1
    fi
    return 0
}

# Build release binary with debug symbols
build_profiling_binary() {
    print_header "Building profiling binary"

    cd "$PROJECT_ROOT"

    # Build release-with-debug profile
    cargo build --profile release-with-debug --quiet

    local binary="$PROJECT_ROOT/target/release-with-debug/cmdrun"
    if [ -f "$binary" ]; then
        print_success "Built: $binary"
        echo "  Size: $(du -h "$binary" | cut -f1)"
    else
        print_error "Failed to build binary"
        exit 1
    fi
}

# Profile startup time using hyperfine
profile_startup() {
    print_header "Profiling Startup Time"

    if ! check_tool "hyperfine" "cargo install hyperfine"; then
        print_error "hyperfine is required for startup profiling"
        return 1
    fi

    local binary="$PROJECT_ROOT/target/release-with-debug/cmdrun"
    local output="$RESULTS_DIR/startup_${TIMESTAMP}.txt"

    print_warning "Running startup benchmarks (this may take 1-2 minutes)..."

    # Benchmark basic commands
    hyperfine \
        --warmup 3 \
        --min-runs 100 \
        --export-markdown "$RESULTS_DIR/startup_${TIMESTAMP}.md" \
        --export-json "$RESULTS_DIR/startup_${TIMESTAMP}.json" \
        "$binary --version" \
        "$binary --help" \
        "$binary list --config examples/basic.toml" \
        > "$output" 2>&1

    print_success "Startup profile saved to:"
    echo "  - $output"
    echo "  - $RESULTS_DIR/startup_${TIMESTAMP}.md"
    echo "  - $RESULTS_DIR/startup_${TIMESTAMP}.json"

    # Display summary
    echo ""
    print_header "Startup Time Summary"
    grep "Time (mean" "$output" | head -n 3
}

# Profile memory usage
profile_memory() {
    print_header "Profiling Memory Usage"

    local binary="$PROJECT_ROOT/target/release-with-debug/cmdrun"
    local output="$RESULTS_DIR/memory_${TIMESTAMP}.txt"

    # Check platform
    if [[ "$OSTYPE" == "darwin"* ]]; then
        print_warning "Memory profiling on macOS"

        # Use /usr/bin/time -l (macOS specific)
        echo "Testing memory usage with various commands..." > "$output"
        echo "" >> "$output"

        echo "1. cmdrun --version" >> "$output"
        /usr/bin/time -l "$binary" --version 2>&1 | grep -E "maximum resident|real|user|sys" >> "$output"
        echo "" >> "$output"

        echo "2. cmdrun list (examples/basic.toml)" >> "$output"
        /usr/bin/time -l "$binary" list --config examples/basic.toml 2>&1 | grep -E "maximum resident|real|user|sys" >> "$output"
        echo "" >> "$output"

        print_success "Memory profile saved to: $output"

        # Display summary
        echo ""
        print_header "Memory Usage Summary"
        grep "maximum resident" "$output" | while read -r line; do
            # Convert bytes to MB
            bytes=$(echo "$line" | awk '{print $1}')
            mb=$(echo "scale=2; $bytes / 1024 / 1024" | bc)
            echo "  $mb MB - $(echo "$line" | cut -d':' -f2-)"
        done

    elif [[ "$OSTYPE" == "linux-gnu"* ]]; then
        print_warning "Memory profiling on Linux"

        # Use /usr/bin/time -v (Linux specific)
        if [ -f /usr/bin/time ]; then
            echo "Testing memory usage with various commands..." > "$output"
            echo "" >> "$output"

            echo "1. cmdrun --version" >> "$output"
            /usr/bin/time -v "$binary" --version 2>&1 | grep -E "Maximum resident|User time|System time|Elapsed" >> "$output"
            echo "" >> "$output"

            echo "2. cmdrun list (examples/basic.toml)" >> "$output"
            /usr/bin/time -v "$binary" list --config examples/basic.toml 2>&1 | grep -E "Maximum resident|User time|System time|Elapsed" >> "$output"
            echo "" >> "$output"

            print_success "Memory profile saved to: $output"

            # Display summary
            echo ""
            print_header "Memory Usage Summary"
            grep "Maximum resident" "$output"
        else
            print_error "/usr/bin/time not found"
            return 1
        fi

        # Check for valgrind
        if check_tool "valgrind" "sudo apt install valgrind"; then
            echo ""
            print_warning "Running valgrind (this may take a few minutes)..."
            valgrind --tool=massif --massif-out-file="$RESULTS_DIR/massif_${TIMESTAMP}.out" \
                "$binary" --version 2>&1 | head -n 20
            print_success "Valgrind massif output: $RESULTS_DIR/massif_${TIMESTAMP}.out"
            echo "  View with: ms_print $RESULTS_DIR/massif_${TIMESTAMP}.out"
        fi

        # Check for heaptrack
        if check_tool "heaptrack" "sudo apt install heaptrack"; then
            echo ""
            print_warning "Running heaptrack..."
            heaptrack "$binary" --version
            print_success "Heaptrack results saved in current directory"
        fi
    else
        print_error "Unsupported OS: $OSTYPE"
        return 1
    fi
}

# Generate flamegraph
profile_flamegraph() {
    print_header "Generating Flamegraph"

    if ! check_tool "cargo-flamegraph" "cargo install flamegraph"; then
        print_error "cargo-flamegraph is required"
        echo ""
        echo "Install with:"
        echo "  cargo install flamegraph"
        echo ""
        echo "On Linux, you may also need:"
        echo "  sudo apt install linux-perf"
        echo ""
        echo "On macOS, you'll need to run with sudo:"
        echo "  sudo -E cargo flamegraph -- --version"
        return 1
    fi

    cd "$PROJECT_ROOT"

    local output="$RESULTS_DIR/flamegraph_${TIMESTAMP}.svg"

    print_warning "Generating flamegraph (may require sudo)..."

    # Try to generate flamegraph
    if [[ "$OSTYPE" == "darwin"* ]]; then
        # macOS requires sudo
        print_warning "macOS detected - flamegraph requires sudo"
        echo "Please enter your password if prompted..."

        sudo -E cargo flamegraph \
            --output="$output" \
            --profile release-with-debug \
            -- list --config examples/basic.toml || {
            print_error "Flamegraph generation failed"
            return 1
        }
    else
        # Linux
        cargo flamegraph \
            --output="$output" \
            --profile release-with-debug \
            -- list --config examples/basic.toml || {
            print_error "Flamegraph generation failed (may need sudo)"
            return 1
        }
    fi

    print_success "Flamegraph saved to: $output"
    echo "  Open with your browser to view"
}

# Run all profiling
profile_all() {
    build_profiling_binary
    echo ""
    profile_startup
    echo ""
    profile_memory
    echo ""

    # Flamegraph is optional (requires sudo)
    read -p "Generate flamegraph? (requires sudo) [y/N]: " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        profile_flamegraph
    else
        print_warning "Skipping flamegraph generation"
    fi
}

# Generate profiling report
generate_report() {
    print_header "Generating Profiling Report"

    local report="$RESULTS_DIR/report_${TIMESTAMP}.md"

    cat > "$report" <<EOF
# cmdrun Performance Profiling Report

**Date:** $(date +"%Y-%m-%d %H:%M:%S")
**Version:** $(cd "$PROJECT_ROOT" && cargo metadata --no-deps --format-version 1 | grep -o '"version":"[^"]*"' | head -n1 | cut -d'"' -f4)
**Platform:** $(uname -s) $(uname -m)
**OS:** $(uname -r)

## Startup Time

EOF

    # Add startup results if available
    local latest_startup=$(ls -t "$RESULTS_DIR"/startup_*.md 2>/dev/null | head -n1)
    if [ -f "$latest_startup" ]; then
        cat "$latest_startup" >> "$report"
    else
        echo "*No startup profiling data available*" >> "$report"
    fi

    cat >> "$report" <<EOF

## Memory Usage

EOF

    # Add memory results if available
    local latest_memory=$(ls -t "$RESULTS_DIR"/memory_*.txt 2>/dev/null | head -n1)
    if [ -f "$latest_memory" ]; then
        echo '```' >> "$report"
        cat "$latest_memory" >> "$report"
        echo '```' >> "$report"
    else
        echo "*No memory profiling data available*" >> "$report"
    fi

    cat >> "$report" <<EOF

## Performance Targets

| Metric | Target | Current | Status |
|--------|--------|---------|--------|
| Startup Time | < 5ms | See above | - |
| Memory (Idle) | < 15MB | See above | - |
| Binary Size | < 5MB | $(du -h "$PROJECT_ROOT/target/release-with-debug/cmdrun" 2>/dev/null | cut -f1 || echo "N/A") | - |

## Analysis

- **Startup Performance:**
- **Memory Efficiency:**
- **Recommendations:**

## Next Steps

- [ ] Review flamegraph for hotspots
- [ ] Compare with previous profiling results
- [ ] Identify optimization opportunities
- [ ] Run benchmarks (cargo bench)

---

*Generated by cmdrun profiling script*
EOF

    print_success "Report saved to: $report"
}

# Main script logic
main() {
    local mode="${1:-all}"

    print_header "cmdrun Performance Profiling"
    echo "Mode: $mode"
    echo "Project: $PROJECT_ROOT"
    echo "Results: $RESULTS_DIR"
    echo ""

    case "$mode" in
        startup)
            build_profiling_binary
            profile_startup
            ;;
        memory)
            build_profiling_binary
            profile_memory
            ;;
        flamegraph)
            build_profiling_binary
            profile_flamegraph
            ;;
        all)
            profile_all
            generate_report
            ;;
        *)
            echo "Usage: $0 [startup|memory|flamegraph|all]"
            echo ""
            echo "Modes:"
            echo "  startup    - Profile startup time using hyperfine"
            echo "  memory     - Profile memory usage"
            echo "  flamegraph - Generate flamegraph (requires sudo)"
            echo "  all        - Run all profiling (default)"
            exit 1
            ;;
    esac

    echo ""
    print_success "Profiling complete!"
    echo ""
    echo "Results directory: $RESULTS_DIR"
    echo ""
}

# Run main function
main "$@"
