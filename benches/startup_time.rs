use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use std::process::Command;
use std::time::{Duration, Instant};

/// Benchmark binary startup time for different commands
fn bench_startup_time(c: &mut Criterion) {
    let mut group = c.benchmark_group("startup_time");

    // Ensure we have a reasonable measurement time for startup tests
    group.measurement_time(Duration::from_secs(10));
    group.sample_size(100); // Smaller sample for process-based tests

    // Test --version flag (should be fastest)
    group.bench_function("version", |b| {
        b.iter_custom(|iters| {
            let start = Instant::now();
            for _ in 0..iters {
                let output = Command::new(env!("CARGO_BIN_EXE_cmdrun"))
                    .arg("--version")
                    .output()
                    .expect("Failed to execute cmdrun --version");
                black_box(output);
            }
            start.elapsed()
        });
    });

    // Test --help flag
    group.bench_function("help", |b| {
        b.iter_custom(|iters| {
            let start = Instant::now();
            for _ in 0..iters {
                let output = Command::new(env!("CARGO_BIN_EXE_cmdrun"))
                    .arg("--help")
                    .output()
                    .expect("Failed to execute cmdrun --help");
                black_box(output);
            }
            start.elapsed()
        });
    });

    // Test list command (requires minimal config parsing)
    group.bench_function("list_no_config", |b| {
        b.iter_custom(|iters| {
            let start = Instant::now();
            for _ in 0..iters {
                let output = Command::new(env!("CARGO_BIN_EXE_cmdrun"))
                    .arg("list")
                    .env("HOME", "/tmp") // Use temp home to avoid config files
                    .output()
                    .unwrap_or_else(|e| panic!("Failed to execute cmdrun list: {}", e));
                black_box(output);
            }
            start.elapsed()
        });
    });

    group.finish();
}

/// Benchmark cold vs warm startup (with config file)
fn bench_cold_warm_startup(c: &mut Criterion) {
    use std::io::Write;
    use tempfile::TempDir;

    let mut group = c.benchmark_group("cold_warm_startup");
    group.measurement_time(Duration::from_secs(8));
    group.sample_size(50);

    // Create a temporary config file
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let config_path = temp_dir.path().join("commands.toml");
    let mut config_file = std::fs::File::create(&config_path).expect("Failed to create config");

    writeln!(
        config_file,
        r#"
[commands.test]
description = "Test command"
cmd = "echo hello"

[commands.build]
description = "Build command"
cmd = "cargo check"
"#
    )
    .expect("Failed to write config");

    // Cold startup (first time loading config)
    group.bench_function("cold_startup", |b| {
        b.iter_custom(|iters| {
            let start = Instant::now();
            for _ in 0..iters {
                let output = Command::new(env!("CARGO_BIN_EXE_cmdrun"))
                    .arg("--config")
                    .arg(&config_path)
                    .arg("list")
                    .output()
                    .expect("Failed to execute cmdrun with config");
                black_box(output);
            }
            start.elapsed()
        });
    });

    group.finish();
}

/// Benchmark different binary sizes impact on startup
fn bench_memory_footprint(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_footprint");
    group.measurement_time(Duration::from_secs(5));

    // Measure baseline memory usage (version command)
    group.bench_function("minimal_memory", |b| {
        b.iter_custom(|iters| {
            let start = Instant::now();
            for _ in 0..iters {
                let output = Command::new(env!("CARGO_BIN_EXE_cmdrun"))
                    .arg("--version")
                    .output()
                    .expect("Failed to execute");
                black_box(output.stdout.len()); // Force memory usage
            }
            start.elapsed()
        });
    });

    group.finish();
}

/// Benchmark argument parsing overhead
fn bench_argument_parsing(c: &mut Criterion) {
    let mut group = c.benchmark_group("argument_parsing");

    let test_cases = vec![
        ("simple", vec!["list"]),
        ("with_flags", vec!["--color", "always", "list"]),
        (
            "complex",
            vec![
                "--config",
                "/tmp/test.toml",
                "--color",
                "never",
                "-v",
                "list",
            ],
        ),
    ];

    for (name, args) in test_cases {
        group.bench_with_input(BenchmarkId::from_parameter(name), &args, |b, args| {
            b.iter_custom(|iters| {
                let start = Instant::now();
                for _ in 0..iters {
                    let mut cmd = Command::new(env!("CARGO_BIN_EXE_cmdrun"));
                    for arg in args {
                        cmd.arg(arg);
                    }
                    let output = cmd.env("HOME", "/tmp").output().unwrap_or_else(|e| {
                        panic!("Failed to execute with args {:?}: {}", args, e)
                    });
                    black_box(output);
                }
                start.elapsed()
            });
        });
    }

    group.finish();
}

/// Benchmark TOML config loading performance
fn bench_config_loading(c: &mut Criterion) {
    use std::io::Write;
    use tempfile::TempDir;

    let mut group = c.benchmark_group("config_loading");

    // Create configs of different sizes
    for num_commands in [5, 20, 50, 100] {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let config_path = temp_dir.path().join("commands.toml");
        let mut config_file = std::fs::File::create(&config_path).expect("Failed to create config");

        // Generate config with specified number of commands
        writeln!(config_file, "[config]").expect("Failed to write config header");
        for i in 0..num_commands {
            writeln!(
                config_file,
                r#"
[commands.cmd{}]
description = "Command {}"
cmd = "echo 'Command {}'"
"#,
                i, i, i
            )
            .expect("Failed to write command config");
        }

        group.throughput(Throughput::Elements(num_commands as u64));
        group.bench_with_input(
            BenchmarkId::new("load_config", num_commands),
            &config_path,
            |b, config_path| {
                b.iter_custom(|iters| {
                    let start = Instant::now();
                    for _ in 0..iters {
                        let output = Command::new(env!("CARGO_BIN_EXE_cmdrun"))
                            .arg("--config")
                            .arg(config_path)
                            .arg("list")
                            .output()
                            .expect("Failed to execute with config");
                        black_box(output);
                    }
                    start.elapsed()
                });
            },
        );
    }

    group.finish();
}

criterion_group!(
    benches,
    bench_startup_time,
    bench_cold_warm_startup,
    bench_memory_footprint,
    bench_argument_parsing,
    bench_config_loading
);
criterion_main!(benches);
