use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use std::io::Write;
use tempfile::NamedTempFile;

/// Create a sample TOML configuration with a specific number of commands
fn create_sample_config(num_commands: usize) -> String {
    let mut config = String::from("[commands]\n");

    for i in 0..num_commands {
        config.push_str(&format!(
            r#"
[commands.cmd-{}]
description = "Test command {}"
run = "echo 'Command {}'"
"#,
            i, i, i
        ));

        // Add some dependencies for complexity
        if i > 0 && i % 3 == 0 {
            config.push_str(&format!(r#"depends_on = ["cmd-{}"]{}"#, i - 1, "\n"));
        }
    }

    config
}

/// Benchmark TOML parsing with different file sizes
fn bench_toml_parsing(c: &mut Criterion) {
    let mut group = c.benchmark_group("toml_parsing");

    for num_commands in [10, 50, 100, 200].iter() {
        let config_str = create_sample_config(*num_commands);

        group.throughput(Throughput::Bytes(config_str.len() as u64));
        group.bench_with_input(
            BenchmarkId::from_parameter(num_commands),
            &config_str,
            |b, config_str| {
                b.iter(|| {
                    let result: Result<toml::Value, _> = toml::from_str(black_box(config_str));
                    black_box(result)
                });
            },
        );
    }

    group.finish();
}

/// Benchmark TOML serialization
fn bench_toml_serialization(c: &mut Criterion) {
    use toml::Value;

    let mut group = c.benchmark_group("toml_serialization");

    for num_commands in [10, 50, 100].iter() {
        let config_str = create_sample_config(*num_commands);
        let config: Value = toml::from_str(&config_str).unwrap();

        group.throughput(Throughput::Elements(*num_commands as u64));
        group.bench_with_input(
            BenchmarkId::from_parameter(num_commands),
            &config,
            |b, config| {
                b.iter(|| {
                    let result = toml::to_string(black_box(config));
                    black_box(result)
                });
            },
        );
    }

    group.finish();
}

/// Benchmark string operations (common in config processing)
fn bench_string_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("string_ops");

    let test_string = "cmd-test-long-name-with-many-parts";

    group.bench_function("split_and_collect", |b| {
        b.iter(|| {
            let parts: Vec<_> = black_box(test_string).split('-').collect();
            black_box(parts)
        });
    });

    group.bench_function("to_lowercase", |b| {
        b.iter(|| {
            let lower = black_box(test_string).to_lowercase();
            black_box(lower)
        });
    });

    group.bench_function("contains_check", |b| {
        b.iter(|| {
            let result = black_box(test_string).contains("test");
            black_box(result)
        });
    });

    group.finish();
}

/// Benchmark file I/O operations
fn bench_file_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("file_io");

    let config_content = create_sample_config(50);

    group.bench_function("write_temp_file", |b| {
        b.iter(|| {
            let mut temp_file = NamedTempFile::new().unwrap();
            temp_file
                .write_all(black_box(config_content.as_bytes()))
                .unwrap();
            temp_file.flush().unwrap();
            black_box(temp_file)
        });
    });

    group.finish();
}

/// Benchmark complex TOML structures
fn bench_complex_toml(c: &mut Criterion) {
    let mut group = c.benchmark_group("complex_toml");

    // Create complex config with nested structures
    let complex_config = r#"
[config]
shell = "bash"
strict_mode = true
parallel = false

[config.env]
RUST_LOG = "info"
BUILD_MODE = "release"

[commands.build]
description = "Build the project"
run = "cargo build --release"

[commands.test]
description = "Run tests"
run = "cargo test"
depends_on = ["build"]

[aliases]
b = "build"
t = "test"
"#;

    group.bench_function("parse_complex", |b| {
        b.iter(|| {
            let result: Result<toml::Value, _> = toml::from_str(black_box(complex_config));
            black_box(result)
        });
    });

    group.bench_function("serialize_complex", |b| {
        let config: toml::Value = toml::from_str(complex_config).unwrap();
        b.iter(|| {
            let result = toml::to_string(black_box(&config));
            black_box(result)
        });
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_toml_parsing,
    bench_toml_serialization,
    bench_string_operations,
    bench_file_operations,
    bench_complex_toml
);
criterion_main!(benches);
