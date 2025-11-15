//! Template command implementation

use anyhow::{Context, Result};
use colored::Colorize;
use dialoguer::{theme::ColorfulTheme, Confirm, Select};
use std::path::PathBuf;

use crate::config::{loader::ConfigLoader, Language};
use crate::i18n::{get_message, MessageKey};
use crate::template::TemplateManager;

/// Handle template add command
pub async fn handle_template_add(name: Option<String>, config_path: Option<PathBuf>) -> Result<()> {
    let config_loader = if let Some(path) = config_path {
        ConfigLoader::with_path(path)?
    } else {
        ConfigLoader::new()
    };
    let config = config_loader.load().await?;
    let language = config.config.language;

    // Get template name
    let template_name = if let Some(n) = name {
        n
    } else {
        dialoguer::Input::<String>::with_theme(&ColorfulTheme::default())
            .with_prompt(get_message(MessageKey::PromptCommandId, language))
            .interact_text()?
    };

    // Validate name
    if template_name.trim().is_empty() {
        anyhow::bail!("{}", get_message(MessageKey::ErrorEmptyCommandId, language));
    }

    // Get description
    let description = dialoguer::Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt(get_message(MessageKey::PromptDescription, language))
        .interact_text()?;

    if description.trim().is_empty() {
        anyhow::bail!(
            "{}",
            get_message(MessageKey::ErrorEmptyDescription, language)
        );
    }

    // Create template from current configuration
    let user_template = crate::template::schema::UserTemplate::from_commands_config(
        template_name.clone(),
        description,
        &config,
    );

    // Save template
    let manager = TemplateManager::new()?;
    manager.save(&user_template)?;

    println!();
    println!(
        "{} Template '{}' created successfully",
        "✓".green().bold(),
        template_name.cyan().bold()
    );
    println!(
        "  {} {}",
        "Saved to:".dimmed(),
        format!("~/.cmdrun/templates/{}.toml", template_name).dimmed()
    );
    println!();

    Ok(())
}

/// Handle template use command
pub async fn handle_template_use(name: String, output: Option<PathBuf>) -> Result<()> {
    let manager = TemplateManager::new()?;

    // Load template
    let template = manager
        .load(&name)
        .with_context(|| format!("Failed to load template '{}'", name))?;

    // Convert to CommandsConfig
    let config = template.to_commands_config();

    // Determine output path
    let output_path = output.unwrap_or_else(|| PathBuf::from("commands.toml"));

    // Check if file exists
    if output_path.exists() {
        let confirm = Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt(format!(
                "File '{}' already exists. Overwrite?",
                output_path.display()
            ))
            .default(false)
            .interact()?;

        if !confirm {
            println!("{}", get_message(MessageKey::Cancelled, Language::English));
            return Ok(());
        }
    }

    // Serialize to TOML
    let toml_content =
        toml::to_string_pretty(&config).context("Failed to serialize configuration to TOML")?;

    // Write to file
    std::fs::write(&output_path, toml_content)
        .with_context(|| format!("Failed to write to {}", output_path.display()))?;

    println!();
    println!(
        "{} Applied template '{}' to {}",
        "✓".green().bold(),
        name.cyan().bold(),
        output_path.display().to_string().bright_white().bold()
    );
    println!(
        "  {} {} commands",
        config.commands.len().to_string().cyan().bold(),
        "defined".dimmed()
    );
    println!();

    Ok(())
}

/// Handle template list command
pub async fn handle_template_list(verbose: bool, language: Language) -> Result<()> {
    let manager = TemplateManager::new()?;
    let templates = manager.list()?;

    if templates.is_empty() {
        println!(
            "{}",
            get_message(MessageKey::TemplateNoTemplatesAvailable, language).yellow()
        );
        return Ok(());
    }

    println!();
    println!(
        "{} {}",
        "Available templates".cyan().bold(),
        format!("({} total)", templates.len()).dimmed()
    );
    println!();

    // Group by builtin and user
    let builtin: Vec<_> = templates.iter().filter(|t| t.builtin).collect();
    let user: Vec<_> = templates.iter().filter(|t| !t.builtin).collect();

    if !builtin.is_empty() {
        println!("{}", "Built-in templates:".bright_white().bold());
        for template in builtin {
            println!(
                "  {} {}",
                template.name.cyan().bold(),
                format!("- {}", template.description).dimmed()
            );
            if verbose {
                println!("    {} {}", "Version:".dimmed(), template.version.dimmed());
            }
        }
        println!();
    }

    if !user.is_empty() {
        println!(
            "{}",
            get_message(MessageKey::TemplateUserTemplates, language)
                .bright_white()
                .bold()
        );
        for template in user {
            println!(
                "  {} {}",
                template.name.green().bold(),
                format!("- {}", template.description).dimmed()
            );
            if verbose {
                println!("    {} {}", "Version:".dimmed(), template.version.dimmed());
            }
        }
        println!();
    }

    println!("{}", "Usage:".dimmed());
    println!(
        "  {} {}",
        "$".dimmed(),
        "cmdrun template use <name>".dimmed()
    );
    println!(
        "  {} {}",
        "$".dimmed(),
        "cmdrun template export <name> <file>".dimmed()
    );
    println!();

    Ok(())
}

