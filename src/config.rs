use std::path::PathBuf;
use std::fs;
use serde::{Serialize, Deserialize};
use dirs;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub api: Option<String>,
}

fn get_config_file_path() -> Option<PathBuf> {
    match dirs::config_dir() {
        Some(path) => Some(path.join(".flomoconfig")),
        None => None,
    }
}

pub fn init_config() {
    match get_config_file_path() {
        Some(path) => {
            if !path.exists() {
                match fs::write(path, "") {
                    Ok(_) => (),
                    Err(_) => (),
                }
            }
        },
        None => (),
    }
}

pub fn read_config() -> Option<Config> {
    match get_config_file_path() {
        Some(path) => {
            match fs::read_to_string(path) {
                Ok(content) => toml::from_str(&content).ok(),
                Err(_) => None,
            }
        },
        None => None,
    }
}

pub fn read_config_string() -> Option<String> {
    match get_config_file_path() {
        Some(path) => {
            match fs::read_to_string(path) {
                Ok(content) => Some(content),
                Err(_) => None,
            }
        },
        None => None,
    }
}

pub fn write_config(config: &Config) -> bool {
    match get_config_file_path() {
        Some(path) => {
            match toml::to_string(config) {
                Ok(content) => {
                    match std::fs::write(path, content) {
                        Ok(_) => true,
                        Err(_) => false,
                    }
                },
                Err(_) => false,
            }
        },
        None => false,
    }
}
