// src/config.rs
use std::fs;
use std::path::Path;
use serde::{Deserialize, Serialize};
use reqwest;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub default_vpn: String,
    pub servers: Vec<String>,
}

impl Config {
    pub fn load_from_url(url: &str) -> Self {
        match reqwest::blocking::get(url) {
            Ok(response) => {
                if let Ok(config_str) = response.text() {
                    if let Ok(config) = serde_json::from_str(&config_str) {
                        return config;
                    }
                }
            }
            Err(_) => println!("Failed to fetch config from URL, using default configuration."),
        }
        Self::default()
    }

    pub fn default() -> Self {
        Config {
            default_vpn: "wireguard".to_string(),
            servers: vec!["1.1.1.1".to_string(), "8.8.8.8".to_string()],
        }
    }
}