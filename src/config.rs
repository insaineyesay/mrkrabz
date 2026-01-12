use anyhow::{Context, Result};
use serde::Deserialize;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(default = "default_filecount_script")]
    pub filecount_script: String,
}

fn default_filecount_script() -> String {
    "mac_zsh".to_string()
}

impl Config {
    /// Load configuration from config.toml
    /// Falls back to defaults if file doesn't exist
    pub fn load() -> Result<Self> {
        let config_path = PathBuf::from("config.toml");

        if !config_path.exists() {
            // Return default config if file doesn't exist
            return Ok(Config {
                filecount_script: default_filecount_script(),
            });
        }

        let contents = fs::read_to_string(&config_path)
            .context("Failed to read config.toml")?;

        let config: Config = toml::from_str(&contents)
            .context("Failed to parse config.toml")?;

        Ok(config)
    }

    /// Get the filecount script path based on configuration
    pub fn get_filecount_script_path(&self) -> String {
        match self.filecount_script.as_str() {
            "mac_bash" => "mac_linux_bash_filecount.sh".to_string(),
            "windows" => "windows_filecount.ps1".to_string(),
            _ => "filecount.sh".to_string(), // Default to mac_zsh
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = Config {
            filecount_script: default_filecount_script(),
        };
        assert_eq!(config.filecount_script, "mac_zsh");
        assert_eq!(config.get_filecount_script_path(), "filecount.sh");
    }

    #[test]
    fn test_mac_bash_config() {
        let config = Config {
            filecount_script: "mac_bash".to_string(),
        };
        assert_eq!(config.get_filecount_script_path(), "mac_linux_bash_filecount.sh");
    }

    #[test]
    fn test_windows_config() {
        let config = Config {
            filecount_script: "windows".to_string(),
        };
        assert_eq!(config.get_filecount_script_path(), "windows_filecount.ps1");
    }

    #[test]
    fn test_unknown_config_defaults_to_mac_zsh() {
        let config = Config {
            filecount_script: "unknown".to_string(),
        };
        assert_eq!(config.get_filecount_script_path(), "filecount.sh");
    }
}
