//! Template manager for storing and retrieving user templates

use anyhow::{Context, Result};
use std::fs;
use std::path::{Path, PathBuf};

use crate::template::builtin::BuiltinTemplate;
use crate::template::schema::UserTemplate;

/// Template manager
pub struct TemplateManager {
    /// Template directory path
    template_dir: PathBuf,
}

impl TemplateManager {
    /// Create a new template manager
    pub fn new() -> Result<Self> {
        let template_dir = Self::get_template_dir()?;

        // Create directory if it doesn't exist
        if !template_dir.exists() {
            fs::create_dir_all(&template_dir).with_context(|| {
                format!(
                    "Failed to create template directory: {}",
                    template_dir.display()
                )
            })?;
        }

        Ok(Self { template_dir })
    }

    /// Get template directory path (~/.cmdrun/templates/)
    fn get_template_dir() -> Result<PathBuf> {
        let home_dir = dirs::home_dir()
            .ok_or_else(|| anyhow::anyhow!("Could not determine home directory"))?;

        Ok(home_dir.join(".cmdrun").join("templates"))
    }

    /// Save a template
    pub fn save(&self, template: &UserTemplate) -> Result<()> {
        // Validate template first
        template.validate()?;

        let file_path = self.template_path(&template.template.name);

        // Check if template already exists
        if file_path.exists() {
            anyhow::bail!("Template '{}' already exists", template.template.name);
        }

        // Serialize to TOML
        let toml_content =
            toml::to_string_pretty(template).context("Failed to serialize template to TOML")?;

        // Write to file
        fs::write(&file_path, toml_content)
            .with_context(|| format!("Failed to write template to {}", file_path.display()))?;

        Ok(())
    }

    /// Load a template
    pub fn load(&self, name: &str) -> Result<UserTemplate> {
        // First, check built-in templates
        if let Some(builtin) = BuiltinTemplate::parse(name) {
            return builtin.parse_template();
        }

        // Then check user templates
        let file_path = self.template_path(name);

        if !file_path.exists() {
            anyhow::bail!("Template '{}' not found", name);
        }

        let content = fs::read_to_string(&file_path)
            .with_context(|| format!("Failed to read template from {}", file_path.display()))?;

        let template: UserTemplate = toml::from_str(&content)
            .with_context(|| format!("Failed to parse template '{}'", name))?;

        // Validate template
        template.validate()?;

        Ok(template)
    }

    /// List all templates (built-in + user)
    pub fn list(&self) -> Result<Vec<TemplateInfo>> {
        let mut templates = Vec::new();

        // Add built-in templates
        for builtin in BuiltinTemplate::all() {
            templates.push(TemplateInfo {
                name: builtin.name().to_string(),
                description: builtin.description_en().to_string(),
                version: "1.0".to_string(),
                builtin: true,
            });
        }

        // Add user templates
        if self.template_dir.exists() {
            for entry in
                fs::read_dir(&self.template_dir).context("Failed to read template directory")?
            {
                let entry = entry.context("Failed to read directory entry")?;
                let path = entry.path();

                if path.extension().and_then(|s| s.to_str()) == Some("toml") {
                    if let Ok(template) = self.load_from_path(&path) {
                        templates.push(TemplateInfo {
                            name: template.template.name.clone(),
                            description: template.template.description.clone(),
                            version: template.template.version.clone(),
                            builtin: false,
                        });
                    }
                }
            }
        }

        Ok(templates)
    }

    /// Remove a user template
    pub fn remove(&self, name: &str) -> Result<()> {
        // Prevent removal of built-in templates
        if BuiltinTemplate::parse(name).is_some() {
            anyhow::bail!("Cannot remove built-in template '{}'", name);
        }

        let file_path = self.template_path(name);

        if !file_path.exists() {
            anyhow::bail!("Template '{}' not found", name);
        }

        fs::remove_file(&file_path)
            .with_context(|| format!("Failed to remove template '{}'", name))?;

        Ok(())
    }

    /// Export template to a specific file
    pub fn export(&self, name: &str, output_path: &Path) -> Result<()> {
        let template = self.load(name)?;

        let toml_content =
            toml::to_string_pretty(&template).context("Failed to serialize template to TOML")?;

        fs::write(output_path, toml_content)
            .with_context(|| format!("Failed to export template to {}", output_path.display()))?;

        Ok(())
    }

    /// Import template from a file
    pub fn import(&self, file_path: &Path) -> Result<String> {
        if !file_path.exists() {
            anyhow::bail!("File not found: {}", file_path.display());
        }

        let content = fs::read_to_string(file_path)
            .with_context(|| format!("Failed to read file: {}", file_path.display()))?;

        let template: UserTemplate = toml::from_str(&content)
            .with_context(|| format!("Failed to parse template from {}", file_path.display()))?;

        // Validate template
        template.validate()?;

        // Save template
        self.save(&template)?;

        Ok(template.template.name.clone())
    }

    /// Check if template exists
    pub fn exists(&self, name: &str) -> bool {
        // Check built-in templates
        if BuiltinTemplate::parse(name).is_some() {
            return true;
        }

        // Check user templates
        self.template_path(name).exists()
    }

    /// Get template file path
    fn template_path(&self, name: &str) -> PathBuf {
        self.template_dir.join(format!("{}.toml", name))
    }

