//! Add command implementation
//!
//! Adds a new command entry to the TOML configuration file while preserving formatting.

use anyhow::{bail, Context, Result};
use colored::*;
use dialoguer::{theme::ColorfulTheme, Input, Select};
use std::fs;
use std::path::PathBuf;
use toml_edit::{value, Array, DocumentMut, InlineTable, Item, Table};

use crate::config::{ConfigLoader, Language};
use crate::i18n::{get_message, MessageKey};

/// Handle the add command
pub async fn handle_add(
    id: Option<String>,
    command: Option<String>,
    description: Option<String>,
    category: Option<String>,
    tags: Option<Vec<String>>,
    config_path: Option<PathBuf>,
) -> Result<()> {
    // Load config to get language setting
    let config_loader = if let Some(path) = &config_path {
        ConfigLoader::with_path(path)
    } else {
        ConfigLoader::new()
    };
    let config = config_loader.load().await.unwrap_or_default();
    let lang = config.config.language;

    // Interactive prompts with navigation support
    let theme = ColorfulTheme::default();

    // If all arguments provided, skip interactive mode
    if id.is_some() && command.is_some() && description.is_some() {
        let id = id.unwrap();
        let command = command.unwrap();
        let description = description.unwrap();

        if id.trim().is_empty() {
            bail!("{}", get_message(MessageKey::ErrorEmptyCommandId, lang));
        }
        if command.trim().is_empty() {
            bail!("{}", get_message(MessageKey::ErrorEmptyCommand, lang));
        }
        if description.trim().is_empty() {
            bail!("{}", get_message(MessageKey::ErrorEmptyDescription, lang));
        }

        return add_command_to_config(id, command, description, category, tags, lang, config_path)
            .await;
    }

    // Interactive mode with back navigation
    let mut input_id = id.clone().unwrap_or_default();
    let mut input_command = command.clone().unwrap_or_default();
    let mut input_description = description.clone().unwrap_or_default();

    // Error messages for validation (static lifetime required by dialoguer)
    let err_empty_id = get_message(MessageKey::ErrorEmptyCommandId, lang);
    let err_empty_cmd = get_message(MessageKey::ErrorEmptyCommand, lang);
    let err_empty_desc = get_message(MessageKey::ErrorEmptyDescription, lang);

    loop {
        println!();
        println!("{}", "=== Add New Command ===".bright_cyan().bold());
        println!();

        // Step 1: Command ID
        input_id = Input::with_theme(&theme)
            .with_prompt(get_message(MessageKey::PromptCommandId, lang))
            .default(input_id.clone())
            .validate_with(move |input: &String| -> Result<(), &str> {
                if input.trim().is_empty() {
                    Err(err_empty_id)
                } else {
                    Ok(())
                }
            })
            .interact_text()?;

        // Step 2: Command
        input_command = Input::with_theme(&theme)
            .with_prompt(get_message(MessageKey::PromptCommand, lang))
            .default(input_command.clone())
            .validate_with(move |input: &String| -> Result<(), &str> {
                if input.trim().is_empty() {
                    Err(err_empty_cmd)
                } else {
                    Ok(())
                }
            })
            .interact_text()?;

        // Step 3: Description
        input_description = Input::with_theme(&theme)
            .with_prompt(get_message(MessageKey::PromptDescription, lang))
            .default(input_description.clone())
            .validate_with(move |input: &String| -> Result<(), &str> {
                if input.trim().is_empty() {
                    Err(err_empty_desc)
                } else {
                    Ok(())
                }
            })
            .interact_text()?;

        // Preview
        println!();
        println!(
            "{}",
            get_message(MessageKey::LabelPreview, lang)
                .bright_cyan()
                .bold()
        );
        println!(
            "  {}: {}",
            get_message(MessageKey::LabelId, lang),
            input_id.green()
        );
        println!(
            "  {}: {}",
            get_message(MessageKey::LabelCommand, lang),
            input_command.yellow()
        );
        println!(
            "  {}: {}",
            get_message(MessageKey::LabelDescription, lang),
            input_description.white()
        );
        println!();

        // Confirmation with options
        let options = vec![
            get_message(MessageKey::OptionYesAdd, lang),
            get_message(MessageKey::OptionNoEdit, lang),
            get_message(MessageKey::OptionCancel, lang),
        ];
        let selection = Select::with_theme(&theme)
            .with_prompt(get_message(MessageKey::PromptWhatToDo, lang))
            .items(&options)
            .default(0)
            .interact()?;

        match selection {
            0 => {
                // Add command
                return add_command_to_config(
                    input_id,
                    input_command,
                    input_description,
                    category,
                    tags,
                    lang,
                    config_path,
                )
                .await;
            }
            1 => {
                // Edit again - loop continues with current values
                continue;
            }
            2 => {
                // Cancel
                println!("{}", get_message(MessageKey::Cancelled, lang).yellow());
                return Ok(());
            }
            _ => unreachable!(),
        }
    }
}

