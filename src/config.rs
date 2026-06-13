use serde::{Serialize,Deserialize};
use std::collections::VecDeque;
use std::env;
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub history: VecDeque<String>,
}

fn get_config_file_path() -> Option<PathBuf> {
    let home = env::home_dir();
    home.map(|home| home.join("dumb_edit.toml"))
}

impl Config {

    pub fn new() -> Self {
        let config_file = get_config_file_path();
        if let Some(config_file) = config_file {
            let contents = fs::read_to_string(config_file);
            if let Ok(contents) = contents {
                let parsed = toml::from_str::<Config>(contents.as_str());
                if let Ok(config) = parsed {
                    return config;
                }
            }
        }

        let history: VecDeque<String> = VecDeque::new();
        Config { history }
    }

    pub fn save(&self) {
        let config_file = get_config_file_path();
        if let Some(config_file) = config_file {
            let config = toml::to_string(&self);
            if let Ok(config) = config {
                _ = fs::write(config_file, config);
            }
        }
    }
}