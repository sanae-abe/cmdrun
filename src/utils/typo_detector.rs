//! Typo detection and command suggestion system
//!
//! Detects typos in command names using Levenshtein distance algorithm
//! and provides intelligent suggestions with multi-language support.

use strsim::levenshtein;

/// Configuration for typo detection
#[derive(Debug, Clone)]
pub struct TypoDetectorConfig {
    /// Maximum Levenshtein distance to consider for suggestions (default: 2)
    pub threshold: usize,
    /// Maximum number of suggestions to return (default: 5)
    pub max_suggestions: usize,
}

impl Default for TypoDetectorConfig {
    fn default() -> Self {
        Self {
            threshold: 2,
            max_suggestions: 5,
        }
    }
}

/// Typo detector for command names
pub struct TypoDetector {
    config: TypoDetectorConfig,
}

impl TypoDetector {
    /// Create a new typo detector with default configuration
    pub fn new() -> Self {
        Self {
            config: TypoDetectorConfig::default(),
        }
    }

    /// Create a typo detector with custom configuration
    pub fn with_config(config: TypoDetectorConfig) -> Self {
        Self { config }
    }

    /// Create a typo detector with custom threshold
    pub fn with_threshold(threshold: usize) -> Self {
        Self {
            config: TypoDetectorConfig {
                threshold,
                ..Default::default()
            },
        }
    }

    /// Find similar commands based on Levenshtein distance
    ///
    /// Returns a vector of (command_name, distance) tuples sorted by distance (ascending).
    /// Only commands within the configured threshold are returned.
    ///
    /// # Arguments
    /// * `input` - The typo'd command name
    /// * `available_commands` - List of valid command names
    ///
    /// # Examples
    /// ```
    /// use cmdrun::utils::typo_detector::TypoDetector;
    ///
    /// let detector = TypoDetector::new();
    /// let commands = vec!["build", "test", "watch", "deploy"];
    /// let suggestions = detector.suggest("buld", &commands);
    ///
    /// assert_eq!(suggestions.len(), 1);
    /// assert_eq!(suggestions[0].0, "build");
    /// assert_eq!(suggestions[0].1, 1); // Distance of 1
    /// ```
    pub fn suggest<'a>(
        &self,
        input: &str,
        available_commands: &[&'a str],
    ) -> Vec<(&'a str, usize)> {
        let mut suggestions: Vec<(&str, usize)> = available_commands
            .iter()
            .map(|&cmd| {
                let distance = levenshtein(input, cmd);
                (cmd, distance)
            })
            .filter(|(_, distance)| *distance <= self.config.threshold && *distance > 0)
            .collect();

        // Sort by distance (ascending), then alphabetically
        suggestions.sort_by(|a, b| a.1.cmp(&b.1).then_with(|| a.0.cmp(b.0)));

        // Limit to max_suggestions
        suggestions.truncate(self.config.max_suggestions);

        suggestions
    }

    /// Check if a command exists exactly (case-sensitive)
    pub fn is_exact_match(&self, input: &str, available_commands: &[&str]) -> bool {
        available_commands.contains(&input)
    }

    /// Find suggestions for subcommands (nested command detection)
    ///
    /// Useful for detecting typos in subcommands like "cmdrun history seach"
    /// where "seach" should be "search".
    pub fn suggest_subcommand<'a>(
        &self,
        input: &str,
        available_subcommands: &[&'a str],
    ) -> Vec<(&'a str, usize)> {
        // Same logic as suggest() - could be extended with different thresholds
        self.suggest(input, available_subcommands)
    }
}

impl Default for TypoDetector {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exact_match() {
        let detector = TypoDetector::new();
        let commands = vec!["build", "test", "watch"];

        assert!(detector.is_exact_match("build", &commands));
        assert!(!detector.is_exact_match("buld", &commands));
    }

    #[test]
    fn test_suggest_close_match() {
        let detector = TypoDetector::new();
        let commands = vec!["build", "test", "watch", "deploy"];

        let suggestions = detector.suggest("buld", &commands);
        assert_eq!(suggestions.len(), 1);
        assert_eq!(suggestions[0].0, "build");
        assert_eq!(suggestions[0].1, 1);
    }

    #[test]
    fn test_suggest_multiple_matches() {
        let detector = TypoDetector::new();
        let commands = vec!["search", "watch", "match", "patch"];

        let suggestions = detector.suggest("seach", &commands);
        assert!(!suggestions.is_empty());
        assert_eq!(suggestions[0].0, "search"); // Distance 1
    }

    #[test]
    fn test_suggest_distance_threshold() {
        let detector = TypoDetector::with_threshold(1);
        let commands = vec!["build", "test", "watch"];

        // "bld" has distance 2 from "build", should not be suggested with threshold 1
        let suggestions = detector.suggest("bld", &commands);
        assert_eq!(suggestions.len(), 0);

        // "buld" has distance 1 from "build", should be suggested
        let suggestions = detector.suggest("buld", &commands);
        assert_eq!(suggestions.len(), 1);
        assert_eq!(suggestions[0].0, "build");
    }

    #[test]
    fn test_suggest_no_exact_match_in_results() {
        let detector = TypoDetector::new();
        let commands = vec!["build", "test", "watch"];

        // Exact match should not be in suggestions (distance 0 is filtered out)
        let suggestions = detector.suggest("build", &commands);
        assert_eq!(suggestions.len(), 0);
    }

    #[test]
    fn test_suggest_sorted_by_distance() {
        let detector = TypoDetector::new();
        let commands = vec!["build", "guild", "wild"];

        let suggestions = detector.suggest("gild", &commands);

        // Should be sorted by distance
        for i in 0..suggestions.len().saturating_sub(1) {
            assert!(suggestions[i].1 <= suggestions[i + 1].1);
        }
    }

    #[test]
    fn test_max_suggestions() {
        let config = TypoDetectorConfig {
            threshold: 2,
            max_suggestions: 2,
        };
        let detector = TypoDetector::with_config(config);
        let commands = vec!["build", "guild", "wild", "child", "field"];

        let suggestions = detector.suggest("ild", &commands);
        assert!(suggestions.len() <= 2);
    }

    #[test]
    fn test_subcommand_suggestion() {
        let detector = TypoDetector::new();
        let subcommands = vec!["list", "search", "clear", "export"];

        let suggestions = detector.suggest_subcommand("serch", &subcommands);
        assert_eq!(suggestions[0].0, "search");
    }

    #[test]
    fn test_empty_input() {
        let detector = TypoDetector::new();
        let commands = vec!["build", "test"];

        let suggestions = detector.suggest("", &commands);
        // Empty string has distance equal to command length
        // For "test" (4 chars) and "build" (5 chars), both are > threshold 2
        assert_eq!(suggestions.len(), 0);
    }

    #[test]
    fn test_case_sensitive() {
        let detector = TypoDetector::new();
        let commands = vec!["Build", "TEST", "watch"];

        // Case-sensitive - "build" vs "Build" has distance 1
        let suggestions = detector.suggest("build", &commands);
        assert!(suggestions.iter().any(|(cmd, _)| *cmd == "Build"));
    }
}
