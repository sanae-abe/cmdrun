//! Hello Plugin - Simple demonstration plugin
//!
//! This plugin demonstrates the basic plugin API by:
//! - Logging when commands are about to execute (pre_execute hook)
//! - Logging when commands complete (post_execute hook)
//! - Greeting the user on load

use ahash::AHashMap;
use cmdrun::plugin::api::{
    CommandResult, Plugin, PluginCapabilities, PluginContext, PluginMetadata,
};
use cmdrun::error::Result;
use std::any::Any;

/// Hello plugin implementation
#[derive(Default)]
pub struct HelloPlugin {
    greeting: String,
}

impl Plugin for HelloPlugin {
    fn metadata(&self) -> PluginMetadata {
        PluginMetadata {
            name: "hello".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            description: "A simple hello world plugin demonstrating the plugin API".to_string(),
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
        // Get custom greeting from config, or use default
        self.greeting = config
            .get("greeting")
            .cloned()
            .unwrap_or_else(|| "Hello from cmdrun plugin!".to_string());

        println!("👋 {}", self.greeting);
        println!("   Plugin loaded successfully!");

        Ok(())
    }

    fn on_unload(&mut self) -> Result<()> {
        println!("👋 Hello plugin unloading. Goodbye!");
        Ok(())
    }

    fn pre_execute(&self, context: &mut PluginContext) -> Result<bool> {
        println!(
            "🚀 [Hello Plugin] About to execute: {}",
            context.command_name
        );
        println!("   Working directory: {}", context.working_dir);

        if !context.env.is_empty() {
            println!("   Environment variables:");
            for (key, value) in &context.env {
                println!("     {}={}", key, value);
            }
        }

        // Always continue with execution
        Ok(true)
    }

    fn post_execute(
        &self,
        context: &PluginContext,
        result: &mut CommandResult,
    ) -> Result<()> {
        let status = if result.exit_code == 0 {
            "✅ Success"
        } else {
            "❌ Failed"
        };

        println!(
            "{} [Hello Plugin] Command '{}' completed",
            status, context.command_name
        );
        println!("   Exit code: {}", result.exit_code);
        println!("   Duration: {}ms", result.duration_ms);

        if !result.stdout.is_empty() {
            println!("   Output: {} bytes", result.stdout.len());
        }

        if !result.stderr.is_empty() {
            println!("   Errors: {} bytes", result.stderr.len());
        }

        Ok(())
    }

    fn on_error(&self, context: &PluginContext, error: &cmdrun::error::CmdrunError) -> Result<()> {
        println!(
            "💥 [Hello Plugin] Error in command '{}': {}",
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
cmdrun::declare_plugin!(HelloPlugin);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plugin_metadata() {
        let plugin = HelloPlugin::default();
        let metadata = plugin.metadata();

        assert_eq!(metadata.name, "hello");
        assert!(metadata.capabilities.pre_execute);
        assert!(metadata.capabilities.post_execute);
        assert!(metadata.capabilities.on_error);
    }

    #[test]
    fn test_plugin_load() {
        let mut plugin = HelloPlugin::default();
        let mut config = AHashMap::new();
        config.insert("greeting".to_string(), "Custom greeting!".to_string());

        assert!(plugin.on_load(&config).is_ok());
        assert_eq!(plugin.greeting, "Custom greeting!");
    }
}
