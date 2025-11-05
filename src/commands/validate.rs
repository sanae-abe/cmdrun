//! Validate command implementation

use crate::config::loader::ConfigLoader;
use crate::config::validation::{ConfigValidator, ValidationError};
use anyhow::{Context, Result};
use colored::Colorize;
use std::path::PathBuf;

/// Validation report
#[derive(Debug)]
pub struct ValidationReport {
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub info: Vec<String>,
}

impl ValidationReport {
    pub fn new() -> Self {
        Self {
            errors: Vec::new(),
            warnings: Vec::new(),
            info: Vec::new(),
        }
    }

    pub fn add_error(&mut self, msg: String) {
        self.errors.push(msg);
    }

    pub fn add_warning(&mut self, msg: String) {
        self.warnings.push(msg);
    }

    pub fn add_info(&mut self, msg: String) {
        self.info.push(msg);
    }

    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    pub fn print(&self, verbose: bool) {
        if !self.errors.is_empty() {
            println!();
            println!("{}", "Errors:".red().bold());
            for err in &self.errors {
                println!("  {} {}", "✗".red(), err);
            }
        }

        if !self.warnings.is_empty() {
            println!();
            println!("{}", "Warnings:".yellow().bold());
            for warn in &self.warnings {
                println!("  {} {}", "⚠".yellow(), warn);
            }
        }

        if verbose && !self.info.is_empty() {
            println!();
            println!("{}", "Information:".cyan().bold());
            for info in &self.info {
                println!("  {} {}", "ℹ".cyan(), info);
            }
        }
    }
}

impl Default for ValidationReport {
    fn default() -> Self {
        Self::new()
    }
}

/// Handle validate command
pub async fn handle_validate(
    path: Option<PathBuf>,
    verbose: bool,
    check_cycles: bool,
) -> Result<()> {
    println!("{}", "Validating configuration...".cyan().bold());
    println!();

    // Load configuration
    let config_loader = if let Some(p) = &path {
        ConfigLoader::with_path(p.clone())
    } else {
        ConfigLoader::new()
    };

    let config = config_loader
        .load()
        .await
        .context("Failed to load configuration")?;

    let config_path = path.unwrap_or_else(|| PathBuf::from("commands.toml"));
    println!(
        "{} Loaded configuration from {}",
        "✓".green(),
        config_path.display()
    );

    // Create validation report
    let mut report = ValidationReport::new();

    // Create validator
    let validator = ConfigValidator::new(&config);

    // Basic validation
    match validator.validate() {
        Ok(_) => {
            report.add_info(format!("{} commands defined", config.commands.len()));
            report.add_info(format!("{} aliases defined", config.aliases.len()));
        }
        Err(e) => {
            if let Some(ve) = e.downcast_ref::<ValidationError>() {
                report.add_error(format!("{}", ve));
            } else {
                report.add_error(format!("{}", e));
            }
        }
    }

    // Check circular dependencies if requested
    if check_cycles {
        println!();
        println!("{}", "Checking for circular dependencies...".cyan());

        for (name, _) in &config.commands {
            if let Err(e) = validator.compute_execution_order(std::slice::from_ref(name)) {
                report.add_error(format!("Circular dependency in '{}': {}", name, e));
            } else if verbose {
                report.add_info(format!("✓ No circular dependencies for '{}'", name));
            }
        }
    }

    // Validate each command
    if verbose {
        println!();
        println!("{}", "Validating commands:".cyan());

        for (name, cmd) in &config.commands {
            println!("  {} {}", "✓".green(), name);

            if verbose {
                println!("    {} {}", "Description:".dimmed(), cmd.description);

                if !cmd.deps.is_empty() {
                    println!("    {} {:?}", "Dependencies:".dimmed(), cmd.deps);
                }

                if !cmd.platform.is_empty() {
                    println!("    {} {:?}", "Platforms:".dimmed(), cmd.platform);
                }
            }
        }
    }

    // Validate aliases
    if verbose && !config.aliases.is_empty() {
        println!();
        println!("{}", "Validating aliases:".cyan());

        for (alias, target) in &config.aliases {
            if config.commands.contains_key(target) {
                println!("  {} {} -> {}", "✓".green(), alias, target);
            } else {
                report.add_error(format!(
                    "Alias '{}' points to non-existent command '{}'",
                    alias, target
                ));
            }
        }
    }

    // Build dependency graph
    if verbose {
        println!();
        println!("{}", "Building dependency graph...".cyan());

        match validator.build_dependency_graph() {
            Ok(_graph) => {
                report.add_info("Dependency graph built successfully".to_string());

                // Show execution order for some commands
                for (name, _) in config.commands.iter().take(3) {
                    if let Ok(order) = validator.compute_execution_order(std::slice::from_ref(name))
                    {
                        if order.len() > 1 {
                            println!("  {} Execution order: {}", "→".blue(), order.join(" → "));
                        }
                    }
                }
            }
            Err(e) => {
                report.add_error(format!("Failed to build dependency graph: {}", e));
            }
        }
    }

    // Print report
    report.print(verbose);

    // Print summary
    println!();
    if report.has_errors() {
        println!(
            "{} Configuration validation failed with {} error(s)",
            "✗".red().bold(),
            report.errors.len()
        );
        anyhow::bail!("Validation failed");
    } else {
        println!(
            "{} Configuration is valid ({} commands, {} aliases)",
            "✓".green().bold(),
            config.commands.len(),
            config.aliases.len()
        );

        if !report.warnings.is_empty() {
            println!(
                "{} {} warning(s) found",
                "⚠".yellow(),
                report.warnings.len()
            );
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validation_report() {
        let mut report = ValidationReport::new();
        assert!(!report.has_errors());

        report.add_error("Test error".to_string());
        assert!(report.has_errors());

        report.add_warning("Test warning".to_string());
        report.add_info("Test info".to_string());

        assert_eq!(report.errors.len(), 1);
        assert_eq!(report.warnings.len(), 1);
        assert_eq!(report.info.len(), 1);
    }
}
