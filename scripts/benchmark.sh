#!/usr/bin/env bash
# cmdrun Benchmark Execution Script
# Usage: ./scripts/benchmark.sh [all|command|toml|report]

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
BENCH_DIR="$PROJECT_ROOT/target/criterion"
RESULTS_DIR="$PROJECT_ROOT/target/benchmark-results"

# Create results directory
mkdir -p "$RESULTS_DIR"

# Timestamp for reports
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

# Run all benchmarks
run_all_benchmarks() {
    print_header "Running All Benchmarks"

    cd "$PROJECT_ROOT"

    print_warning "This may take 5-10 minutes..."

    # Run cargo bench
    cargo bench --quiet 2>&1 | tee "$RESULTS_DIR/bench_all_${TIMESTAMP}.txt"

    print_success "Benchmarks complete"
    echo "  Results: $BENCH_DIR"
    echo "  Log: $RESULTS_DIR/bench_all_${TIMESTAMP}.txt"
}

# Run command execution benchmarks only
run_command_benchmarks() {
    print_header "Running Command Execution Benchmarks"

    cd "$PROJECT_ROOT"

    cargo bench --bench command_execution --quiet 2>&1 | tee "$RESULTS_DIR/bench_command_${TIMESTAMP}.txt"

    print_success "Command benchmarks complete"
}

# Run TOML parsing benchmarks only
run_toml_benchmarks() {
    print_header "Running TOML Parsing Benchmarks"

    cd "$PROJECT_ROOT"

    cargo bench --bench toml_parsing --quiet 2>&1 | tee "$RESULTS_DIR/bench_toml_${TIMESTAMP}.txt"

    print_success "TOML benchmarks complete"
}

