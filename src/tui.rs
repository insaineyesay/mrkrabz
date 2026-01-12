use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use octocrab::models::Repository;
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Wrap},
    Frame, Terminal,
};
use std::io;
use tui_input::backend::crossterm::EventHandler;
use tui_input::Input;

use std::collections::HashMap;

/// Main application state
pub struct App {
    pub input: Input,                               // Search input field
    pub results: Vec<Repository>,                   // Search results from GitHub
    pub list_state: ListState,                      // Which result is selected
    pub searching: bool,                            // Currently performing search
    pub error_message: Option<String>,              // Error to display
    pub total_count: Option<u64>,                   // Total results from GitHub
    pub file_counts: HashMap<String, String>,       // Cached file counts per repo URL
    pub counting_files: bool,                       // Currently counting files
    pub details_scroll: u16,                        // Scroll offset for details panel
    pub repo_size_filter: Option<String>,           // Current size filter: small, medium, large, or None
    pub cloning: bool,                              // Currently cloning a repository
    pub clone_status: Option<String>,               // Status message from last clone attempt
}

impl App {
    pub fn new() -> Self {
        Self {
            input: Input::default(),
            results: Vec::new(),
            list_state: ListState::default(),
            searching: false,
            error_message: None,
            total_count: None,
            file_counts: HashMap::new(),
            counting_files: false,
            details_scroll: 0,
            repo_size_filter: None,
            cloning: false,
            clone_status: None,
        }
    }

    /// Set the repository size filter
    pub fn set_size_filter(&mut self, filter: Option<String>) {
        self.repo_size_filter = filter;
    }

    /// Set search results and automatically select the first item
    pub fn set_results(&mut self, results: Vec<Repository>, total_count: u64) {
        self.results = results;
        self.total_count = Some(total_count);
        if !self.results.is_empty() {
            self.list_state.select(Some(0)); // Auto-select first result
        }
        self.searching = false;
    }

    /// Set an error message to display
    pub fn set_error(&mut self, error: String) {
        self.error_message = Some(error);
        self.searching = false;
    }

