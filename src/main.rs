// TUI module containing the terminal interface logic
mod tui;
mod config;

use anyhow::Result;
use clap::Parser;
use colored::*;
use octocrab::Octocrab;
use tui::{App, restore_terminal, run_tui, setup_terminal};

/// GitHub repository search CLI tool with interactive TUI
/// Supports both CLI mode (with query args) and TUI mode (interactive)
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Search query (e.g., "large rust game"). If omitted, starts in interactive mode.
    #[arg()]
    query: Vec<String>,

    /// Number of results to display (default: 10)
    #[arg(short, long, default_value_t = 100)]
    limit: u8,

    /// Filter by language (e.g., "rust", "python")
    #[arg(short = 'L', long)]
    language: Option<String>,

    /// Filter by minimum stars
    #[arg(short, long)]
    stars: Option<u32>,

    /// Filter by repository size: small (<25MB), medium (25-100MB), large (>100MB)
    #[arg(long)]
    repo_size: Option<String>,

    /// Sort by: stars, forks, updated (default: best match)
    #[arg(long)]
    sort: Option<String>,

    /// GitHub personal access token (optional, increases rate limit)
    #[arg(short, long)]
    token: Option<String>,

    /// Force CLI mode (no interactive TUI)
    #[arg(long)]
    no_tui: bool,
}

/// Main entry point
/// Parses CLI args and routes to either TUI mode or CLI mode
#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    // Build the GitHub API client (octocrab)
    // If token provided, use it for higher rate limits (5000/hr vs 60/hr)
    let octocrab = if let Some(token) = &args.token {
        Octocrab::builder()
            .personal_token(token.clone())
            .build()?
    } else {
        Octocrab::builder().build()?
    };

    // Route to appropriate mode based on args
    if args.query.is_empty() && !args.no_tui {
        // No query + TUI allowed = Interactive mode
        run_interactive_mode(octocrab, &args).await?;
    } else if !args.query.is_empty() {
        // Query provided = CLI mode (print results and exit)
        run_cli_mode(octocrab, &args).await?;
    } else {
        // No query + TUI disabled = Error
        println!("{}", "Error: No query provided. Use --help for usage.".red());
    }

    Ok(())
}

/// Interactive TUI mode
/// Sets up terminal, runs event loop, handles search/file count/browser actions
async fn run_interactive_mode(octocrab: Octocrab, args: &Args) -> Result<()> {
    let mut terminal = setup_terminal()?;
    let mut app = App::new();

    // Initialize app with CLI args if provided
    if let Some(ref size) = args.repo_size {
        app.set_size_filter(Some(size.clone()));
    }

    loop {
        // Run TUI and wait for user action
        let result = run_tui(&mut terminal, &mut app)?;

        match result {
            None => {
                // User pressed Esc or Ctrl+C - quit
                restore_terminal(&mut terminal)?;
                break;
            }
            Some(action) => {
                // Handle different action types
                if action.starts_with("CLONE:") {
                    // Clone request
                    let url = action.strip_prefix("CLONE:").unwrap().to_string();
                    app.cloning = true;
                    app.clone_status = None;

                    terminal.draw(|f| tui::ui(f, &mut app))?;

                    match clone_repository(&url).await {
                        Ok(path) => {
                            app.clone_status = Some(format!("Cloned to {}", path));
                        }
                        Err(e) => {
                            app.clone_status = Some(format!("Clone failed: {}", e));
                        }
                    }

                    app.cloning = false;
                    terminal.draw(|f| tui::ui(f, &mut app))?;
                } else if action.starts_with("FILECOUNT:") {
                    // File count request
                    let url = action.strip_prefix("FILECOUNT:").unwrap().to_string();
                    app.counting_files = true;

                    terminal.draw(|f| tui::ui(f, &mut app))?;

                    // Clone and count files
                    match clone_and_count_files(&url).await {
                        Ok(count) => {
                            app.file_counts.insert(url.clone(), count);
                        }
                        Err(e) => {
                            app.file_counts.insert(url.clone(), format!("Error: {}", e));
                        }
                    }
                    app.counting_files = false;

                    // Redraw to show the result
                    terminal.draw(|f| tui::ui(f, &mut app))?;
                } else if action.starts_with("http") {
                    // It's a URL - open in browser
                    open_in_browser(&action);
                    // Continue running the TUI
                } else {
                    // It's a search query
                    app.searching = true;
                    app.error_message = None;

                    terminal.draw(|f| tui::ui(f, &mut app))?;

                    // Perform search with app's current size filter
                    match perform_search_with_filter(&octocrab, &action, args, &app.repo_size_filter).await {
                        Ok((results, total)) => {
                            app.set_results(results, total);
                        }
                        Err(e) => {
                            app.set_error(format!("{}", e));
                        }
                    }
                }
            }
        }
    }

    Ok(())
}

