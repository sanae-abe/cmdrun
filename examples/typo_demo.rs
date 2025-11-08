//! Demonstration of the typo detection system
//!
//! Run with: cargo run --example typo_demo

use cmdrun::utils::typo_detector::TypoDetector;

fn main() {
    println!("=== cmdrun Typo Detection System Demo ===\n");

    let detector = TypoDetector::new();
    let commands = vec![
        "run", "list", "init", "validate", "graph", "completion", "remove", "add", "open",
        "edit", "info", "search", "config", "watch", "env", "history", "retry", "template",
        "plugin",
    ];

    println!("Available commands:");
    for cmd in &commands {
        print!("{} ", cmd);
    }
    println!("\n");

    // Test various typos
    let test_typos = vec![
        ("lst", "list"),
        ("serch", "search"),
        ("wacth", "watch"),
        ("histroy", "history"),
        ("plugn", "plugin"),
        ("tempalte", "template"),
        ("initi", "init"),
        ("buld", "build"), // Will not find (build not in list)
    ];

    for (typo, expected_correction) in test_typos {
        println!("Input: '{}'", typo);

        let suggestions = detector.suggest(typo, &commands);

        if suggestions.is_empty() {
            println!("  No suggestions found");
        } else {
            println!("  Did you mean:");
            for (suggestion, distance) in &suggestions {
                println!("    {} (distance: {})", suggestion, distance);
                if suggestion == expected_correction {
                    println!("      ✓ Correct!");
                }
            }
        }
        println!();
    }

    println!("\n=== Custom Configuration Demo ===\n");

    use cmdrun::utils::typo_detector::TypoDetectorConfig;

    let strict_detector = TypoDetector::with_config(TypoDetectorConfig {
        threshold: 1, // Only allow 1 character difference
        max_suggestions: 3,
    });

    println!("Strict detector (threshold=1):");
    let suggestions = strict_detector.suggest("lst", &commands);
    println!("'lst' suggestions: {:?}", suggestions);

    let suggestions = strict_detector.suggest("histroy", &commands); // distance=2, won't match
    println!("'histroy' suggestions: {:?}", suggestions);
}
