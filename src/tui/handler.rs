//! Event handling for TUI interactive mode

use super::app::App;
use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use std::time::Duration;

/// Actions that can be triggered by user input
pub enum Action {
    /// Quit the application
    Quit,
    /// Execute the selected command
    Execute(String),
}

/// Handle terminal events and update application state
///
/// Returns Some(Action) if an action should be performed, None otherwise
pub fn handle_events(app: &mut App) -> Result<Option<Action>> {
    // Poll for events with timeout
    if event::poll(Duration::from_millis(100))? {
        if let Event::Key(key) = event::read()? {
            // Only handle key press events (ignore release)
            if key.kind == KeyEventKind::Press {
                return handle_key_event(app, key);
            }
        }
    }
    Ok(None)
}

/// Handle keyboard input
fn handle_key_event(app: &mut App, key: KeyEvent) -> Result<Option<Action>> {
    match key.code {
        // Quit keys
        KeyCode::Esc | KeyCode::Char('q') => {
            // Don't quit if we're typing 'q' in the search field
            if key.code == KeyCode::Char('q') && !app.search_input.is_empty() {
                app.push_char('q');
                return Ok(None);
            }
            Ok(Some(Action::Quit))
        }

        // Ctrl+C to quit
        KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            Ok(Some(Action::Quit))
        }

        // Execute selected command
        KeyCode::Enter => {
            if let Some(command_name) = app.selected_command() {
                Ok(Some(Action::Execute(command_name.clone())))
            } else {
                Ok(None)
            }
        }

        // Navigation
        KeyCode::Up | KeyCode::Char('k') if key.modifiers.is_empty() => {
            app.select_previous();
            Ok(None)
        }
        KeyCode::Down | KeyCode::Char('j') if key.modifiers.is_empty() => {
            app.select_next();
            Ok(None)
        }

        // Ctrl+P / Ctrl+N for navigation (Emacs-style)
        KeyCode::Char('p') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            app.select_previous();
            Ok(None)
        }
        KeyCode::Char('n') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            app.select_next();
            Ok(None)
        }

        // Backspace to delete character
        KeyCode::Backspace => {
            app.pop_char();
            Ok(None)
        }

        // Ctrl+U to clear search
        KeyCode::Char('u') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            app.clear_search();
            Ok(None)
        }

        // Ctrl+W to delete word
        KeyCode::Char('w') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            // Find last space or beginning of string
            if let Some(pos) = app.search_input.rfind(' ') {
                app.update_search(app.search_input[..=pos].to_string());
            } else {
                app.clear_search();
            }
            Ok(None)
        }

        // Regular character input
        KeyCode::Char(c) if key.modifiers.is_empty() || key.modifiers == KeyModifiers::SHIFT => {
            app.push_char(c);
            Ok(None)
        }

        _ => Ok(None),
    }
}
