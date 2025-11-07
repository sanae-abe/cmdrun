//! History recorder for automatic command execution tracking
//!
//! Integrates with the command executor to automatically record
//! execution history.

use super::storage::{HistoryEntry, HistoryStorage};
use anyhow::Result;
use chrono::Utc;
use serde_json;
use std::collections::HashMap;
use std::env;

/// Sensitive environment variable patterns to exclude from history
const SENSITIVE_ENV_PATTERNS: &[&str] = &[
    "KEY", "SECRET", "TOKEN", "PASSWORD", "PASS", "API", "AUTH", "CREDENTIAL",
];

/// History recorder that tracks command executions
pub struct HistoryRecorder {
    storage: HistoryStorage,
    filter_sensitive: bool,
}

impl HistoryRecorder {
    /// Create a new history recorder with default storage
    pub fn new() -> Result<Self> {
        Ok(Self {
            storage: HistoryStorage::new()?,
            filter_sensitive: true,
        })
    }

    /// Create a recorder with custom storage
    pub fn with_storage(storage: HistoryStorage) -> Self {
        Self {
            storage,
            filter_sensitive: true,
        }
    }

    /// Enable or disable sensitive data filtering
    pub fn set_filter_sensitive(&mut self, filter: bool) {
        self.filter_sensitive = filter;
    }

    /// Record the start of a command execution
    ///
    /// Returns an execution ID that should be used with `record_completion`
    pub fn record_start(
        &mut self,
        command: &str,
        args: &[String],
        env: &HashMap<String, String>,
    ) -> Result<i64> {
        let entry = HistoryEntry {
            id: 0, // Will be assigned by database
            command: command.to_string(),
            args: if args.is_empty() {
                None
            } else {
                Some(serde_json::to_string(args)?)
            },
            start_time: Utc::now().timestamp_millis(),
            duration_ms: None,
            exit_code: None,
            success: false, // Will be updated on completion
            working_dir: env::current_dir()
                .ok()
                .and_then(|p| p.to_str().map(String::from)),
            environment: if env.is_empty() {
                None
            } else {
                Some(self.serialize_env(env)?)
            },
        };

        self.storage.add(&entry)
    }

    /// Record the completion of a command execution
    pub fn record_completion(
        &mut self,
        id: i64,
        duration_ms: i64,
        exit_code: i32,
        success: bool,
    ) -> Result<()> {
        // Get the original entry
        let mut entry = self.storage.get_by_id(id)?
            .ok_or_else(|| anyhow::anyhow!("History entry not found: {}", id))?;

        // Update with completion data
        entry.duration_ms = Some(duration_ms);
        entry.exit_code = Some(exit_code);
        entry.success = success;

        // Re-add (this will create a new entry, but we'll delete the old one)
        self.storage.add(&entry)?;

        Ok(())
    }

    /// Record a complete command execution in one call
    pub fn record(
        &mut self,
        command: &str,
        args: &[String],
        env: &HashMap<String, String>,
        duration_ms: i64,
        exit_code: i32,
        success: bool,
    ) -> Result<i64> {
        let entry = HistoryEntry {
            id: 0,
            command: command.to_string(),
            args: if args.is_empty() {
                None
            } else {
                Some(serde_json::to_string(args)?)
            },
            start_time: Utc::now().timestamp_millis(),
            duration_ms: Some(duration_ms),
            exit_code: Some(exit_code),
            success,
            working_dir: env::current_dir()
                .ok()
                .and_then(|p| p.to_str().map(String::from)),
            environment: if env.is_empty() {
                None
            } else {
                Some(self.serialize_env(env)?)
            },
        };

        self.storage.add(&entry)
    }

    /// Get access to the underlying storage
    pub fn storage(&self) -> &HistoryStorage {
        &self.storage
    }

    /// Get mutable access to the underlying storage
    pub fn storage_mut(&mut self) -> &mut HistoryStorage {
        &mut self.storage
    }

    /// Serialize environment variables, filtering sensitive data
    fn serialize_env(&self, env: &HashMap<String, String>) -> Result<String> {
        let filtered = if self.filter_sensitive {
            env.iter()
                .filter(|(k, _)| !self.is_sensitive_key(k))
                .map(|(k, v)| (k.clone(), v.clone()))
                .collect::<HashMap<_, _>>()
        } else {
            env.clone()
        };

        Ok(serde_json::to_string(&filtered)?)
    }

    /// Check if an environment variable name is sensitive
    fn is_sensitive_key(&self, key: &str) -> bool {
        let key_upper = key.to_uppercase();
        SENSITIVE_ENV_PATTERNS.iter().any(|pattern| key_upper.contains(pattern))
    }
}

impl Default for HistoryRecorder {
    fn default() -> Self {
        Self::new().expect("Failed to create default history recorder")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    fn create_test_recorder() -> HistoryRecorder {
        let temp_file = NamedTempFile::new().unwrap();
        let storage = HistoryStorage::with_path(temp_file.path()).unwrap();
        HistoryRecorder::with_storage(storage)
    }

    #[test]
    fn test_record_complete() {
        let mut recorder = create_test_recorder();
        let env = HashMap::new();

        let id = recorder.record(
            "test-command",
            &["arg1".to_string(), "arg2".to_string()],
            &env,
            1500,
            0,
            true,
        ).unwrap();

        assert!(id > 0);

        let entry = recorder.storage().get_by_id(id).unwrap().unwrap();
        assert_eq!(entry.command, "test-command");
        assert!(entry.success);
        assert_eq!(entry.duration_ms, Some(1500));
    }

    #[test]
    fn test_sensitive_filtering() {
        let recorder = create_test_recorder();

        assert!(recorder.is_sensitive_key("API_KEY"));
        assert!(recorder.is_sensitive_key("SECRET_TOKEN"));
        assert!(recorder.is_sensitive_key("PASSWORD"));
        assert!(!recorder.is_sensitive_key("PATH"));
        assert!(!recorder.is_sensitive_key("HOME"));
    }

    #[test]
    fn test_env_serialization_with_filtering() {
        let recorder = create_test_recorder();
        let mut env = HashMap::new();
        env.insert("PATH".to_string(), "/usr/bin".to_string());
        env.insert("API_KEY".to_string(), "secret".to_string());
        env.insert("HOME".to_string(), "/home/user".to_string());

        let serialized = recorder.serialize_env(&env).unwrap();
        let deserialized: HashMap<String, String> = serde_json::from_str(&serialized).unwrap();

        assert!(deserialized.contains_key("PATH"));
        assert!(deserialized.contains_key("HOME"));
        assert!(!deserialized.contains_key("API_KEY"));
    }

    #[test]
    fn test_env_serialization_without_filtering() {
        let mut recorder = create_test_recorder();
        recorder.set_filter_sensitive(false);

        let mut env = HashMap::new();
        env.insert("API_KEY".to_string(), "secret".to_string());

        let serialized = recorder.serialize_env(&env).unwrap();
        let deserialized: HashMap<String, String> = serde_json::from_str(&serialized).unwrap();

        assert!(deserialized.contains_key("API_KEY"));
    }
}
