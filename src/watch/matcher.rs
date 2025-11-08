//! Pattern matching for file paths

use anyhow::{Context, Result};
use globset::{Glob, GlobSet, GlobSetBuilder};
use ignore::gitignore::{Gitignore, GitignoreBuilder};
use std::path::Path;

use super::config::WatchConfig;

/// Pattern matcher for filtering watched files
pub struct PatternMatcher {
    /// Include patterns (glob set)
    include: GlobSet,

    /// Exclude patterns (glob set)
    exclude: GlobSet,

    /// Gitignore matcher (optional)
    gitignore: Option<Gitignore>,
}

impl PatternMatcher {
    /// Create a new pattern matcher from configuration
    pub fn from_config(config: &WatchConfig, base_path: &Path) -> Result<Self> {
        // Build include patterns
        let mut include_builder = GlobSetBuilder::new();
        for pattern in &config.patterns {
            let glob = Glob::new(&pattern.pattern)
                .with_context(|| format!("Invalid include pattern: {}", pattern.pattern))?;
            include_builder.add(glob);
        }
        let include = include_builder
            .build()
            .context("Failed to build include pattern set")?;

        // Build exclude patterns
        let mut exclude_builder = GlobSetBuilder::new();
        for pattern in &config.exclude {
            let glob = Glob::new(pattern)
                .with_context(|| format!("Invalid exclude pattern: {}", pattern))?;
            exclude_builder.add(glob);
        }
        let exclude = exclude_builder
            .build()
            .context("Failed to build exclude pattern set")?;

        // Build gitignore matcher if needed
        let gitignore = if !config.ignore_gitignore {
            let mut builder = GitignoreBuilder::new(base_path);
            let gitignore_path = base_path.join(".gitignore");
            if gitignore_path.exists() {
                builder.add(&gitignore_path);
            }
            Some(builder.build()?)
        } else {
            None
        };

        Ok(Self {
            include,
            exclude,
            gitignore,
        })
    }

    /// Check if a path should be watched
    pub fn should_watch(&self, path: &Path) -> bool {
        // Check exclude patterns first (faster rejection)
        if self.exclude.is_match(path) {
            return false;
        }

        // Check gitignore if enabled
        if let Some(ref gitignore) = self.gitignore {
            if gitignore.matched(path, path.is_dir()).is_ignore() {
                return false;
            }
        }

        // Check include patterns
        self.include.is_match(path)
    }

    /// Check if a path matches include patterns only (ignore exclude/gitignore)
    pub fn matches_include(&self, path: &Path) -> bool {
        self.include.is_match(path)
    }

    /// Check if a path is excluded
    pub fn is_excluded(&self, path: &Path) -> bool {
        self.exclude.is_match(path)
            || self
                .gitignore
                .as_ref()
                .map(|g| g.matched(path, path.is_dir()).is_ignore())
                .unwrap_or(false)
    }
}

#[cfg(test)]
mod tests {
    use super::super::config::WatchPattern;
    use super::*;
    use std::path::PathBuf;
    use tempfile::TempDir;

    fn create_test_config(patterns: Vec<&str>, exclude: Vec<&str>) -> WatchConfig {
        WatchConfig {
            paths: vec![PathBuf::from(".")],
            patterns: patterns
                .into_iter()
                .map(|p| WatchPattern {
                    pattern: p.to_string(),
                    case_insensitive: false,
                })
                .collect(),
            exclude: exclude.into_iter().map(String::from).collect(),
            debounce_ms: 500,
            ignore_gitignore: true, // Disable gitignore for tests
            follow_symlinks: false,
            warn_on_symlinks: false,
            recursive: true,
        }
    }

    #[test]
    fn test_basic_pattern_matching() {
        let config = create_test_config(vec!["**/*.rs"], vec![]);
        let temp_dir = TempDir::new().unwrap();
        let matcher = PatternMatcher::from_config(&config, temp_dir.path()).unwrap();

        assert!(matcher.should_watch(Path::new("src/main.rs")));
        assert!(matcher.should_watch(Path::new("src/lib/mod.rs")));
        assert!(!matcher.should_watch(Path::new("README.md")));
        assert!(!matcher.should_watch(Path::new("Cargo.toml")));
    }

    #[test]
    fn test_exclude_patterns() {
        let config = create_test_config(vec!["**/*"], vec!["target/**", "*.tmp"]);
        let temp_dir = TempDir::new().unwrap();
        let matcher = PatternMatcher::from_config(&config, temp_dir.path()).unwrap();

        assert!(matcher.should_watch(Path::new("src/main.rs")));
        assert!(!matcher.should_watch(Path::new("target/debug/app")));
        assert!(!matcher.should_watch(Path::new("temp.tmp")));
    }

    #[test]
    fn test_multiple_include_patterns() {
        let config = create_test_config(vec!["**/*.rs", "**/*.toml"], vec![]);
        let temp_dir = TempDir::new().unwrap();
        let matcher = PatternMatcher::from_config(&config, temp_dir.path()).unwrap();

        assert!(matcher.should_watch(Path::new("src/main.rs")));
        assert!(matcher.should_watch(Path::new("Cargo.toml")));
        assert!(!matcher.should_watch(Path::new("README.md")));
    }

    #[test]
    fn test_is_excluded() {
        let config = create_test_config(vec!["**/*"], vec!["target/**"]);
        let temp_dir = TempDir::new().unwrap();
        let matcher = PatternMatcher::from_config(&config, temp_dir.path()).unwrap();

        assert!(!matcher.is_excluded(Path::new("src/main.rs")));
        assert!(matcher.is_excluded(Path::new("target/debug/app")));
    }

    #[test]
    fn test_matches_include() {
        let config = create_test_config(vec!["**/*.rs"], vec!["target/**"]);
        let temp_dir = TempDir::new().unwrap();
        let matcher = PatternMatcher::from_config(&config, temp_dir.path()).unwrap();

        // target/**/*.rs matches include pattern even though target/** is excluded
        assert!(matcher.matches_include(Path::new("target/debug/main.rs")));
        assert!(!matcher.should_watch(Path::new("target/debug/main.rs"))); // But it's excluded
    }
}
