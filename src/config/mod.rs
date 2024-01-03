use std::env;
use std::fs;
use toml;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub node_address: String,
}

/// Represents a configuration for the node_tester_cli application.
/// 
/// The `Config` struct provides methods for creating, loading, and saving configurations.
/// 
/// # Example
/// 
/// ```
/// use node_tester_cli::config::Config;
/// 
/// // Create a new configuration
/// let config = Config::new();
/// 
/// // Load a configuration from file
/// let loaded_config = Config::load();
/// 
/// // Save the current configuration to file
/// config.save();
/// ```
impl Config {
    pub fn new() -> Config {
        Config {
            node_address: String::new(),
        }
    }

    pub fn load() -> Config {
        let config_path = env::current_dir().unwrap().join("config.toml");
        if let Ok(content) = fs::read_to_string(&config_path) {
            toml::from_str(&content).unwrap_or_else(|_| Config::new())
        } else {
            Config::new()
        }
    }

    pub fn save(&self) {
        let config_path = env::current_dir().unwrap().join("config.toml");
        let content = toml::to_string_pretty(self).unwrap();
        fs::write(config_path, content).unwrap();
    }
}