    /// Navigate to the next repository in the list (wraps around)
    pub fn next(&mut self) {
        if self.results.is_empty() {
            return;
        }
        let i = match self.list_state.selected() {
            Some(i) => {
                if i >= self.results.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.list_state.select(Some(i));
    }

    /// Navigate to the previous repository in the list (wraps around)
    pub fn previous(&mut self) {
        if self.results.is_empty() {
            return;
        }
        let i = match self.list_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.results.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.list_state.select(Some(i));
    }

    /// Get the currently selected repository
    pub fn get_selected_repo(&self) -> Option<&Repository> {
        self.list_state.selected().and_then(|i| self.results.get(i))
    }

    /// Scroll details panel down
    pub fn scroll_details_down(&mut self) {
        self.details_scroll = self.details_scroll.saturating_add(1);
    }

    /// Scroll details panel up
    pub fn scroll_details_up(&mut self) {
        self.details_scroll = self.details_scroll.saturating_sub(1);
    }

    /// Reset details scroll when changing repos
    pub fn reset_details_scroll(&mut self) {
        self.details_scroll = 0;
    }
}

/// Main TUI event loop
/// Returns None if user wants to quit, or Some(action) for:
/// - Search queries (plain text)
/// - File count requests ("FILECOUNT:url")
/// - Clone requests ("CLONE:url")
/// - Browser open requests (http URLs)
pub fn run_tui<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> Result<Option<String>> {
    loop {
        // Render the UI
        terminal.draw(|f| ui(f, app))?;

        // Wait for keyboard input
        if let Event::Key(key) = event::read()? {
            // Ctrl+C to quit
            if key.code == KeyCode::Char('c') && key.modifiers.contains(KeyModifiers::CONTROL) {
                return Ok(None);
            }

            match key.code {
                KeyCode::Esc => return Ok(None),
                KeyCode::Down => {
                    app.next();
                    app.reset_details_scroll(); // Reset scroll when changing repos
                }
                KeyCode::Up => {
                    app.previous();
                    app.reset_details_scroll(); // Reset scroll when changing repos
                }
                KeyCode::Left => app.scroll_details_up(),    // Scroll details up
                KeyCode::Right => app.scroll_details_down(),  // Scroll details down
                KeyCode::Enter => {
                    // Otherwise, submit search query if input is not empty
                    if !app.input.value().is_empty() {
                        return Ok(Some(app.input.value().to_string()));
                    }
                }
                KeyCode::Char('o') if key.modifiers.contains(KeyModifiers::ALT) => {
                    // If we have results and something is selected, open it
                    if !app.results.is_empty() && app.list_state.selected().is_some() {
                        if let Some(repo) = app.get_selected_repo() {
                            if let Some(url) = &repo.html_url {
                                return Ok(Some(url.to_string()));
                            }
                        }
                    }
                }
                KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::ALT) => {
                    // Alt+C to clear search input
                    app.input.reset();
                }
                KeyCode::Char('f') if key.modifiers.contains(KeyModifiers::ALT) => {
                    // Trigger file count for selected repo
                    if !app.results.is_empty() && app.list_state.selected().is_some() {
                        if let Some(repo) = app.get_selected_repo() {
                            if let Some(url) = &repo.html_url {
                                return Ok(Some(format!("FILECOUNT:{}", url)));
                            }
                        }
                    }
                }
                KeyCode::Char('1') => {
                    // Set size filter to small
                    app.set_size_filter(Some("small".to_string()));
                }
                KeyCode::Char('2') => {
                    // Set size filter to medium
                    app.set_size_filter(Some("medium".to_string()));
                }
                KeyCode::Char('3') => {
                    // Set size filter to large
                    app.set_size_filter(Some("large".to_string()));
                }
                KeyCode::Char('0') => {
                    // Clear size filter
                    app.set_size_filter(None);
                }
                KeyCode::Char('g') if key.modifiers.contains(KeyModifiers::ALT) => {
                    // Alt+G to clone selected repo
                    if !app.results.is_empty() && app.list_state.selected().is_some() {
                        if let Some(repo) = app.get_selected_repo() {
                            if let Some(url) = &repo.html_url {
                                return Ok(Some(format!("CLONE:{}", url)));
                            }
                        }
                    }
                }
                _ => {
                    // Handle text input
                    app.input.handle_event(&Event::Key(key));
                }
            }
        }
    }
}

/// Main UI rendering function
/// Splits screen into 4 sections: Search input, Results list, Details panel, Help bar
pub fn ui(f: &mut Frame, app: &mut App) {
    // Define layout: split terminal into 4 vertical sections
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(3),  // Search box: fixed 3 lines
            Constraint::Min(10),    // Results: at least 10 lines, expands with screen
            Constraint::Min(20),    // Details: at least 20 lines for file count output
            Constraint::Length(3),  // Help: fixed 3 lines
        ])
        .split(f.area());

    // Search input box with size filter indicator
    let title = if let Some(filter) = &app.repo_size_filter {
        format!(" 🔍 Search GitHub Repositories [Filter: {}] ", filter)
    } else {
        " 🔍 Search GitHub Repositories ".to_string()
    };

    let input_block = Block::default()
        .borders(Borders::ALL)
        .title(title)
        .border_style(Style::default().fg(Color::Cyan));

    let input_text = app.input.value();
    let input_widget = Paragraph::new(input_text)
        .block(input_block)
        .style(Style::default().fg(Color::Black));

    f.render_widget(input_widget, chunks[0]);

    // Set cursor position
    let cursor_x = chunks[0].x + app.input.visual_cursor() as u16 + 1;
    let cursor_y = chunks[0].y + 1;
    f.set_cursor_position((cursor_x, cursor_y));

    // Results list
    if app.searching {
        let searching_text = Paragraph::new("Searching... ⏳")
            .block(Block::default().borders(Borders::ALL).title(" Results "))
            .alignment(Alignment::Center)
            .style(Style::default().fg(Color::Yellow));
        f.render_widget(searching_text, chunks[1]);
    } else if let Some(error) = &app.error_message {
        let error_text = Paragraph::new(format!("❌ Error: {}", error))
            .block(Block::default().borders(Borders::ALL).title(" Results "))
            .style(Style::default().fg(Color::Red))
            .wrap(Wrap { trim: true });
        f.render_widget(error_text, chunks[1]);
    } else if app.results.is_empty() {
        let welcome_text = vec![
            Line::from(""),
            Line::from(Span::styled(
                "Welcome to Mr Krabz GitHub Search! 👋",
                Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
            )),
            Line::from(""),
            Line::from("Type a search query and press Enter to search."),
            Line::from("Examples: 'rust game', 'web framework', 'machine learning'"),
            Line::from(""),
            Line::from("Use ↑/↓ to navigate results, Enter to open in browser."),
        ];

        let welcome = Paragraph::new(welcome_text)
            .block(Block::default().borders(Borders::ALL).title(" Results "))
            .alignment(Alignment::Center)
            .style(Style::default().fg(Color::Gray));
        f.render_widget(welcome, chunks[1]);
    } else {
        let title = if let Some(total) = app.total_count {
            format!(" Results ({} total) ", total)
        } else {
            " Results ".to_string()
        };

        let items: Vec<ListItem> = app
            .results
            .iter()
            .map(|repo| {
                let name = repo.full_name.as_ref().map(|s| s.as_str()).unwrap_or("Unknown");
                let stars = repo.stargazers_count.unwrap_or(0);
                let language = repo.language
                    .as_ref()
                    .and_then(|v| v.as_str())
                    .unwrap_or("Unknown");
                let size = repo.size.unwrap_or(0);

                let line = Line::from(vec![
                    Span::styled(name, Style::default().fg(Color::Gray).add_modifier(Modifier::BOLD)),
                    Span::raw(" | "),
                    Span::styled(format!("⭐ {}", stars), Style::default().fg(Color::Yellow)),
                    Span::raw(" | "),
                    Span::styled(language, Style::default().fg(Color::Blue)),
                    Span::styled(format!("📦 {}", size), Style::default().fg(Color::Gray))
                ]);

                ListItem::new(line)
            })
            .collect();

        let list = List::new(items)
            .block(Block::default().borders(Borders::ALL).title(title))
            .highlight_style(
                Style::default()
                    .bg(Color::DarkGray)
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol("▶ ");

        f.render_stateful_widget(list, chunks[1], &mut app.list_state);
    }

    // Details panel
    let details_block = Block::default().borders(Borders::ALL).title(" Details ");

    if let Some(repo) = app.get_selected_repo() {
        let description = repo.description.as_deref().unwrap_or("No description");
        let stars = repo.stargazers_count.unwrap_or(0);
        let forks = repo.forks_count.unwrap_or(0);
        let language = repo.language
            .as_ref()
            .and_then(|v| v.as_str())
            .unwrap_or("Unknown");
        let url = repo.html_url.as_ref().map(|u| u.to_string()).unwrap_or_default();

        let mut details_text = vec![
            Line::from(vec![
                Span::styled("Description: ", Style::default().fg(Color::Gray)),
                Span::raw(description),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("⭐ Stars: ", Style::default().fg(Color::Yellow)),
                Span::raw(format!("{}", stars)),
                Span::raw("  "),
                Span::styled("🍴 Forks: ", Style::default().fg(Color::Green)),
                Span::raw(format!("{}", forks)),
                Span::raw("  "),
                Span::styled("💻 Language: ", Style::default().fg(Color::Blue)),
                Span::raw(language),
            ]),
            Line::from(""),
        ];

        // Show clone status if available
        if let Some(status) = &app.clone_status {
            details_text.push(Line::from(vec![
                Span::styled("📦 Clone: ", Style::default().fg(Color::Cyan)),
                Span::styled(status, Style::default().fg(Color::Green)),
            ]));
            details_text.push(Line::from(""));
        }

        // Add file count if available
        if app.counting_files {
            details_text.push(Line::from(vec![
                Span::styled("📁 Files: ", Style::default().fg(Color::Magenta)),
                Span::styled("Cloning and counting...", Style::default().fg(Color::Yellow)),
            ]));
        } else if app.cloning {
            details_text.push(Line::from(vec![
                Span::styled("📦 Cloning: ", Style::default().fg(Color::Cyan)),
                Span::styled("Please wait...", Style::default().fg(Color::Yellow)),
            ]));
        } else if let Some(count) = app.file_counts.get(&url) {
            details_text.push(Line::from(vec![
                Span::styled("📁 File Count:", Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD)),
            ]));
            details_text.push(Line::from(""));

            // Split the output into lines and add each as a separate Line
            for line in count.lines() {
                details_text.push(Line::from(Span::raw(line)));
            }
        } else {
            details_text.push(Line::from(vec![
                Span::styled("📁 Files: ", Style::default().fg(Color::Magenta)),
                Span::styled("Press 'f' to count", Style::default().fg(Color::DarkGray)),
            ]));
        }

        details_text.push(Line::from(""));
        details_text.push(Line::from(vec![
            Span::styled("🔗 ", Style::default().fg(Color::Cyan)),
            Span::styled(url, Style::default().fg(Color::Cyan).add_modifier(Modifier::UNDERLINED)),
        ]));

        // Create scrollable paragraph with scroll offset
        let details = Paragraph::new(details_text)
            .block(details_block)
            .wrap(Wrap { trim: true })
            .scroll((app.details_scroll, 0)); // Apply vertical scroll
        f.render_widget(details, chunks[2]);
    } else {
        let empty = Paragraph::new("Select a repository to see details")
            .block(details_block)
            .style(Style::default().fg(Color::Gray))
            .alignment(Alignment::Center);
        f.render_widget(empty, chunks[2]);
    }

    // Help text
    let help_text = vec![
        Line::from(vec![
            Span::styled("Enter", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
            Span::raw(": Search  "),
            Span::styled("Alt+O", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
            Span::raw(": Open  "),
            Span::styled("↑↓", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
            Span::raw(": Navigate  "),
            Span::styled("Alt+F", Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD)),
            Span::raw(": Count  "),
            Span::styled("Alt+G", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
            Span::raw(": Clone  "),
            Span::styled("1/2/3/0", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            Span::raw(": Filter  "),
            Span::styled("Esc", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)),
            Span::raw(": Quit"),
        ]),
    ];

    let help = Paragraph::new(help_text)
        .block(Block::default().borders(Borders::ALL).title(" Help "))
        .alignment(Alignment::Center);
    f.render_widget(help, chunks[3]);
}

pub fn setup_terminal() -> Result<Terminal<CrosstermBackend<io::Stdout>>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend)?;
    Ok(terminal)
}

