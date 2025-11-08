//! History storage implementation using SQLite
//!
//! Provides persistent storage for command execution history with
//! efficient querying and export capabilities.

use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use rusqlite::{params, Connection, OptionalExtension};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

/// Default maximum number of history entries to retain
const DEFAULT_MAX_HISTORY: usize = 1000;

/// Command execution history entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryEntry {
    /// Unique entry ID
    pub id: i64,
    /// Command name
    pub command: String,
    /// Command arguments (serialized as JSON)
    pub args: Option<String>,
    /// Execution start time (Unix timestamp in milliseconds)
    pub start_time: i64,
    /// Execution duration in milliseconds
    pub duration_ms: Option<i64>,
    /// Process exit code
    pub exit_code: Option<i32>,
    /// Whether the command succeeded
    pub success: bool,
    /// Working directory at execution time
    pub working_dir: Option<String>,
    /// Environment variables (serialized as JSON, sensitive data filtered)
    pub environment: Option<String>,
}

impl HistoryEntry {
    /// Get the start time as a DateTime
    pub fn start_time_as_datetime(&self) -> DateTime<Utc> {
        DateTime::from_timestamp_millis(self.start_time).unwrap_or_else(|| Utc::now())
    }

    /// Get execution duration as a formatted string
    pub fn duration_string(&self) -> String {
        match self.duration_ms {
            Some(ms) if ms < 1000 => format!("{}ms", ms),
            Some(ms) => format!("{:.2}s", ms as f64 / 1000.0),
            None => "N/A".to_string(),
        }
    }

    /// Get status string
    pub fn status(&self) -> &str {
        if self.success {
            "success"
        } else {
            "failed"
        }
    }
}

/// SQLite-based history storage
pub struct HistoryStorage {
    conn: Connection,
    max_entries: usize,
}

impl HistoryStorage {
    /// Create a new history storage with the default database location
    ///
    /// Database is stored at: ~/.local/share/cmdrun/history.db (Linux/macOS)
    /// or %APPDATA%/cmdrun/history.db (Windows)
    pub fn new() -> Result<Self> {
        let db_path = Self::default_db_path()?;
        Self::with_path(db_path)
    }

    /// Create a new history storage with a custom database path
    pub fn with_path<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref();

