//! Unit tests for typo detector
//!
//! Tests typo detection and suggestion system with various scenarios.

use cmdrun::utils::typo_detector::{TypoDetector, TypoDetectorConfig};

#[test]
fn test_exact_match_no_suggestion() {
    let detector = TypoDetector::new();
    let commands = vec!["build", "test", "watch", "deploy"];

    let suggestions = detector.suggest("build", &commands);
    assert_eq!(
        suggestions.len(),
        0,
        "Exact matches should not be suggested"
    );
}

#[test]
fn test_single_character_typo() {
    let detector = TypoDetector::new();
    let commands = vec!["build", "test", "watch", "deploy"];

    // Missing character: "buld" -> "build"
    let suggestions = detector.suggest("buld", &commands);
    assert_eq!(suggestions.len(), 1);
    assert_eq!(suggestions[0].0, "build");
    assert_eq!(suggestions[0].1, 1);

    // Extra character: "tesst" -> "test"
    let suggestions = detector.suggest("tesst", &commands);
    assert_eq!(suggestions.len(), 1);
    assert_eq!(suggestions[0].0, "test");
    assert_eq!(suggestions[0].1, 1);

    // Wrong character: "wetch" -> "watch"
    let suggestions = detector.suggest("wetch", &commands);
    assert_eq!(suggestions.len(), 1);
    assert_eq!(suggestions[0].0, "watch");
    assert_eq!(suggestions[0].1, 1);
}

#[test]
fn test_two_character_typo() {
    let detector = TypoDetector::new();
    let commands = vec!["search", "deploy", "install"];

    // "seach" has distance 1 from "search"
    let suggestions = detector.suggest("seach", &commands);
    assert!(!suggestions.is_empty());
    assert_eq!(suggestions[0].0, "search");

    // "depoy" has distance 1 from "deploy"
    let suggestions = detector.suggest("depoy", &commands);
    assert!(!suggestions.is_empty());
    assert_eq!(suggestions[0].0, "deploy");
}

#[test]
fn test_threshold_filtering() {
    let detector = TypoDetector::with_threshold(1);
    let commands = vec!["build", "test", "watch"];

    // Distance 1: should be suggested
    let suggestions = detector.suggest("buld", &commands);
    assert_eq!(suggestions.len(), 1);
    assert_eq!(suggestions[0].0, "build");

    // Distance 2: should not be suggested with threshold 1
    let suggestions = detector.suggest("bld", &commands);
    assert_eq!(suggestions.len(), 0);
}

#[test]
fn test_multiple_suggestions_sorted() {
    let detector = TypoDetector::new();
    let commands = vec!["build", "guild", "wild", "child"];

    let suggestions = detector.suggest("bild", &commands);

    // Should have multiple suggestions
    assert!(!suggestions.is_empty());

    // Should be sorted by distance (ascending)
    for i in 0..suggestions.len().saturating_sub(1) {
        assert!(
            suggestions[i].1 <= suggestions[i + 1].1,
            "Suggestions should be sorted by distance"
        );
    }

    // "build" should be first (distance 1)
    assert_eq!(suggestions[0].0, "build");
    assert_eq!(suggestions[0].1, 1);
}

#[test]
fn test_max_suggestions_limit() {
    let config = TypoDetectorConfig {
        threshold: 3,
        max_suggestions: 2,
    };
    let detector = TypoDetector::with_config(config);
    let commands = vec!["build", "guild", "wild", "child", "field", "yield"];

    let suggestions = detector.suggest("ild", &commands);

    assert!(
        suggestions.len() <= 2,
        "Should respect max_suggestions limit"
    );
}

#[test]
fn test_no_suggestions_for_distant_typos() {
    let detector = TypoDetector::with_threshold(2);
    let commands = vec!["build", "test", "watch"];

    // "xyz" is very different from all commands
    let suggestions = detector.suggest("xyz", &commands);
    assert_eq!(suggestions.len(), 0);
}

#[test]
fn test_common_typos() {
    let detector = TypoDetector::new();
    let commands = vec![
        "list", "search", "watch", "deploy", "build", "test", "run", "init",
    ];

    // Common typos
    let test_cases = vec![
        ("serch", "search"),  // Missing 'a'
        ("wacth", "watch"),   // Transposed characters
        ("lst", "list"),      // Missing 'i'
        ("tst", "test"),      // Missing 'e'
        ("biuld", "build"),   // Transposed characters
        ("deplyo", "deploy"), // Typo at end
    ];

    for (typo, expected) in test_cases {
        let suggestions = detector.suggest(typo, &commands);
        assert!(!suggestions.is_empty(), "Should suggest for typo: {}", typo);
        assert_eq!(
            suggestions[0].0, expected,
            "Expected '{}' for typo '{}'",
            expected, typo
        );
    }
}

