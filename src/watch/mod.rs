//! File watching infrastructure for cmdrun
//!
//! This module provides the built-in file watching capabilities used by the watch mode feature.
//! It uses the `notify` crate for cross-platform filesystem monitoring with debouncing support.

mod config;
mod debouncer;
mod executor;
mod matcher;
mod watcher;

pub use config::{WatchConfig, WatchPattern};
pub use debouncer::FileDebouncer;
pub use executor::CommandExecutor;
pub use matcher::PatternMatcher;
pub use watcher::{WatchEvent, WatchRunner};

use anyhow::Result;

/// Initialize the watch system
pub fn init() -> Result<()> {
    tracing::info!("File watching system initialized");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init() {
        assert!(init().is_ok());
    }
}
