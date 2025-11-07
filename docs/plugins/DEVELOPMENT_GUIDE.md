# Plugin Development Guide

## Getting Started

This guide will walk you through creating a cmdrun plugin from scratch.

## Prerequisites

- Rust 1.75 or later
- cmdrun installed or built from source
- Basic understanding of Rust traits and dynamic libraries

## Creating Your First Plugin

### Step 1: Project Setup

Create a new library project:

```bash
cargo new --lib my_cmdrun_plugin
cd my_cmdrun_plugin
```

### Step 2: Configure Cargo.toml

```toml
[package]
name = "my_cmdrun_plugin"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]  # Important: Build as dynamic library

[dependencies]
cmdrun = { version = "1.0", features = ["plugin-system"] }
ahash = "0.8"  # For HashMap compatibility

[profile.release]
opt-level = 3
lto = true
strip = true
```

### Step 3: Implement the Plugin Trait

Create `src/lib.rs`:

```rust
use ahash::AHashMap;
use cmdrun::plugin::api::{
    CommandResult, Plugin, PluginCapabilities, PluginContext, PluginMetadata,
};
use cmdrun::error::Result;
use std::any::Any;

#[derive(Default)]
pub struct MyPlugin {
    // Plugin state
}

impl Plugin for MyPlugin {
    fn metadata(&self) -> PluginMetadata {
        PluginMetadata {
            name: "my-plugin".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            description: "My awesome cmdrun plugin".to_string(),
            authors: vec!["Your Name <your@email.com>".to_string()],
            license: Some("MIT".to_string()),
            homepage: Some("https://github.com/yourusername/my-plugin".to_string()),
            min_cmdrun_version: Some("1.0.0".to_string()),
            capabilities: PluginCapabilities {
                pre_execute: true,
                post_execute: true,
                on_error: false,
                custom_commands: false,
                config_modification: false,
            },
        }
    }

    fn on_load(&mut self, config: &AHashMap<String, String>) -> Result<()> {
        println!("Plugin loaded with config: {:?}", config);
        Ok(())
    }

    fn pre_execute(&self, context: &mut PluginContext) -> Result<bool> {
        println!("About to execute: {}", context.command_name);
        Ok(true) // Continue execution
    }

    fn post_execute(
        &self,
        context: &PluginContext,
        result: &mut CommandResult,
    ) -> Result<()> {
        println!(
            "Command '{}' completed with exit code: {}",
            context.command_name, result.exit_code
        );
        Ok(())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

// Export the plugin
cmdrun::declare_plugin!(MyPlugin);
```

### Step 4: Build the Plugin

```bash
cargo build --release
```

The plugin will be at `target/release/libmy_cmdrun_plugin.so` (Linux),
`target/release/libmy_cmdrun_plugin.dylib` (macOS), or
`target/release/my_cmdrun_plugin.dll` (Windows).

### Step 5: Configure cmdrun

Create or edit `commands.toml`:

```toml
[plugins]
enabled = ["my-plugin"]

[plugins.my-plugin]
path = "target/release/libmy_cmdrun_plugin.so"
# Plugin-specific config
custom_option = "value"

[commands.test]
cmd = "echo 'Testing plugin'"
```

### Step 6: Test the Plugin

```bash
cmdrun test
```

You should see your plugin's messages in the output.

## Common Plugin Patterns

### 1. Logging Plugin

```rust
use chrono::Utc;
use std::fs::OpenOptions;
use std::io::Write;

pub struct LoggerPlugin {
    log_file: PathBuf,
}

impl Plugin for LoggerPlugin {
    fn pre_execute(&self, context: &mut PluginContext) -> Result<bool> {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.log_file)?;

        writeln!(
            file,
            "[{}] Executing: {}",
            Utc::now().to_rfc3339(),
            context.command_name
        )?;

        Ok(true)
    }

    // ... other methods
}
```

### 2. Notification Plugin

