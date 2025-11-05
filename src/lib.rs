//! cmdrun library
//!
//! A fast, secure, and cross-platform command runner.

pub mod cli;
pub mod command;
pub mod config;
pub mod error;
pub mod output;
pub mod platform;
pub mod utils;

// Re-export commonly used types
pub use error::{CmdrunError, Result};
