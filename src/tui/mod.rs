//! Terminal User Interface (TUI) interactive mode
//!
//! This module provides an interactive fuzzy finder interface for command selection
//! and execution using ratatui.

pub mod app;
pub mod fuzzy;
pub mod handler;
pub mod ui;

use anyhow::Result;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;

pub use app::App;

/// Run the interactive TUI mode
///
/// This function initializes the terminal, creates the application state,
/// and runs the main event loop for the interactive command selector.
pub async fn run_interactive(
    config: crate::config::schema::CommandsConfig,
    config_path: Option<std::path::PathBuf>,
) -> Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app state
    let app = App::new(config, config_path);
    let res = run_app(&mut terminal, app).await;

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        eprintln!("Error: {}", err);
    }

    Ok(())
}

/// Run the main application loop
async fn run_app<B>(terminal: &mut Terminal<B>, mut app: App) -> Result<()>
where
    B: ratatui::backend::Backend,
    B: std::io::Write,
{
    loop {
        terminal.draw(|f| ui::draw(f, &mut app))?;

        if let Some(action) = handler::handle_events(&mut app)? {
            match action {
                handler::Action::Quit => break,
                handler::Action::Execute(command_name) => {
                    // Exit TUI mode to execute command
                    disable_raw_mode()?;
                    execute!(
                        terminal.backend_mut(),
                        LeaveAlternateScreen,
                        DisableMouseCapture
                    )?;
                    terminal.show_cursor()?;

                    // Execute the selected command
                    if let Err(e) = execute_command(&command_name, &app).await {
                        eprintln!("Error executing command: {}", e);
                        // Wait for user to press Enter to continue
                        println!("\nPress Enter to continue...");
                        let mut input = String::new();
                        io::stdin().read_line(&mut input)?;
                    }

                    // Re-enter TUI mode
                    enable_raw_mode()?;
                    execute!(
                        terminal.backend_mut(),
                        EnterAlternateScreen,
                        EnableMouseCapture
                    )?;
                }
            }
        }
    }

    Ok(())
}

/// Execute a selected command
async fn execute_command(command_name: &str, app: &App) -> Result<()> {
    use crate::command::executor::{CommandExecutor, ExecutionContext};
    use crate::platform::shell::detect_shell;
    use colored::*;

    let command = app
        .config
        .commands
        .get(command_name)
        .ok_or_else(|| anyhow::anyhow!("Command not found: {}", command_name))?;

    println!(
        "\n{} {}",
        "Running:".cyan().bold(),
        command.description.bright_white()
    );

    // Create execution context
    let ctx = ExecutionContext {
        working_dir: app.config.config.working_dir.clone(),
        env: app.config.config.env.clone(),
        shell: detect_shell()
            .map(|s| s.name)
            .unwrap_or_else(|_| app.config.config.shell.clone()),
        timeout: command.timeout.or(Some(app.config.config.timeout)),
        strict: app.config.config.strict_mode,
        echo: true,
        color: true,
        language: app.config.config.language,
    };

    let executor = CommandExecutor::new(ctx);
    let result = executor.execute(command).await?;

    if result.success {
        println!(
            "{} Completed in {:.2}s",
            "✓".green().bold(),
            result.duration.as_secs_f64()
        );
    } else {
        anyhow::bail!("Command failed with exit code {}", result.exit_code);
    }

    Ok(())
}
