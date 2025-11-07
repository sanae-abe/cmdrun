//! Plugin registry
//!
//! Manages loaded plugins and their lifecycle.

use super::api::{HookPhase, Plugin, PluginContext, PluginMetadata};
use crate::error::{CmdrunError, Result};
use ahash::AHashMap;
use std::sync::{Arc, RwLock};
use tracing::{debug, info, warn};

/// Plugin instance wrapper
struct PluginInstance {
    /// Plugin implementation
    plugin: Box<dyn Plugin>,

    /// Plugin metadata cache
    metadata: PluginMetadata,

    /// Plugin enabled state
    enabled: bool,
}

/// Plugin registry
///
/// Thread-safe registry for managing loaded plugins.
pub struct PluginRegistry {
    /// Loaded plugins
    plugins: Arc<RwLock<AHashMap<String, PluginInstance>>>,
}

impl PluginRegistry {
    /// Create a new plugin registry
    pub fn new() -> Self {
        Self {
            plugins: Arc::new(RwLock::new(AHashMap::new())),
        }
    }

    /// Register a plugin
    ///
    /// # Arguments
    ///
    /// * `plugin` - Plugin instance to register
    ///
    /// # Errors
    ///
    /// Returns error if plugin with same name already exists.
    pub fn register(&self, mut plugin: Box<dyn Plugin>) -> Result<()> {
        let metadata = plugin.metadata();
        let name = metadata.name.clone();

        debug!("Registering plugin: {} v{}", name, metadata.version);

        // Check for duplicate
        {
            let plugins = self.plugins.read().map_err(|e| CmdrunError::PluginError {
                plugin: name.clone(),
                message: format!("Failed to acquire read lock: {}", e),
            })?;

            if plugins.contains_key(&name) {
                return Err(CmdrunError::PluginError {
                    plugin: name,
                    message: "Plugin already registered".to_string(),
                });
            }
        }

        // Initialize plugin with empty config
        let config = AHashMap::new();
        plugin.on_load(&config).map_err(|e| CmdrunError::PluginError {
            plugin: name.clone(),
            message: format!("Failed to initialize plugin: {}", e),
        })?;

        // Add to registry
        let mut plugins = self.plugins.write().map_err(|e| CmdrunError::PluginError {
            plugin: name.clone(),
            message: format!("Failed to acquire write lock: {}", e),
        })?;

        plugins.insert(
            name.clone(),
            PluginInstance {
                plugin,
                metadata,
                enabled: true,
            },
        );

        info!("Plugin registered successfully: {}", name);
        Ok(())
    }

    /// Unregister a plugin
    ///
    /// # Arguments
    ///
    /// * `name` - Plugin name
    pub fn unregister(&self, name: &str) -> Result<()> {
        debug!("Unregistering plugin: {}", name);

        let mut plugins = self.plugins.write().map_err(|e| CmdrunError::PluginError {
            plugin: name.to_string(),
            message: format!("Failed to acquire write lock: {}", e),
        })?;

        if let Some(mut instance) = plugins.remove(name) {
            instance.plugin.on_unload().map_err(|e| CmdrunError::PluginError {
                plugin: name.to_string(),
                message: format!("Failed to unload plugin: {}", e),
            })?;
            info!("Plugin unregistered successfully: {}", name);
            Ok(())
        } else {
            Err(CmdrunError::PluginError {
                plugin: name.to_string(),
                message: "Plugin not found".to_string(),
            })
        }
    }

    /// Enable a plugin
    pub fn enable(&self, name: &str) -> Result<()> {
        let mut plugins = self.plugins.write().map_err(|e| CmdrunError::PluginError {
            plugin: name.to_string(),
            message: format!("Failed to acquire write lock: {}", e),
        })?;

        if let Some(instance) = plugins.get_mut(name) {
            instance.enabled = true;
            info!("Plugin enabled: {}", name);
            Ok(())
        } else {
            Err(CmdrunError::PluginError {
                plugin: name.to_string(),
                message: "Plugin not found".to_string(),
            })
        }
    }

    /// Disable a plugin
    pub fn disable(&self, name: &str) -> Result<()> {
        let mut plugins = self.plugins.write().map_err(|e| CmdrunError::PluginError {
            plugin: name.to_string(),
            message: format!("Failed to acquire write lock: {}", e),
        })?;

        if let Some(instance) = plugins.get_mut(name) {
            instance.enabled = false;
            info!("Plugin disabled: {}", name);
            Ok(())
        } else {
            Err(CmdrunError::PluginError {
                plugin: name.to_string(),
                message: "Plugin not found".to_string(),
            })
        }
    }

    /// Check if a plugin is enabled
    pub fn is_enabled(&self, name: &str) -> bool {
        if let Ok(plugins) = self.plugins.read() {
            plugins.get(name).map(|p| p.enabled).unwrap_or(false)
        } else {
            false
        }
    }

    /// Get plugin metadata
    pub fn get_metadata(&self, name: &str) -> Option<PluginMetadata> {
        if let Ok(plugins) = self.plugins.read() {
            plugins.get(name).map(|p| p.metadata.clone())
        } else {
            None
        }
    }

    /// List all registered plugins
    pub fn list(&self) -> Vec<PluginMetadata> {
        if let Ok(plugins) = self.plugins.read() {
            plugins.values().map(|p| p.metadata.clone()).collect()
        } else {
            Vec::new()
        }
    }

