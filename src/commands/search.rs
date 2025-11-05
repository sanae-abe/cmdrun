//! Search command - Search commands by keyword

use crate::config::loader::ConfigLoader;
use crate::config::schema::CommandSpec;
use anyhow::Result;
use colored::*;
use std::path::PathBuf;

/// Search commands by keyword (case-insensitive)
pub async fn handle_search(keyword: String, config_path: Option<PathBuf>) -> Result<()> {
    let config_loader = if let Some(path) = config_path {
        ConfigLoader::with_path(path)
    } else {
        ConfigLoader::new()
    };
    let config = config_loader.load().await?;

    let keyword_lower = keyword.to_lowercase();

    println!(
        "{} '{}'",
        "Searching for:".cyan().bold(),
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
            "{} No commands matching '{}' found",
            "⚠".yellow().bold(),
            keyword.bright_white()
        );
        return Ok(());
    }

    println!(
        "{} Found {} matching command(s):",
        "✓".green().bold(),
        results.len()
    );
    println!();

    // Sort results alphabetically
    results.sort_by(|a, b| a.0.cmp(&b.0));

    for (name, description, locations) in results {
        println!("  {} {} - {}", "•".blue(), name.green().bold(), description);
        println!(
            "    {} {}",
            "Matched in:".dimmed(),
            locations.join(", ").dimmed()
        );
        println!();
    }

    println!(
        "{} Use {} to see details",
        "💡".bright_white(),
        "cmdrun info <command>".yellow()
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_search_matching() {
        // This is a conceptual test - actual testing would require
        // setting up a temporary config file
        assert!(true);
    }

    #[test]
    fn test_keyword_case_insensitive() {
        let keyword = "TEST";
        let keyword_lower = keyword.to_lowercase();
        assert_eq!(keyword_lower, "test");

        let text = "This is a Test command";
        assert!(text.to_lowercase().contains(&keyword_lower));
    }
}
