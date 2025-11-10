#!/usr/bin/env python3
"""
Benchmark comparison tool for cmdrun performance results.
Compares current benchmark results against historical data.
"""

import json
import argparse
import sys
from datetime import datetime
from pathlib import Path
from typing import Dict, Any, List, Optional

def load_benchmark_data(file_path: Path) -> Dict[str, Any]:
    """Load benchmark data from JSON file."""
    try:
        with open(file_path, 'r') as f:
            return json.load(f)
    except FileNotFoundError:
        print(f"❌ Benchmark file not found: {file_path}")
        sys.exit(1)
    except json.JSONDecodeError as e:
        print(f"❌ Invalid JSON in {file_path}: {e}")
        sys.exit(1)

def extract_performance_metrics(data: Dict[str, Any]) -> Dict[str, float]:
    """Extract key performance metrics from benchmark data."""
    metrics = {}

    # This is a simplified parser - adjust based on actual Criterion output format
    if 'benches' in data:
        for bench in data['benches']:
            name = bench.get('name', '')
            if 'startup_time' in name:
                # Extract startup time metrics
                if 'mean' in bench:
                    metrics['startup_time_ns'] = bench['mean']
                    metrics['startup_time_ms'] = bench['mean'] / 1_000_000

    # Add more metric extraction as needed
    return metrics

def compare_metrics(current: Dict[str, float], baseline: Dict[str, float]) -> Dict[str, Dict[str, Any]]:
    """Compare current metrics against baseline."""
    comparison = {}

    for metric, current_value in current.items():
        if metric in baseline:
            baseline_value = baseline[metric]
            if baseline_value > 0:
                change_percent = ((current_value - baseline_value) / baseline_value) * 100
                comparison[metric] = {
                    'current': current_value,
                    'baseline': baseline_value,
                    'change_percent': change_percent,
                    'improved': change_percent < 0,  # Lower is better for performance metrics
                    'regression': change_percent > 10  # More than 10% increase is a regression
                }

    return comparison

def format_duration(ns: float) -> str:
    """Format nanoseconds into human-readable duration."""
    if ns < 1000:
        return f"{ns:.2f}ns"
    elif ns < 1_000_000:
        return f"{ns/1000:.2f}μs"
    elif ns < 1_000_000_000:
        return f"{ns/1_000_000:.2f}ms"
    else:
        return f"{ns/1_000_000_000:.2f}s"

def generate_report(comparison: Dict[str, Dict[str, Any]], output_format: str = 'markdown') -> str:
    """Generate a formatted comparison report."""
    if output_format == 'markdown':
        return generate_markdown_report(comparison)
    elif output_format == 'json':
        return json.dumps(comparison, indent=2)
    else:
        return generate_text_report(comparison)

def generate_markdown_report(comparison: Dict[str, Dict[str, Any]]) -> str:
    """Generate a Markdown report."""
    report = [
        "# 📊 Performance Benchmark Comparison",
        f"",
        f"**Generated**: {datetime.now().strftime('%Y-%m-%d %H:%M:%S UTC')}",
        "",
        "## 📈 Performance Changes",
        "",
        "| Metric | Current | Baseline | Change | Status |",
        "|--------|---------|----------|--------|--------|"
    ]

    for metric, data in comparison.items():
        current = data['current']
        baseline = data['baseline']
        change_percent = data['change_percent']

        # Format values based on metric type
        if 'time' in metric:
            if 'ms' in metric:
                current_str = f"{current:.2f}ms"
                baseline_str = f"{baseline:.2f}ms"
            else:
                current_str = format_duration(current)
                baseline_str = format_duration(baseline)
        else:
            current_str = f"{current:.2f}"
            baseline_str = f"{baseline:.2f}"

        # Determine status
        if data['regression']:
            status = "🔴 Regression"
        elif data['improved']:
            status = "🟢 Improved"
        else:
            status = "🟡 Similar"

        change_str = f"{change_percent:+.1f}%"

        report.append(f"| {metric} | {current_str} | {baseline_str} | {change_str} | {status} |")

    # Add summary
    report.extend([
        "",
        "## 📋 Summary",
        ""
    ])

    regressions = [m for m, d in comparison.items() if d['regression']]
    improvements = [m for m, d in comparison.items() if d['improved']]

    if regressions:
        report.append(f"⚠️  **{len(regressions)} regression(s) detected:**")
        for metric in regressions:
            change = comparison[metric]['change_percent']
            report.append(f"- {metric}: {change:+.1f}%")
        report.append("")

    if improvements:
        report.append(f"🎉 **{len(improvements)} improvement(s):**")
        for metric in improvements:
            change = comparison[metric]['change_percent']
            report.append(f"- {metric}: {change:+.1f}%")
        report.append("")

    stable_metrics = len(comparison) - len(regressions) - len(improvements)
    if stable_metrics > 0:
        report.append(f"✅ **{stable_metrics} metric(s) stable** (< 10% change)")

    return "\n".join(report)

def generate_text_report(comparison: Dict[str, Dict[str, Any]]) -> str:
    """Generate a plain text report."""
    lines = [
        "Performance Benchmark Comparison",
        "=" * 35,
        f"Generated: {datetime.now().strftime('%Y-%m-%d %H:%M:%S UTC')}",
        ""
    ]

    for metric, data in comparison.items():
        current = data['current']
        baseline = data['baseline']
        change_percent = data['change_percent']

        status_icon = "🔴" if data['regression'] else "🟢" if data['improved'] else "🟡"

        lines.extend([
            f"{status_icon} {metric}:",
            f"  Current:  {current:.2f}",
            f"  Baseline: {baseline:.2f}",
            f"  Change:   {change_percent:+.1f}%",
            ""
        ])

    return "\n".join(lines)

def main():
    parser = argparse.ArgumentParser(description="Compare cmdrun benchmark results")
    parser.add_argument("current", type=Path, help="Current benchmark results (JSON)")
    parser.add_argument("baseline", type=Path, help="Baseline benchmark results (JSON)")
    parser.add_argument("--format", choices=['markdown', 'json', 'text'], default='markdown',
                        help="Output format (default: markdown)")
    parser.add_argument("--output", type=Path, help="Output file (default: stdout)")
    parser.add_argument("--fail-on-regression", action='store_true',
                        help="Exit with error code if regressions detected")

    args = parser.parse_args()

    # Load benchmark data
    current_data = load_benchmark_data(args.current)
    baseline_data = load_benchmark_data(args.baseline)

    # Extract metrics
    current_metrics = extract_performance_metrics(current_data)
    baseline_metrics = extract_performance_metrics(baseline_data)

    if not current_metrics:
        print("❌ No performance metrics found in current data")
        sys.exit(1)

    if not baseline_metrics:
        print("❌ No performance metrics found in baseline data")
        sys.exit(1)

    # Compare metrics
    comparison = compare_metrics(current_metrics, baseline_metrics)

    if not comparison:
        print("❌ No comparable metrics found")
        sys.exit(1)

    # Generate report
    report = generate_report(comparison, args.format)

    # Output report
    if args.output:
        with open(args.output, 'w') as f:
            f.write(report)
        print(f"📊 Report written to: {args.output}")
    else:
        print(report)

    # Check for regressions
    if args.fail_on_regression:
        regressions = [m for m, d in comparison.items() if d['regression']]
        if regressions:
            print(f"\n❌ {len(regressions)} performance regression(s) detected!")
            sys.exit(1)

    print("\n✅ Benchmark comparison complete")

if __name__ == "__main__":
    main()