/// Handle template remove command
pub async fn handle_template_remove(name: String, force: bool) -> Result<()> {
    let manager = TemplateManager::new()?;

    // Check if template exists
    if !manager.exists(&name) {
        anyhow::bail!("Template '{}' not found", name);
    }

    // Confirm removal
    if !force {
        let confirm = Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt(format!("Remove template '{}'?", name))
            .default(false)
            .interact()?;

        if !confirm {
            println!("{}", get_message(MessageKey::Cancelled, Language::English));
            return Ok(());
        }
    }

    // Remove template
    manager.remove(&name)?;

    println!();
    println!(
        "{} Template '{}' removed successfully",
        "✓".green().bold(),
        name.cyan().bold()
    );
    println!();

    Ok(())
}

/// Handle template export command
pub async fn handle_template_export(name: String, output: PathBuf) -> Result<()> {
    let manager = TemplateManager::new()?;

    // Export template
    manager
        .export(&name, &output)
        .with_context(|| format!("Failed to export template '{}'", name))?;

    println!();
    println!(
        "{} Template '{}' exported to {}",
        "✓".green().bold(),
        name.cyan().bold(),
        output.display().to_string().bright_white().bold()
    );
    println!();

    Ok(())
}

/// Handle template import command
pub async fn handle_template_import(file: PathBuf) -> Result<()> {
    let manager = TemplateManager::new()?;

    // Import template
    let template_name = manager
        .import(&file)
        .with_context(|| format!("Failed to import template from {}", file.display()))?;

    println!();
    println!(
        "{} Template '{}' imported successfully",
        "✓".green().bold(),
        template_name.cyan().bold()
    );
    println!(
        "  {} {}",
        "From:".dimmed(),
        file.display().to_string().dimmed()
    );
    println!();

    Ok(())
}

/// Handle interactive template selection
pub async fn select_template_interactive(language: Language) -> Result<String> {
    let manager = TemplateManager::new()?;
    let templates = manager.list()?;

    if templates.is_empty() {
        anyhow::bail!("No templates available");
    }

    println!();
    println!(
        "{}",
        get_message(MessageKey::PromptSelectTemplate, language)
            .cyan()
            .bold()
    );
    println!();

    let items: Vec<String> = templates
        .iter()
        .map(|t| {
            let prefix = if t.builtin { "[builtin]" } else { "[user]" };
            format!("{} {} - {}", prefix, t.name, t.description)
        })
        .collect();

    let selection = Select::with_theme(&ColorfulTheme::default())
        .items(&items)
        .default(0)
        .interact()
        .context("Failed to read selection")?;

    Ok(templates[selection].name.clone())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_handle_template_use() {
        let temp_dir = TempDir::new().unwrap();
        let output = temp_dir.path().join("commands.toml");

        let result = handle_template_use("rust-cli".to_string(), Some(output.clone())).await;
        assert!(result.is_ok(), "Template use failed: {:?}", result.err());
        assert!(output.exists(), "Output file should exist");

        // Verify content
        let content = std::fs::read_to_string(&output).unwrap();
        assert!(content.contains("[commands.build]"));
    }

    #[tokio::test]
    async fn test_handle_template_list() {
        let result = handle_template_list(false, Language::English).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_handle_template_export() {
        let temp_dir = TempDir::new().unwrap();
        let output = temp_dir.path().join("exported.toml");

        let result = handle_template_export("rust-cli".to_string(), output.clone()).await;
        assert!(result.is_ok());
        assert!(output.exists());
    }
}