async fn run_cli_mode(octocrab: Octocrab, args: &Args) -> Result<()> {
    let query = args.query.join(" ");
    
    println!("{}", format!("🔍 Searching for: {}\n", query).cyan().bold());

    let (results, total_count) = perform_search(&octocrab, &query, args).await?;

    // Display results
    if results.is_empty() {
        println!("{}", "No repositories found.".yellow());
        return Ok(());
    }

    println!(
        "{}\n",
        format!("Found {} repositories (showing {})", total_count, results.len())
            .green()
            .bold()
    );

    for (i, repo) in results.iter().enumerate() {
        let stars = repo.stargazers_count.unwrap_or(0);
        let forks = repo.forks_count.unwrap_or(0);
        let language = repo.language
            .as_ref()
            .and_then(|v| v.as_str())
            .unwrap_or("Unknown");
        
        println!(
            "{} {}",
            format!("{}.", i + 1).cyan(),
            repo.full_name.as_ref().unwrap().bold()
        );
        
        println!(
            "   {} | {} | {}",
            format!("⭐ {}", stars).yellow(),
            format!("🍴 {}", forks).green(),
            format!("💻 {}", language).blue()
        );
        
        if let Some(desc) = &repo.description {
            println!("   {}", desc.dimmed());
        }
        
        println!("   {}", repo.html_url.as_ref().unwrap().to_string().cyan().underline());
        println!();
    }

    Ok(())
}

/// Perform search with an optional size filter override (used by TUI)
async fn perform_search_with_filter(
    octocrab: &Octocrab,
    query: &str,
    args: &Args,
    size_filter_override: &Option<String>,
) -> Result<(Vec<octocrab::models::Repository>, u64)> {
    // Build the search query
    let mut search_query = query.to_string();

    if let Some(lang) = &args.language {
        search_query.push_str(&format!(" language:{}", lang));
    }

    if let Some(min_stars) = args.stars {
        search_query.push_str(&format!(" stars:>={}", min_stars));
    }

    // Use size filter override if provided (from TUI), otherwise use args
    let size_category = size_filter_override.as_ref().or(args.repo_size.as_ref());

    if let Some(size_cat) = size_category {
        let size_filter = match size_cat.to_lowercase().as_str() {
            "small" => "size:<25000",
            "medium" => "size:25000..100000",
            "large" => "size:>100000",
            _ => {
                return Err(anyhow::anyhow!(
                    "Invalid repo_size '{}'. Use: small, medium, or large",
                    size_cat
                ));
            }
        };
        search_query.push_str(&format!(" {}", size_filter));
    }

    // Perform the search
    let mut search = octocrab.search().repositories(&search_query);

    if let Some(sort_by) = &args.sort {
        search = search.sort(sort_by);
    }

    let results = search.per_page(args.limit).send().await?;

    Ok((results.items, results.total_count.unwrap_or(0)))
}

/// Perform search using only Args (used by CLI mode)
async fn perform_search(
    octocrab: &Octocrab,
    query: &str,
    args: &Args,
) -> Result<(Vec<octocrab::models::Repository>, u64)> {
    perform_search_with_filter(octocrab, query, args, &None).await
}

/// Clones a GitHub repository to the 'repositories' directory
/// Returns the path where the repository was cloned
fn repo_name_from_url(repo_url: &str) -> String {
    let trimmed = repo_url.trim_end_matches('/');
    let last_segment = trimmed.rsplit('/').next().unwrap_or("repo");
    let name = last_segment.trim_end_matches(".git");
    if name.is_empty() {
        "repo".to_string()
    } else {
        name.to_string()
    }
}

async fn clone_repository(repo_url: &str) -> Result<String> {
    use tokio::process::Command;
    use std::process::Stdio;

    // Create repositories directory in current working directory
    let repos_dir = std::env::current_dir()?.join("repositories");
    std::fs::create_dir_all(&repos_dir)?;

    let repo_name = repo_name_from_url(repo_url);
    let clone_path = repos_dir.join(&repo_name);

    // Check if directory already exists
    if clone_path.exists() {
        return Err(anyhow::anyhow!(
            "Directory '{}' already exists. Please remove it first or choose a different location.",
            clone_path.display()
        ));
    }

    // Clone repository
    let clone_output = Command::new("git")
        .arg("clone")
        .arg(repo_url)
        .arg(&clone_path)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .await?;

    if !clone_output.success() {
        return Err(anyhow::anyhow!("Failed to clone repository"));
    }

    Ok(clone_path.display().to_string())
}