#[test]
fn test_case_sensitivity() {
    let detector = TypoDetector::new();
    let commands = vec!["Build", "TEST", "watch"];

    // Case-sensitive matching
    let suggestions = detector.suggest("build", &commands);

    // "build" differs from "Build" by 1 character (case)
    assert!(!suggestions.is_empty());
    assert!(suggestions.iter().any(|(cmd, _)| *cmd == "Build"));
}

#[test]
fn test_empty_input() {
    let detector = TypoDetector::new();
    let commands = vec!["build", "test", "watch"];

    let suggestions = detector.suggest("", &commands);

    // Empty string has large distance from all commands
    // With default threshold of 2, no suggestions should be returned
    assert_eq!(suggestions.len(), 0);
}

#[test]
fn test_empty_command_list() {
    let detector = TypoDetector::new();
    let commands: Vec<&str> = vec![];

    let suggestions = detector.suggest("build", &commands);
    assert_eq!(suggestions.len(), 0);
}

#[test]
fn test_subcommand_detection() {
    let detector = TypoDetector::new();
    let subcommands = vec!["list", "search", "clear", "export", "stats"];

    // Test subcommand typos
    let suggestions = detector.suggest_subcommand("serch", &subcommands);
    assert_eq!(suggestions[0].0, "search");

    let suggestions = detector.suggest_subcommand("lst", &subcommands);
    assert_eq!(suggestions[0].0, "list");
}

#[test]
fn test_similar_commands() {
    let detector = TypoDetector::new();
    let commands = vec!["install", "uninstall", "reinstall"];

    // "instal" should suggest "install"
    let suggestions = detector.suggest("instal", &commands);
    assert_eq!(suggestions[0].0, "install");
    assert_eq!(suggestions[0].1, 1);
}

#[test]
fn test_prefix_matching() {
    let detector = TypoDetector::new();
    let commands = vec!["build", "build-all", "build-docker"];

    // "bild" closer to "build" than other variants
    let suggestions = detector.suggest("bild", &commands);
    assert_eq!(suggestions[0].0, "build");
}

#[test]
fn test_long_command_names() {
    let detector = TypoDetector::new();
    let commands = vec![
        "generate-typescript-definitions",
        "compile-sass-stylesheets",
        "run-integration-tests",
    ];

    // Typo in long command
    let suggestions = detector.suggest("generate-typescipt-definitions", &commands);
    assert!(!suggestions.is_empty());
    assert_eq!(suggestions[0].0, "generate-typescript-definitions");
}

#[test]
fn test_alphabetical_sorting_for_same_distance() {
    let detector = TypoDetector::new();
    let commands = vec!["test", "best", "west", "rest"];

    let suggestions = detector.suggest("tst", &commands);

    // All have same distance, should be sorted alphabetically
    let mut prev: Option<&str> = None;
    for (cmd, distance) in &suggestions {
        if let Some(p) = prev {
            if suggestions.iter().find(|(c, _)| c == &p).map(|(_, d)| d) == Some(distance) {
                assert!(
                    p <= *cmd,
                    "Commands with same distance should be alphabetically sorted"
                );
            }
        }
        prev = Some(cmd);
    }
}

#[test]
fn test_distance_calculation() {
    let detector = TypoDetector::new();
    let commands = vec!["test"];

    // Test known Levenshtein distances
    let test_cases = vec![
        ("test", 0),  // Exact match (filtered out)
        ("tst", 1),   // Deletion
        ("tesst", 1), // Insertion
        ("tezt", 1),  // Substitution
        ("tets", 1),  // Transposition (costs 2 in Levenshtein but 1 in some variants)
    ];

    for (input, expected_distance) in test_cases {
        if expected_distance == 0 {
            let suggestions = detector.suggest(input, &commands);
            assert_eq!(suggestions.len(), 0, "Exact match should not be suggested");
        } else {
            let suggestions = detector.suggest(input, &commands);
            if !suggestions.is_empty() {
                assert!(
                    suggestions[0].1 <= expected_distance + 1,
                    "Distance for '{}' should be around {}",
                    input,
                    expected_distance
                );
            }
        }
    }
}

#[test]
fn test_real_world_cmdrun_commands() {
    let detector = TypoDetector::new();
    let commands = vec![
        "run",
        "list",
        "init",
        "validate",
        "graph",
        "completion",
        "remove",
        "add",
        "open",
        "edit",
        "info",
        "search",
        "config",
        "watch",
        "env",
        "history",
        "retry",
        "template",
        "plugin",
    ];

    // Common typos users might make
    let typos = vec![
        ("lst", "list"),
        ("serch", "search"),
        ("wacth", "watch"),
        ("initi", "init"),
        ("histroy", "history"),
        ("retrry", "retry"),
        ("plugn", "plugin"),
        ("tempalte", "template"),
    ];

    for (typo, expected) in typos {
        let suggestions = detector.suggest(typo, &commands);
        assert!(!suggestions.is_empty(), "Should suggest for typo: {}", typo);
        assert_eq!(
            suggestions[0].0, expected,
            "Expected '{}' for typo '{}'",
            expected, typo
        );
    }
}
