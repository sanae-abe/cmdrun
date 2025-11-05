//! Init command implementation

use anyhow::{Context, Result};
use colored::Colorize;
use dialoguer::{theme::ColorfulTheme, Select};
use std::path::{Path, PathBuf};

/// Template types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Template {
    Web,
    Rust,
    Node,
    Python,
    Default,
}

impl Template {
    pub fn parse_template(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "web" => Some(Self::Web),
            "rust" => Some(Self::Rust),
            "node" | "nodejs" | "npm" => Some(Self::Node),
            "python" | "py" => Some(Self::Python),
            "default" => Some(Self::Default),
            _ => None,
        }
    }

    pub fn get_content(&self) -> &'static str {
        match self {
            Self::Web => include_str!("../../templates/web.toml"),
            Self::Rust => include_str!("../../templates/rust.toml"),
            Self::Node => include_str!("../../templates/node.toml"),
            Self::Python => include_str!("../../templates/python.toml"),
            Self::Default => include_str!("../../templates/commands.toml"),
        }
    }

    pub fn description(&self) -> &str {
        match self {
            Self::Web => "Web development (HTML/CSS/JS)",
            Self::Rust => "Rust project",
            Self::Node => "Node.js project",
            Self::Python => "Python project",
            Self::Default => "Default template",
        }
    }

    pub fn name(&self) -> &str {
        match self {
            Self::Web => "web",
            Self::Rust => "rust",
            Self::Node => "node",
            Self::Python => "python",
            Self::Default => "default",
        }
    }

    pub fn all() -> Vec<Self> {
        vec![
            Self::Default,
            Self::Web,
            Self::Rust,
            Self::Node,
            Self::Python,
        ]
    }
}

impl std::fmt::Display for Template {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} - {}", self.name(), self.description())
    }
}

/// Handle init command
pub async fn handle_init(
    template: Option<String>,
    interactive: bool,
    output: Option<PathBuf>,
) -> Result<()> {
    let output_path = output.unwrap_or_else(|| PathBuf::from("commands.toml"));

    // Check if file already exists
    if output_path.exists() {
        anyhow::bail!(
            "Configuration file already exists: {}",
            output_path.display()
        );
    }

    // Determine template
    let template = if interactive {
        select_template_interactive()?
    } else if let Some(t) = template {
        Template::parse_template(&t).ok_or_else(|| {
            anyhow::anyhow!(
                "Unknown template: {}. Available templates: web, rust, node, python, default",
                t
            )
        })?
    } else {
        Template::Default
    };

    // Get template content
    let content = template.get_content();

    // Write file
    std::fs::write(&output_path, content)
        .with_context(|| format!("Failed to write to {}", output_path.display()))?;

    // Success message
    println!();
    println!(
        "{} {} {}",
        "✓".green().bold(),
        "Created".green(),
        output_path.display().to_string().bright_white().bold()
    );
    println!(
        "  {} {} template",
        "Using".dimmed(),
        template.name().cyan().bold()
    );
    println!();

    // Next steps
    print_next_steps(&output_path);

    Ok(())
}

/// Interactive template selection
fn select_template_interactive() -> Result<Template> {
    println!();
    println!("{}", "Select a template:".cyan().bold());
    println!();

    let templates = Template::all();
    let items: Vec<String> = templates.iter().map(|t| format!("{}", t)).collect();

    let selection = Select::with_theme(&ColorfulTheme::default())
        .items(&items)
        .default(0)
        .interact()
        .context("Failed to read selection")?;

    Ok(templates[selection])
}

