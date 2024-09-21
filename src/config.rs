use serde::{ Deserialize, Serialize };
use std::{ fs, path::PathBuf };

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub api_key: String,
    pub base_uri: String,
}

impl Config {
    pub fn get_config_path() -> PathBuf {
        let config_dir = dirs::config_dir().unwrap();
        config_dir.join("roh-config.json")
    }

    pub fn load() -> Option<Self> {
        let config_path = Self::get_config_path();
        if config_path.exists() {
            let config_data = fs::read_to_string(config_path).expect("Unable to read config file");
            serde_json::from_str(&config_data).ok()
        } else {
            None
        }
    }

    pub fn save(&self) {
        let config_path = Self::get_config_path();
        let config_data = serde_json::to_string(self).expect("Unable to serialize config");
        fs::write(config_path, config_data).expect("Unable to write config file");
    }
}
