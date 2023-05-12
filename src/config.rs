use dirs;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

const CONFIG_FILE_NAME: &'static str = ".flomo";

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub api: Option<String>,
}

fn get_config_file_path() -> Option<PathBuf> {
    let config_dir = dirs::config_dir()?;

    Some(config_dir.join(CONFIG_FILE_NAME))
}

pub fn init_config() -> bool {
    match get_config_file_path() {
        Some(file) => {
            if !file.exists() {
                return match fs::write(file, "") {
                    Ok(_) => true,
                    Err(_) => false,
                };
            }

            true
        }
        None => false,
    }
}

pub fn read_config() -> Option<Config> {
    let file = get_config_file_path()?;

    let content = fs::read_to_string(file).ok()?;

    toml::from_str(&content).ok()
}

pub fn read_config_string() -> Option<String> {
    let config = read_config()?;

    toml::to_string(&config).ok()
}

pub fn write_config(config: &Config) -> bool {
    let file = match get_config_file_path() {
        Some(file) => file,
        None => return false,
    };

    let str = match toml::to_string(config) {
        Ok(str) => str,
        Err(_) => return false,
    };

    match fs::write(file, str) {
        Ok(_) => true,
        Err(_) => false,
    }
}
