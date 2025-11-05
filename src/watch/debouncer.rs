//! Event debouncing to reduce redundant filesystem events

use ahash::AHashMap;
use std::path::PathBuf;
use std::time::{Duration, Instant};

/// Debouncer for filesystem events
pub struct FileDebouncer {
    /// Debounce duration
    debounce_duration: Duration,

    /// Last event times for each path
    last_events: AHashMap<PathBuf, Instant>,
}

impl FileDebouncer {
    /// Create a new debouncer with the specified duration
    pub fn new(debounce_duration: Duration) -> Self {
        Self {
            debounce_duration,
            last_events: AHashMap::new(),
        }
    }

    /// Check if an event should be processed (not debounced)
    ///
    /// Returns true if enough time has passed since the last event for this path,
    /// or if this is the first event for this path.
    pub fn should_process(&mut self, path: &PathBuf) -> bool {
        let now = Instant::now();

        if let Some(&last_time) = self.last_events.get(path) {
            if now.duration_since(last_time) < self.debounce_duration {
                // Too soon, debounce this event
                return false;
            }
        }

        // Update last event time
        self.last_events.insert(path.clone(), now);
        true
    }

    /// Clear debounce state for a specific path
    pub fn clear_path(&mut self, path: &PathBuf) {
        self.last_events.remove(path);
    }

    /// Clear all debounce state
    pub fn clear_all(&mut self) {
        self.last_events.clear();
    }

    /// Get the number of paths being tracked
    pub fn tracked_paths_count(&self) -> usize {
        self.last_events.len()
    }

    /// Remove old entries to prevent unbounded growth
    ///
    /// Removes entries that haven't been updated in the last `retention` duration.
    pub fn cleanup(&mut self, retention: Duration) {
        let now = Instant::now();
        self.last_events
            .retain(|_, &mut last_time| now.duration_since(last_time) < retention);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread::sleep;

    #[test]
    fn test_first_event_always_processed() {
        let mut debouncer = FileDebouncer::new(Duration::from_millis(100));
        let path = PathBuf::from("test.txt");

        assert!(debouncer.should_process(&path));
        assert_eq!(debouncer.tracked_paths_count(), 1);
    }

    #[test]
    fn test_debounce_blocks_rapid_events() {
        let mut debouncer = FileDebouncer::new(Duration::from_millis(100));
        let path = PathBuf::from("test.txt");

        // First event should be processed
        assert!(debouncer.should_process(&path));

        // Immediate second event should be debounced
        assert!(!debouncer.should_process(&path));
        assert!(!debouncer.should_process(&path));
    }

    #[test]
    fn test_debounce_allows_delayed_events() {
        let mut debouncer = FileDebouncer::new(Duration::from_millis(50));
        let path = PathBuf::from("test.txt");

        // First event
        assert!(debouncer.should_process(&path));

        // Wait for debounce period
        sleep(Duration::from_millis(60));

        // Should be processed after debounce period
        assert!(debouncer.should_process(&path));
    }

    #[test]
    fn test_different_paths_independent() {
        let mut debouncer = FileDebouncer::new(Duration::from_millis(100));
        let path1 = PathBuf::from("test1.txt");
        let path2 = PathBuf::from("test2.txt");

        assert!(debouncer.should_process(&path1));
        assert!(debouncer.should_process(&path2));

        assert_eq!(debouncer.tracked_paths_count(), 2);

        // Rapid events on different paths don't affect each other
        assert!(!debouncer.should_process(&path1));
        assert!(!debouncer.should_process(&path2));
    }

    #[test]
    fn test_clear_path() {
        let mut debouncer = FileDebouncer::new(Duration::from_millis(100));
        let path = PathBuf::from("test.txt");

        debouncer.should_process(&path);
        assert_eq!(debouncer.tracked_paths_count(), 1);

        debouncer.clear_path(&path);
        assert_eq!(debouncer.tracked_paths_count(), 0);

        // After clearing, next event should be processed
        assert!(debouncer.should_process(&path));
    }

    #[test]
    fn test_clear_all() {
        let mut debouncer = FileDebouncer::new(Duration::from_millis(100));

        debouncer.should_process(&PathBuf::from("test1.txt"));
        debouncer.should_process(&PathBuf::from("test2.txt"));
        assert_eq!(debouncer.tracked_paths_count(), 2);

        debouncer.clear_all();
        assert_eq!(debouncer.tracked_paths_count(), 0);
    }

    #[test]
    fn test_cleanup() {
        let mut debouncer = FileDebouncer::new(Duration::from_millis(50));
        let path1 = PathBuf::from("test1.txt");
        let path2 = PathBuf::from("test2.txt");

        debouncer.should_process(&path1);
        sleep(Duration::from_millis(60));
        debouncer.should_process(&path2);

        // path1 is old, path2 is recent
        debouncer.cleanup(Duration::from_millis(50));

        // path1 should be removed, path2 should remain
        assert_eq!(debouncer.tracked_paths_count(), 1);
        assert!(!debouncer.last_events.contains_key(&path1));
        assert!(debouncer.last_events.contains_key(&path2));
    }
}
