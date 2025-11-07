//! Logger Plugin - Advanced logging plugin
//!
//! This plugin demonstrates advanced plugin features by:
//! - Logging all command executions to a JSON file
//! - Tracking execution statistics
//! - Providing detailed error logging

use ahash::AHashMap;
use chrono::{DateTime, Utc};
use cmdrun::error::Result;
use cmdrun::plugin::api::{
    CommandResult, Plugin, PluginCapabilities, PluginContext, PluginMetadata,
};
use serde::{Deserialize, Serialize};
use std::any::Any;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;

/// Log entry structure
#[derive(Debug, Clone, Serialize, Deserialize)]
struct LogEntry {
    timestamp: DateTime<Utc>,
    command_name: String,
    exit_code: Option<i32>,
    duration_ms: Option<u64>,
    working_dir: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
}

/// Logger plugin implementation
pub struct LoggerPlugin {
    log_file: PathBuf,
    log_level: String,
}

impl Default for LoggerPlugin {
    fn default() -> Self {
        Self {
            log_file: PathBuf::from("cmdrun.log"),
            log_level: "info".to_string(),
        }
    }
}

impl LoggerPlugin {
    /// Write a log entry to the log file
    fn write_log(&self, entry: &LogEntry) -> Result<()> {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.log_file)
            .map_err(|e| cmdrun::error::CmdrunError::PluginError {
                plugin: "logger".to_string(),
                message: format!("Failed to open log file: {}", e),
            })?;

        let json = serde_json::to_string(entry).map_err(|e| {
            cmdrun::error::CmdrunError::PluginError {
                plugin: "logger".to_string(),
                message: format!("Failed to serialize log entry: {}", e),
            }
        })?;

        writeln!(file, "{}", json).map_err(|e| cmdrun::error::CmdrunError::PluginError {
            plugin: "logger".to_string(),
            message: format!("Failed to write log entry: {}", e),
        })?;

        Ok(())
    }
}

impl Plugin for LoggerPlugin {
    fn metadata(&self) -> PluginMetadata {
        PluginMetadata {
            name: "logger".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            description: "Advanced logging plugin with JSON output and statistics".to_string(),
            authors: vec!["cmdrun team".to_string()],
            license: Some("MIT".to_string()),
            homepage: Some("https://github.com/sanae-abe/cmdrun".to_string()),
            min_cmdrun_version: Some("1.0.0".to_string()),
            capabilities: PluginCapabilities {
                pre_execute: true,
                post_execute: true,
                on_error: true,
                custom_commands: false,
                config_modification: false,
            },
        }
    }

    fn on_load(&mut self, config: &AHashMap<String, String>) -> Result<()> {
        // Configure log file path
        if let Some(log_path) = config.get("log_file") {
            self.log_file = PathBuf::from(log_path);
        }

        // Configure log level
        if let Some(level) = config.get("level") {
            self.log_level = level.clone();
        }

        println!("📝 Logger plugin loaded");
        println!("   Log file: {}", self.log_file.display());
        println!("   Log level: {}", self.log_level);

        Ok(())
    }

    fn on_unload(&mut self) -> Result<()> {
        println!("📝 Logger plugin unloading");
        Ok(())
    }

    fn pre_execute(&self, context: &mut PluginContext) -> Result<bool> {
        let entry = LogEntry {
            timestamp: Utc::now(),
            command_name: context.command_name.clone(),
            exit_code: None,
            duration_ms: None,
            working_dir: context.working_dir.clone(),
            error: None,
        };

        self.write_log(&entry)?;

        if self.log_level == "debug" || self.log_level == "trace" {
            println!(
                "📝 [Logger] Pre-execute: {} at {}",
                context.command_name,
                entry.timestamp.format("%Y-%m-%d %H:%M:%S")
            );
        }

        Ok(true)
    }

    fn post_execute(&self, context: &PluginContext, result: &mut CommandResult) -> Result<()> {
        let entry = LogEntry {
            timestamp: Utc::now(),
            command_name: context.command_name.clone(),
            exit_code: Some(result.exit_code),
            duration_ms: Some(result.duration_ms),
            working_dir: context.working_dir.clone(),
            error: None,
        };

        self.write_log(&entry)?;

        if self.log_level == "info" || self.log_level == "debug" || self.log_level == "trace" {
            let status_emoji = if result.exit_code == 0 { "✅" } else { "❌" };
            println!(
                "📝 [Logger] {} Command: {}, Duration: {}ms, Exit: {}",
                status_emoji,
                context.command_name,
                result.duration_ms,
                result.exit_code
            );
        }

        // Add metadata to result
        result.metadata.insert(
            "logged_at".to_string(),
            entry.timestamp.to_rfc3339(),
        );

        Ok(())
    }

    fn on_error(&self, context: &PluginContext, error: &cmdrun::error::CmdrunError) -> Result<()> {
        let entry = LogEntry {
            timestamp: Utc::now(),
            command_name: context.command_name.clone(),
            exit_code: None,
            duration_ms: None,
            working_dir: context.working_dir.clone(),
            error: Some(error.to_string()),
        };

        self.write_log(&entry)?;

        println!(
            "📝 [Logger] Error in {}: {}",
            context.command_name, error
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
cmdrun::declare_plugin!(LoggerPlugin);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plugin_metadata() {
        let plugin = LoggerPlugin::default();
        let metadata = plugin.metadata();

        assert_eq!(metadata.name, "logger");
        assert!(metadata.capabilities.pre_execute);
        assert!(metadata.capabilities.post_execute);
        assert!(metadata.capabilities.on_error);
    }

    #[test]
    fn test_plugin_load_with_config() {
        let mut plugin = LoggerPlugin::default();
        let mut config = AHashMap::new();
        config.insert("log_file".to_string(), "/tmp/test.log".to_string());
        config.insert("level".to_string(), "debug".to_string());

        assert!(plugin.on_load(&config).is_ok());
        assert_eq!(plugin.log_file, PathBuf::from("/tmp/test.log"));
        assert_eq!(plugin.log_level, "debug");
    }
}
