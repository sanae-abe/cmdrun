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

/// List all installed plugins
#[cfg(feature = "plugin-system")]
pub async fn handle_plugin_list(
    only_enabled: bool,
    verbose: bool,
    config_path: Option<PathBuf>,
) -> Result<()> {
    let loader = if let Some(path) = config_path {
        ConfigLoader::with_path(path)
    } else {
        ConfigLoader::new()
    };
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

/// Show detailed information about a plugin
#[cfg(feature = "plugin-system")]
pub async fn handle_plugin_info(name: &str, config_path: Option<PathBuf>) -> Result<()> {
    let loader = if let Some(path) = config_path {
        ConfigLoader::with_path(path)
    } else {
        ConfigLoader::new()
    };
    let config = loader.load().await?;

    let mut manager = PluginManager::new();
    manager.load_plugins(&config.plugins.plugins)?;

    let metadata =
        manager
            .get_metadata(name)
            .ok_or_else(|| crate::error::CmdrunError::PluginError {
                plugin: name.to_string(),
                message: "Plugin not found".to_string(),
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
        if caps.pre_execute {
            "✓".green()
        } else {
            "✗".red()
        }
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
        if caps.on_error {
            "✓".green()
        } else {
            "✗".red()
        }
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

/// Enable a plugin
#[cfg(feature = "plugin-system")]
pub async fn handle_plugin_enable(name: &str, config_path: Option<PathBuf>) -> Result<()> {
    let loader = if let Some(path) = config_path {
        ConfigLoader::with_path(path)
    } else {
        ConfigLoader::new()
    };
    let config = loader.load().await?;

    let mut manager = PluginManager::new();
    manager.load_plugins(&config.plugins.plugins)?;

    manager.enable_plugin(name)?;

    println!("{}", format!("Plugin '{}' enabled", name).green());

    Ok(())
}

/// Disable a plugin
#[cfg(feature = "plugin-system")]
pub async fn handle_plugin_disable(name: &str, config_path: Option<PathBuf>) -> Result<()> {
    let loader = if let Some(path) = config_path {
        ConfigLoader::with_path(path)
    } else {
        ConfigLoader::new()
    };
    let config = loader.load().await?;

    let mut manager = PluginManager::new();
    manager.load_plugins(&config.plugins.plugins)?;

    manager.disable_plugin(name)?;

    println!("{}", format!("Plugin '{}' disabled", name).yellow());

    Ok(())
}
