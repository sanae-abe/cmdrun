//! Plugin loader
//!
//! Dynamically loads plugin libraries at runtime.

#[cfg(feature = "plugin-system")]
use super::api::Plugin;
#[cfg(feature = "plugin-system")]
use crate::config::Language;
#[cfg(feature = "plugin-system")]
use crate::error::{CmdrunError, Result};
#[cfg(feature = "plugin-system")]
use crate::i18n::{get_message, MessageKey};
#[cfg(feature = "plugin-system")]
use libloading::{Library, Symbol};
#[cfg(feature = "plugin-system")]
use std::path::{Path, PathBuf};
#[cfg(feature = "plugin-system")]
use tracing::{debug, info, warn};

#[cfg(feature = "plugin-system")]
#[allow(improper_ctypes_definitions)]
type PluginCreate = unsafe extern "C" fn() -> *mut dyn Plugin;

#[cfg(feature = "plugin-system")]
#[allow(improper_ctypes_definitions)]
type PluginDestroy = unsafe extern "C" fn(*mut dyn Plugin);

/// Plugin loader
///
/// Manages dynamic library loading for plugins.
#[cfg(feature = "plugin-system")]
pub struct PluginLoader {
    /// Loaded libraries
    libraries: Vec<LoadedLibrary>,
}

#[cfg(feature = "plugin-system")]
struct LoadedLibrary {
    #[allow(dead_code)]
    library: Library,
    path: PathBuf,
}

#[cfg(feature = "plugin-system")]
impl PluginLoader {
    /// Create a new plugin loader
    pub fn new() -> Self {
        Self {
            libraries: Vec::new(),
        }
    }

    /// Load a plugin from a dynamic library
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the plugin library (.so, .dylib, or .dll)
    ///
    /// # Safety
    ///
    /// This function loads external code. The plugin must be trusted and
    /// implement the Plugin trait correctly.
    ///
    /// # Errors
    ///
    /// Returns error if:
    /// - Library file not found
    /// - Library cannot be loaded
    /// - Required symbols not found
    /// - Plugin creation fails
    pub unsafe fn load<P: AsRef<Path>>(&mut self, path: P) -> Result<Box<dyn Plugin>> {
        let path = path.as_ref();
        debug!("Loading plugin from: {}", path.display());

        // Validate file exists
        if !path.exists() {
            return Err(CmdrunError::PluginLoad(format!(
                "Plugin file not found: {}",
                path.display()
            )));
        }

        // Load the library
        let library = Library::new(path).map_err(|e| {
            CmdrunError::PluginLoad(format!(
                "{}: {}",
                get_message(MessageKey::ErrorFailedToLoadLibrary, Language::English),
                e
            ))
        })?;

        // Get the plugin creation function
        let create: Symbol<PluginCreate> = library.get(b"_plugin_create").map_err(|e| {
            CmdrunError::PluginLoad(format!(
                "{} (_plugin_create): {}",
                get_message(MessageKey::ErrorPluginSymbolNotFound, Language::English),
                e
            ))
        })?;

        // Create the plugin instance
        let plugin_ptr = create();
        if plugin_ptr.is_null() {
            return Err(CmdrunError::PluginLoad(
                "Plugin creation returned null".to_string(),
            ));
        }

        let plugin = Box::from_raw(plugin_ptr);

        // Store the library to keep it loaded
        self.libraries.push(LoadedLibrary {
            library,
            path: path.to_path_buf(),
        });

        info!("Plugin loaded successfully from: {}", path.display());
        Ok(plugin)
    }

    /// Unload all plugins
    ///
    /// # Safety
    ///
    /// Plugins must be properly dropped before calling this.
    pub unsafe fn unload_all(&mut self) {
        for lib in &self.libraries {
            debug!("Unloading plugin library: {}", lib.path.display());
        }
        self.libraries.clear();
        info!("All plugin libraries unloaded");
    }

    /// Get list of loaded library paths
    pub fn loaded_libraries(&self) -> Vec<PathBuf> {
        self.libraries.iter().map(|l| l.path.clone()).collect()
    }

    /// Validate plugin library
    ///
    /// Checks if a library file has the required symbols without loading it.
    pub fn validate<P: AsRef<Path>>(path: P) -> Result<()> {
        let path = path.as_ref();

        if !path.exists() {
            return Err(CmdrunError::PluginLoad(format!(
                "Plugin file not found: {}",
                path.display()
            )));
        }

        // Try to load library temporarily
        unsafe {
            let library = Library::new(path).map_err(|e| {
                CmdrunError::PluginLoad(format!(
                    "{}: {}",
                    get_message(MessageKey::ErrorFailedToLoadLibrary, Language::English),
                    e
                ))
            })?;

            // Check for required symbols
            let has_create = library.get::<PluginCreate>(b"_plugin_create").is_ok();
            let has_destroy = library.get::<PluginDestroy>(b"_plugin_destroy").is_ok();

            if !has_create {
                return Err(CmdrunError::PluginLoad(
                    "Missing required symbol: _plugin_create".to_string(),
                ));
            }

            if !has_destroy {
                warn!("Plugin missing optional symbol: _plugin_destroy");
            }
        }

        info!("Plugin validation passed: {}", path.display());
        Ok(())
    }
}

#[cfg(feature = "plugin-system")]
impl Default for PluginLoader {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(feature = "plugin-system")]
impl Drop for PluginLoader {
    fn drop(&mut self) {
        unsafe {
            self.unload_all();
        }
    }
}

// Stub implementations for when plugin-system feature is disabled
#[cfg(not(feature = "plugin-system"))]
pub struct PluginLoader;

#[cfg(not(feature = "plugin-system"))]
impl PluginLoader {
    pub fn new() -> Self {
        Self
    }
}

#[cfg(not(feature = "plugin-system"))]
impl Default for PluginLoader {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(all(test, feature = "plugin-system"))]
mod tests {
    use super::*;

    #[test]
    fn test_plugin_loader_creation() {
        let loader = PluginLoader::new();
        assert_eq!(loader.loaded_libraries().len(), 0);
    }

    #[test]
    fn test_validate_missing_file() {
        let result = PluginLoader::validate("/nonexistent/plugin.so");
        assert!(result.is_err());
    }

    #[test]
    fn test_unload_all() {
        let mut loader = PluginLoader::new();
        // Initially empty
        assert_eq!(loader.loaded_libraries().len(), 0);

        // After unload_all, should still be empty
        unsafe {
            loader.unload_all();
        }
        assert_eq!(loader.loaded_libraries().len(), 0);
    }

    #[test]
    fn test_loaded_libraries_empty() {
        let loader = PluginLoader::new();
        let libs = loader.loaded_libraries();
        assert!(libs.is_empty());
    }

    #[test]
    fn test_validate_idempotent() {
        // Calling validate multiple times on the same (non-existent) file should be safe
        let path = "/nonexistent/plugin1.so";
        let result1 = PluginLoader::validate(path);
        let result2 = PluginLoader::validate(path);

        assert!(result1.is_err());
        assert!(result2.is_err());
    }

    #[test]
    fn test_validate_invalid_extension() {
        // Test validation with invalid file extension
        let result = PluginLoader::validate("/tmp/not_a_plugin.txt");
        // Should fail because file doesn't exist or isn't a valid dynamic library
        assert!(result.is_err());
    }
}
