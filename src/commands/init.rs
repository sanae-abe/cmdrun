//! Init command implementation

use anyhow::{Context, Result};
use colored::Colorize;
use dialoguer::{theme::ColorfulTheme, Select};
use std::path::{Path, PathBuf};

use crate::config::Language;
use crate::i18n::{get_message, MessageKey};

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
            "{}",
            get_message(MessageKey::ErrorFileAlreadyExists, Language::English)
        );
    }

    // Select language first (always in English for initial prompt)
    let selected_language = if interactive {
        select_language_interactive()?
    } else {
        Language::default()
    };

    // Determine template (using selected language for prompts)
    let template = if interactive {
        select_template_interactive(selected_language)?
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

    // Get template content and inject language setting
    let content = inject_language_setting(template.get_content(), selected_language);

    // Write file
    std::fs::write(&output_path, content)
        .with_context(|| format!("Failed to write to {}", output_path.display()))?;

    // Success message (in selected language)
    println!();
    println!(
        "{} {} {}",
        "✓".green().bold(),
        get_message(MessageKey::InitCreated, selected_language).green(),
        output_path.display().to_string().bright_white().bold()
    );
    println!(
        "  {} {} {}",
        get_message(MessageKey::InitUsing, selected_language).dimmed(),
        template.name().cyan().bold(),
        get_message(MessageKey::InitTemplateDescription, selected_language).dimmed()
    );

    // Show language confirmation
    let lang_display = match selected_language {
        Language::English => "English",
        Language::Japanese => "日本語",
        Language::ChineseSimplified => "简体中文",
        Language::ChineseTraditional => "繁體中文",
    };
    println!(
        "  {} {}",
        get_message(MessageKey::InitLanguageSet, selected_language).dimmed(),
        lang_display.cyan().bold()
    );
    println!();

    // Next steps (in selected language)
    print_next_steps(&output_path, selected_language);

    Ok(())
}

/// Interactive language selection
fn select_language_interactive() -> Result<Language> {
    println!();
    println!(
        "{}",
        "Select your preferred language / 言語を選択してください"
            .cyan()
            .bold()
    );
    println!();

    let items = vec!["English", "日本語 (Japanese)"];

    let selection = Select::with_theme(&ColorfulTheme::default())
        .items(&items)
        .default(0)
        .interact()
        .context("Failed to read language selection")?;

    Ok(match selection {
        0 => Language::English,
        1 => Language::Japanese,
        _ => Language::English,
    })
}

/// Interactive template selection
fn select_template_interactive(language: Language) -> Result<Template> {
    println!();
    println!(
        "{}",
        get_message(MessageKey::PromptSelectTemplate, language)
            .cyan()
            .bold()
    );
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

/// Inject language setting into TOML content
fn inject_language_setting(content: &str, language: Language) -> String {
    let language_str = match language {
        Language::English => "english",
        Language::Japanese => "japanese",
        Language::ChineseSimplified => "chinese_simplified",
        Language::ChineseTraditional => "chinese_traditional",
    };

    // Find the [config] section and add language setting
    if let Some(config_pos) = content.find("[config]") {
        // Find the end of the [config] section (next section or end of content)
        let after_config = &content[config_pos..];
        if let Some(next_section_pos) = after_config[8..].find('[') {
            // Insert before next section
            let insert_pos = config_pos + 8 + next_section_pos;
            format!(
                "{}language = \"{}\"\n\n{}",
                &content[..insert_pos],
                language_str,
                &content[insert_pos..]
            )
        } else {
            // No next section, append to the end of config
            format!("{}\nlanguage = \"{}\"\n", content, language_str)
        }
    } else {
        // No [config] section, add it at the beginning
        format!("[config]\nlanguage = \"{}\"\n\n{}", language_str, content)
    }
}

/// Print next steps after initialization
fn print_next_steps(output_path: &Path, language: Language) {
    println!(
        "{}",
        format!("{}:", get_message(MessageKey::InitNextSteps, language))
            .cyan()
            .bold()
    );
    println!();

    let step1_msg = match language {
        Language::English => format!("Edit {} to define your commands", output_path.display()),
        Language::Japanese => format!("{} を編集してコマンドを定義", output_path.display()),
        Language::ChineseSimplified => format!("编辑 {} 来定义您的命令", output_path.display()),
        Language::ChineseTraditional => format!("編輯 {} 來定義您的命令", output_path.display()),
    };

    let step2_msg = match language {
        Language::English => "Run cmdrun list to list available commands",
        Language::Japanese => "cmdrun list で利用可能なコマンド一覧を表示",
        Language::ChineseSimplified => "运行 cmdrun list 列出可用命令",
        Language::ChineseTraditional => "執行 cmdrun list 列出可用命令",
    };

    let step3_msg = match language {
        Language::English => "Run cmdrun run <name> to execute a command",
        Language::Japanese => "cmdrun run <名前> でコマンドを実行",
        Language::ChineseSimplified => "运行 cmdrun run <名称> 执行命令",
        Language::ChineseTraditional => "執行 cmdrun run <名稱> 執行命令",
    };

    println!("  {} {}", "1.".bright_white().bold(), step1_msg.yellow());
    println!("  {} {}", "2.".bright_white().bold(), step2_msg);
    println!("  {} {}", "3.".bright_white().bold(), step3_msg);
    println!();

    // Example commands
    let example_label = match language {
        Language::English => "Example commands:",
        Language::Japanese => "コマンド例:",
        Language::ChineseSimplified => "示例命令：",
        Language::ChineseTraditional => "範例命令：",
    };

    println!("{}", example_label.dimmed());
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

    #[test]
    fn test_inject_language_setting() {
        let content = r#"[config]
shell = "bash"
strict_mode = true

[commands.test]
cmd = "echo test"
"#;

        let result = inject_language_setting(content, Language::Japanese);
        assert!(result.contains("language = \"japanese\""));
        assert!(result.contains("[config]"));
        assert!(result.contains("shell = \"bash\""));

        let result_en = inject_language_setting(content, Language::English);
        assert!(result_en.contains("language = \"english\""));
    }

    #[test]
    fn test_inject_language_setting_no_config() {
        let content = r#"[commands.test]
cmd = "echo test"
"#;

        let result = inject_language_setting(content, Language::Japanese);
        assert!(result.contains("[config]"));
        assert!(result.contains("language = \"japanese\""));
        assert!(result.contains("[commands.test]"));
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
        // Default language should be English
        assert!(
            content.contains("language = \"english\""),
            "Default language should be English"
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
            assert!(
                content.contains("language = \"english\""),
                "{} should have default language setting",
                template
            );
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
