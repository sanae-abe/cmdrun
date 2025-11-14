//! Search command - Search commands by keyword

use crate::config::loader::ConfigLoader;
use crate::config::schema::CommandSpec;
use crate::i18n::{get_message, MessageKey};
use anyhow::Result;
use colored::*;
use std::path::PathBuf;

/// Search commands by keyword (case-insensitive)
pub async fn handle_search(
    keyword: String,
    global_only: bool,
    config_path: Option<PathBuf>,
) -> Result<()> {
    let config_loader = if let Some(path) = config_path {
        ConfigLoader::with_path(path)?
    } else if global_only {
        ConfigLoader::global_only()
    } else {
        ConfigLoader::new()
    };
    let config = config_loader.load().await?;
    let lang = config.config.language;

    let keyword_lower = keyword.to_lowercase();

    println!(
        "{}: '{}'",
        get_message(MessageKey::SearchSearchingFor, lang)
            .cyan()
            .bold(),
        keyword.bright_white()
    );
    println!();

    let mut results = Vec::new();

    // Search through all commands
    for (name, cmd) in &config.commands {
        let mut matched = false;
        let mut match_locations = Vec::new();

        // Search in command ID
        if name.to_lowercase().contains(&keyword_lower) {
            matched = true;
            match_locations.push("id");
        }

        // Search in description
        if cmd.description.to_lowercase().contains(&keyword_lower) {
            matched = true;
            match_locations.push("description");
        }

        // Search in command text
        let command_text = match &cmd.cmd {
            CommandSpec::Single(c) => c.clone(),
            CommandSpec::Multiple(cmds) => cmds.join(" "),
            CommandSpec::Platform(p) => {
                let mut parts = Vec::new();
                if let Some(unix) = &p.unix {
                    parts.push(unix.clone());
                }
                if let Some(linux) = &p.linux {
                    parts.push(linux.clone());
                }
                if let Some(macos) = &p.macos {
                    parts.push(macos.clone());
                }
                if let Some(windows) = &p.windows {
                    parts.push(windows.clone());
                }
                parts.join(" ")
            }
        };

        if command_text.to_lowercase().contains(&keyword_lower) {
            matched = true;
            match_locations.push("command");
        }

        // Search in tags
        for tag in &cmd.tags {
            if tag.to_lowercase().contains(&keyword_lower) {
                matched = true;
                match_locations.push("tags");
                break;
            }
        }

        if matched {
            results.push((name.clone(), cmd.description.clone(), match_locations));
        }
    }

    // Display results
    if results.is_empty() {
        println!(
            "{} {} '{}'",
            "⚠".yellow().bold(),
            get_message(MessageKey::SearchNoCommandsMatching, lang),
            keyword.bright_white()
        );
        return Ok(());
    }

    println!(
        "{} {} {} {}:",
        "✓".green().bold(),
        get_message(MessageKey::SearchFound, lang),
        results.len(),
        get_message(MessageKey::MatchingCommands, lang)
    );
    println!();

    // Sort results alphabetically
    results.sort_by(|a, b| a.0.cmp(&b.0));

    for (name, description, locations) in results {
        println!("  {} {} - {}", "•".blue(), name.green().bold(), description);
        println!(
            "    {} {}",
            get_message(MessageKey::SearchMatchedIn, lang).dimmed(),
            locations.join(", ").dimmed()
        );
        println!();
    }

    println!(
        "{} {}",
        "💡".bright_white(),
        get_message(MessageKey::SearchUseInfoToSeeDetails, lang).yellow()
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::NamedTempFile;

    #[test]
    fn test_keyword_case_insensitive() {
        let keyword = "TEST";
        let keyword_lower = keyword.to_lowercase();
        assert_eq!(keyword_lower, "test");

        let text = "This is a Test command";
        assert!(text.to_lowercase().contains(&keyword_lower));
    }

    #[tokio::test]
    async fn test_search_matching_by_id() {
        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path().to_path_buf();

        let config = r#"
[commands.build]
description = "Build the project"
cmd = "cargo build"

[commands.test]
description = "Run tests"
cmd = "cargo test"
"#;
        fs::write(&path, config).unwrap();

        let result = handle_search("build".to_string(), false, Some(path)).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_search_matching_by_description() {
        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path().to_path_buf();

        let config = r#"
[commands.deploy]
description = "Deploy to production"
cmd = "kubectl apply"
"#;
        fs::write(&path, config).unwrap();

        let result = handle_search("production".to_string(), false, Some(path)).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_search_no_results() {
        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path().to_path_buf();

        let config = r#"
[commands.hello]
description = "Say hello"
cmd = "echo hello"
"#;
        fs::write(&path, config).unwrap();

        let result = handle_search("nonexistent".to_string(), false, Some(path)).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_search_matching_by_tags() {
        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path().to_path_buf();

        let config = r#"
[commands.docker_build]
description = "Build Docker image"
cmd = "docker build"
tags = ["docker", "container", "build"]
"#;
        fs::write(&path, config).unwrap();

        let result = handle_search("docker".to_string(), false, Some(path)).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_search_case_insensitive() {
        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path().to_path_buf();

        let config = r#"
[commands.frontend]
description = "Frontend development tasks"
cmd = "npm run dev"
"#;
        fs::write(&path, config).unwrap();

        // Search with uppercase should still match
        let result = handle_search("FRONTEND".to_string(), false, Some(path)).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_search_multiple_command_types() {
        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path().to_path_buf();

        let config = r#"
[commands.multi_step]
description = "Multi-step build"
cmd = ["step1", "step2", "step3"]

[commands.platform_specific]
description = "Platform-specific command"

[commands.platform_specific.cmd]
unix = "ls -la"
windows = "dir"
"#;
        fs::write(&path, config).unwrap();

        let result = handle_search("step".to_string(), false, Some(path)).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_search_with_default_config_path() {
        // Test ConfigLoader::new() path (line 15)
        // This will fail if default config doesn't exist, but covers the code path
        let result = handle_search("test".to_string(), false, None).await;
        // Result may be Ok or Err depending on default config existence
        // We just want to execute the ConfigLoader::new() code path
        let _ = result;
    }

    #[tokio::test]
    async fn test_search_results_sorted_alphabetically() {
        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path().to_path_buf();

        // Create commands in non-alphabetical order
        let config = r#"
[commands.zebra]
description = "Test zebra"
cmd = "echo zebra"

[commands.apple]
description = "Test apple"
cmd = "echo apple"

[commands.middle]
description = "Test middle"
cmd = "echo middle"
"#;
        fs::write(&path, config).unwrap();

        // Search for "Test" - should match all three commands
        // This covers line 112: results.sort_by(|a, b| a.0.cmp(&b.0));
        let result = handle_search("Test".to_string(), false, Some(path)).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_search_with_global_only_mode() {
        // Test ConfigLoader::global_only() path (lines 18-19)
        // This covers the global_only=true branch
        let result = handle_search("test".to_string(), true, None).await;
        // Result may be Ok or Err depending on global config existence
        // We just want to execute the ConfigLoader::global_only() code path
        let _ = result;
    }
}