    /// Execute plugin hook
    ///
    /// # Arguments
    ///
    /// * `phase` - Hook phase to execute
    /// * `context` - Plugin execution context
    ///
    /// # Returns
    ///
    /// For PreExecute: Returns `Ok(true)` if execution should continue
    pub fn execute_hook(&self, phase: HookPhase, context: &mut PluginContext) -> Result<bool> {
        let plugins = self.plugins.read().map_err(|e| CmdrunError::PluginError {
            plugin: "registry".to_string(),
            message: format!("Failed to acquire read lock: {}", e),
        })?;

        let mut should_continue = true;

        for (name, instance) in plugins.iter() {
            if !instance.enabled {
                continue;
            }

            // Check if plugin supports this hook
            let supports_hook = match phase {
                HookPhase::PreExecute => instance.metadata.capabilities.pre_execute,
                HookPhase::PostExecute => instance.metadata.capabilities.post_execute,
                HookPhase::OnError => instance.metadata.capabilities.on_error,
            };

            if !supports_hook {
                continue;
            }

            debug!("Executing {} hook for plugin: {}", phase, name);

            let result = match phase {
                HookPhase::PreExecute => instance.plugin.pre_execute(context),
                HookPhase::PostExecute => {
                    // For PostExecute, we need to get result from context
                    // This will be handled by the manager
                    Ok(true)
                }
                HookPhase::OnError => {
                    // Error will be provided by the manager
                    Ok(true)
                }
            };

            match result {
                Ok(cont) => {
                    if !cont {
                        info!("Plugin {} requested to skip execution", name);
                        should_continue = false;
                        break;
                    }
                }
                Err(e) => {
                    warn!("Plugin {} hook failed: {}", name, e);
                    return Err(CmdrunError::PluginError {
                        plugin: name.clone(),
                        message: format!("Hook execution failed: {}", e),
                    });
                }
            }
        }

        Ok(should_continue)
    }

    /// Get count of registered plugins
    pub fn count(&self) -> usize {
        if let Ok(plugins) = self.plugins.read() {
            plugins.len()
        } else {
            0
        }
    }

    /// Get count of enabled plugins
    pub fn enabled_count(&self) -> usize {
        if let Ok(plugins) = self.plugins.read() {
            plugins.values().filter(|p| p.enabled).count()
        } else {
            0
        }
    }

    /// Unload all plugins
    pub fn unload_all(&self) -> Result<()> {
        let mut plugins = self.plugins.write().map_err(|e| CmdrunError::PluginError {
            plugin: "registry".to_string(),
            message: format!("Failed to acquire write lock: {}", e),
        })?;

        let plugin_names: Vec<String> = plugins.keys().cloned().collect();

        for name in plugin_names {
            if let Some(mut instance) = plugins.remove(&name) {
                if let Err(e) = instance.plugin.on_unload() {
                    warn!("Failed to unload plugin {}: {}", name, e);
                }
            }
        }

        info!("All plugins unloaded");
        Ok(())
    }
}

impl Default for PluginRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for PluginRegistry {
    fn drop(&mut self) {
        if let Err(e) = self.unload_all() {
            warn!("Error during plugin registry cleanup: {}", e);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::plugin::api::{CommandResult, PluginCapabilities};
    use std::any::Any;

    struct TestPlugin {
        name: String,
    }

    impl Plugin for TestPlugin {
        fn metadata(&self) -> PluginMetadata {
            PluginMetadata {
                name: self.name.clone(),
                version: "1.0.0".to_string(),
                description: "Test plugin".to_string(),
                authors: vec!["Test".to_string()],
                license: None,
                homepage: None,
                min_cmdrun_version: None,
                capabilities: PluginCapabilities {
                    pre_execute: true,
                    post_execute: true,
                    on_error: true,
                    custom_commands: false,
                    config_modification: false,
                },
            }
        }

        fn on_load(&mut self, _config: &AHashMap<String, String>) -> Result<()> {
            Ok(())
        }

        fn pre_execute(&self, _context: &mut PluginContext) -> Result<bool> {
            Ok(true)
        }

        fn post_execute(
            &self,
            _context: &PluginContext,
            _result: &mut CommandResult,
        ) -> Result<()> {
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
    fn test_register_plugin() {
        let registry = PluginRegistry::new();
        let plugin = Box::new(TestPlugin {
            name: "test".to_string(),
        });

        assert!(registry.register(plugin).is_ok());
        assert_eq!(registry.count(), 1);
    }

    #[test]
    fn test_duplicate_registration() {
        let registry = PluginRegistry::new();
        let plugin1 = Box::new(TestPlugin {
            name: "test".to_string(),
        });
        let plugin2 = Box::new(TestPlugin {
            name: "test".to_string(),
        });

        assert!(registry.register(plugin1).is_ok());
        assert!(registry.register(plugin2).is_err());
    }

    #[test]
    fn test_enable_disable() {
        let registry = PluginRegistry::new();
        let plugin = Box::new(TestPlugin {
            name: "test".to_string(),
        });

        registry.register(plugin).unwrap();
        assert!(registry.is_enabled("test"));

        registry.disable("test").unwrap();
        assert!(!registry.is_enabled("test"));

        registry.enable("test").unwrap();
        assert!(registry.is_enabled("test"));
    }

    #[test]
    fn test_unregister() {
        let registry = PluginRegistry::new();
        let plugin = Box::new(TestPlugin {
            name: "test".to_string(),
        });

        registry.register(plugin).unwrap();
        assert_eq!(registry.count(), 1);

        registry.unregister("test").unwrap();
        assert_eq!(registry.count(), 0);
    }

    #[test]
    fn test_list_plugins() {
        let registry = PluginRegistry::new();
        let plugin1 = Box::new(TestPlugin {
            name: "test1".to_string(),
        });
        let plugin2 = Box::new(TestPlugin {
            name: "test2".to_string(),
        });

        registry.register(plugin1).unwrap();
        registry.register(plugin2).unwrap();

        let list = registry.list();
        assert_eq!(list.len(), 2);
    }
}
