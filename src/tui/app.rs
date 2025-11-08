//! Application state for TUI interactive mode

use crate::config::schema::{Command, CommandsConfig};
use crate::history::HistoryStorage;
use ahash::AHashMap;
use std::path::PathBuf;

/// Application state for the interactive TUI
pub struct App {
    /// Configuration containing all commands
    pub config: CommandsConfig,

    /// Optional path to the config file
    pub config_path: Option<PathBuf>,

    /// Current search input
    pub search_input: String,

    /// Filtered list of command names based on search
    pub filtered_commands: Vec<String>,

    /// Currently selected command index
    pub selected_index: usize,

    /// History storage for execution counts and last run times
    pub history_storage: Option<HistoryStorage>,

    /// Cached command statistics (name -> (count, last_run_timestamp))
    pub command_stats: AHashMap<String, (i64, Option<String>)>,
}

impl App {
    /// Create a new App instance with the given configuration
    pub fn new(config: CommandsConfig, config_path: Option<PathBuf>) -> Self {
        // Initialize history storage (ignore errors)
        let history_storage = HistoryStorage::new().ok();

        // Load command statistics from history
        let mut command_stats = AHashMap::new();
        if let Some(ref storage) = history_storage {
            // Get statistics for all commands
            for command_name in config.commands.keys() {
                if let Ok(stats) = storage.get_command_stats(command_name) {
                    command_stats
                        .insert(command_name.clone(), (stats.total_count, stats.last_run_at));
                }
            }
        }

        // Initially show all commands
        let mut filtered_commands: Vec<String> = config.commands.keys().cloned().collect();
        filtered_commands.sort();

        Self {
            config,
            config_path,
            search_input: String::new(),
            filtered_commands,
            selected_index: 0,
            history_storage,
            command_stats,
        }
    }

    /// Update the search input and filter commands
    pub fn update_search(&mut self, input: String) {
        self.search_input = input;
        self.filter_commands();
    }

    /// Filter commands based on current search input
    fn filter_commands(&mut self) {
        use crate::tui::fuzzy::fuzzy_match_commands;

        if self.search_input.is_empty() {
            // Show all commands when search is empty
            self.filtered_commands = self.config.commands.keys().cloned().collect();
            self.filtered_commands.sort();
        } else {
            // Fuzzy match commands
            self.filtered_commands = fuzzy_match_commands(
                &self.search_input,
                &self.config.commands.keys().cloned().collect::<Vec<_>>(),
            );
        }

        // Reset selection to first item
        self.selected_index = 0;
    }

    /// Move selection up
    pub fn select_previous(&mut self) {
        if self.selected_index > 0 {
            self.selected_index -= 1;
        }
    }

    /// Move selection down
    pub fn select_next(&mut self) {
        if !self.filtered_commands.is_empty()
            && self.selected_index < self.filtered_commands.len() - 1
        {
            self.selected_index += 1;
        }
    }

    /// Get the currently selected command
    pub fn selected_command(&self) -> Option<&String> {
        self.filtered_commands.get(self.selected_index)
    }

    /// Get the currently selected command details
    pub fn selected_command_details(&self) -> Option<(&String, &Command)> {
        self.selected_command()
            .and_then(|name| self.config.commands.get(name).map(|cmd| (name, cmd)))
    }

    /// Get statistics for a command (execution count, last run time)
    pub fn get_command_stats(&self, command_name: &str) -> Option<(i64, Option<&str>)> {
        self.command_stats
            .get(command_name)
            .map(|(count, last_run)| (*count, last_run.as_deref()))
    }

    /// Add a character to the search input
    pub fn push_char(&mut self, c: char) {
        self.search_input.push(c);
        self.filter_commands();
    }

    /// Remove the last character from the search input
    pub fn pop_char(&mut self) {
        self.search_input.pop();
        self.filter_commands();
    }

    /// Clear the search input
    pub fn clear_search(&mut self) {
        self.search_input.clear();
        self.filter_commands();
    }
}