```rust
pub struct NotificationPlugin;

impl Plugin for NotificationPlugin {
    fn post_execute(
        &self,
        context: &PluginContext,
        result: &mut CommandResult,
    ) -> Result<()> {
        if result.duration_ms > 60000 {
            // Command took more than 1 minute
            send_notification(&format!(
                "Command '{}' completed after {}s",
                context.command_name,
                result.duration_ms / 1000
            ));
        }
        Ok(())
    }

    // ... other methods
}
```

### 3. Environment Modifier Plugin

```rust
pub struct EnvPlugin;

impl Plugin for EnvPlugin {
    fn pre_execute(&self, context: &mut PluginContext) -> Result<bool> {
        // Add environment variables
        context.env.insert(
            "PLUGIN_LOADED".to_string(),
            "true".to_string()
        );

        context.env.insert(
            "COMMAND_START_TIME".to_string(),
            Utc::now().timestamp().to_string()
        );

        Ok(true)
    }

    // ... other methods
}
```

### 4. Validation Plugin

```rust
pub struct ValidationPlugin {
    forbidden_commands: Vec<String>,
}

impl Plugin for ValidationPlugin {
    fn pre_execute(&self, context: &mut PluginContext) -> Result<bool> {
        if self.forbidden_commands.contains(&context.command_name) {
            println!("⛔ Command '{}' is forbidden", context.command_name);
            return Ok(false); // Skip execution
        }
        Ok(true)
    }

    // ... other methods
}
```

### 5. Metrics Plugin

```rust
use std::sync::atomic::{AtomicU64, Ordering};

pub struct MetricsPlugin {
    total_executions: AtomicU64,
    total_failures: AtomicU64,
}

impl Plugin for MetricsPlugin {
    fn post_execute(
        &self,
        _context: &PluginContext,
        result: &mut CommandResult,
    ) -> Result<()> {
        self.total_executions.fetch_add(1, Ordering::Relaxed);

        if result.exit_code != 0 {
            self.total_failures.fetch_add(1, Ordering::Relaxed);
        }

        // Add metrics to result metadata
        result.metadata.insert(
            "total_executions".to_string(),
            self.total_executions.load(Ordering::Relaxed).to_string(),
        );

        Ok(())
    }

    // ... other methods
}
```

## Advanced Topics

### State Management

Plugins can maintain state across invocations:

```rust
use std::sync::Mutex;

pub struct StatefulPlugin {
    state: Mutex<PluginState>,
}

struct PluginState {
    execution_count: u64,
    last_command: String,
}

impl Plugin for StatefulPlugin {
    fn pre_execute(&self, context: &mut PluginContext) -> Result<bool> {
        let mut state = self.state.lock().unwrap();
        state.execution_count += 1;
        state.last_command = context.command_name.clone();
        Ok(true)
    }
}
```

### Configuration Validation

```rust
impl Plugin for MyPlugin {
    fn on_load(&mut self, config: &AHashMap<String, String>) -> Result<()> {
        // Validate required config
        let api_key = config.get("api_key").ok_or_else(|| {
            cmdrun::error::CmdrunError::PluginError {
                plugin: "my-plugin".to_string(),
                message: "Missing required config: api_key".to_string(),
            }
        })?;

        // Validate config values
        if api_key.is_empty() {
            return Err(cmdrun::error::CmdrunError::PluginError {
                plugin: "my-plugin".to_string(),
                message: "api_key cannot be empty".to_string(),
            });
        }

        Ok(())
    }
}
```

### Custom Commands

```rust
impl Plugin for MyPlugin {
    fn custom_commands(&self) -> AHashMap<String, Command> {
        let mut commands = AHashMap::new();

        commands.insert("plugin-status".to_string(), Command {
            description: "Show plugin status".to_string(),
            cmd: CommandSpec::Single("echo 'Plugin active'".to_string()),
            // ... other fields
        });

        commands
    }

    fn execute_custom_command(
        &self,
        command_name: &str,
        context: &PluginContext,
    ) -> Result<CommandResult> {
        match command_name {
            "plugin-status" => {
                Ok(CommandResult {
                    exit_code: 0,
                    stdout: "Plugin is active and healthy".to_string(),
                    stderr: String::new(),
                    duration_ms: 0,
                    metadata: AHashMap::new(),
                })
            }
            _ => Err(cmdrun::error::CmdrunError::PluginError {
                plugin: self.metadata().name,
                message: format!("Unknown command: {}", command_name),
            })
        }
    }
}
```