/// Add command to TOML configuration
async fn add_command_to_config(
    id: String,
    command: String,
    description: String,
    category: Option<String>,
    tags: Option<Vec<String>>,
    lang: Language,
    config_file_path: Option<PathBuf>,
) -> Result<()> {
    // Determine config file path
    let config_path = if let Some(path) = config_file_path {
        path
    } else {
        get_config_path()?
    };

    println!(
        "{} {} '{}' {}",
        "📝".bright_white(),
        get_message(MessageKey::AddingCommand, lang),
        id.green().bold(),
        config_path.display()
    );

    // Read existing TOML file
    let toml_content = fs::read_to_string(&config_path)
        .with_context(|| format!("Failed to read config file: {}", config_path.display()))?;

    // Parse TOML document (preserves formatting)
    let mut doc = toml_content
        .parse::<DocumentMut>()
        .context("Failed to parse TOML file")?;

    // Check for duplicate ID
    if let Some(commands_table) = doc.get("commands").and_then(|item| item.as_table()) {
        if commands_table.contains_key(&id) {
            bail!("{}", get_message(MessageKey::ErrorCommandExists, lang));
        }
    }

    // Create new command entry
    let mut command_table = InlineTable::new();
    command_table.insert("description", value(description).into_value().unwrap());
    command_table.insert("cmd", value(command).into_value().unwrap());

    // Add optional fields
    if let Some(cat) = category {
        command_table.insert("category", value(cat).into_value().unwrap());
    }

    if let Some(tag_list) = tags {
        let mut tag_array = Array::new();
        for tag in tag_list {
            tag_array.push(tag);
        }
        command_table.insert("tags", value(tag_array).into_value().unwrap());
    }

    // Ensure [commands] table exists
    if !doc.contains_key("commands") {
        let mut commands = Table::new();
        commands.set_implicit(true);
        doc["commands"] = Item::Table(commands);
    }

    // Add the new command to the commands table
    if let Some(commands_table) = doc["commands"].as_table_mut() {
        commands_table.insert(&id, Item::Value(command_table.into()));
    } else {
        bail!("Failed to access commands table");
    }

    // Write back to file
    fs::write(&config_path, doc.to_string())
        .with_context(|| format!("Failed to write config file: {}", config_path.display()))?;

    println!(
        "{} {} '{}'",
        "✓".green().bold(),
        get_message(MessageKey::CommandAdded, lang),
        id.green().bold()
    );
    println!(
        "  {} {}",
        format!("{}:", get_message(MessageKey::LabelDescription, lang)).dimmed(),
        doc["commands"][&id]["description"]
            .as_str()
            .unwrap_or("N/A")
    );
    println!(
        "  {} {}",
        format!("{}:", get_message(MessageKey::LabelCommand, lang)).dimmed(),
        doc["commands"][&id]["cmd"].as_str().unwrap_or("N/A")
    );

    Ok(())
}

/// Get the configuration file path
///
/// Priority:
/// 1. Project-local: ./commands.toml
/// 2. Global: ~/.cmdrun/commands.toml
fn get_config_path() -> Result<PathBuf> {
    // Check project-local first
    let local_path = PathBuf::from("commands.toml");
    if local_path.exists() {
        return Ok(local_path);
    }

    // Check global config
    let home_dir = dirs::home_dir().context("Failed to determine home directory")?;
    let global_path = home_dir.join(".cmdrun").join("commands.toml");

    if !global_path.exists() {
        // Create global config directory if it doesn't exist
        if let Some(parent) = global_path.parent() {
            fs::create_dir_all(parent).context("Failed to create .cmdrun directory")?;
        }

        // Create empty TOML with [commands] section
        let initial_content = "# cmdrun commands configuration\n\n[commands]\n";
        fs::write(&global_path, initial_content).context("Failed to create initial config file")?;

        println!(
            "{} Created new config file at {}",
            "📄".bright_white(),
            global_path.display()
        );
    }

    Ok(global_path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[tokio::test]
    async fn test_add_command_to_empty_file() {
        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path();

        // Create initial TOML structure
        fs::write(path, "[commands]\n").unwrap();

        // Simulate adding a command
        let content = fs::read_to_string(path).unwrap();
        let mut doc = content.parse::<DocumentMut>().unwrap();

        let mut command_table = InlineTable::new();
        command_table.insert("description", Value::from("Test command"));
        command_table.insert("cmd", Value::from("echo test"));

        if let Some(commands_table) = doc["commands"].as_table_mut() {
            commands_table.insert("test", Item::Value(command_table.into()));
        }

        fs::write(path, doc.to_string()).unwrap();

        // Verify
        let result = fs::read_to_string(path).unwrap();
        assert!(result.contains("test"));
        assert!(result.contains("Test command"));
        assert!(result.contains("echo test"));
    }

    #[tokio::test]
    async fn test_detect_duplicate_id() {
        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path();

        // Create TOML with existing command
        let content = r#"
[commands]
existing = { description = "Existing", cmd = "echo existing" }
"#;
        fs::write(path, content).unwrap();

        // Parse and check for duplicate
        let toml_content = fs::read_to_string(path).unwrap();
        let doc = toml_content.parse::<DocumentMut>().unwrap();

        if let Some(commands_table) = doc.get("commands").and_then(|item| item.as_table()) {
            assert!(commands_table.contains_key("existing"));
        }
    }
}
