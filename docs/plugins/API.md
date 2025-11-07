# Plugin API Specification

## Overview

The cmdrun plugin system provides a powerful way to extend cmdrun's functionality through dynamically loaded libraries. Plugins can hook into command execution lifecycle, provide custom commands, and modify configuration.

## API Version

Current Plugin API Version: **1**

## Core Traits and Types

### Plugin Trait

The main trait that all plugins must implement:

```rust
pub trait Plugin: Send + Sync {
    fn metadata(&self) -> PluginMetadata;
    fn on_load(&mut self, config: &AHashMap<String, String>) -> Result<()>;
    fn on_unload(&mut self) -> Result<()>;
    fn pre_execute(&self, context: &mut PluginContext) -> Result<bool>;
    fn post_execute(&self, context: &PluginContext, result: &mut CommandResult) -> Result<()>;
    fn on_error(&self, context: &PluginContext, error: &CmdrunError) -> Result<()>;
    fn custom_commands(&self) -> AHashMap<String, Command>;
    fn execute_custom_command(&self, command_name: &str, context: &PluginContext) -> Result<CommandResult>;
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}
```

### PluginMetadata

Describes the plugin:

```rust
pub struct PluginMetadata {
    pub name: String,
    pub version: String,
    pub description: String,
    pub authors: Vec<String>,
    pub license: Option<String>,
    pub homepage: Option<String>,
    pub min_cmdrun_version: Option<String>,
    pub capabilities: PluginCapabilities,
}
```

### PluginCapabilities

Declares what features the plugin uses:

```rust
pub struct PluginCapabilities {
    pub pre_execute: bool,
    pub post_execute: bool,
    pub on_error: bool,
    pub custom_commands: bool,
    pub config_modification: bool,
}
```

### PluginContext

Context provided to plugin hooks:

```rust
pub struct PluginContext {
    pub command_name: String,
    pub command: Command,
    pub env: AHashMap<String, String>,
    pub working_dir: String,
    pub data: AHashMap<String, String>,
}
```

### CommandResult

Result of command execution:

```rust
pub struct CommandResult {
    pub exit_code: i32,
    pub stdout: String,
    pub stderr: String,
    pub duration_ms: u64,
    pub metadata: AHashMap<String, String>,
}
```

## Hook Execution Order

1. **on_load**: Called when plugin is first loaded
2. **pre_execute**: Called before command execution
   - Return `Ok(true)` to continue execution
   - Return `Ok(false)` to skip execution
   - Return `Err(_)` to abort with error
3. **Command Execution**: The actual command runs
4. **post_execute**: Called after successful execution
5. **on_error**: Called if execution fails
6. **on_unload**: Called when plugin is being unloaded

## Thread Safety

All plugins must be `Send + Sync`. cmdrun may call hooks from multiple threads concurrently. Ensure your plugin implementation is thread-safe.

## Memory Safety

Plugins are loaded as dynamic libraries using `libloading`. You must ensure:

- No panics in hook methods (use `Result` for errors)
- Proper cleanup in `on_unload`
- No unsafe memory access without proper documentation
- Compatible ABI with the cmdrun version

## Error Handling

All hook methods return `Result<T>`. Use the provided error types:

```rust
use cmdrun::error::{CmdrunError, Result};

fn my_hook(&self) -> Result<()> {
    Err(CmdrunError::PluginError {
        plugin: self.metadata().name,
        message: "Something went wrong".to_string(),
    })
}
```

## Configuration

Plugins receive configuration through the `on_load` method:

```toml
[plugins]
enabled = ["my-plugin"]

[plugins.my-plugin]
path = "/path/to/my_plugin.so"
option1 = "value1"
option2 = "value2"
```

```rust
fn on_load(&mut self, config: &AHashMap<String, String>) -> Result<()> {
    if let Some(option1) = config.get("option1") {
        // Use option1
    }
    Ok(())
}
```

## Custom Commands

Plugins can provide custom commands:

```rust
fn custom_commands(&self) -> AHashMap<String, Command> {
    let mut commands = AHashMap::new();
    commands.insert("my-command".to_string(), Command {
        description: "My custom command".to_string(),
        cmd: CommandSpec::Single("echo hello".to_string()),
        // ... other fields
    });
    commands
}

fn execute_custom_command(&self, command_name: &str, context: &PluginContext) -> Result<CommandResult> {
    match command_name {
        "my-command" => {
            // Execute custom logic
            Ok(CommandResult {
                exit_code: 0,
                stdout: "Hello from plugin!".to_string(),
                stderr: String::new(),
                duration_ms: 0,
                metadata: AHashMap::new(),
            })
        }
        _ => Err(CmdrunError::PluginError {
            plugin: self.metadata().name,
            message: format!("Unknown command: {}", command_name),
        })
    }
}
```

## Plugin Declaration

Use the `declare_plugin!` macro to export your plugin:

```rust
cmdrun::declare_plugin!(MyPlugin);
```

This macro generates the required C ABI exports:
- `_plugin_create`: Creates a new plugin instance
- `_plugin_destroy`: Destroys a plugin instance

## Build Configuration

Plugins must be built as dynamic libraries (cdylib):

```toml
[lib]
crate-type = ["cdylib"]

[dependencies]
cmdrun = { version = "1.0", features = ["plugin-system"] }
```

## Versioning and Compatibility

- Plugin API uses semantic versioning
- Major version changes indicate breaking changes
- Plugins should specify `min_cmdrun_version` in metadata
- cmdrun will refuse to load incompatible plugins

## Security Considerations

1. **Trust**: Only load plugins from trusted sources
2. **Sandboxing**: Plugins run in the same process - no sandboxing
3. **Permissions**: Plugins have full access to cmdrun's runtime
4. **Validation**: Always validate inputs in hooks
5. **Secrets**: Never log or expose sensitive information

## Performance Best Practices

1. Keep hooks lightweight - they're called frequently
2. Use lazy initialization for expensive resources
3. Avoid blocking operations in hooks
4. Cache computed values when possible
5. Profile your plugin under real workloads

## Testing

Test your plugin thoroughly:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plugin_metadata() {
        let plugin = MyPlugin::default();
        let metadata = plugin.metadata();
        assert_eq!(metadata.name, "my-plugin");
    }

    #[test]
    fn test_pre_execute() {
        let plugin = MyPlugin::default();
        let mut context = PluginContext {
            command_name: "test".to_string(),
            // ... initialize fields
        };
        assert!(plugin.pre_execute(&mut context).is_ok());
    }
}
```

## Examples

See the example plugins:
- `examples/plugins/hello_plugin` - Simple demonstration plugin
- `examples/plugins/logger_plugin` - Advanced logging plugin

## Troubleshooting

### Plugin Not Loading

1. Check library extension (.so on Linux, .dylib on macOS, .dll on Windows)
2. Verify `declare_plugin!` macro is present
3. Ensure `crate-type = ["cdylib"]` in Cargo.toml
4. Check cmdrun logs for detailed error messages

### ABI Compatibility Issues

1. Use the same Rust version as cmdrun
2. Match optimization levels (debug vs release)
3. Consider using `abi_stable` for cross-version compatibility

### Hook Not Called

1. Verify capability is enabled in `PluginCapabilities`
2. Check plugin is enabled in configuration
3. Ensure hook method signature matches trait definition

## Future Enhancements

Planned features for future API versions:

- Plugin dependency management
- Plugin communication/IPC
- Configuration schema validation
- Capability-based security model
- WebAssembly plugin support
