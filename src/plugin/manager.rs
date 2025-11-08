//! Plugin manager
//!
//! High-level interface for plugin management and execution.

use ahash::AHashMap;

#[cfg(feature = "plugin-system")]
use super::api::{CommandResult, HookPhase, PluginContext, PluginMetadata};
#[cfg(feature = "plugin-system")]
use super::loader::PluginLoader;
#[cfg(feature = "plugin-system")]
use super::registry::PluginRegistry;
#[cfg(feature = "plugin-system")]
use crate::error::{CmdrunError, Result};
#[cfg(feature = "plugin-system")]
use std::path::Path;
#[cfg(feature = "plugin-system")]
use std::sync::Arc;
#[cfg(feature = "plugin-system")]
use tracing::{debug, info, warn};

/// Plugin configuration from TOML
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct PluginConfig {
    /// Plugin library path
    pub path: String,

    /// Plugin enabled state
    #[serde(default = "default_true")]
    pub enabled: bool,

    /// Plugin-specific configuration
    #[serde(default)]
    pub config: AHashMap<String, String>,
}

fn default_true() -> bool {
    true
}

/// Plugin manager
///
/// Coordinates plugin loading, registration, and hook execution.
#[cfg(feature = "plugin-system")]
pub struct PluginManager {
    /// Plugin loader
    loader: PluginLoader,

    /// Plugin registry
    registry: Arc<PluginRegistry>,
}

#[cfg(feature = "plugin-system")]
impl PluginManager {
    /// Create a new plugin manager
    pub fn new() -> Self {
        Self {
            loader: PluginLoader::new(),
            registry: Arc::new(PluginRegistry::new()),
        }
    }

    /// Load and register a plugin from a library file
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the plugin library
    /// * `config` - Plugin configuration
    ///
    /// # Safety
    ///
    /// This function loads external code. Only load trusted plugins.
    pub fn load_plugin<P: AsRef<Path>>(
        &mut self,
        path: P,
        config: &AHashMap<String, String>,
    ) -> Result<()> {
        let path = path.as_ref();
        info!("Loading plugin from: {}", path.display());

        // Validate plugin first
        PluginLoader::validate(path)?;

        // Load the plugin
        let mut plugin = unsafe { self.loader.load(path)? };

        // Initialize with config
        plugin.on_load(config).map_err(|e| CmdrunError::PluginError {
            plugin: plugin.metadata().name,
            message: format!("Failed to initialize plugin: {}", e),
        })?;

        // Register the plugin
        self.registry.register(plugin)?;

        info!("Plugin loaded and registered successfully");
        Ok(())
    }

    /// Load multiple plugins from configuration
    ///
    /// # Arguments
    ///
    /// * `plugins` - Map of plugin name to plugin configuration
    pub fn load_plugins(&mut self, plugins: &AHashMap<String, PluginConfig>) -> Result<()> {
        for (name, plugin_config) in plugins {
            debug!("Loading plugin: {}", name);

            match self.load_plugin(&plugin_config.path, &plugin_config.config) {
                Ok(_) => {
                    if !plugin_config.enabled {
                        self.registry.disable(name)?;
                    }
                }
                Err(e) => {
                    warn!("Failed to load plugin {}: {}", name, e);
                    // Continue loading other plugins
                }
            }
        }

        Ok(())
    }

    /// Execute pre-execution hooks
    ///
    /// # Returns
    ///
    /// Returns `Ok(true)` if execution should continue, `Ok(false)` if execution
    /// should be skipped.
    pub fn execute_pre_hooks(&self, context: &mut PluginContext) -> Result<bool> {
        debug!("Executing pre-execution hooks");
        self.registry.execute_hook(HookPhase::PreExecute, context)
    }

    /// Execute post-execution hooks
    pub fn execute_post_hooks(
        &self,
        _context: &PluginContext,
        _result: &mut CommandResult,
    ) -> Result<()> {
        debug!("Executing post-execution hooks");

        let plugins = self.registry.list();
        for metadata in plugins {
            if !self.registry.is_enabled(&metadata.name) {
                continue;
            }

            if !metadata.capabilities.post_execute {
                continue;
            }

            // Note: This is a simplified implementation
            // In a real implementation, we would need access to the plugin instance
            debug!(
                "Would execute post_execute hook for plugin: {}",
                metadata.name
            );
        }

        Ok(())
    }

    /// Execute error hooks
    pub fn execute_error_hooks(&self, _context: &PluginContext, _error: &CmdrunError) -> Result<()> {
        debug!("Executing error hooks");

        let plugins = self.registry.list();
        for metadata in plugins {
            if !self.registry.is_enabled(&metadata.name) {
                continue;
            }

            if !metadata.capabilities.on_error {
                continue;
            }

            debug!("Would execute on_error hook for plugin: {}", metadata.name);
        }

        Ok(())
    }

    /// Get plugin metadata
    pub fn get_metadata(&self, name: &str) -> Option<PluginMetadata> {
        self.registry.get_metadata(name)
    }

    /// List all loaded plugins
    pub fn list_plugins(&self) -> Vec<PluginMetadata> {
        self.registry.list()
    }

    /// Enable a plugin
    pub fn enable_plugin(&self, name: &str) -> Result<()> {
        self.registry.enable(name)
    }

    /// Disable a plugin
    pub fn disable_plugin(&self, name: &str) -> Result<()> {
        self.registry.disable(name)
    }

    /// Check if a plugin is enabled
    pub fn is_plugin_enabled(&self, name: &str) -> bool {
        self.registry.is_enabled(name)
    }

    /// Unload a plugin
    pub fn unload_plugin(&self, name: &str) -> Result<()> {
        self.registry.unregister(name)
    }

    /// Unload all plugins
    pub fn unload_all(&mut self) -> Result<()> {
        self.registry.unload_all()?;
        unsafe {
            self.loader.unload_all();
        }
        Ok(())
    }

    /// Get registry reference
    pub fn registry(&self) -> Arc<PluginRegistry> {
        Arc::clone(&self.registry)
    }

    /// Get count of loaded plugins
    pub fn plugin_count(&self) -> usize {
        self.registry.count()
    }

    /// Get count of enabled plugins
    pub fn enabled_count(&self) -> usize {
        self.registry.enabled_count()
    }
}

#[cfg(feature = "plugin-system")]
impl Default for PluginManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(feature = "plugin-system")]
impl Drop for PluginManager {
    fn drop(&mut self) {
        if let Err(e) = self.unload_all() {
            warn!("Error during plugin manager cleanup: {}", e);
        }
    }
}

// Stub implementation for when plugin-system feature is disabled
#[cfg(not(feature = "plugin-system"))]
pub struct PluginManager;

#[cfg(not(feature = "plugin-system"))]
impl PluginManager {
    pub fn new() -> Self {
        Self
    }

    pub fn load_plugins(&mut self, _plugins: &ahash::AHashMap<String, PluginConfig>) -> Result<(), crate::error::CmdrunError> {
        Ok(())
    }
}

#[cfg(not(feature = "plugin-system"))]
impl Default for PluginManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(all(test, feature = "plugin-system"))]
mod tests {
    use super::*;

    #[test]
    fn test_plugin_manager_creation() {
        let manager = PluginManager::new();
        assert_eq!(manager.plugin_count(), 0);
        assert_eq!(manager.enabled_count(), 0);
    }

    #[test]
    fn test_list_empty_plugins() {
        let manager = PluginManager::new();
        let plugins = manager.list_plugins();
        assert_eq!(plugins.len(), 0);
    }
}
