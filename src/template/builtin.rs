//! Built-in template definitions

use crate::template::schema::UserTemplate;

/// Built-in template types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BuiltinTemplate {
    /// Rust CLI project
    RustCli,
    /// Node.js Web project
    NodejsWeb,
    /// Python data science project
    PythonData,
    /// React application
    ReactApp,
}

impl BuiltinTemplate {
    /// Get all built-in templates
    pub fn all() -> Vec<Self> {
        vec![
            Self::RustCli,
            Self::NodejsWeb,
            Self::PythonData,
            Self::ReactApp,
        ]
    }

    /// Parse template from string
    pub fn parse(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "rust-cli" | "rust_cli" | "rustcli" => Some(Self::RustCli),
            "nodejs-web" | "nodejs_web" | "node-web" | "node_web" | "nodejsweb" => {
                Some(Self::NodejsWeb)
            }
            "python-data" | "python_data" | "pythondata" | "py-data" => Some(Self::PythonData),
            "react-app" | "react_app" | "reactapp" | "react" => Some(Self::ReactApp),
            _ => None,
        }
    }

    /// Get template name
    pub fn name(&self) -> &'static str {
        match self {
            Self::RustCli => "rust-cli",
            Self::NodejsWeb => "nodejs-web",
            Self::PythonData => "python-data",
            Self::ReactApp => "react-app",
        }
    }

    /// Get template description (English)
    pub fn description_en(&self) -> &'static str {
        match self {
            Self::RustCli => "Rust CLI tool project with cargo commands",
            Self::NodejsWeb => "Node.js web development with npm scripts",
            Self::PythonData => "Python data science with virtual environment",
            Self::ReactApp => "React application with modern tooling",
        }
    }

    /// Get template description (Japanese)
    pub fn description_ja(&self) -> &'static str {
        match self {
            Self::RustCli => "Rust CLIツールプロジェクト（cargoコマンド）",
            Self::NodejsWeb => "Node.js Web開発（npmスクリプト）",
            Self::PythonData => "Pythonデータサイエンス（仮想環境）",
            Self::ReactApp => "Reactアプリケーション（モダンツール）",
        }
    }

    /// Get template content (TOML)
    pub fn content(&self) -> &'static str {
        match self {
            Self::RustCli => include_str!("../../templates/builtin/rust-cli.toml"),
            Self::NodejsWeb => include_str!("../../templates/builtin/nodejs-web.toml"),
            Self::PythonData => include_str!("../../templates/builtin/python-data.toml"),
            Self::ReactApp => include_str!("../../templates/builtin/react-app.toml"),
        }
    }

    /// Parse template content
    pub fn parse_template(&self) -> anyhow::Result<UserTemplate> {
        let content = self.content();
        let template: UserTemplate = toml::from_str(content)
            .map_err(|e| anyhow::anyhow!("Failed to parse {} template: {}", self.name(), e))?;

        // Validate template
        template.validate()?;

        Ok(template)
    }

    /// Get description based on language
    pub fn description(&self, language: crate::config::Language) -> &'static str {
        match language {
            crate::config::Language::English => self.description_en(),
            crate::config::Language::Japanese => self.description_ja(),
        }
    }
}

impl std::fmt::Display for BuiltinTemplate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} - {}", self.name(), self.description_en())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builtin_template_parse() {
        assert_eq!(BuiltinTemplate::parse("rust-cli"), Some(BuiltinTemplate::RustCli));
        assert_eq!(BuiltinTemplate::parse("rustcli"), Some(BuiltinTemplate::RustCli));
        assert_eq!(
            BuiltinTemplate::parse("nodejs-web"),
            Some(BuiltinTemplate::NodejsWeb)
        );
        assert_eq!(
            BuiltinTemplate::parse("python-data"),
            Some(BuiltinTemplate::PythonData)
        );
        assert_eq!(BuiltinTemplate::parse("react-app"), Some(BuiltinTemplate::ReactApp));
        assert_eq!(BuiltinTemplate::parse("unknown"), None);
    }

    #[test]
    fn test_builtin_template_name() {
        assert_eq!(BuiltinTemplate::RustCli.name(), "rust-cli");
        assert_eq!(BuiltinTemplate::NodejsWeb.name(), "nodejs-web");
        assert_eq!(BuiltinTemplate::PythonData.name(), "python-data");
        assert_eq!(BuiltinTemplate::ReactApp.name(), "react-app");
    }

    #[test]
    fn test_builtin_template_all() {
        let templates = BuiltinTemplate::all();
        assert_eq!(templates.len(), 4);
        assert!(templates.contains(&BuiltinTemplate::RustCli));
        assert!(templates.contains(&BuiltinTemplate::NodejsWeb));
        assert!(templates.contains(&BuiltinTemplate::PythonData));
        assert!(templates.contains(&BuiltinTemplate::ReactApp));
    }

    #[test]
    fn test_builtin_template_content_not_empty() {
        for template in BuiltinTemplate::all() {
            let content = template.content();
            assert!(!content.is_empty(), "{} template is empty", template.name());
        }
    }

    #[test]
    fn test_builtin_template_parse_template() {
        for builtin in BuiltinTemplate::all() {
            let result = builtin.parse_template();
            assert!(
                result.is_ok(),
                "{} template failed to parse: {:?}",
                builtin.name(),
                result.err()
            );
        }
    }

    #[test]
    fn test_builtin_template_display() {
        let template = BuiltinTemplate::RustCli;
        let display = format!("{}", template);
        assert!(display.contains("rust-cli"));
        assert!(display.contains("Rust CLI"));
    }

    #[test]
    fn test_builtin_template_description_language() {
        use crate::config::Language;

        let template = BuiltinTemplate::RustCli;
        assert_eq!(
            template.description(Language::English),
            "Rust CLI tool project with cargo commands"
        );
        assert_eq!(
            template.description(Language::Japanese),
            "Rust CLIツールプロジェクト（cargoコマンド）"
        );
    }
}
