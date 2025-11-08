//! Fuzzy matching logic for command search

use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;

/// Fuzzy match commands and return sorted results
///
/// Uses the Skim fuzzy matching algorithm to find commands that match
/// the search query. Results are sorted by match score (best matches first).
pub fn fuzzy_match_commands(query: &str, commands: &[String]) -> Vec<String> {
    let matcher = SkimMatcherV2::default();
    let mut matches: Vec<(String, i64)> = Vec::new();

    for command in commands {
        if let Some(score) = matcher.fuzzy_match(command, query) {
            matches.push((command.clone(), score));
        }
    }

    // Sort by score (descending - higher score = better match)
    matches.sort_by(|a, b| b.1.cmp(&a.1));

    matches.into_iter().map(|(cmd, _)| cmd).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fuzzy_match_exact() {
        let commands = vec!["build".to_string(), "test".to_string(), "dev".to_string()];
        let results = fuzzy_match_commands("build", &commands);
        assert_eq!(results[0], "build");
    }

    #[test]
    fn test_fuzzy_match_partial() {
        let commands = vec![
            "build-prod".to_string(),
            "build-dev".to_string(),
            "test".to_string(),
        ];
        let results = fuzzy_match_commands("bld", &commands);
        assert!(results.len() >= 2);
        assert!(results.contains(&"build-prod".to_string()));
        assert!(results.contains(&"build-dev".to_string()));
    }

    #[test]
    fn test_fuzzy_match_empty_query() {
        let commands = vec!["build".to_string(), "test".to_string()];
        let results = fuzzy_match_commands("", &commands);
        // Empty query should match all commands
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn test_fuzzy_match_no_results() {
        let commands = vec!["build".to_string(), "test".to_string()];
        let results = fuzzy_match_commands("xyz", &commands);
        // No matches for completely unrelated query
        assert!(results.is_empty());
    }
}
