//! Plugin system
//!
//! Provides extensibility through dynamic plugin loading.
//!
//! # Features
//!
//! - Dynamic library loading
//! - Hook system (pre/post execution, error handling)
//! - Custom command injection
//! - Thread-safe plugin management
//!
//! # Example
//!
//! ```rust,ignore
//! use cmdrun::plugin::{PluginManager, PluginConfig};
//! use ahash::AHashMap;
//!
//! let mut manager = PluginManager::new();
//!
//! let mut plugins = AHashMap::new();
//! plugins.insert("my-plugin".to_string(), PluginConfig {
//!     path: "/path/to/plugin.so".to_string(),
//!     enabled: true,
//!     config: AHashMap::new(),
//! });
//!
//! manager.load_plugins(&plugins).unwrap();
//! ```
//!
//! # Security
//!
//! Only load plugins from trusted sources. Plugins have full access to the
//! cmdrun runtime and can execute arbitrary code.

pub mod api;
pub mod loader;
pub mod manager;
pub mod registry;

// Re-export commonly used types
pub use api::{
    CommandResult, HookPhase, Plugin, PluginCapabilities, PluginContext, PluginMetadata,
};
pub use manager::{PluginConfig, PluginManager};

#[cfg(feature = "plugin-system")]
pub use loader::PluginLoader;
#[cfg(feature = "plugin-system")]
pub use registry::PluginRegistry;

/// Plugin system version
pub const PLUGIN_SYSTEM_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Plugin API version
///
/// Increment this when making breaking changes to the Plugin trait.
pub const PLUGIN_API_VERSION: u32 = 1;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plugin_api_version() {
        assert_eq!(PLUGIN_API_VERSION, 1);
    }
}
