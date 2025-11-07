//! Basic plugin system tests
//!
//! Tests core plugin functionality including loading, registration, and hook execution.

#[cfg(feature = "plugin-system")]
mod plugin_tests {
    use cmdrun::plugin::api::{
        CommandResult, Plugin, PluginCapabilities, PluginContext, PluginMetadata,
    };
    use cmdrun::plugin::{PluginManager, PluginRegistry};
    use cmdrun::Result;
    use std::any::Any;
    use std::collections::HashMap;

    /// Mock plugin for testing
    struct MockPlugin {
        name: String,
        pre_called: std::sync::Arc<std::sync::Mutex<bool>>,
        post_called: std::sync::Arc<std::sync::Mutex<bool>>,
        error_called: std::sync::Arc<std::sync::Mutex<bool>>,
    }

    impl MockPlugin {
        fn new(name: &str) -> Self {
            Self {
                name: name.to_string(),
                pre_called: std::sync::Arc::new(std::sync::Mutex::new(false)),
                post_called: std::sync::Arc::new(std::sync::Mutex::new(false)),
                error_called: std::sync::Arc::new(std::sync::Mutex::new(false)),
            }
        }
    }

    impl Plugin for MockPlugin {
        fn metadata(&self) -> PluginMetadata {
            PluginMetadata {
                name: self.name.clone(),
                version: "1.0.0".to_string(),
                description: "Mock plugin for testing".to_string(),
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

        fn on_load(&mut self, _config: &HashMap<String, String>) -> Result<()> {
            Ok(())
        }

        fn pre_execute(&self, _context: &mut PluginContext) -> Result<bool> {
            *self.pre_called.lock().unwrap() = true;
            Ok(true)
        }

        fn post_execute(
            &self,
            _context: &PluginContext,
            _result: &mut CommandResult,
        ) -> Result<()> {
            *self.post_called.lock().unwrap() = true;
            Ok(())
        }

        fn on_error(&self, _context: &PluginContext, _error: &cmdrun::CmdrunError) -> Result<()> {
            *self.error_called.lock().unwrap() = true;
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
    fn test_plugin_registry_creation() {
        let registry = PluginRegistry::new();
        assert_eq!(registry.count(), 0);
        assert_eq!(registry.enabled_count(), 0);
    }

    #[test]
    fn test_plugin_registration() {
        let registry = PluginRegistry::new();
        let plugin = Box::new(MockPlugin::new("test"));

        assert!(registry.register(plugin).is_ok());
        assert_eq!(registry.count(), 1);
        assert_eq!(registry.enabled_count(), 1);
    }

    #[test]
    fn test_duplicate_plugin_registration() {
        let registry = PluginRegistry::new();
        let plugin1 = Box::new(MockPlugin::new("test"));
        let plugin2 = Box::new(MockPlugin::new("test"));

        assert!(registry.register(plugin1).is_ok());
        assert!(registry.register(plugin2).is_err());
        assert_eq!(registry.count(), 1);
    }

    #[test]
    fn test_plugin_enable_disable() {
        let registry = PluginRegistry::new();
        let plugin = Box::new(MockPlugin::new("test"));

        registry.register(plugin).unwrap();
        assert!(registry.is_enabled("test"));

        registry.disable("test").unwrap();
        assert!(!registry.is_enabled("test"));
        assert_eq!(registry.enabled_count(), 0);

        registry.enable("test").unwrap();
        assert!(registry.is_enabled("test"));
        assert_eq!(registry.enabled_count(), 1);
    }

    #[test]
    fn test_plugin_unregistration() {
        let registry = PluginRegistry::new();
        let plugin = Box::new(MockPlugin::new("test"));

        registry.register(plugin).unwrap();
        assert_eq!(registry.count(), 1);

        registry.unregister("test").unwrap();
        assert_eq!(registry.count(), 0);
    }

    #[test]
    fn test_plugin_metadata() {
        let registry = PluginRegistry::new();
        let plugin = Box::new(MockPlugin::new("test"));

        registry.register(plugin).unwrap();

        let metadata = registry.get_metadata("test");
        assert!(metadata.is_some());

        let metadata = metadata.unwrap();
        assert_eq!(metadata.name, "test");
        assert_eq!(metadata.version, "1.0.0");
    }

    #[test]
    fn test_plugin_list() {
        let registry = PluginRegistry::new();
        let plugin1 = Box::new(MockPlugin::new("test1"));
        let plugin2 = Box::new(MockPlugin::new("test2"));

        registry.register(plugin1).unwrap();
        registry.register(plugin2).unwrap();

        let list = registry.list();
        assert_eq!(list.len(), 2);
    }

    #[test]
    fn test_plugin_manager_creation() {
        let manager = PluginManager::new();
        assert_eq!(manager.plugin_count(), 0);
    }

    #[test]
    fn test_plugin_manager_load_plugins() {
        let mut manager = PluginManager::new();
        let plugins = HashMap::new();

        assert!(manager.load_plugins(&plugins).is_ok());
        assert_eq!(manager.plugin_count(), 0);
    }
}

#[cfg(not(feature = "plugin-system"))]
#[test]
fn test_plugin_system_disabled() {
    // This test ensures the code compiles even when plugin-system is disabled
    let manager = cmdrun::plugin::PluginManager::new();
    let plugins = std::collections::HashMap::new();
    assert!(manager.load_plugins(&plugins).is_ok());
}
