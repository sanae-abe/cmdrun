# History Feature Usage Examples

## Quick Start

```bash
# View recent command history
cmdrun history list

# Search for specific commands
cmdrun history search build

# Retry the last failed command
cmdrun retry

# View statistics
cmdrun history stats
```

## Advanced Usage

### Analyzing Build Performance

```bash
# Search for all build commands
cmdrun history search build

# Export build history for analysis
cmdrun history export --format json | jq '.[] | select(.command == "build") | {duration_ms, success}'
```

### Debugging Failed Tests

```bash
# Show only failed commands
cmdrun history list --failed

# Get the last failed command details
cmdrun history list --failed --limit 1

# Retry it
cmdrun retry
```

### Team Collaboration

```bash
# Export your command history to share with team
cmdrun history export --format csv -o my-commands-$(date +%Y%m%d).csv

# Import patterns from successful builds
cmdrun history list --limit 100 | grep "success.*build"
```

### Maintenance

```bash
# Check history statistics
cmdrun history stats

# Export for backup before clearing
cmdrun history export --format json -o backup-$(date +%Y%m%d).json

# Clear old history
cmdrun history clear --force
```

## Integration with Shell

### Bash/Zsh

```bash
# Add to ~/.bashrc or ~/.zshrc
alias chr='cmdrun history list'
alias chs='cmdrun history search'
alias chr-retry='cmdrun retry'
alias chr-stats='cmdrun history stats'
```

### Fish

```fish
# Add to ~/.config/fish/config.fish
abbr chr 'cmdrun history list'
abbr chs 'cmdrun history search'
abbr chr-retry 'cmdrun retry'
abbr chr-stats 'cmdrun history stats'
```

## Data Analysis Examples

### Using jq to analyze JSON exports

```bash
# Get average duration of successful builds
cmdrun history export --format json | jq '[.[] | select(.command == "build" and .success) | .duration_ms] | add / length'

# Find slowest commands
cmdrun history export --format json | jq 'sort_by(.duration_ms) | reverse | .[0:5] | .[] | {command, duration_ms}'

# Success rate by command
cmdrun history export --format json | jq 'group_by(.command) | .[] | {command: .[0].command, total: length, successful: [.[] | select(.success)] | length}'
```

### Using Python for analysis

```python
import json
import pandas as pd

# Load history
with open('history.json') as f:
    data = json.load(f)

df = pd.DataFrame(data)

# Convert timestamp to datetime
df['start_time'] = pd.to_datetime(df['start_time'], unit='ms')

# Calculate statistics
print(df.groupby('command').agg({
    'duration_ms': ['mean', 'std'],
    'success': 'mean',
    'id': 'count'
}))
```

## CI/CD Integration

### GitHub Actions

```yaml
name: Build with History

on: [push]

jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      - name: Install cmdrun
        run: cargo install cmdrun
      - name: Run build
        run: cmdrun run build
      - name: Export history on failure
        if: failure()
        run: |
          cmdrun history export --format json -o build-history.json
          cat build-history.json
      - name: Upload history
        if: always()
        uses: actions/upload-artifact@v2
        with:
          name: build-history
          path: build-history.json
```

### GitLab CI

```yaml
build:
  script:
    - cmdrun run build
  after_script:
    - cmdrun history export --format json -o build-history.json
  artifacts:
    paths:
      - build-history.json
    when: always
```
