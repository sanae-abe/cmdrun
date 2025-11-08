# Typo Detection and Suggestion System

## Overview

cmdrun now includes an intelligent typo detection and suggestion system that helps users when they mistype command names. The system uses the Levenshtein distance algorithm to find similar command names and provides helpful suggestions in multiple languages.

## Features

### 1. **Automatic Typo Detection**
When a user types an unknown command, cmdrun automatically detects potential typos and suggests corrections.

Example:
```bash
$ cmdrun serch test
Unknown command 'serch'

💡 Did you mean one of these?
  → search (distance: 1)

ℹ Run 'cmdrun --help' for available commands
```

### 2. **Multi-Language Support**
Error messages and suggestions are displayed in the user's configured language:
- English
- Japanese (日本語)
- Simplified Chinese (简体中文)
- Traditional Chinese (繁體中文)

### 3. **Configurable Detection**
The typo detection system can be customized through configuration:

```toml
[config]
typo_detection = true      # Enable/disable typo detection
typo_threshold = 2         # Maximum distance for suggestions (default: 2)
auto_correct = false       # Reserved for future auto-correction feature
```

### 4. **Distance-Based Ranking**
Suggestions are sorted by similarity (Levenshtein distance), showing the most likely corrections first.

## Implementation Details

### Core Components

#### 1. **TypoDetector** (`src/utils/typo_detector.rs`)
The main detection engine that:
- Calculates Levenshtein distance between input and available commands
- Filters suggestions based on configurable threshold
- Sorts results by distance and alphabetically
- Limits the number of suggestions displayed

```rust
use cmdrun::utils::typo_detector::TypoDetector;

let detector = TypoDetector::new();
let commands = vec!["build", "test", "watch", "deploy"];
let suggestions = detector.suggest("buld", &commands);
// Returns: [("build", 1)]
```

#### 2. **Configuration Schema** (`src/config/schema.rs`)
Added three new configuration options:
- `typo_detection`: bool (default: true)
- `typo_threshold`: usize (default: 2)
- `auto_correct`: bool (default: false, reserved for future use)

#### 3. **Internationalization** (`src/i18n.rs`)
New message keys for all supported languages:
- `TypoUnknownCommand`: "Unknown command"
- `TypoDidYouMean`: "Did you mean one of these?"
- `TypoSuggestions`: "Suggestions"
- `TypoRunHelp`: "Run 'cmdrun --help' for available commands"

#### 4. **Integration** (`src/main.rs`)
Integrated into the command execution flow:
- Detects when a command is not found
- Calls typo detector with available command names
- Displays formatted suggestions with distance information
- Respects user's language configuration

## Usage

### For Users

1. **Default behavior**: Typo detection is enabled by default
2. **Disable typo detection**:
   ```toml
   [config]
   typo_detection = false
   ```

3. **Adjust sensitivity**:
   ```toml
   [config]
   typo_threshold = 1  # Stricter: only 1-character typos
   # or
   typo_threshold = 3  # More lenient: up to 3-character differences
   ```

### For Developers

#### Basic Usage
```rust
use cmdrun::utils::typo_detector::TypoDetector;

let detector = TypoDetector::new();
let available = vec!["run", "build", "test"];
let suggestions = detector.suggest("biuld", &available);

for (suggestion, distance) in suggestions {
    println!("{} (distance: {})", suggestion, distance);
}
```

#### Custom Configuration
```rust
use cmdrun::utils::typo_detector::{TypoDetector, TypoDetectorConfig};

let config = TypoDetectorConfig {
    threshold: 1,           // Only 1-character differences
    max_suggestions: 3,     // Show at most 3 suggestions
};

let detector = TypoDetector::with_config(config);
```

#### Subcommand Detection
```rust
let subcommands = vec!["list", "search", "clear", "export"];
let suggestions = detector.suggest_subcommand("serch", &subcommands);
```

## Algorithm: Levenshtein Distance

The Levenshtein distance measures the minimum number of single-character edits (insertions, deletions, or substitutions) required to change one string into another.

### Examples:
- `"buld"` → `"build"` = distance 1 (missing 'i')
- `"serch"` → `"search"` = distance 1 (missing 'a')
- `"wacth"` → `"watch"` = distance 2 (transposed 'ct')

### Why Levenshtein?
- Simple and efficient
- Well-suited for command name typos
- Language-agnostic
- Predictable results

## Testing

### Unit Tests
Comprehensive test suite in `tests/unit_typo_detector.rs`:
- Exact match handling
- Single and multi-character typos
- Threshold filtering
- Distance-based sorting
- Case sensitivity
- Edge cases (empty input, no matches)
- Real-world cmdrun command typos

### Running Tests
```bash
# Run all typo detector tests
cargo test typo_detector

# Run specific test
cargo test test_common_typos

# Run with output
cargo test typo_detector -- --nocapture
```

### Example Demo
```bash
# Run the interactive demonstration
cargo run --example typo_demo
```

## Performance

- **Startup Impact**: Minimal (typo detection only runs on error)
- **Detection Speed**: O(n*m) where n = number of commands, m = average command length
- **Memory**: O(n) for storing suggestions
- **Typical Performance**: < 1ms for ~20 commands

## Future Enhancements

1. **Auto-Correction** (`auto_correct` config)
   - Automatically execute the closest match with confirmation
   - Configurable confidence threshold

2. **Fuzzy Matching Algorithms**
   - Jaro-Winkler distance for better prefix matching
   - N-gram similarity for complex typos

3. **Learning System**
   - Track common typos and corrections
   - Personalized suggestions based on history

4. **Context-Aware Suggestions**
   - Consider command category and tags
   - Weight recently used commands higher

## Configuration Examples

### Disable for Scripts
```toml
# commands.toml for automated scripts
[config]
typo_detection = false  # Faster failure for automation
strict_mode = true
```

### Lenient for Beginners
```toml
# commands.toml for new users
[config]
typo_detection = true
typo_threshold = 3      # More forgiving
language = "japanese"   # Or user's preferred language
```

### Strict for Power Users
```toml
# commands.toml for experienced users
[config]
typo_detection = true
typo_threshold = 1      # Only obvious typos
```

## Troubleshooting

### No Suggestions Appear
- Check that `typo_detection = true` in config
- Verify typo is within threshold distance
- Ensure command list is not empty

### Too Many Suggestions
- Reduce `typo_threshold` value
- Commands may be too similar to each other

### Wrong Suggestions
- Input may be too different from any command (distance > threshold)
- Consider whether it's actually a typo or a new command request

## Dependencies

- **strsim**: 0.11 - Levenshtein distance calculation
  - Zero dependencies
  - Fast, pure Rust implementation
  - Well-tested and maintained

## Related Documentation

- [Configuration Guide](../user-guide/configuration.md)
- [Command System](../technical/command-system.md)
- [Internationalization](../technical/i18n.md)
- [Error Handling](../technical/error-handling.md)

## Changelog

### Version 1.0.0
- Initial implementation of typo detection system
- Levenshtein distance-based suggestions
- Multi-language support (English, Japanese, Chinese)
- Configurable threshold and max suggestions
- Integration with command execution flow
- Comprehensive test suite
- Example demonstration program

---

**Author**: Rust Engineering Team
**Last Updated**: 2025-11-08
**Status**: Stable
