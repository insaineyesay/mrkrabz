# Mr Krabz GitHub Search CLI

A powerful command-line tool to search GitHub repositories with both CLI and interactive TUI modes, built with Rust and Octocrab.

![TUI Screenshot](image.png)
![Demo](mrkrabzdemo.gif)

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
cargo run
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
cargo run -- large rust game
```

### Prerequisites
- Rust 1.75 or higher
- Cargo

### Build from source

```bash
# Clone or create the project
cd mrkrabz

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
cargo run

# Or specify options that will apply to searches
cargo run -- -L rust --sort stars
```

In TUI mode:
1. Type your search query (e.g., "rust game")
2. Press 1/2/3 to choose small, medium, or large repos. 
3. Press **Enter** to search
4. Press **Alt+F** to perform a file count (Currently Mac Only, Windows coming soon)
5. Use **↑/↓** to navigate results
6. Press **Alt+O** on a result to open in browser
7. Press **Esc** or **Ctrl+C** to quit

### CLI Mode

```bash
# Basic search
cargo run -- large rust game

# Limit results
cargo run -- rust web framework -l 5

# Force CLI mode even without query
cargo run -- --no-tui
```

### Advanced filtering

```bash
# Filter by language
cargo run -- machine learning -L python

# Filter by minimum stars
cargo run -- web scraper -s 1000

# Combine filters
cargo run -- game engine -L rust -s 500 -l 10
```

### Sorting

```bash
# Sort by stars (descending)
cargo run -- neural network --sort stars

# Sort by last updated
cargo run -- cli tool --sort updated

# Sort by forks
cargo run -- dotfiles --sort forks
```

### Using with GitHub Token

For higher rate limits (5000/hour vs 60/hour), use a GitHub Personal Access Token:

```bash
# Method 1: Pass as argument
cargo run -- rust game -t YOUR_TOKEN_HERE

# Method 2: Set environment variable
export GITHUB_TOKEN=YOUR_TOKEN_HERE
cargo run -- rust game
```

**To create a token:**
1. Go to https://github.com/settings/tokens
2. Click "Generate new token (classic)"
3. Select no scopes (public repo search doesn't need permissions)
4. Copy the token

## Examples

```bash
# Find popular Rust games
cargo run -- large rust game -s 100 --sort stars

# Find Python data science projects
cargo run -- data science -L python -s 1000 -l 15

# Find recently updated web frameworks
cargo run -- web framework --sort updated -l 20

# Find React component libraries
cargo run -- react components -L javascript -s 500
```

## Command-line Options

```
Usage: cargo run -- [OPTIONS] [QUERY]...

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

## Configuration

The tool supports different filecount scripts for different platforms. Create a `config.toml` file in the project root:

```toml
# Options: "mac_zsh", "mac_bash", "windows"
filecount_script = "mac_zsh"
```

### Available Options:

- **`mac_zsh`** (default) - Uses `filecount.sh` - Tested and recommended for macOS with zsh
- **`mac_bash`** - Uses `mac_linux_bash_filecount.sh` - For macOS/Linux with bash (⚠️ untested)
- **`windows`** - Uses `windows_filecount.ps1` - For Windows PowerShell (⚠️ untested)

**Note:** The bash and Windows versions have not been fully tested. If you encounter issues, please open an issue or submit a PR with fixes. Contributions are welcome!

If no `config.toml` is present, the tool defaults to `mac_zsh`.

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

## Contributing

Contributions welcome! Feel free to open issues or submit PRs.
