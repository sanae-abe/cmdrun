//! Completion command implementation

use crate::cli::Cli;
use clap::CommandFactory;
use clap_complete::{generate, Shell};
use colored::Colorize;
use std::io::{self, Write};

/// Handle completion command
pub fn handle_completion(shell: Shell) {
    let mut cmd = Cli::command();
    let mut buf = Vec::new();
    generate(shell, &mut cmd, "cmdrun", &mut buf);

    // Write generated completion
    io::stdout().write_all(&buf).unwrap();

    // Add custom completion functions for command descriptions
    add_custom_completions(shell);

    // Print installation instructions to stderr (won't interfere with eval)
    eprintln!();
    eprintln!("{}", "Installation instructions:".cyan().bold());
    print_installation_instructions(shell);
}

/// Add custom completion functions for command descriptions
fn add_custom_completions(shell: Shell) {
    match shell {
        Shell::Bash => {
            println!(r#"
# Custom completion for 'cmdrun run' and 'cmdrun info' with descriptions
_cmdrun_complete_commands() {{
    local IFS=$'\n'
    local suggestions=($(cmdrun completion-list 2>/dev/null))

    if [ "${{#suggestions[@]}}" == "0" ]; then
        return 1
    fi

    # Parse "name:description" format
    local names=()
    local descs=()
    COMPREPLY=()

    for suggestion in "${{suggestions[@]}}"; do
        local name="${{suggestion%%:*}}"
        local desc="${{suggestion#*:}}"
        names+=("$name")
        descs+=("$desc")
        # Add name only to COMPREPLY (Bash doesn't natively support descriptions)
        COMPREPLY+=("$name")
    done

    # Try to display descriptions using bash-completion 2.11+ feature
    # This may not work on all systems
    if declare -F _comp_cmd_complete__descriptions &>/dev/null; then
        _comp_cmd_complete__descriptions "${{names[@]}}" -- "${{descs[@]}}"
    fi

    return 0
}}

# Intercept and wrap the clap-generated _cmdrun function
# This must be done after clap generates the completion function
_cmdrun_wrap_completion() {{
    # Check if _cmdrun exists
    if ! declare -F _cmdrun > /dev/null 2>&1; then
        return 1
    fi

    # Save original function
    eval "$(declare -f _cmdrun | sed '1s/^_cmdrun/_cmdrun_original/')"

    # Redefine _cmdrun with our hook
    _cmdrun() {{
        # Check if completing the name argument for 'run' or 'info'
        if [[ $COMP_CWORD -eq 2 ]]; then
            case "${{COMP_WORDS[1]}}" in
                run|r|info|i)
                    _cmdrun_complete_commands && return 0
                    ;;
            esac
        fi

        # Fall back to original completion
        _cmdrun_original "$@"
    }}
}}

# Execute the wrapper after this script loads
_cmdrun_wrap_completion
"#);
        }
        Shell::Zsh => {
            println!(r#"
# Custom completion for 'cmdrun run' and 'cmdrun info' with descriptions
_cmdrun_commands_with_desc() {{
    local -a commands
    local line

    while IFS= read -r line; do
        [[ -z "$line" ]] && continue
        commands+=("$line")
    done < <(cmdrun completion-list 2>/dev/null)

    # If no commands found, return failure
    if (( ${{#commands}} == 0 )); then
        return 1
    fi

    # Use _describe (simple and reliable)
    _describe 'available commands' commands
}}

# Wrap the original _cmdrun function to inject custom completion
# Save the original function
functions[_cmdrun_original]=${{functions[_cmdrun]}}

# Redefine _cmdrun with custom logic
_cmdrun() {{
    # Check if we're completing 'run' or 'info' subcommand's name argument
    if [[ ${{words[2]}} == "run" || ${{words[2]}} == "info" || ${{words[2]}} == "r" || ${{words[2]}} == "i" ]]; then
        if [[ $CURRENT == 3 ]]; then
            _cmdrun_commands_with_desc && return 0
        fi
    fi

    # Otherwise, use the original completion
    _cmdrun_original "$@"
}}

# Configure completion style for cmdrun to show menu immediately
zstyle ':completion:*:*:cmdrun:*' menu yes select
"#);
        }
        Shell::Fish => {
            println!(r#"
# Custom completion for 'cmdrun run' and 'cmdrun info' with descriptions
complete -c cmdrun -n "__fish_seen_subcommand_from run; and not __fish_seen_subcommand_from (cmdrun completion-list 2>/dev/null | string replace -r ':.*' '')" -f -a "
(cmdrun completion-list 2>/dev/null | string replace ':' \t)"

complete -c cmdrun -n "__fish_seen_subcommand_from info; and not __fish_seen_subcommand_from (cmdrun completion-list 2>/dev/null | string replace -r ':.*' '')" -f -a "
(cmdrun completion-list 2>/dev/null | string replace ':' \t)"
"#);
        }
        _ => {
            // Other shells don't have good support for descriptions in completions
        }
    }
}

/// Print shell-specific installation instructions
fn print_installation_instructions(shell: Shell) {
    match shell {
        Shell::Bash => {
            eprintln!();
            eprintln!("  Add to your {}:", "~/.bashrc".green());
            eprintln!("    {}", "eval \"$(cmdrun completion bash)\"".dimmed());
            eprintln!();
            eprintln!("  Or save to completion directory:");
            eprintln!(
                "    {}",
                "cmdrun completion bash > /etc/bash_completion.d/cmdrun".dimmed()
            );
        }
        Shell::Zsh => {
            eprintln!();
            eprintln!("  Add to your {}:", "~/.zshrc".green());
            eprintln!("    {}", "eval \"$(cmdrun completion zsh)\"".dimmed());
            eprintln!();
            eprintln!("  Or save to completion directory:");
            eprintln!(
                "    {}",
                "cmdrun completion zsh > \"${fpath[1]}/_cmdrun\"".dimmed()
            );
        }
        Shell::Fish => {
            eprintln!();
            eprintln!("  Save to Fish completion directory:");
            eprintln!(
                "    {}",
                "cmdrun completion fish > ~/.config/fish/completions/cmdrun.fish".dimmed()
            );
        }
        Shell::PowerShell => {
            eprintln!();
            eprintln!("  Add to your PowerShell profile:");
            eprintln!(
                "    {}",
                "cmdrun completion powershell | Out-String | Invoke-Expression".dimmed()
            );
            eprintln!();
            eprintln!("  Or save to a file and dot-source it:");
            eprintln!(
                "    {}",
                "cmdrun completion powershell > cmdrun.ps1".dimmed()
            );
            eprintln!("    {}", ". ./cmdrun.ps1".dimmed());
        }
        Shell::Elvish => {
            eprintln!();
            eprintln!("  Add to your {}:", "~/.elvish/rc.elv".green());
            eprintln!("    {}", "eval (cmdrun completion elvish)".dimmed());
        }
        _ => {
            eprintln!();
            eprintln!("  See your shell's documentation for completion setup.");
        }
    }

    eprintln!();
    eprintln!(
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
            assert!(
                !buf.is_empty(),
                "Failed to generate completion for {:?}",
                shell
            );
        }
    }
}
