//! Command execution history tracking
//!
//! This module provides functionality to record, search, and manage
//! command execution history.

pub mod recorder;
pub mod storage;

pub use recorder::HistoryRecorder;
pub use storage::{HistoryEntry, HistoryStorage};
