# TUI Mode Guide

## Overview

The Terminal User Interface (TUI) mode provides an interactive way to search GitHub repositories with a beautiful, keyboard-driven interface.

## Starting TUI Mode

```bash
# Start with no options (default settings)
github-search-cli

# Start with language filter pre-set
github-search-cli -L rust

# Start with sorting preference
github-search-cli --sort stars -l 20
```

## Interface Layout

```
┌─────────────────────────────────────────┐
│ 🔍 Search GitHub Repositories           │
│ [your search query here]                │
└─────────────────────────────────────────┘
┌─────────────────────────────────────────┐
│ Results (1,234 total)                   │
│ ▶ user/repo | ⭐ 1.5k | 💻 Rust         │
│   other/project | ⭐ 890 | 💻 Python    │
│   ...                                   │
└─────────────────────────────────────────┘
┌─────────────────────────────────────────┐
│ Details                                 │
│ Description: A cool project...          │
│ ⭐ Stars: 1500  🍴 Forks: 120          │
│ 🔗 https://github.com/user/repo        │
└─────────────────────────────────────────┘
┌─────────────────────────────────────────┐
│ Enter: Search/Open  ↑↓: Navigate       │
└─────────────────────────────────────────┘
```

## Workflow

### 1. Search for Repositories
- Type your query in the search box (e.g., "rust web framework")
- Press **Enter** to execute the search
- Wait for results to load (you'll see a "Searching..." message)

### 2. Browse Results
- Use **↑** and **↓** arrow keys to navigate through the list
- The selected repository is highlighted with a **▶** symbol
- Details panel updates automatically as you navigate

### 3. View Details
The details panel shows:
- Repository description
- Star count, fork count, and primary language
- GitHub URL

### 4. Open in Browser
- Select a repository with arrow keys
- Press **Enter** to open it in your default browser
- The application will exit after opening

### 5. New Search
- Press **Backspace** to clear your search
- Type a new query
- Press **Enter** to search again

### 6. Exit
- Press **Esc** to quit
- Or press **Ctrl+C** to quit

## Tips & Tricks

### Search Query Syntax
You can use GitHub's search syntax:
- `"web framework"` - Exact phrase
- `language:rust` - Filter by language (also available via `-L` flag)
- `stars:>1000` - Repositories with more than 1000 stars (also via `-s` flag)
- `topic:game` - Filter by topic
- `in:name,description` - Search in specific fields

### Pre-applying Filters
Start the TUI with filters already set:
```bash
# Only show Rust projects
github-search-cli -L rust

# Only show projects with 500+ stars
github-search-cli -s 500

# Combine filters
github-search-cli -L rust -s 500 --sort stars
```

### Keyboard Efficiency
- No need to click anything - it's all keyboard-driven
- Arrow keys for navigation
- Enter for actions (search/open)
- Esc for quick exit

## Common Use Cases

### Exploring New Projects
1. Start TUI: `github-search-cli`
2. Search: "rust cli tools"
3. Browse results with arrow keys
4. Open interesting repos in browser

### Research Mode
```bash
# Start with specific filters for research
github-search-cli -L python -s 1000 --sort stars
# Then search: "machine learning"
```

### Quick Lookup
```bash
# Just use CLI mode for quick lookups
github-search-cli actix web
```

## Troubleshooting

### TUI doesn't start
- Make sure your terminal supports ANSI colors
- Try updating your terminal emulator
- Use `--no-tui` flag to force CLI mode

### Search is slow
- Get a GitHub token to increase rate limits
- Export it: `export GITHUB_TOKEN=your_token_here`
- Or pass with `-t` flag

### Browser doesn't open
- Make sure you have a default browser set
- On Linux, install `xdg-utils`
- On macOS, browser opening is automatic
- On Windows, ensure default browser is configured

## Advanced

### Using with GitHub Token
```bash
# Set token in environment
export GITHUB_TOKEN=ghp_your_token_here

# Start TUI
github-search-cli

# Now you have 5000 requests/hour instead of 60
```

### Scripting searches
For automated searches, use CLI mode:
```bash
# Get results as colored text
github-search-cli rust game --sort stars -l 5 > results.txt

# Use in scripts
#!/bin/bash
for lang in rust python go; do
    github-search-cli web framework -L $lang --no-tui
done
```

## Comparison: TUI vs CLI

| Feature | TUI Mode | CLI Mode |
|---------|----------|----------|
| Interactive | ✅ Yes | ❌ No |
| Browser opening | ✅ Yes | ❌ No |
| Multiple searches | ✅ Easy | ❌ Re-run command |
| Details panel | ✅ Live | ❌ No |
| Scriptable | ❌ No | ✅ Yes |
| Pipe-able output | ❌ No | ✅ Yes |
| Speed | 🐢 Interactive | 🚀 Instant |

Use TUI for exploration, CLI for automation!
