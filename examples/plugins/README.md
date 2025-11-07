# cmdrun Plugin Examples

This directory contains example plugins demonstrating the cmdrun plugin system.

## Available Plugins

### 1. hello_plugin

A simple "Hello World" plugin that demonstrates basic plugin functionality.

**Features:**
- Pre-execution logging
- Post-execution statistics
- Error handling
- Custom greeting configuration

**Build:**
```bash
cd hello_plugin
cargo build --release
```

**Usage:**
```toml
# commands.toml
[plugins.hello]
path = "examples/plugins/hello_plugin/target/release/libhello_plugin.so"
greeting = "Hello from my cmdrun setup!"

[commands.test]
cmd = "echo 'Testing plugin system'"
```

### 2. logger_plugin

An advanced logging plugin that writes execution logs to JSON files.

**Features:**
- JSON-formatted logging
- Execution statistics tracking
- Configurable log file path
- Log level control
- Result metadata injection

**Build:**
```bash
cd logger_plugin
cargo build --release
```

**Usage:**
```toml
# commands.toml
[plugins.logger]
path = "examples/plugins/logger_plugin/target/release/liblogger_plugin.so"
log_file = "cmdrun.log"
level = "info"

[commands.build]
cmd = "cargo build"
```

**Log Output:**
```json
{"timestamp":"2025-11-07T10:30:00Z","command_name":"build","exit_code":0,"duration_ms":1234,"working_dir":"."}
```

## Building All Plugins

Use the provided build script:

```bash
./build_plugins.sh
```

Or build individually:

```bash
# Build hello_plugin
cd hello_plugin && cargo build --release && cd ..

# Build logger_plugin
cd logger_plugin && cargo build --release && cd ..
```

## Testing Plugins

### Quick Test

```bash
# Create test config
cat > test_commands.toml << 'EOF'
[plugins]
enabled = ["hello", "logger"]

[plugins.hello]
path = "examples/plugins/hello_plugin/target/release/libhello_plugin.so"

[plugins.logger]
path = "examples/plugins/logger_plugin/target/release/liblogger_plugin.so"
log_file = "test.log"

[commands.test]
cmd = "echo 'Plugin test'"
EOF

# Run with plugins
cargo run --features plugin-system -- -c test_commands.toml test

# Check log output
cat test.log
```

### Integration Test

```bash
# Run cmdrun tests with plugin support
cargo test --features plugin-system
```

## Plugin Library Locations

After building, plugins will be located at:

**Linux:**
- `hello_plugin/target/release/libhello_plugin.so`
- `logger_plugin/target/release/liblogger_plugin.so`

**macOS:**
- `hello_plugin/target/release/libhello_plugin.dylib`
- `logger_plugin/target/release/liblogger_plugin.dylib`

**Windows:**
- `hello_plugin/target/release/hello_plugin.dll`
- `logger_plugin/target/release/logger_plugin.dll`

## Creating Your Own Plugin

See the [Plugin Development Guide](../../docs/plugins/DEVELOPMENT_GUIDE.md) for detailed instructions.

Quick start:

```bash
# Create new plugin
cargo new --lib my_plugin
cd my_plugin

# Configure Cargo.toml
cat > Cargo.toml << 'EOF'
[package]
name = "my_plugin"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[dependencies]
cmdrun = { path = "../../../", features = ["plugin-system"] }
ahash = "0.8"
EOF

# Implement plugin in src/lib.rs
# See hello_plugin for example

# Build
cargo build --release
```

## Troubleshooting

### Plugin Not Loading

1. **Check file path**: Ensure the path in `commands.toml` is correct
2. **Verify build**: Make sure `cargo build --release` succeeded
3. **Check extension**: Use correct extension for your platform (.so/.dylib/.dll)
4. **Enable feature**: cmdrun must be built with `--features plugin-system`

### Runtime Errors

1. **Enable debug logging**:
   ```bash
   RUST_LOG=debug cargo run --features plugin-system -- test
   ```

2. **Check plugin logs**: Plugins may print error messages

3. **Verify configuration**: Ensure all required config keys are present

### Build Errors

1. **Update dependencies**:
   ```bash
   cargo update
   ```

2. **Clean build**:
   ```bash
   cargo clean
   cargo build --release
   ```

3. **Check Rust version**: Requires Rust 1.75+

## Performance

Both example plugins are optimized for minimal overhead:

- **hello_plugin**: ~1μs per hook call
- **logger_plugin**: ~100μs per hook call (includes file I/O)

Enable LTO and optimization in release builds for best performance.

## Security

**Important**: Only load plugins from trusted sources. Plugins have full access to cmdrun's runtime environment and can execute arbitrary code.

## Documentation

- [Plugin API Specification](../../docs/plugins/API.md)
- [Development Guide](../../docs/plugins/DEVELOPMENT_GUIDE.md)
- [cmdrun Documentation](../../docs/README.md)

## License

These example plugins are released under the MIT License, same as cmdrun.
