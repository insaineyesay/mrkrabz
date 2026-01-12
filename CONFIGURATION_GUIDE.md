# File Count Script Configuration Guide

This guide explains how to configure which filecount script the GitHub Search CLI uses.

## Quick Start

Create a `config.toml` file in the project root directory:

```toml
# GitHub Search CLI Configuration

# File count script selection
# Options: "mac_zsh", "mac_bash", "windows"
filecount_script = "mac_zsh"
```

## Available Scripts

### 1. mac_zsh (Default) ✅ Tested
- **File**: `filecount.sh`
- **Platform**: macOS with zsh shell
- **Status**: Fully tested and working
- **Use when**: Running on macOS with default zsh shell

### 2. mac_bash ⚠️ Untested
- **File**: `mac_linux_bash_filecount.sh`
- **Platform**: macOS or Linux with bash shell
- **Status**: Not fully tested
- **Use when**: Running on macOS with bash or Linux systems
- **Note**: Should work but hasn't been verified in production

### 3. windows ⚠️ Untested
- **File**: `windows_filecount.ps1`
- **Platform**: Windows with PowerShell
- **Status**: Not fully tested
- **Use when**: Running on Windows
- **Known issues**:
  - May fail on binary files (needs error handling)
  - Regex pattern may need adjustment
- **Note**: Requires PowerShell execution policy to allow scripts

## Configuration Details

### How It Works

1. When you press 'f' to count files in the TUI, the application:
   - Loads `config.toml` (if it exists)
   - Determines which script to use based on `filecount_script` setting
   - Falls back to `mac_zsh` if no config exists
   - Clones the repository to a temp directory
   - Copies the appropriate script to the cloned repo
   - Executes the script and displays results

### File Locations

All filecount scripts must be in the project root directory:
```
mrkrabsgithubclient/
├── config.toml                      # Configuration file
├── filecount.sh                     # macOS zsh (default)
├── mac_linux_bash_filecount.sh      # macOS/Linux bash
└── windows_filecount.ps1            # Windows PowerShell
```

### Default Behavior

If `config.toml` doesn't exist or doesn't specify `filecount_script`, the tool defaults to:
- **Script**: `filecount.sh`
- **Platform**: macOS with zsh

## Platform-Specific Setup

### macOS (zsh) - Default
No configuration needed! Just use the tool.

### macOS (bash) or Linux
1. Create `config.toml`:
```toml
filecount_script = "mac_bash"
```

2. Ensure `mac_linux_bash_filecount.sh` is executable:
```bash
chmod +x mac_linux_bash_filecount.sh
```

### Windows
1. Create `config.toml`:
```toml
filecount_script = "windows"
```

2. Ensure PowerShell execution policy allows scripts:
```powershell
Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope CurrentUser
```

## Testing Your Configuration

After configuring, test it:

1. Run the TUI:
```bash
cargo run
```

2. Search for a repository
3. Press 'f' on a selected repository
4. Check if the file count displays correctly

## Troubleshooting

### "Script not found" Error
- Ensure the script file exists in the project root
- Check the filename matches exactly (case-sensitive on Unix)
- Verify `config.toml` has the correct `filecount_script` value

### Script Won't Execute (Unix/macOS)
- Make the script executable: `chmod +x <script_name>`
- Verify the shebang line is correct: `#!/bin/bash` or `#!/bin/zsh`

### PowerShell Script Won't Run (Windows)
- Check execution policy: `Get-ExecutionPolicy`
- Allow scripts: `Set-ExecutionPolicy RemoteSigned`
- Run as administrator if needed

### Incorrect Results or Errors
- The bash and Windows scripts are untested and may have bugs
- Please open an issue on GitHub with:
  - Your platform (OS version, shell version)
  - The error message or incorrect output
  - Sample repository where it failed

## Contributing

If you test the bash or Windows scripts and find issues:

1. Fork the repository
2. Fix the script
3. Test thoroughly
4. Submit a PR with:
   - Description of the fix
   - Platform tested on
   - Sample output showing it works

We especially need help testing and improving:
- `mac_linux_bash_filecount.sh` on Linux systems
- `windows_filecount.ps1` on Windows

## Technical Details

### Code Location
The configuration system is implemented in:
- **Config module**: `src/config.rs`
- **Usage**: `src/main.rs` in `clone_and_count_files()`

### Script Selection Logic
```rust
match config.filecount_script.as_str() {
    "mac_bash" => "mac_linux_bash_filecount.sh",
    "windows" => "windows_filecount.ps1",
    _ => "filecount.sh",  // Default
}
```

### Tests
Run configuration tests:
```bash
cargo test config::
```

All tests should pass:
- `test_default_config` - Verifies default is mac_zsh
- `test_mac_bash_config` - Verifies bash script selection
- `test_windows_config` - Verifies Windows script selection
- `test_unknown_config_defaults_to_mac_zsh` - Verifies fallback behavior
