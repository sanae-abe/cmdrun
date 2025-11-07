//! Plugin API definitions
//!
//! Defines the Plugin trait and related types for cmdrun's plugin system.

use crate::config::schema::Command;
use crate::error::Result;
use ahash::AHashMap;
use serde::{Deserialize, Serialize};
use std::any::Any;
use std::fmt;

/// Plugin metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginMetadata {
    /// Plugin name
    pub name: String,

    /// Plugin version (semver)
    pub version: String,

    /// Plugin description
    pub description: String,

    /// Plugin author(s)
    pub authors: Vec<String>,

    /// Plugin license
    pub license: Option<String>,

    /// Plugin homepage URL
    pub homepage: Option<String>,

    /// Minimum cmdrun version required
    pub min_cmdrun_version: Option<String>,

    /// Plugin capabilities
    pub capabilities: PluginCapabilities,
}

/// Plugin capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginCapabilities {
    /// Can hook into pre-execution
    pub pre_execute: bool,

    /// Can hook into post-execution
    pub post_execute: bool,

    /// Can hook into error handling
    pub on_error: bool,

    /// Can provide custom commands
    pub custom_commands: bool,

    /// Can modify configuration
    pub config_modification: bool,
}

impl Default for PluginCapabilities {
    fn default() -> Self {
        Self {
            pre_execute: false,
            post_execute: false,
            on_error: false,
            custom_commands: false,
            config_modification: false,
        }
    }
}

/// Command execution result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandResult {
    /// Exit code
    pub exit_code: i32,

    /// Standard output
    pub stdout: String,

    /// Standard error
    pub stderr: String,

    /// Execution duration (milliseconds)
    pub duration_ms: u64,

    /// Custom metadata from plugins
    #[serde(default)]
    pub metadata: AHashMap<String, String>,
}

/// Plugin execution context
#[derive(Debug, Clone)]
pub struct PluginContext {
    /// Command being executed
    pub command_name: String,

    /// Command configuration
    pub command: Command,

    /// Environment variables
    pub env: AHashMap<String, String>,

    /// Working directory
    pub working_dir: String,

    /// Additional context data
    pub data: AHashMap<String, String>,
}

/// Plugin hook phase
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HookPhase {
    /// Before command execution
    PreExecute,

    /// After command execution
    PostExecute,

    /// On error
    OnError,
}

impl fmt::Display for HookPhase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HookPhase::PreExecute => write!(f, "pre_execute"),
            HookPhase::PostExecute => write!(f, "post_execute"),
            HookPhase::OnError => write!(f, "on_error"),
        }
    }
}

/// Plugin trait - must be implemented by all plugins
///
/// # Safety
///
/// Plugins are loaded as dynamic libraries. Implementors must ensure:
/// - Thread safety (Send + Sync)
/// - No unsafe memory access
/// - Proper error handling
/// - No panics in hook methods
pub trait Plugin: Send + Sync {
    /// Get plugin metadata
    fn metadata(&self) -> PluginMetadata;

    /// Called when plugin is loaded
    ///
    /// Use this to initialize plugin state, validate configuration, etc.
    fn on_load(&mut self, config: &AHashMap<String, String>) -> Result<()>;

    /// Called when plugin is unloaded
    ///
    /// Use this to cleanup resources, save state, etc.
    fn on_unload(&mut self) -> Result<()> {
        Ok(())
    }

    /// Pre-execution hook
    ///
    /// Called before command execution. Can modify context or prevent execution.
    ///
    /// # Returns
    ///
    /// - `Ok(true)` - Continue with execution
    /// - `Ok(false)` - Skip execution (command will be marked as successful)
    /// - `Err(_)` - Abort execution with error
    fn pre_execute(&self, _context: &mut PluginContext) -> Result<bool> {
        Ok(true)
    }

    /// Post-execution hook
    ///
    /// Called after successful command execution. Can modify result or trigger
    /// additional actions.
    fn post_execute(
        &self,
        _context: &PluginContext,
        _result: &mut CommandResult,
    ) -> Result<()> {
        Ok(())
    }

    /// Error handling hook
    ///
    /// Called when command execution fails. Can perform recovery actions,
    /// logging, notifications, etc.
    fn on_error(&self, _context: &PluginContext, _error: &crate::error::CmdrunError) -> Result<()> {
        Ok(())
    }

    /// Get custom commands provided by this plugin
    ///
    /// Returns a map of command name to command configuration.
    fn custom_commands(&self) -> AHashMap<String, Command> {
        AHashMap::new()
    }

    /// Handle custom command execution
    ///
    /// Called when a custom command provided by this plugin is executed.
    fn execute_custom_command(
        &self,
        _command_name: &str,
        _context: &PluginContext,
    ) -> Result<CommandResult> {
        Err(crate::error::CmdrunError::PluginError {
            plugin: self.metadata().name,
            message: "Custom command execution not implemented".to_string(),
        })
    }

    /// Get plugin as Any for downcasting
    fn as_any(&self) -> &dyn Any;

    /// Get mutable plugin as Any for downcasting
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

/// Plugin declaration macro
///
/// Use this macro in your plugin crate to export the plugin:
///
/// ```rust,ignore
/// use cmdrun::plugin::*;
///
/// struct MyPlugin;
/// impl Plugin for MyPlugin { ... }
///
/// declare_plugin!(MyPlugin);
/// ```
#[macro_export]
macro_rules! declare_plugin {
    ($plugin_type:ty) => {
        #[no_mangle]
        pub extern "C" fn _plugin_create() -> *mut dyn $crate::plugin::api::Plugin {
            let plugin = <$plugin_type>::default();
            let boxed: Box<dyn $crate::plugin::api::Plugin> = Box::new(plugin);
            Box::into_raw(boxed)
        }

        #[no_mangle]
        pub extern "C" fn _plugin_destroy(ptr: *mut dyn $crate::plugin::api::Plugin) {
            if !ptr.is_null() {
                unsafe {
                    let _ = Box::from_raw(ptr);
                }
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestPlugin;

    impl Plugin for TestPlugin {
        fn metadata(&self) -> PluginMetadata {
            PluginMetadata {
                name: "test".to_string(),
                version: "1.0.0".to_string(),
                description: "Test plugin".to_string(),
                authors: vec!["Test Author".to_string()],
                license: Some("MIT".to_string()),
                homepage: None,
                min_cmdrun_version: None,
                capabilities: PluginCapabilities::default(),
            }
        }

        fn on_load(&mut self, _config: &AHashMap<String, String>) -> Result<()> {
            Ok(())
        }

        fn as_any(&self) -> &dyn Any {
            self
        }

        fn as_any_mut(&mut self) -> &mut dyn Any {
            self
        }
    }

    #[test]
    fn test_plugin_metadata() {
        let plugin = TestPlugin;
        let metadata = plugin.metadata();
        assert_eq!(metadata.name, "test");
        assert_eq!(metadata.version, "1.0.0");
    }

    #[test]
    fn test_plugin_capabilities() {
        let caps = PluginCapabilities::default();
        assert!(!caps.pre_execute);
        assert!(!caps.post_execute);
        assert!(!caps.on_error);
        assert!(!caps.custom_commands);
        assert!(!caps.config_modification);
    }

    #[test]
    fn test_hook_phase_display() {
        assert_eq!(HookPhase::PreExecute.to_string(), "pre_execute");
        assert_eq!(HookPhase::PostExecute.to_string(), "post_execute");
        assert_eq!(HookPhase::OnError.to_string(), "on_error");
    }
}
