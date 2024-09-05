use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub hosts: Vec<HostConfig>, // Vec to hold a list of hosts
    pub users: Vec<UserConfig>, // Vec to hold a list of users
}

#[derive(Debug, Deserialize)]
pub struct HostConfig {
    pub name: String,
    pub ip: String,
}

#[derive(Debug, Deserialize)]
pub struct UserConfig {
    pub username: String,
    pub password: String,
    pub groups: Vec<String>,
}

impl Config {
    pub fn load(file_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let config_str = fs::read_to_string(file_path)?;
        let config: Config = toml::from_str(&config_str)?;
        Ok(config)
    }

    // Retrieve a list of hosts
    pub fn get_hosts(&self) -> &Vec<HostConfig> {
        &self.hosts
    }
}