        // Ensure parent directory exists
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).with_context(|| {
                format!("Failed to create history directory: {}", parent.display())
            })?;
        }

        let conn = Connection::open(path)
            .with_context(|| format!("Failed to open history database: {}", path.display()))?;

        let mut storage = Self {
            conn,
            max_entries: DEFAULT_MAX_HISTORY,
        };

        storage.initialize_schema()?;
        Ok(storage)
    }

    /// Get the default database path
    fn default_db_path() -> Result<PathBuf> {
        let data_dir = dirs::data_local_dir()
            .ok_or_else(|| anyhow::anyhow!("Failed to determine local data directory"))?;

        Ok(data_dir.join("cmdrun").join("history.db"))
    }

    /// Initialize database schema
    fn initialize_schema(&mut self) -> Result<()> {
        self.conn.execute(
            r#"
            CREATE TABLE IF NOT EXISTS command_history (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                command TEXT NOT NULL,
                args TEXT,
                start_time INTEGER NOT NULL,
                duration_ms INTEGER,
                exit_code INTEGER,
                success BOOLEAN NOT NULL,
                working_dir TEXT,
                environment TEXT,
                created_at INTEGER DEFAULT (strftime('%s', 'now') * 1000)
            )
            "#,
            [],
        )?;

        // Create indexes for efficient querying
        self.conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_command ON command_history(command)",
            [],
        )?;

        self.conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_start_time ON command_history(start_time DESC)",
            [],
        )?;

        self.conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_success ON command_history(success)",
            [],
        )?;

        Ok(())
    }

    /// Set the maximum number of history entries to retain
    pub fn set_max_entries(&mut self, max: usize) {
        self.max_entries = max;
    }

    /// Add a new history entry
    pub fn add(&mut self, entry: &HistoryEntry) -> Result<i64> {
        let id = self.conn.execute(
            r#"
            INSERT INTO command_history
                (command, args, start_time, duration_ms, exit_code, success, working_dir, environment)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)
            "#,
            params![
                entry.command,
                entry.args,
                entry.start_time,
                entry.duration_ms,
                entry.exit_code,
                entry.success,
                entry.working_dir,
                entry.environment,
            ],
        )?;

        // Enforce max entries limit
        self.cleanup_old_entries()?;

        Ok(id as i64)
    }

    /// Get the most recent history entry
    pub fn get_last(&self) -> Result<Option<HistoryEntry>> {
        self.conn
            .query_row(
                "SELECT id, command, args, start_time, duration_ms, exit_code, success, working_dir, environment
                 FROM command_history
                 ORDER BY start_time DESC
                 LIMIT 1",
                [],
                Self::row_to_entry,
            )
            .optional()
            .map_err(Into::into)
    }

    /// Get the most recent failed command
    pub fn get_last_failed(&self) -> Result<Option<HistoryEntry>> {
        self.conn
            .query_row(
                "SELECT id, command, args, start_time, duration_ms, exit_code, success, working_dir, environment
                 FROM command_history
                 WHERE success = 0
                 ORDER BY start_time DESC
                 LIMIT 1",
                [],
                Self::row_to_entry,
            )
            .optional()
            .map_err(Into::into)
    }

    /// Get a specific history entry by ID
    pub fn get_by_id(&self, id: i64) -> Result<Option<HistoryEntry>> {
        self.conn
            .query_row(
                "SELECT id, command, args, start_time, duration_ms, exit_code, success, working_dir, environment
                 FROM command_history
                 WHERE id = ?1",
                [id],
                Self::row_to_entry,
            )
            .optional()
            .map_err(Into::into)
    }

    /// List history entries with optional filters
    pub fn list(&self, limit: Option<usize>, offset: Option<usize>) -> Result<Vec<HistoryEntry>> {
        let limit = limit.unwrap_or(50);
        let offset = offset.unwrap_or(0);

        let mut stmt = self.conn.prepare(
            "SELECT id, command, args, start_time, duration_ms, exit_code, success, working_dir, environment
             FROM command_history
             ORDER BY start_time DESC
             LIMIT ?1 OFFSET ?2",
        )?;

        let entries = stmt
            .query_map(params![limit, offset], Self::row_to_entry)?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(entries)
    }

    /// Search history entries by command name or arguments
    pub fn search(&self, query: &str, limit: Option<usize>) -> Result<Vec<HistoryEntry>> {
        let limit = limit.unwrap_or(50);

        // SQLワイルドカードエスケープ（情報漏洩防止）
        let escaped_query = query
            .replace('\\', r"\\")
            .replace('%', r"\%")
            .replace('_', r"\_")
            .replace('[', r"\[");

        let search_pattern = format!("%{}%", escaped_query);

        let mut stmt = self.conn.prepare(
            "SELECT id, command, args, start_time, duration_ms, exit_code, success, working_dir, environment
             FROM command_history
             WHERE command LIKE ?1 ESCAPE '\\' OR args LIKE ?1 ESCAPE '\\'
             ORDER BY start_time DESC
             LIMIT ?2",
        )?;

        let entries = stmt
            .query_map(params![search_pattern, limit], Self::row_to_entry)?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(entries)
    }

    /// Get history statistics
    pub fn get_stats(&self) -> Result<HistoryStats> {
        let total: i64 =
            self.conn
                .query_row("SELECT COUNT(*) FROM command_history", [], |row| row.get(0))?;

        let successful: i64 = self.conn.query_row(
            "SELECT COUNT(*) FROM command_history WHERE success = 1",
            [],
            |row| row.get(0),
        )?;

        let failed: i64 = self.conn.query_row(
            "SELECT COUNT(*) FROM command_history WHERE success = 0",
            [],
            |row| row.get(0),
        )?;

        let avg_duration: Option<f64> = self
            .conn
            .query_row(
                "SELECT AVG(duration_ms) FROM command_history WHERE duration_ms IS NOT NULL",
                [],
                |row| row.get(0),
            )
            .optional()?
            .flatten();

        Ok(HistoryStats {
            total: total as usize,
            successful: successful as usize,
            failed: failed as usize,
            avg_duration_ms: avg_duration,
        })
    }

    /// Clear all history entries
    pub fn clear(&mut self) -> Result<usize> {
        let count = self.conn.execute("DELETE FROM command_history", [])?;
        // Reset auto-increment counter
        self.conn.execute(
            "DELETE FROM sqlite_sequence WHERE name='command_history'",
            [],
        )?;
        Ok(count)
    }

    /// Export history to JSON format
    pub fn export_json(&self, limit: Option<usize>) -> Result<String> {
        let entries = self.list(limit, None)?;
        serde_json::to_string_pretty(&entries).context("Failed to serialize history to JSON")
    }

    /// Export history to CSV format
    pub fn export_csv(&self, limit: Option<usize>) -> Result<String> {
        let entries = self.list(limit, None)?;
        let mut csv =
            String::from("id,command,args,start_time,duration_ms,exit_code,success,working_dir\n");

        for entry in entries {
            csv.push_str(&format!(
                "{},{},{},{},{},{},{},{}\n",
                entry.id,
                Self::escape_csv(&entry.command),
                entry
                    .args
                    .as_deref()
                    .map(Self::escape_csv)
                    .unwrap_or_default(),
                entry.start_time_as_datetime().to_rfc3339(),
                entry.duration_ms.map(|d| d.to_string()).unwrap_or_default(),
                entry.exit_code.map(|c| c.to_string()).unwrap_or_default(),
                entry.success,
                entry
                    .working_dir
                    .as_deref()
                    .map(Self::escape_csv)
                    .unwrap_or_default(),
            ));
        }

        Ok(csv)
    }

    /// Cleanup old entries to enforce max_entries limit
    fn cleanup_old_entries(&mut self) -> Result<()> {
        let count: i64 =
            self.conn
                .query_row("SELECT COUNT(*) FROM command_history", [], |row| row.get(0))?;

        if count > self.max_entries as i64 {
            let to_delete = count - self.max_entries as i64;
            self.conn.execute(
                "DELETE FROM command_history
                 WHERE id IN (
                     SELECT id FROM command_history
                     ORDER BY start_time ASC
                     LIMIT ?1
                 )",
                [to_delete],
            )?;
        }

        Ok(())
    }

    /// Convert database row to HistoryEntry
    fn row_to_entry(row: &rusqlite::Row) -> rusqlite::Result<HistoryEntry> {
        Ok(HistoryEntry {
            id: row.get(0)?,
            command: row.get(1)?,
            args: row.get(2)?,
            start_time: row.get(3)?,
            duration_ms: row.get(4)?,
            exit_code: row.get(5)?,
            success: row.get(6)?,
            working_dir: row.get(7)?,
            environment: row.get(8)?,
        })
    }

    /// Escape CSV field (with formula injection protection)
    fn escape_csv(s: &str) -> String {
        // 数式インジェクション防止：危険な文字で始まる場合は'でプレフィックス
        let sanitized = if s.starts_with('=') || s.starts_with('+') ||
                          s.starts_with('-') || s.starts_with('@') ||
                          s.starts_with('\t') || s.starts_with('\r') {
            format!("'{}", s)
        } else {
            s.to_string()
        };

        // 標準CSVエスケープ
        if sanitized.contains(',') || sanitized.contains('"') || sanitized.contains('\n') {
            format!("\"{}\"", sanitized.replace('"', "\"\""))
        } else {
            sanitized
        }
    }
}

