//! Info command - Show detailed command information

use crate::config::loader::ConfigLoader;
use crate::config::schema::{CommandSpec, CommandsConfig};
use anyhow::Result;
use colored::*;
use std::io::{self, Write};

/// Show detailed information about a command
pub async fn handle_info(command_id: Option<String>) -> Result<()> {
    let config_loader = ConfigLoader::new();
    let config = config_loader.load().await?;

    // Get command ID (from argument or interactive selection)
    let id = if let Some(id) = command_id {
        id
    } else {
        select_command_interactive(&config)?
    };

    // Find command
    let command = config
        .commands
        .get(&id)
        .ok_or_else(|| anyhow::anyhow!("Command '{}' not found", id))?;

    // Display detailed information
    println!("{} {}", "Command details:".cyan().bold(), id.white().bold());
    println!("{}", "━".repeat(50).cyan());
    println!();

    // Basic information
    println!("{} {}", "Description:".white().bold(), command.description);
    println!();

    // Command specification
    println!("{}", "Command:".white().bold());
    match &command.cmd {
        CommandSpec::Single(cmd) => {
            println!("  {}", cmd.bright_white());
        }
        CommandSpec::Multiple(cmds) => {
            for (idx, cmd) in cmds.iter().enumerate() {
                println!("  {}. {}", idx + 1, cmd.bright_white());
            }
        }
        CommandSpec::Platform(platform_cmds) => {
            if let Some(unix) = &platform_cmds.unix {
                println!("  {} {}", "Unix:".dimmed(), unix.bright_white());
            }
            if let Some(linux) = &platform_cmds.linux {
                println!("  {} {}", "Linux:".dimmed(), linux.bright_white());
            }
            if let Some(macos) = &platform_cmds.macos {
                println!("  {} {}", "macOS:".dimmed(), macos.bright_white());
            }
            if let Some(windows) = &platform_cmds.windows {
                println!("  {} {}", "Windows:".dimmed(), windows.bright_white());
            }
        }
    }
    println!();

    // Dependencies
    if !command.deps.is_empty() {
        println!("{}", "Dependencies:".white().bold());
        for dep in &command.deps {
            println!("  {} {}", "→".blue(), dep.bright_white());
        }
        println!();
    }

    // Tags
    if !command.tags.is_empty() {
        println!("{} {}", "Tags:".white().bold(), command.tags.join(", "));
        println!();
    }

    // Working directory
    if let Some(working_dir) = &command.working_dir {
        println!(
            "{} {}",
            "Working directory:".white().bold(),
            working_dir.display()
        );
        println!();
    }

    // Environment variables
    if !command.env.is_empty() {
        println!("{}", "Environment variables:".white().bold());
        for (key, value) in &command.env {
            println!("  {} = {}", key.yellow(), value.bright_white());
        }
        println!();
    }

    // Execution settings
    println!("{}", "Execution settings:".white().bold());
    println!(
        "  {} {}",
        "Parallel:".dimmed(),
        format_bool(command.parallel)
    );
    println!(
        "  {} {}",
        "Confirm:".dimmed(),
        format_bool(command.confirm)
    );
    if let Some(timeout) = command.timeout {
        println!("  {} {}s", "Timeout:".dimmed(), timeout);
    }
    println!();

    // Platform support
    if !command.platform.is_empty() {
        println!(
            "{} {}",
            "Platforms:".white().bold(),
            command
                .platform
                .iter()
                .map(|p| format!("{:?}", p))
                .collect::<Vec<_>>()
                .join(", ")
        );
        println!();
    }

    Ok(())
}

/// Select a command interactively from the list
fn select_command_interactive(config: &CommandsConfig) -> Result<String> {
    if config.commands.is_empty() {
        anyhow::bail!("No commands available");
    }

    println!("{}", "Select command to view details:".cyan().bold());
    println!();

    let mut commands: Vec<_> = config.commands.iter().collect();
    commands.sort_by_key(|(name, _)| *name);

    for (idx, (name, cmd)) in commands.iter().enumerate() {
        println!(
            "  {}. {} - {}",
            (idx + 1).to_string().yellow(),
            name.green().bold(),
            cmd.description
        );
    }

    println!();
    print!("{} ", "Enter number:".bright_white());
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let selection: usize = input
        .trim()
        .parse()
        .map_err(|_| anyhow::anyhow!("Invalid selection"))?;

    if selection < 1 || selection > commands.len() {
        anyhow::bail!("Selection out of range");
    }

    Ok(commands[selection - 1].0.clone())
}

/// Format boolean value with colored output
fn format_bool(value: bool) -> ColoredString {
    if value {
        "yes".green()
    } else {
        "no".red()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_bool() {
        let yes = format_bool(true);
        assert!(yes.to_string().contains("yes"));

        let no = format_bool(false);
        assert!(no.to_string().contains("no"));
    }
}
