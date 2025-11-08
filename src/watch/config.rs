//! Watch configuration structures

use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::time::Duration;

/// Watch configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WatchConfig {
    /// Paths to watch
    pub paths: Vec<PathBuf>,

    /// Patterns to include (glob patterns)
    pub patterns: Vec<WatchPattern>,

    /// Patterns to exclude (glob patterns)
    pub exclude: Vec<String>,

    /// Debounce delay in milliseconds
    #[serde(default = "default_debounce_ms")]
    pub debounce_ms: u64,

    /// Whether to ignore .gitignore files
    #[serde(default)]
    pub ignore_gitignore: bool,

    /// Whether to watch recursively
    #[serde(default = "default_recursive")]
    pub recursive: bool,

    /// Whether to follow symlinks (default: false for security)
    #[serde(default)]
    pub follow_symlinks: bool,

    /// Whether to warn about symlinks (default: true)
    #[serde(default = "default_warn_symlinks")]
    pub warn_on_symlinks: bool,
}

/// Watch pattern configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WatchPattern {
    /// Glob pattern (e.g., "**/*.rs")
    pub pattern: String,

    /// Whether to match case-insensitively
    #[serde(default)]
    pub case_insensitive: bool,
}

impl Default for WatchConfig {
    fn default() -> Self {
        Self {
            paths: vec![PathBuf::from(".")],
            patterns: vec![WatchPattern {
                pattern: "**/*".to_string(),
                case_insensitive: false,
            }],
            exclude: vec![],
            debounce_ms: default_debounce_ms(),
            ignore_gitignore: false,
            recursive: default_recursive(),
            follow_symlinks: false,
            warn_on_symlinks: default_warn_symlinks(),
        }
    }
}

fn default_debounce_ms() -> u64 {
    500 // 500ms default debounce
}

fn default_recursive() -> bool {
    true
}

fn default_warn_symlinks() -> bool {
    true
}

impl WatchConfig {
    /// Validate watch path for symlink security
    pub fn validate_watch_path(&self, path: &Path) -> anyhow::Result<()> {
        use anyhow::anyhow;
        use tracing::warn;

        if path.is_symlink() {
            let target = std::fs::read_link(path).unwrap_or_else(|_| PathBuf::from("<unreadable>"));

            if self.warn_on_symlinks {
                warn!(
                    "⚠️  Watching symlink: {} -> {}",
                    path.display(),
                    target.display()
                );
                warn!("   Symlinks can be used to watch sensitive system files");
            }

            if !self.follow_symlinks {
                return Err(anyhow!(
                    "Symlink watching disabled for security: {} -> {}. Use --follow-symlinks to override",
                    path.display(),
                    target.display()
                ));
            }
        }

        Ok(())
    }
}

impl WatchConfig {
    /// Create a new watch configuration
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a path to watch
    pub fn add_path<P: Into<PathBuf>>(mut self, path: P) -> Self {
        self.paths.push(path.into());
        self
    }

    /// Add a pattern to include
    pub fn add_pattern<S: Into<String>>(mut self, pattern: S) -> Self {
        self.patterns.push(WatchPattern {
            pattern: pattern.into(),
            case_insensitive: false,
        });
        self
    }

    /// Add a pattern to exclude
    pub fn add_exclude<S: Into<String>>(mut self, pattern: S) -> Self {
        self.exclude.push(pattern.into());
        self
    }

    /// Set debounce delay
    pub fn debounce(mut self, ms: u64) -> Self {
        self.debounce_ms = ms;
        self
    }

    /// Get debounce duration
    pub fn debounce_duration(&self) -> Duration {
        Duration::from_millis(self.debounce_ms)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = WatchConfig::default();
        assert_eq!(config.paths.len(), 1);
        assert_eq!(config.debounce_ms, 500);
        assert!(config.recursive);
    }

    #[test]
    fn test_builder_pattern() {
        let config = WatchConfig::new()
            .add_path("/tmp/test")
            .add_pattern("*.rs")
            .add_exclude("target/**")
            .debounce(300);

        assert_eq!(config.paths.len(), 2);
        assert_eq!(config.patterns.len(), 2);
        assert_eq!(config.exclude.len(), 1);
        assert_eq!(config.debounce_ms, 300);
    }

    #[test]
    fn test_debounce_duration() {
        let config = WatchConfig::new().debounce(1000);
        assert_eq!(config.debounce_duration(), Duration::from_millis(1000));
    }
}
