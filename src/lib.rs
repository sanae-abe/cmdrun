//! cmdrun library
//!
//! A fast, secure, and cross-platform command runner.

pub mod cli;
pub mod command;
pub mod commands;
pub mod config;
pub mod error;
pub mod history;
pub mod i18n;
pub mod output;
pub mod platform;
pub mod plugin;
pub mod security;
pub mod template;
pub mod utils;
pub mod watch;

// Re-export commonly used types
pub use error::{CmdrunError, Result};