## Testing Your Plugin

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metadata() {
        let plugin = MyPlugin::default();
        let metadata = plugin.metadata();

        assert_eq!(metadata.name, "my-plugin");
        assert!(metadata.capabilities.pre_execute);
    }

    #[test]
    fn test_configuration() {
        let mut plugin = MyPlugin::default();
        let mut config = AHashMap::new();
        config.insert("key".to_string(), "value".to_string());

        assert!(plugin.on_load(&config).is_ok());
    }

    #[test]
    fn test_pre_execute() {
        let plugin = MyPlugin::default();
        let mut context = create_test_context();

        let result = plugin.pre_execute(&mut context);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), true);
    }

    fn create_test_context() -> PluginContext {
        PluginContext {
            command_name: "test".to_string(),
            command: create_test_command(),
            env: AHashMap::new(),
            working_dir: ".".to_string(),
            data: AHashMap::new(),
        }
    }
}
```

### Integration Tests

Create a test commands.toml and run cmdrun:

```bash
#!/bin/bash
# test_plugin.sh

# Build plugin
cargo build --release

# Create test config
cat > test_commands.toml << EOF
[plugins.my-plugin]
path = "target/release/libmy_cmdrun_plugin.so"

[commands.test]
cmd = "echo 'test'"
EOF

# Run cmdrun with test config
cmdrun -c test_commands.toml test
```

## Debugging

### Enable Logging

```rust
use tracing::{debug, info, warn, error};

impl Plugin for MyPlugin {
    fn pre_execute(&self, context: &mut PluginContext) -> Result<bool> {
        debug!("Pre-execute hook called for: {}", context.command_name);
        info!("Context: {:?}", context);
        Ok(true)
    }
}
```

Run cmdrun with logging:

```bash
RUST_LOG=debug cmdrun test
```

### Common Issues

1. **Plugin not found**: Check file path and extension
2. **Symbol not found**: Ensure `declare_plugin!` macro is present
3. **Panics on load**: Add error handling in `on_load`
4. **Hook not called**: Verify capability is enabled

## Best Practices

1. **Keep hooks fast** - Avoid expensive operations
2. **Handle errors gracefully** - Never panic in hooks
3. **Document configuration** - Clear docs for users
4. **Version your plugin** - Use semantic versioning
5. **Test thoroughly** - Unit + integration tests
6. **Log appropriately** - Use tracing for debug info
7. **Validate inputs** - Check context data
8. **Clean up resources** - Implement `on_unload`

## Publishing Your Plugin

### Documentation

Create comprehensive documentation:

```markdown
# My Plugin

## Description
Brief description of what your plugin does.

## Installation
How to install and configure.

## Configuration
All available configuration options.

## Examples
Usage examples.

## License
Your chosen license.
```

### GitHub Repository

```
my-cmdrun-plugin/
├── src/
│   └── lib.rs
├── tests/
│   └── integration.rs
├── examples/
│   └── config.toml
├── Cargo.toml
├── README.md
├── LICENSE
└── CHANGELOG.md
```

## Resources

- [Plugin API Specification](API.md)
- [Example Plugins](../../examples/plugins/)
- [cmdrun Documentation](../README.md)
- [Rust Book - FFI](https://doc.rust-lang.org/book/ch19-01-unsafe-rust.html)

## Community

- Report issues: https://github.com/sanae-abe/cmdrun/issues
- Discussions: https://github.com/sanae-abe/cmdrun/discussions
- Share your plugin in the discussions!
