//! Completion command implementation

use crate::cli::Cli;
use clap::CommandFactory;
use clap_complete::{generate, Shell};
use colored::Colorize;
use std::io;

/// Handle completion command
pub fn handle_completion(shell: Shell) {
    println!(
        "{} Generating {} completion script...",
        "→".cyan(),
        shell.to_string().green()
    );
    println!();

    let mut cmd = Cli::command();
    generate(shell, &mut cmd, "cmdrun", &mut io::stdout());

    println!();
    println!("{}", "Installation instructions:".cyan().bold());
    print_installation_instructions(shell);
}

/// Print shell-specific installation instructions
fn print_installation_instructions(shell: Shell) {
    match shell {
        Shell::Bash => {
            println!();
            println!("  Add to your {}:", "~/.bashrc".green());
            println!("    {}", "eval \"$(cmdrun completion bash)\"".dimmed());
            println!();
            println!("  Or save to completion directory:");
            println!("    {}", "cmdrun completion bash > /etc/bash_completion.d/cmdrun".dimmed());
        }
        Shell::Zsh => {
            println!();
            println!("  Add to your {}:", "~/.zshrc".green());
            println!("    {}", "eval \"$(cmdrun completion zsh)\"".dimmed());
            println!();
            println!("  Or save to completion directory:");
            println!("    {}", "cmdrun completion zsh > \"${fpath[1]}/_cmdrun\"".dimmed());
        }
        Shell::Fish => {
            println!();
            println!("  Save to Fish completion directory:");
            println!(
                "    {}",
                "cmdrun completion fish > ~/.config/fish/completions/cmdrun.fish".dimmed()
            );
        }
        Shell::PowerShell => {
            println!();
            println!("  Add to your PowerShell profile:");
            println!(
                "    {}",
                "cmdrun completion powershell | Out-String | Invoke-Expression".dimmed()
            );
            println!();
            println!("  Or save to a file and dot-source it:");
            println!(
                "    {}",
                "cmdrun completion powershell > cmdrun.ps1".dimmed()
            );
            println!("    {}", ". ./cmdrun.ps1".dimmed());
        }
        Shell::Elvish => {
            println!();
            println!("  Add to your {}:", "~/.elvish/rc.elv".green());
            println!("    {}", "eval (cmdrun completion elvish)".dimmed());
        }
        _ => {
            println!();
            println!("  See your shell's documentation for completion setup.");
        }
    }

    println!();
    println!(
        "{} After installation, restart your shell or source the config file.",
        "Note:".yellow()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_completion() {
        // Test that completion generation doesn't panic
        let mut cmd = Cli::command();
        let mut buf = Vec::new();
        generate(Shell::Bash, &mut cmd, "cmdrun", &mut buf);
        assert!(!buf.is_empty());
    }

    #[test]
    fn test_all_shells() {
        let shells = [
            Shell::Bash,
            Shell::Zsh,
            Shell::Fish,
            Shell::PowerShell,
            Shell::Elvish,
        ];

        for shell in shells {
            let mut cmd = Cli::command();
            let mut buf = Vec::new();
            generate(shell, &mut cmd, "cmdrun", &mut buf);
            assert!(!buf.is_empty(), "Failed to generate completion for {:?}", shell);
        }
    }
}
