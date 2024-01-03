use std::env;
use std::fs;
use toml;

#[derive(Debug, Deserialize, Serialize)]
struct Config {
    node_address: String,
}

impl Config {
    fn new() -> Config {
        Config {
            node_address: String::new(),
        }
    }

    fn load() -> Config {
        let config_path = env::current_dir().unwrap().join("config.toml");
        if let Ok(content) = fs::read_to_string(&config_path) {
            toml::from_str(&content).unwrap_or_else(|_| Config::new())
        } else {
            Config::new()
        }
    }

    fn save(&self) {
        let config_path = env::current_dir().unwrap().join("config.toml");
        let content = toml::to_string_pretty(self).unwrap();
        fs::write(config_path, content).unwrap();
    }
}
