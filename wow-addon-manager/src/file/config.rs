use serde::Deserialize;
use std::fs;
use std::io;

#[derive(Debug, Deserialize)]
pub struct Config {
    path:       String,
    github:     Option<Vec<String>>,
    curseforge: Option<Vec<u32>>,
}

#[derive(Debug)]
pub enum ConfigError {
    IOError(io::Error),
    ParseError(serde_yaml::Error),
}

impl From<io::Error> for ConfigError {
    fn from(error: io::Error) -> Self {
        Self::IOError(error)
    }
}

impl From<serde_yaml::Error> for ConfigError {
    fn from(error: serde_yaml::Error) -> Self {
        Self::ParseError(error)
    }
}

pub fn read_config(path: &str) -> Result<Config, ConfigError> {
    let content = fs::read_to_string(path)?;
    let config  = serde_yaml::from_str(&content)?;
    Ok(config)
}
