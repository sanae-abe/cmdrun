//! Plugin management command
//!
//! Manages cmdrun plugins: list, enable, disable, info.

#[cfg(feature = "plugin-system")]
use crate::config::loader::ConfigLoader;
#[cfg(feature = "plugin-system")]
use crate::error::Result;
#[cfg(feature = "plugin-system")]
use crate::plugin::PluginManager;
#[cfg(feature = "plugin-system")]
use colored::Colorize;
#[cfg(feature = "plugin-system")]
use std::path::PathBuf;

#[cfg(feature = "plugin-system")]
#[derive(Debug, clap::Parser)]
pub struct PluginCommand {
    #[clap(subcommand)]
    pub action: PluginAction,
}

#[cfg(feature = "plugin-system")]
#[derive(Debug, clap::Subcommand)]
pub enum PluginAction {
    /// List all installed plugins
    List {
        /// Show only enabled plugins
        #[clap(short, long)]
        enabled: bool,

        /// Show detailed information
        #[clap(short, long)]
        verbose: bool,
    },

    /// Show plugin information
    Info {
        /// Plugin name
        name: String,
    },

    /// Enable a plugin
    Enable {
        /// Plugin name
        name: String,
    },

    /// Disable a plugin
    Disable {
        /// Plugin name
        name: String,
    },
}

#[cfg(feature = "plugin-system")]
impl PluginCommand {
    pub async fn execute(&self, _config_path: Option<PathBuf>) -> Result<()> {
        match &self.action {
            PluginAction::List { enabled, verbose } => {
                self.list_plugins(*enabled, *verbose).await
            }
            PluginAction::Info { name } => self.show_plugin_info(name).await,
            PluginAction::Enable { name } => self.enable_plugin(name).await,
            PluginAction::Disable { name } => self.disable_plugin(name).await,
        }
    }

    async fn list_plugins(&self, only_enabled: bool, verbose: bool) -> Result<()> {
        let loader = ConfigLoader::new();
        let config = loader.load().await?;

        let mut manager = PluginManager::new();
        manager.load_plugins(&config.plugins.plugins)?;

        let plugins = manager.list_plugins();

        if plugins.is_empty() {
            println!("{}", "No plugins installed".yellow());
            return Ok(());
        }

        println!("{}", "Installed Plugins".bold());
        println!();

        for plugin in plugins {
            let is_enabled = manager.is_plugin_enabled(&plugin.name);

            if only_enabled && !is_enabled {
                continue;
            }

            let status = if is_enabled {
                "enabled".green()
            } else {
                "disabled".red()
            };

            println!("{} {} [{}]", "•".cyan(), plugin.name.bold(), status);

            if verbose {
                println!("  Version: {}", plugin.version);
                println!("  Description: {}", plugin.description);

                if !plugin.authors.is_empty() {
                    println!("  Authors: {}", plugin.authors.join(", "));
                }

                if let Some(license) = &plugin.license {
                    println!("  License: {}", license);
                }

                if let Some(homepage) = &plugin.homepage {
                    println!("  Homepage: {}", homepage);
                }

                let caps = &plugin.capabilities;
                let mut capabilities = Vec::new();
                if caps.pre_execute {
                    capabilities.push("pre_execute");
                }
                if caps.post_execute {
                    capabilities.push("post_execute");
                }
                if caps.on_error {
                    capabilities.push("on_error");
                }
                if caps.custom_commands {
                    capabilities.push("custom_commands");
                }
                if caps.config_modification {
                    capabilities.push("config_modification");
                }

                if !capabilities.is_empty() {
                    println!("  Capabilities: {}", capabilities.join(", "));
                }

                println!();
            }
        }

        println!(
            "\n{}: {} / {} {}",
            "Summary",
            manager.enabled_count().to_string().green(),
            manager.plugin_count(),
            "enabled"
        );

        Ok(())
    }

    async fn show_plugin_info(&self, name: &str) -> Result<()> {
        let loader = ConfigLoader::new();
        let config = loader.load().await?;

        let mut manager = PluginManager::new();
        manager.load_plugins(&config.plugins.plugins)?;

        let metadata = manager.get_metadata(name).ok_or_else(|| {
            crate::error::CmdrunError::PluginError {
                plugin: name.to_string(),
                message: "Plugin not found".to_string(),
            }
        })?;

        let is_enabled = manager.is_plugin_enabled(name);

        println!("{}", "Plugin Information".bold());
        println!();
        println!("Name: {}", metadata.name.bold());
        println!("Version: {}", metadata.version);
        println!(
            "Status: {}",
            if is_enabled {
                "enabled".green()
            } else {
                "disabled".red()
            }
        );
        println!("Description: {}", metadata.description);

        if !metadata.authors.is_empty() {
            println!("Authors: {}", metadata.authors.join(", "));
        }

        if let Some(license) = &metadata.license {
            println!("License: {}", license);
        }

        if let Some(homepage) = &metadata.homepage {
            println!("Homepage: {}", homepage);
        }

        if let Some(min_version) = &metadata.min_cmdrun_version {
            println!("Minimum cmdrun version: {}", min_version);
        }

        println!();
        println!("{}", "Capabilities".bold());

        let caps = &metadata.capabilities;
        println!(
            "  Pre-execute hook: {}",
            if caps.pre_execute { "✓".green() } else { "✗".red() }
        );
        println!(
            "  Post-execute hook: {}",
            if caps.post_execute {
                "✓".green()
            } else {
                "✗".red()
            }
        );
        println!(
            "  Error hook: {}",
            if caps.on_error { "✓".green() } else { "✗".red() }
        );
        println!(
            "  Custom commands: {}",
            if caps.custom_commands {
                "✓".green()
            } else {
                "✗".red()
            }
        );
        println!(
            "  Config modification: {}",
            if caps.config_modification {
                "✓".green()
            } else {
                "✗".red()
            }
        );

        Ok(())
    }

    async fn enable_plugin(&self, name: &str) -> Result<()> {
        let loader = ConfigLoader::new();
        let config = loader.load().await?;

        let mut manager = PluginManager::new();
        manager.load_plugins(&config.plugins.plugins)?;

        manager.enable_plugin(name)?;

        println!("{}", format!("Plugin '{}' enabled", name).green());

        Ok(())
    }

    async fn disable_plugin(&self, name: &str) -> Result<()> {
        let loader = ConfigLoader::new();
        let config = loader.load().await?;

        let mut manager = PluginManager::new();
        manager.load_plugins(&config.plugins.plugins)?;

        manager.disable_plugin(name)?;

        println!("{}", format!("Plugin '{}' disabled", name).yellow());

        Ok(())
    }
}

// Stub for when plugin-system feature is disabled
#[cfg(not(feature = "plugin-system"))]
#[derive(Debug, clap::Parser)]
pub struct PluginCommand;

#[cfg(not(feature = "plugin-system"))]
impl PluginCommand {
    pub async fn execute(
        &self,
        _config_path: Option<std::path::PathBuf>,
    ) -> crate::error::Result<()> {
        eprintln!("Plugin system is not enabled. Rebuild with --features plugin-system");
        std::process::exit(1);
    }
}