/// Print next steps after initialization
fn print_next_steps(output_path: &Path) {
    println!("{}", "Next steps:".cyan().bold());
    println!();
    println!(
        "  {} Edit {} to define your commands",
        "1.".bright_white().bold(),
        output_path.display().to_string().yellow()
    );
    println!(
        "  {} Run {} to list available commands",
        "2.".bright_white().bold(),
        "cmdrun list".green().bold()
    );
    println!(
        "  {} Run {} to execute a command",
        "3.".bright_white().bold(),
        "cmdrun run <name>".green().bold()
    );
    println!();

    // Example commands based on templates
    println!("{}", "Example commands:".dimmed());
    println!("  {} {}", "$".dimmed(), "cmdrun list --verbose".dimmed());
    println!("  {} {}", "$".dimmed(), "cmdrun run dev".dimmed());
    println!("  {} {}", "$".dimmed(), "cmdrun run build".dimmed());
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_template_from_str() {
        assert_eq!(Template::parse_template("web"), Some(Template::Web));
        assert_eq!(Template::parse_template("rust"), Some(Template::Rust));
        assert_eq!(Template::parse_template("node"), Some(Template::Node));
        assert_eq!(Template::parse_template("nodejs"), Some(Template::Node));
        assert_eq!(Template::parse_template("npm"), Some(Template::Node));
        assert_eq!(Template::parse_template("python"), Some(Template::Python));
        assert_eq!(Template::parse_template("py"), Some(Template::Python));
        assert_eq!(Template::parse_template("default"), Some(Template::Default));
        assert_eq!(Template::parse_template("unknown"), None);
    }

    #[test]
    fn test_template_name() {
        assert_eq!(Template::Web.name(), "web");
        assert_eq!(Template::Rust.name(), "rust");
        assert_eq!(Template::Node.name(), "node");
        assert_eq!(Template::Python.name(), "python");
        assert_eq!(Template::Default.name(), "default");
    }

    #[test]
    fn test_template_description() {
        assert_eq!(Template::Web.description(), "Web development (HTML/CSS/JS)");
        assert_eq!(Template::Rust.description(), "Rust project");
        assert_eq!(Template::Node.description(), "Node.js project");
        assert_eq!(Template::Python.description(), "Python project");
        assert_eq!(Template::Default.description(), "Default template");
    }

    #[test]
    fn test_template_content() {
        // Verify that each template returns non-empty content
        for template in Template::all() {
            let content = template.get_content();
            assert!(!content.is_empty(), "{} template is empty", template.name());
            assert!(
                content.contains("[config]") || content.contains("[commands"),
                "{} template doesn't contain expected TOML structure",
                template.name()
            );
        }
    }

    #[test]
    fn test_template_display() {
        assert_eq!(
            format!("{}", Template::Web),
            "web - Web development (HTML/CSS/JS)"
        );
        assert_eq!(format!("{}", Template::Rust), "rust - Rust project");
    }

    #[tokio::test]
    async fn test_handle_init_default() {
        let temp_dir = TempDir::new().unwrap();
        let output = temp_dir.path().join("commands.toml");

        let result = handle_init(None, false, Some(output.clone())).await;
        assert!(result.is_ok(), "Init should succeed: {:?}", result.err());
        assert!(output.exists(), "Output file should exist");

        // Verify content
        let content = std::fs::read_to_string(&output).unwrap();
        assert!(!content.is_empty(), "File should not be empty");
        assert!(
            content.contains("[config]") || content.contains("[commands"),
            "File should contain TOML configuration"
        );
    }

    #[tokio::test]
    async fn test_handle_init_with_template() {
        let temp_dir = TempDir::new().unwrap();

        for template in &["web", "rust", "node", "python"] {
            let output = temp_dir.path().join(format!("{}.toml", template));
            let result = handle_init(Some(template.to_string()), false, Some(output.clone())).await;

            assert!(
                result.is_ok(),
                "Init with {} template should succeed: {:?}",
                template,
                result.err()
            );
            assert!(output.exists(), "{} output file should exist", template);

            let content = std::fs::read_to_string(&output).unwrap();
            assert!(!content.is_empty(), "{} file should not be empty", template);
        }
    }

    #[tokio::test]
    async fn test_handle_init_file_exists() {
        let temp_dir = TempDir::new().unwrap();
        let output = temp_dir.path().join("commands.toml");

        // Create file first
        std::fs::write(&output, "existing content").unwrap();

        // Should fail if file exists
        let result = handle_init(None, false, Some(output)).await;
        assert!(result.is_err(), "Init should fail if file exists");
        assert!(
            result.unwrap_err().to_string().contains("already exists"),
            "Error message should mention file already exists"
        );
    }

    #[tokio::test]
    async fn test_handle_init_invalid_template() {
        let temp_dir = TempDir::new().unwrap();
        let output = temp_dir.path().join("commands.toml");

        let result = handle_init(Some("invalid_template".to_string()), false, Some(output)).await;

        assert!(result.is_err(), "Init should fail with invalid template");
        assert!(
            result.unwrap_err().to_string().contains("Unknown template"),
            "Error message should mention unknown template"
        );
    }

    #[test]
    fn test_template_all() {
        let templates = Template::all();
        assert_eq!(templates.len(), 5, "Should have 5 templates");
        assert!(templates.contains(&Template::Default));
        assert!(templates.contains(&Template::Web));
        assert!(templates.contains(&Template::Rust));
        assert!(templates.contains(&Template::Node));
        assert!(templates.contains(&Template::Python));
    }

    // === Edge Case Tests ===

    #[test]
    fn test_template_from_str_case_insensitive() {
        assert_eq!(Template::parse_template("WEB"), Some(Template::Web));
        assert_eq!(Template::parse_template("RUST"), Some(Template::Rust));
        assert_eq!(Template::parse_template("NODE"), Some(Template::Node));
        assert_eq!(Template::parse_template("PYTHON"), Some(Template::Python));
        assert_eq!(Template::parse_template("Default"), Some(Template::Default));
    }

    #[test]
    fn test_template_from_str_edge_cases() {
        // Empty string
        assert_eq!(Template::parse_template(""), None);

        // Whitespace
        assert_eq!(Template::parse_template(" "), None);
        assert_eq!(Template::parse_template("  web  "), None);

        // Special characters
        assert_eq!(Template::parse_template("web!"), None);
        assert_eq!(Template::parse_template("rust@"), None);
    }

    #[test]
    fn test_template_content_validity() {
        for template in Template::all() {
            let content = template.get_content();

            // Verify non-empty
            assert!(!content.is_empty(), "{} template is empty", template.name());

            // Verify valid TOML structure
            assert!(
                content.contains("[config]") || content.contains("[commands"),
                "{} template missing expected TOML structure",
                template.name()
            );

            // Verify parseable TOML
            let parse_result = toml::from_str::<toml::Value>(content);
            assert!(
                parse_result.is_ok(),
                "{} template contains invalid TOML: {:?}",
                template.name(),
                parse_result.err()
            );
        }
    }

    #[tokio::test]
    async fn test_handle_init_empty_template_string() {
        let temp_dir = TempDir::new().unwrap();
        let output = temp_dir.path().join("commands.toml");

        // Empty template string should fail
        let result = handle_init(Some("".to_string()), false, Some(output)).await;
        assert!(result.is_err(), "Empty template string should fail");
    }

    #[tokio::test]
    async fn test_handle_init_whitespace_template() {
        let temp_dir = TempDir::new().unwrap();
        let output = temp_dir.path().join("commands.toml");

        let result = handle_init(Some("   ".to_string()), false, Some(output)).await;
        assert!(result.is_err(), "Whitespace template should fail");
    }

    #[tokio::test]
    async fn test_handle_init_readonly_directory() {
        // Skip this test if running as root (e.g., in Docker CI)
        // Root user can write to read-only directories
        #[cfg(unix)]
        {
            // Check if we can write to a read-only directory (indicates root)
            let test_dir = std::env::temp_dir().join("root_check_test");
            let _ = std::fs::create_dir(&test_dir);

            #[cfg(unix)]
            {
                use std::fs::Permissions;
                use std::os::unix::fs::PermissionsExt;
                let _ = std::fs::set_permissions(&test_dir, Permissions::from_mode(0o555));
            }

            let test_file = test_dir.join("test.txt");
            if std::fs::write(&test_file, "test").is_ok() {
                // Running as root, skip test
                let _ = std::fs::remove_dir_all(&test_dir);
                eprintln!("Skipping test_handle_init_readonly_directory: running as root");
                return;
            }
            let _ = std::fs::remove_dir_all(&test_dir);
        }

        // Create a temporary directory
        let temp_dir = TempDir::new().unwrap();
        let readonly_dir = temp_dir.path().join("readonly");
        std::fs::create_dir(&readonly_dir).unwrap();

        // On Unix-like systems, make directory read-only
        #[cfg(unix)]
        {
            use std::fs::Permissions;
            use std::os::unix::fs::PermissionsExt;

            // Set directory to read-only (no write permission: 0o555)
            let perms = Permissions::from_mode(0o555);
            std::fs::set_permissions(&readonly_dir, perms).unwrap();

            let output = readonly_dir.join("commands.toml");
            let result = handle_init(None, false, Some(output)).await;

            // Restore permissions before assertion for cleanup
            let write_perms = Permissions::from_mode(0o755);
            std::fs::set_permissions(&readonly_dir, write_perms).unwrap();

            // Should fail due to permission
            assert!(
                result.is_err(),
                "Should fail on read-only directory, but got: {:?}",
                result
            );
        }
    }

    #[tokio::test]
    async fn test_handle_init_nested_path() {
        let temp_dir = TempDir::new().unwrap();
        let nested = temp_dir
            .path()
            .join("nested")
            .join("path")
            .join("commands.toml");

        // Should fail because parent directory doesn't exist
        let result = handle_init(None, false, Some(nested)).await;
        assert!(
            result.is_err(),
            "Should fail on non-existent parent directory"
        );
    }

    #[tokio::test]
    async fn test_handle_init_all_templates_valid() {
        let temp_dir = TempDir::new().unwrap();

        for template in Template::all() {
            let output = temp_dir
                .path()
                .join(format!("{}_test.toml", template.name()));
            let result = handle_init(
                Some(template.name().to_string()),
                false,
                Some(output.clone()),
            )
            .await;

            assert!(
                result.is_ok(),
                "{} template init failed: {:?}",
                template.name(),
                result.err()
            );

            assert!(
                output.exists(),
                "{} output file should exist",
                template.name()
            );

            // Verify content is valid TOML
            let content = std::fs::read_to_string(&output).unwrap();
            let parse_result = toml::from_str::<toml::Value>(&content);
            assert!(
                parse_result.is_ok(),
                "{} template generated invalid TOML: {:?}",
                template.name(),
                parse_result.err()
            );
        }
    }

    #[tokio::test]
    async fn test_handle_init_symlink_existing_file() {
        #[cfg(unix)]
        {
            use std::os::unix::fs::symlink;

            let temp_dir = TempDir::new().unwrap();
            let target = temp_dir.path().join("target.toml");
            let link = temp_dir.path().join("link.toml");

            // Create target file
            std::fs::write(&target, "existing content").unwrap();

            // Create symlink
            symlink(&target, &link).unwrap();

            // Should fail because symlink points to existing file
            let result = handle_init(None, false, Some(link)).await;
            assert!(
                result.is_err(),
                "Should fail when symlink points to existing file"
            );
        }
    }

    #[test]
    fn test_template_equality() {
        assert_eq!(Template::Web, Template::Web);
        assert_ne!(Template::Web, Template::Rust);
        assert_ne!(Template::Node, Template::Python);
    }

    #[test]
    fn test_template_clone() {
        let original = Template::Rust;
        let cloned = original;
        assert_eq!(original, cloned);
    }

    #[test]
    fn test_template_display_all() {
        for template in Template::all() {
            let display = format!("{}", template);
            assert!(display.contains(template.name()));
            assert!(display.contains(template.description()));
            assert!(display.contains(" - "));
        }
    }
}