    /// Load template from specific path
    fn load_from_path(&self, path: &Path) -> Result<UserTemplate> {
        let content = fs::read_to_string(path)
            .with_context(|| format!("Failed to read template from {}", path.display()))?;

        let template: UserTemplate = toml::from_str(&content)
            .with_context(|| format!("Failed to parse template from {}", path.display()))?;

        template.validate()?;

        Ok(template)
    }
}

impl Default for TemplateManager {
    fn default() -> Self {
        Self::new().expect("Failed to create template manager")
    }
}

/// Template information
#[derive(Debug, Clone)]
pub struct TemplateInfo {
    pub name: String,
    pub description: String,
    pub version: String,
    pub builtin: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::schema::{Command, CommandSpec};
    use crate::template::schema::TemplateMetadata;
    use ahash::AHashMap;
    use tempfile::TempDir;

    fn create_test_manager() -> (TemplateManager, TempDir) {
        let temp_dir = TempDir::new().unwrap();
        let manager = TemplateManager {
            template_dir: temp_dir.path().to_path_buf(),
        };
        (manager, temp_dir)
    }

    fn create_test_template(name: &str) -> UserTemplate {
        let mut commands = AHashMap::new();
        commands.insert(
            "test".to_string(),
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
            },
        );

        UserTemplate {
            template: TemplateMetadata {
                name: name.to_string(),
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

    #[test]
    fn test_save_and_load_template() {
        let (manager, _temp_dir) = create_test_manager();
        let template = create_test_template("test-template");

        // Save template
        let result = manager.save(&template);
        assert!(
            result.is_ok(),
            "Failed to save template: {:?}",
            result.err()
        );

        // Load template
        let loaded = manager.load("test-template").unwrap();
        assert_eq!(loaded.template.name, "test-template");
        assert_eq!(loaded.commands.len(), 1);
    }

    #[test]
    fn test_save_duplicate_template() {
        let (manager, _temp_dir) = create_test_manager();
        let template = create_test_template("duplicate");

        // First save should succeed
        assert!(manager.save(&template).is_ok());

        // Second save should fail
        assert!(manager.save(&template).is_err());
    }

    #[test]
    fn test_load_nonexistent_template() {
        let (manager, _temp_dir) = create_test_manager();
        let result = manager.load("nonexistent");
        assert!(result.is_err());
    }

    #[test]
    fn test_load_builtin_template() {
        let (manager, _temp_dir) = create_test_manager();
        let result = manager.load("rust-cli");
        assert!(
            result.is_ok(),
            "Failed to load builtin template: {:?}",
            result.err()
        );
    }

    #[test]
    fn test_list_templates() {
        let (manager, _temp_dir) = create_test_manager();

        // Add a user template
        let template = create_test_template("user-template");
        manager.save(&template).unwrap();

        let templates = manager.list().unwrap();

        // Should have built-in templates + user template
        assert!(templates.len() >= 5); // 4 built-in + 1 user

        // Check built-in templates exist
        assert!(templates.iter().any(|t| t.name == "rust-cli"));
        assert!(templates.iter().any(|t| t.name == "nodejs-web"));

        // Check user template exists
        assert!(templates.iter().any(|t| t.name == "user-template"));
    }

    #[test]
    fn test_remove_user_template() {
        let (manager, _temp_dir) = create_test_manager();
        let template = create_test_template("removable");

        // Save and then remove
        manager.save(&template).unwrap();
        assert!(manager.exists("removable"));

        let result = manager.remove("removable");
        assert!(result.is_ok());
        assert!(!manager.exists("removable"));
    }

    #[test]
    fn test_remove_builtin_template() {
        let (manager, _temp_dir) = create_test_manager();

        // Should not be able to remove built-in templates
        let result = manager.remove("rust-cli");
        assert!(result.is_err());
    }

    #[test]
    fn test_export_template() {
        let (manager, temp_dir) = create_test_manager();
        let template = create_test_template("exportable");
        manager.save(&template).unwrap();

        let export_path = temp_dir.path().join("exported.toml");
        let result = manager.export("exportable", &export_path);

        assert!(result.is_ok());
        assert!(export_path.exists());

        // Verify content
        let content = fs::read_to_string(&export_path).unwrap();
        assert!(content.contains("exportable"));
    }

    #[test]
    fn test_import_template() {
        let (manager, temp_dir) = create_test_manager();

        // Create a template file
        let template = create_test_template("importable");
        let import_path = temp_dir.path().join("import.toml");
        let toml_content = toml::to_string_pretty(&template).unwrap();
        fs::write(&import_path, toml_content).unwrap();

        // Import template
        let result = manager.import(&import_path);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "importable");

        // Verify template was imported
        assert!(manager.exists("importable"));
    }

    #[test]
    fn test_exists() {
        let (manager, _temp_dir) = create_test_manager();
        let template = create_test_template("exists-test");

        assert!(!manager.exists("exists-test"));

        manager.save(&template).unwrap();

        assert!(manager.exists("exists-test"));
    }

    #[test]
    fn test_exists_builtin() {
        let (manager, _temp_dir) = create_test_manager();

        // Built-in templates should exist
        assert!(manager.exists("rust-cli"));
        assert!(manager.exists("nodejs-web"));
        assert!(manager.exists("python-data"));
        assert!(manager.exists("react-app"));
    }
}