pub fn restore_terminal(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> Result<()> {
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_initialization() {
        let app = App::new();
        assert!(app.input.value().is_empty());
        assert!(app.results.is_empty());
        assert_eq!(app.repo_size_filter, None);
        assert!(!app.searching);
        assert!(!app.counting_files);
        assert!(!app.cloning);
    }

    #[test]
    fn test_set_size_filter() {
        let mut app = App::new();

        app.set_size_filter(Some("large".to_string()));
        assert_eq!(app.repo_size_filter, Some("large".to_string()));

        app.set_size_filter(None);
        assert_eq!(app.repo_size_filter, None);
    }

    #[test]
    fn test_details_scroll() {
        let mut app = App::new();

        assert_eq!(app.details_scroll, 0);

        app.scroll_details_down();
        assert_eq!(app.details_scroll, 1);

        app.scroll_details_down();
        assert_eq!(app.details_scroll, 2);

        app.scroll_details_up();
        assert_eq!(app.details_scroll, 1);

        app.scroll_details_up();
        assert_eq!(app.details_scroll, 0);

        // Test that it doesn't go below 0
        app.scroll_details_up();
        assert_eq!(app.details_scroll, 0);

        // Test reset
        app.scroll_details_down();
        app.reset_details_scroll();
        assert_eq!(app.details_scroll, 0);
    }

    #[test]
    fn test_get_selected_repo_with_no_results() {
        let mut app = App::new();

        // No selection returns None
        assert!(app.get_selected_repo().is_none());

        // With selection but no results returns None
        app.list_state.select(Some(0));
        assert!(app.get_selected_repo().is_none());
    }

    #[test]
    fn test_input_operations() {
        let mut app = App::new();

        // Test that input starts empty
        assert!(app.input.value().is_empty());

        // Test Alt+C clear functionality (input.reset())
        // We can't easily test the actual key handler without mocking terminal input,
        // but we can test the underlying function
        app.input = Input::from("test query");
        assert_eq!(app.input.value(), "test query");

        app.input.reset();
        assert!(app.input.value().is_empty());
    }

    #[test]
    fn test_searching_and_counting_flags() {
        let mut app = App::new();

        assert!(!app.searching);
        assert!(!app.counting_files);
        assert!(!app.cloning);

        app.searching = true;
        assert!(app.searching);

        app.counting_files = true;
        assert!(app.counting_files);

        app.cloning = true;
        assert!(app.cloning);
    }

    #[test]
    fn test_set_results_auto_selects_first() {
        let mut app = App::new();

        // set_results auto-selects first item when results is not empty
        app.set_results(Vec::new(), 0);
        assert!(app.list_state.selected().is_none());

        // We can't easily create a full Repository here, so we test the logic
        // The actual functionality is: if results is not empty, select index 0
    }

    #[test]
    fn test_navigation_wrapping() {
        let mut app = App::new();

        // Test that navigation doesn't panic with empty results
        app.next();
        assert!(app.list_state.selected().is_none());

        app.previous();
        assert!(app.list_state.selected().is_none());
    }
}
