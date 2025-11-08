//! Template schema definitions

use ahash::AHashMap;
use serde::{Deserialize, Serialize};

/// User template structure
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserTemplate {
    /// Template metadata
    pub template: TemplateMetadata,

    /// Commands included in the template
    #[serde(default)]
    pub commands: AHashMap<String, crate::config::schema::Command>,

    /// Global configuration (optional)
    #[serde(default)]
    pub config: Option<crate::config::schema::GlobalConfig>,

    /// Aliases (optional)
    #[serde(default)]
    pub aliases: Option<AHashMap<String, String>>,
}

/// Template metadata
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TemplateMetadata {
    /// Template name (unique identifier)
    pub name: String,

    /// Human-readable description
    pub description: String,

    /// Template version
    #[serde(default = "default_version")]
    pub version: String,

    /// Author (optional)
    #[serde(default)]
    pub author: Option<String>,

    /// Tags for categorization
    #[serde(default)]
    pub tags: Vec<String>,
}

fn default_version() -> String {
    "1.0".to_string()
}

impl UserTemplate {
    /// Validate template structure
    pub fn validate(&self) -> anyhow::Result<()> {
        // Check name is not empty
        if self.template.name.trim().is_empty() {
            anyhow::bail!("Template name cannot be empty");
        }

        // Check description is not empty
        if self.template.description.trim().is_empty() {
            anyhow::bail!("Template description cannot be empty");
        }

        // Check at least one command is defined
        if self.commands.is_empty() {
            anyhow::bail!("Template must contain at least one command");
        }

        // Validate command IDs
        for (id, _) in &self.commands {
            if id.trim().is_empty() {
                anyhow::bail!("Command ID cannot be empty");
            }

            // Check for invalid characters in ID
            if id.contains(|c: char| c.is_whitespace() || c == '/' || c == '\\') {
                anyhow::bail!("Command ID '{}' contains invalid characters", id);
            }
        }

        Ok(())
    }

    /// Convert template to CommandsConfig
    pub fn to_commands_config(&self) -> crate::config::schema::CommandsConfig {
        crate::config::schema::CommandsConfig {
            config: self.config.clone().unwrap_or_default(),
            commands: self.commands.clone(),
            aliases: self.aliases.clone().unwrap_or_default(),
            hooks: crate::config::schema::Hooks::default(),
            plugins: crate::config::schema::PluginsConfig::default(),
        }
    }

    /// Create template from CommandsConfig
    pub fn from_commands_config(
        name: String,
        description: String,
        config: &crate::config::schema::CommandsConfig,
    ) -> Self {
        Self {
            template: TemplateMetadata {
                name,
                description,
                version: default_version(),
                author: None,
                tags: Vec::new(),
            },
            commands: config.commands.clone(),
            config: Some(config.config.clone()),
            aliases: if config.aliases.is_empty() {
                None
            } else {
                Some(config.aliases.clone())
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::schema::{Command, CommandSpec};

    #[test]
    fn test_template_validation_success() {
        let template = create_valid_template();
        assert!(template.validate().is_ok());
    }

    #[test]
    fn test_template_validation_empty_name() {
        let mut template = create_valid_template();
        template.template.name = "".to_string();
        assert!(template.validate().is_err());
    }

    #[test]
    fn test_template_validation_empty_description() {
        let mut template = create_valid_template();
        template.template.description = "".to_string();
        assert!(template.validate().is_err());
    }

    #[test]
    fn test_template_validation_no_commands() {
        let mut template = create_valid_template();
        template.commands.clear();
        assert!(template.validate().is_err());
    }

    #[test]
    fn test_template_validation_invalid_command_id() {
        let mut template = create_valid_template();
        template
            .commands
            .insert("invalid id with spaces".to_string(), create_test_command());
        assert!(template.validate().is_err());
    }

    #[test]
    fn test_to_commands_config() {
        let template = create_valid_template();
        let config = template.to_commands_config();

        assert_eq!(config.commands.len(), 1);
        assert!(config.commands.contains_key("test"));
    }

    #[test]
    fn test_from_commands_config() {
        let mut config = crate::config::schema::CommandsConfig::default();
        config
            .commands
            .insert("test".to_string(), create_test_command());

        let template = UserTemplate::from_commands_config(
            "test-template".to_string(),
            "Test description".to_string(),
            &config,
        );

        assert_eq!(template.template.name, "test-template");
        assert_eq!(template.template.description, "Test description");
        assert_eq!(template.commands.len(), 1);
    }

    fn create_valid_template() -> UserTemplate {
        let mut commands = AHashMap::new();
        commands.insert("test".to_string(), create_test_command());

        UserTemplate {
            template: TemplateMetadata {
                name: "test".to_string(),
                description: "Test template".to_string(),
                version: "1.0".to_string(),
                author: None,
                tags: Vec::new(),
            },
            commands,
            config: None,
            aliases: None,
        }
    }

    fn create_test_command() -> Command {
        Command {
            description: "Test command".to_string(),
            cmd: CommandSpec::Single("echo test".to_string()),
            env: AHashMap::new(),
            working_dir: None,
            deps: Vec::new(),
            platform: Vec::new(),
            tags: Vec::new(),
            timeout: None,
            parallel: false,
            confirm: false,
        }
    }
}
