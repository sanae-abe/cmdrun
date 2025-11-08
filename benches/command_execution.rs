use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use std::time::Duration;

/// Benchmark simple shell command execution via Command
fn bench_simple_commands(c: &mut Criterion) {
    let mut group = c.benchmark_group("shell_command");
    group.measurement_time(Duration::from_secs(5));

    // Echo command (cross-platform)
    group.bench_function("echo_hello", |b| {
        b.iter(|| {
            let output = std::process::Command::new("echo")
                .arg(black_box("hello"))
                .output()
                .expect("Failed to execute");
            black_box(output)
        });
    });

    group.finish();
}

/// Benchmark variable interpolation regex performance
fn bench_regex_matching(c: &mut Criterion) {
    use regex::Regex;

    let var_pattern =
        Regex::new(r"\$\{([A-Za-z_][A-Za-z0-9_]*|[0-9]+)(:[?+\-])?([^}]*)?\}").unwrap();

    let mut group = c.benchmark_group("regex_matching");

    let test_cases = [
        "Hello, ${name}!",
        "${var1} and ${var2} and ${var3}",
        "Complex: ${VAR:-default} ${1} ${2}",
        "No variables here",
    ];

    for (idx, case) in test_cases.iter().enumerate() {
        group.bench_with_input(BenchmarkId::from_parameter(idx), case, |b, case| {
            b.iter(|| {
                let matches: Vec<_> = var_pattern.find_iter(black_box(case)).collect();
                black_box(matches)
            });
        });
    }

    group.finish();
}

/// Benchmark string replacements (simulated variable interpolation)
fn bench_string_replacement(c: &mut Criterion) {
    let mut group = c.benchmark_group("string_replacement");

    let template = "Hello, ${name}! Your score is ${score}.";
    let name_value = "World";
    let score_value = "100";

    group.bench_function("replace_two_vars", |b| {
        b.iter(|| {
            let result = template
                .replace(black_box("${name}"), black_box(name_value))
                .replace(black_box("${score}"), black_box(score_value));
            black_box(result)
        });
    });

    group.finish();
}

/// Benchmark HashMap operations (for dependency resolution)
fn bench_hashmap_operations(c: &mut Criterion) {
    use ahash::AHashMap;

    let mut group = c.benchmark_group("hashmap_ops");

    for num_entries in [10, 50, 100, 500].iter() {
        let mut map = AHashMap::new();
        for i in 0..*num_entries {
            map.insert(format!("cmd-{}", i), format!("echo {}", i));
        }

        group.throughput(Throughput::Elements(*num_entries as u64));
        group.bench_with_input(
            BenchmarkId::new("lookup", num_entries),
            num_entries,
            |b, _| {
                b.iter(|| {
                    for i in 0..*num_entries {
                        let _ = map.get(&format!("cmd-{}", i));
                    }
                });
            },
        );
    }

    group.finish();
}

/// Benchmark path operations
fn bench_path_operations(c: &mut Criterion) {
    use std::path::PathBuf;

    let mut group = c.benchmark_group("path_ops");

    group.bench_function("join_paths", |b| {
        b.iter(|| {
            let path = PathBuf::from(black_box("/home/user"))
                .join(black_box("project"))
                .join(black_box("src"))
                .join(black_box("main.rs"));
            black_box(path)
        });
    });

    group.bench_function("canonicalize_current", |b| {
        b.iter(|| {
            let current = std::env::current_dir().ok();
            black_box(current)
        });
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_simple_commands,
    bench_regex_matching,
    bench_string_replacement,
    bench_hashmap_operations,
    bench_path_operations
);
criterion_main!(benches);