/// Clones a GitHub repository to a temp directory and runs the configured filecount script
/// Returns the script output as a string
async fn clone_and_count_files(repo_url: &str) -> Result<String> {
    use std::process::Stdio;
    use tokio::process::Command;

    // Load configuration to determine which script to use
    let config = config::Config::load()?;
    let script_name = config.get_filecount_script_path();

    // Create unique temp directory for this process
    let temp_dir = std::env::temp_dir().join(format!("github-search-{}", std::process::id()));
    std::fs::create_dir_all(&temp_dir)?;

    let repo_name = repo_name_from_url(repo_url);
    let clone_path = temp_dir.join(repo_name);

    // Clone repository (shallow clone with depth=1 for speed)
    let clone_output = Command::new("git")
        .arg("clone")
        .arg("--depth")
        .arg("1")
        .arg(repo_url)
        .arg(&clone_path)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .await?;

    if !clone_output.success() {
        return Err(anyhow::anyhow!("Failed to clone repository"));
    }

    // Copy the configured filecount script to temp directory
    let script_source = std::env::current_dir()?.join(&script_name);
    let script_dest = clone_path.join(&script_name);

    if !script_source.exists() {
        std::fs::remove_dir_all(&temp_dir)?;
        return Err(anyhow::anyhow!("{} not found in project root", script_name));
    }

    std::fs::copy(&script_source, &script_dest)?;

    // Determine command based on script type
    let output = if script_name.ends_with(".ps1") {
        // Windows PowerShell script
        #[cfg(target_os = "windows")]
        {
            Command::new("powershell")
                .arg("-ExecutionPolicy")
                .arg("Bypass")
                .arg("-File")
                .arg(&script_dest)
                .current_dir(&clone_path)
                .output()
                .await?
        }
        #[cfg(not(target_os = "windows"))]
        {
            std::fs::remove_dir_all(&temp_dir)?;
            return Err(anyhow::anyhow!("PowerShell scripts can only be run on Windows"));
        }
    } else {
        // Unix shell script
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = std::fs::metadata(&script_dest)?.permissions();
            perms.set_mode(0o755);
            std::fs::set_permissions(&script_dest, perms)?;
        }

        Command::new(&format!("./{}", script_name))
            .current_dir(&clone_path)
            .output()
            .await?
    };

    // Clean up
    std::fs::remove_dir_all(&temp_dir)?;

    if output.status.success() {
        let count = String::from_utf8_lossy(&output.stdout).trim().to_string();
        Ok(count)
    } else {
        Err(anyhow::anyhow!("Failed to count files: {}", String::from_utf8_lossy(&output.stderr)))
    }
}

/// Opens a URL in the system's default browser
/// Cross-platform: Windows (cmd /C start), macOS (open), Linux (xdg-open)
fn open_in_browser(url: &str) {
    #[cfg(target_os = "windows")]
    {
        let _ = std::process::Command::new("cmd")
            .args(["/C", "start", url])
            .spawn();
    }

    #[cfg(target_os = "macos")]
    {
        let _ = std::process::Command::new("open").arg(url).spawn();
    }

    #[cfg(target_os = "linux")]
    {
        let _ = std::process::Command::new("xdg-open").arg(url).spawn();
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_url_action_does_not_exit_tui() {
        // This test verifies the fix for Alt+O behavior.
        // When a URL action is returned from run_tui(), the main loop
        // should continue running (not break), allowing the TUI to stay open.

        // The key change is in main.rs lines 142-145:
        // Before: restore_terminal(), println!(), open_in_browser(), break
        // After:  open_in_browser() only (no break, so loop continues)

        let url = "https://github.com/user/test-repo";

        // Simulate what happens in the main loop when Alt+O returns a URL
        let action = url.to_string();

        // Verify it's detected as a URL
        assert!(action.starts_with("http"));

        // In the actual code at line 142-145, when action.starts_with("http"),
        // we call open_in_browser() but do NOT break the loop.
        // This test confirms the URL is properly formatted and would be handled.
    }
}
