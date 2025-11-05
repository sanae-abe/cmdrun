//! Main file watcher implementation

use anyhow::{Context, Result};
use notify::{EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use notify_debouncer_full::{new_debouncer, DebounceEventResult, Debouncer, FileIdMap};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::mpsc;
use tracing::{debug, error, info};

use super::config::WatchConfig;
use super::debouncer::FileDebouncer;
use super::executor::CommandExecutor;
use super::matcher::PatternMatcher;

/// Watch event information
#[derive(Debug, Clone)]
pub struct WatchEvent {
    /// Path that changed
    pub path: PathBuf,

    /// Event kind
    pub kind: EventKind,
}

/// Watch runner that monitors filesystem changes and executes commands
pub struct WatchRunner {
    /// Watch configuration
    config: WatchConfig,

    /// Command to execute on file changes
    command: String,

    /// Pattern matcher
    matcher: Arc<PatternMatcher>,

    /// Custom debouncer
    debouncer: FileDebouncer,

    /// Command executor
    executor: CommandExecutor,
}

impl WatchRunner {
    /// Create a new watch runner
    pub fn new(config: WatchConfig, command: String, base_path: &Path) -> Result<Self> {
        let matcher = Arc::new(PatternMatcher::from_config(&config, base_path)?);
        let debouncer = FileDebouncer::new(config.debounce_duration());
        let executor = CommandExecutor::new(base_path.to_path_buf());

        Ok(Self {
            config,
            command,
            matcher,
            debouncer,
            executor,
        })
    }

    /// Start watching and executing commands
    pub async fn run(&mut self) -> Result<()> {
        info!(
            paths = ?self.config.paths,
            command = %self.command,
            "Starting watch mode"
        );

        let (tx, mut rx) = mpsc::channel(100);
        let matcher = Arc::clone(&self.matcher);

        // Create debounced watcher
        let mut debouncer: Debouncer<RecommendedWatcher, FileIdMap> = new_debouncer(
            self.config.debounce_duration(),
            None,
            move |result: DebounceEventResult| {
                match result {
                    Ok(events) => {
                        for event in events {
                            // Filter events by pattern matcher
                            for path in &event.paths {
                                if matcher.should_watch(path) {
                                    let watch_event = WatchEvent {
                                        path: path.clone(),
                                        kind: event.kind,
                                    };
                                    // Send event to channel (ignore send errors if receiver dropped)
                                    let _ = tx.blocking_send(watch_event);
                                }
                            }
                        }
                    }
                    Err(errors) => {
                        for error in errors {
                            error!(error = %error, "Watch error");
                        }
                    }
                }
            },
        )
        .context("Failed to create file watcher")?;

        // Add paths to watch
        for path in &self.config.paths {
            let mode = if self.config.recursive {
                RecursiveMode::Recursive
            } else {
                RecursiveMode::NonRecursive
            };

            debouncer
                .watcher()
                .watch(path, mode)
                .with_context(|| format!("Failed to watch path: {}", path.display()))?;

            info!(path = %path.display(), recursive = self.config.recursive, "Watching path");
        }

        info!("Watch mode started. Press Ctrl+C to stop.");

        // Process events
        while let Some(event) = rx.recv().await {
            if self.debouncer.should_process(&event.path) {
                debug!(
                    path = %event.path.display(),
                    kind = ?event.kind,
                    "File changed, executing command"
                );

                if let Err(e) = self.executor.execute(&self.command, &event.path).await {
                    error!(
                        error = %e,
                        path = %event.path.display(),
                        "Failed to execute command"
                    );
                } else {
                    info!(path = %event.path.display(), "Command executed successfully");
                }
            } else {
                debug!(path = %event.path.display(), "Event debounced");
            }
        }

        Ok(())
    }

    /// Get a reference to the matcher
    pub fn matcher(&self) -> &PatternMatcher {
        &self.matcher
    }

    /// Get a mutable reference to the executor
    pub fn executor_mut(&mut self) -> &mut CommandExecutor {
        &mut self.executor
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_watch_event_creation() {
        let event = WatchEvent {
            path: PathBuf::from("test.txt"),
            kind: EventKind::Create(notify::event::CreateKind::File),
        };

        assert_eq!(event.path, PathBuf::from("test.txt"));
    }

    #[tokio::test]
    async fn test_watch_runner_creation() {
        let temp_dir = TempDir::new().unwrap();
        let config = WatchConfig::new()
            .add_path(temp_dir.path())
            .add_pattern("*.txt");

        let runner = WatchRunner::new(config, "echo test".to_string(), temp_dir.path());
        assert!(runner.is_ok());
    }

    #[tokio::test]
    async fn test_watch_runner_with_invalid_pattern() {
        let temp_dir = TempDir::new().unwrap();
        let config = WatchConfig::new()
            .add_path(temp_dir.path())
            .add_pattern("[invalid");

        let runner = WatchRunner::new(config, "echo test".to_string(), temp_dir.path());
        assert!(runner.is_err());
    }

    #[tokio::test]
    async fn test_matcher_access() {
        let temp_dir = TempDir::new().unwrap();
        let mut config = WatchConfig::new();
        config.paths = vec![temp_dir.path().to_path_buf()];
        config.patterns = vec![super::super::config::WatchPattern {
            pattern: "*.rs".to_string(),
            case_insensitive: false,
        }];

        let runner = WatchRunner::new(config, "echo test".to_string(), temp_dir.path()).unwrap();

        let matcher = runner.matcher();
        assert!(matcher.should_watch(Path::new("test.rs")));
        assert!(!matcher.should_watch(Path::new("test.txt")));
    }

    #[tokio::test]
    async fn test_executor_mut_access() {
        let temp_dir = TempDir::new().unwrap();
        let config = WatchConfig::new().add_path(temp_dir.path());

        let mut runner =
            WatchRunner::new(config, "echo test".to_string(), temp_dir.path()).unwrap();

        let executor = runner.executor_mut();
        // Test that executor can be modified
        assert_eq!(executor.working_dir, temp_dir.path());
    }
}
