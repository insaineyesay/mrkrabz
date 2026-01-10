# GitHub Search CLI

A powerful command-line tool to search GitHub repositories with both CLI and interactive TUI modes, built with Rust and Octocrab.

## ✨ Features

- 🎨 **Interactive TUI Mode** - Beautiful terminal interface with live search and keyboard navigation
- 🔍 **CLI Mode** - Classic command-line interface for scripting
- 🎯 Filter by programming language, stars, and more
- 📊 Sort by stars, forks, or last updated
- 🌈 Colorized output for easy reading
- 🚀 Fast and lightweight
- 🔑 Optional GitHub token for higher rate limits
- 🌐 Open repositories directly in your browser

## 🎬 Modes

### Interactive TUI Mode (Default)
Run without arguments to start the interactive mode:
```bash
github-search-cli
```

**TUI Features:**
- Type your search query in the search box
- Press Enter to search
- Use ↑/↓ arrow keys to navigate results
- Press Enter on a result to open in browser
- See live details panel with description and stats
- Press Esc or Ctrl+C to quit

### CLI Mode
Provide a search query to use CLI mode:
```bash
github-search-cli large rust game
```

### Prerequisites
- Rust 1.75 or higher
- Cargo

### Build from source

```bash
# Clone or create the project
cd github-search-cli

# Build the project
cargo build --release

# The binary will be at target/release/github-search-cli
```

### Optional: Add to PATH

```bash
# Linux/macOS
cp target/release/github-search-cli ~/.local/bin/
# or
sudo cp target/release/github-search-cli /usr/local/bin/

# Windows
# Copy target\release\github-search-cli.exe to a directory in your PATH
```

## Usage

### Interactive TUI Mode (Recommended)

```bash
# Start interactive mode
github-search-cli

# Or specify options that will apply to searches
github-search-cli -L rust --sort stars
```

In TUI mode:
1. Type your search query (e.g., "web framework")
2. Press **Enter** to search
3. Use **↑/↓** to navigate results
4. Press **Enter** on a result to open in browser
5. Press **Esc** or **Ctrl+C** to quit

### CLI Mode

```bash
# Basic search
github-search-cli large rust game

# Limit results
github-search-cli rust web framework -l 5

# Force CLI mode even without query
github-search-cli --no-tui
```

### Advanced filtering

```bash
# Filter by language
github-search-cli machine learning -L python

# Filter by minimum stars
github-search-cli web scraper -s 1000

# Combine filters
github-search-cli game engine -L rust -s 500 -l 10
```

### Sorting

```bash
# Sort by stars (descending)
github-search-cli neural network --sort stars

# Sort by last updated
github-search-cli cli tool --sort updated

# Sort by forks
github-search-cli dotfiles --sort forks
```

### Using with GitHub Token

For higher rate limits (5000/hour vs 60/hour), use a GitHub Personal Access Token:

```bash
# Method 1: Pass as argument
github-search-cli rust game -t YOUR_TOKEN_HERE

# Method 2: Set environment variable
export GITHUB_TOKEN=YOUR_TOKEN_HERE
github-search-cli rust game
```

**To create a token:**
1. Go to https://github.com/settings/tokens
2. Click "Generate new token (classic)"
3. Select no scopes (public repo search doesn't need permissions)
4. Copy the token

## Examples

```bash
# Find popular Rust games
github-search-cli large rust game -s 100 --sort stars

# Find Python data science projects
github-search-cli data science -L python -s 1000 -l 15

# Find recently updated web frameworks
github-search-cli web framework --sort updated -l 20

# Find React component libraries
github-search-cli react components -L javascript -s 500
```

## Command-line Options

```
Usage: github-search-cli [OPTIONS] [QUERY]...

Arguments:
  [QUERY]...  Search query (e.g., "large rust game"). Omit to start TUI mode.

Options:
  -l, --limit <LIMIT>          Number of results to display [default: 10]
  -L, --language <LANGUAGE>    Filter by language (e.g., "rust", "python")
  -s, --stars <STARS>          Filter by minimum stars
      --sort <SORT>            Sort by: stars, forks, updated (default: best match)
  -t, --token <TOKEN>          GitHub personal access token [env: GITHUB_TOKEN]
      --no-tui                 Force CLI mode (no interactive TUI)
  -h, --help                   Print help
  -V, --version                Print version
```

## Keyboard Shortcuts (TUI Mode)

| Key | Action |
|-----|--------|
| **Enter** | Search (in search box) / Open in browser (on result) |
| **↑ / ↓** | Navigate results |
| **Esc** | Quit |
| **Ctrl+C** | Quit |
| **Any key** | Type in search box |

## Development

### Run without building

```bash
cargo run -- large rust game
```

### Run tests

```bash
cargo test
```

### Format code

```bash
cargo fmt
```

### Lint

```bash
cargo clippy
```

## Rate Limits

- **Without token:** 60 requests/hour
- **With token:** 5,000 requests/hour

The tool will work fine without a token for casual use, but if you're doing extensive searches, a token is recommended.

## Future Enhancements

Potential features to add:
- [x] Interactive TUI mode ✅
- [x] Open repositories in browser ✅
- [ ] Pagination (next/previous page)
- [ ] Filter by date ranges
- [ ] Search for trending repositories
- [ ] Clone repository directly from TUI
- [ ] Display README preview in TUI
- [ ] Filter by topics/tags
- [ ] Save favorite searches
- [ ] Export results to JSON/CSV
- [ ] Search history with recall

## License

MIT

## Contributing

Contributions welcome! Feel free to open issues or submit PRs.