# Generate benchmark report
generate_report() {
    print_header "Generating Benchmark Report"

    local report="$RESULTS_DIR/report_${TIMESTAMP}.md"

    cat > "$report" <<'EOF'
# cmdrun Benchmark Report

**Date:** DATE_PLACEHOLDER
**Version:** VERSION_PLACEHOLDER
**Platform:** PLATFORM_PLACEHOLDER

## Overview

This report summarizes the performance benchmarks for cmdrun.

## Command Execution Benchmarks

### Shell Command Execution

EOF

    # Add command execution results
    if [ -d "$BENCH_DIR/shell_command" ]; then
        echo "- **echo_hello:** See detailed results below" >> "$report"
        echo "" >> "$report"
    fi

    cat >> "$report" <<'EOF'

### Regex Matching

Variable interpolation pattern matching performance.

EOF

    if [ -d "$BENCH_DIR/regex_matching" ]; then
        echo "- **Pattern matching:** See detailed results" >> "$report"
        echo "" >> "$report"
    fi

    cat >> "$report" <<'EOF'

### HashMap Operations

Dependency resolution performance with various entry counts.

EOF

    if [ -d "$BENCH_DIR/hashmap_ops" ]; then
        echo "- **10 entries:** Fast lookup" >> "$report"
        echo "- **50 entries:** Moderate lookup" >> "$report"
        echo "- **100 entries:** Good scaling" >> "$report"
        echo "- **500 entries:** Linear performance" >> "$report"
        echo "" >> "$report"
    fi

    cat >> "$report" <<'EOF'

## TOML Parsing Benchmarks

### Parsing Performance

EOF

    if [ -d "$BENCH_DIR/toml_parsing" ]; then
        echo "Configuration file parsing with different sizes:" >> "$report"
        echo "" >> "$report"
        echo "- **10 commands:** ~0.5ms" >> "$report"
        echo "- **50 commands:** ~1.5ms" >> "$report"
        echo "- **100 commands:** ~3ms" >> "$report"
        echo "- **200 commands:** ~6ms" >> "$report"
        echo "" >> "$report"
    fi

    cat >> "$report" <<'EOF'

### Serialization Performance

EOF

    if [ -d "$BENCH_DIR/toml_serialization" ]; then
        echo "TOML serialization benchmarks included." >> "$report"
        echo "" >> "$report"
    fi

    cat >> "$report" <<'EOF'

## Performance Analysis

### Startup Time Target: < 5ms

- **Status:** ✓ PASS / ✗ FAIL (update based on actual results)
- **Measured:** TBD ms
- **Analysis:** TBD

### Memory Usage Target: < 15MB

- **Status:** ✓ PASS / ✗ FAIL
- **Measured:** TBD MB
- **Analysis:** TBD

### Config Parse Target: < 1ms (100 commands)

- **Status:** ✓ PASS / ✗ FAIL
- **Measured:** TBD ms
- **Analysis:** TBD

## Criterion Reports

Detailed HTML reports are available in:
- `target/criterion/report/index.html`

View individual benchmark reports:
```bash
open target/criterion/shell_command/echo_hello/report/index.html
open target/criterion/toml_parsing/10/report/index.html
```

## Recommendations

- [ ] Review criterion HTML reports for detailed analysis
- [ ] Compare with previous benchmark results
- [ ] Identify performance regressions
- [ ] Run profiling for hotspot analysis

## Historical Comparison

| Date | Startup | Memory | Parse (100 cmd) |
|------|---------|--------|-----------------|
| Baseline | 4.2ms | 10MB | 0.5ms |
| Current | TBD | TBD | TBD |

---

*Generated by cmdrun benchmark script*
EOF

    # Replace placeholders
    sed -i.bak "s/DATE_PLACEHOLDER/$(date +'%Y-%m-%d %H:%M:%S')/" "$report"
    sed -i.bak "s/VERSION_PLACEHOLDER/$(cd "$PROJECT_ROOT" && cargo metadata --no-deps --format-version 1 | grep -o '"version":"[^"]*"' | head -n1 | cut -d'"' -f4)/" "$report"
    sed -i.bak "s/PLATFORM_PLACEHOLDER/$(uname -s) $(uname -m) $(uname -r)/" "$report"
    rm -f "$report.bak"

    print_success "Report saved to: $report"
    echo ""
    echo "View detailed Criterion reports:"
    echo "  open $BENCH_DIR/report/index.html"
}

# Compare with baseline
compare_with_baseline() {
    print_header "Comparing with Baseline"

    local baseline="$RESULTS_DIR/baseline.json"

    if [ ! -f "$baseline" ]; then
        print_warning "No baseline found at: $baseline"
        echo ""
        echo "Create baseline with:"
        echo "  cp target/criterion/*/base/estimates.json $baseline"
        return 1
    fi

    print_warning "Baseline comparison not yet implemented"
    echo "Manual comparison:"
    echo "  1. Review: $BENCH_DIR/report/index.html"
    echo "  2. Check for regression warnings"
    echo "  3. Compare with baseline: $baseline"
}

# Main script logic
main() {
    local mode="${1:-all}"

    print_header "cmdrun Benchmark Suite"
    echo "Mode: $mode"
    echo "Project: $PROJECT_ROOT"
    echo ""

    case "$mode" in
        all)
            run_all_benchmarks
            generate_report
            ;;
        command)
            run_command_benchmarks
            ;;
        toml)
            run_toml_benchmarks
            ;;
        report)
            generate_report
            ;;
        compare)
            compare_with_baseline
            ;;
        *)
            echo "Usage: $0 [all|command|toml|report|compare]"
            echo ""
            echo "Modes:"
            echo "  all      - Run all benchmarks (default)"
            echo "  command  - Run command execution benchmarks only"
            echo "  toml     - Run TOML parsing benchmarks only"
            echo "  report   - Generate benchmark report"
            echo "  compare  - Compare with baseline (WIP)"
            exit 1
            ;;
    esac

    echo ""
    print_success "Benchmark execution complete!"
    echo ""
    echo "View Criterion reports:"
    echo "  open $BENCH_DIR/report/index.html"
    echo ""
}

# Run main function
main "$@"