/// History statistics
#[derive(Debug, Clone, Serialize)]
pub struct HistoryStats {
    pub total: usize,
    pub successful: usize,
    pub failed: usize,
    pub avg_duration_ms: Option<f64>,
}

impl HistoryStats {
    /// Get success rate as a percentage
    pub fn success_rate(&self) -> f64 {
        if self.total == 0 {
            0.0
        } else {
            (self.successful as f64 / self.total as f64) * 100.0
        }
    }

    /// Get average duration as a formatted string
    pub fn avg_duration_string(&self) -> String {
        match self.avg_duration_ms {
            Some(ms) if ms < 1000.0 => format!("{:.0}ms", ms),
            Some(ms) => format!("{:.2}s", ms / 1000.0),
            None => "N/A".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn create_test_storage() -> HistoryStorage {
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("test_history.db");
        let storage = HistoryStorage::with_path(&db_path).unwrap();

        // Keep temp_dir alive by leaking it (acceptable for tests)
        std::mem::forget(temp_dir);

        storage
    }

    fn create_test_entry(command: &str, success: bool) -> HistoryEntry {
        HistoryEntry {
            id: 0,
            command: command.to_string(),
            args: Some(r#"["arg1", "arg2"]"#.to_string()),
            start_time: Utc::now().timestamp_millis(),
            duration_ms: Some(1500),
            exit_code: if success { Some(0) } else { Some(1) },
            success,
            working_dir: Some("/tmp".to_string()),
            environment: Some(r#"{"PATH": "/usr/bin"}"#.to_string()),
        }
    }

    #[test]
    fn test_storage_creation() {
        let storage = create_test_storage();
        assert_eq!(storage.max_entries, DEFAULT_MAX_HISTORY);
    }

    #[test]
    fn test_add_and_retrieve() {
        let mut storage = create_test_storage();
        let entry = create_test_entry("test-command", true);

        let id = storage.add(&entry).unwrap();
        assert!(id > 0);

        let retrieved = storage.get_by_id(id).unwrap().unwrap();
        assert_eq!(retrieved.command, "test-command");
        assert!(retrieved.success);
    }

    #[test]
    fn test_get_last() {
        let mut storage = create_test_storage();

        storage.add(&create_test_entry("command1", true)).unwrap();
        std::thread::sleep(std::time::Duration::from_millis(10));
        storage.add(&create_test_entry("command2", false)).unwrap();

        let last = storage.get_last().unwrap().unwrap();
        assert_eq!(last.command, "command2");
    }

    #[test]
    fn test_get_last_failed() {
        let mut storage = create_test_storage();

        storage.add(&create_test_entry("success", true)).unwrap();
        storage.add(&create_test_entry("failure", false)).unwrap();

        let last_failed = storage.get_last_failed().unwrap().unwrap();
        assert_eq!(last_failed.command, "failure");
        assert!(!last_failed.success);
    }

    #[test]
    fn test_search() {
        let mut storage = create_test_storage();

        storage.add(&create_test_entry("build", true)).unwrap();
        storage.add(&create_test_entry("test", true)).unwrap();
        storage
            .add(&create_test_entry("build-release", true))
            .unwrap();

        let results = storage.search("build", None).unwrap();
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn test_clear() {
        let mut storage = create_test_storage();

        storage.add(&create_test_entry("cmd1", true)).unwrap();
        storage.add(&create_test_entry("cmd2", true)).unwrap();

        let deleted = storage.clear().unwrap();
        assert_eq!(deleted, 2);

        let list = storage.list(None, None).unwrap();
        assert!(list.is_empty());
    }

    #[test]
    fn test_stats() {
        let mut storage = create_test_storage();

        storage.add(&create_test_entry("cmd1", true)).unwrap();
        storage.add(&create_test_entry("cmd2", false)).unwrap();
        storage.add(&create_test_entry("cmd3", true)).unwrap();

        let stats = storage.get_stats().unwrap();
        assert_eq!(stats.total, 3);
        assert_eq!(stats.successful, 2);
        assert_eq!(stats.failed, 1);
        assert!((stats.success_rate() - 66.67).abs() < 0.1);
    }
}
