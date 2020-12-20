use serde::Deserialize;
use std::fmt;
use std::fs;
use std::io;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub path:       String,
    pub github:     Option<Vec<String>>,
    pub curseforge: Option<Vec<u32>>,
}

#[derive(Debug)]
pub enum ConfigError {
    IOError(io::Error),
    ParseError(serde_yaml::Error),
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::IOError(e)    => fmt::Display::fmt(e, f),
            ConfigError::ParseError(e) => fmt::Display::fmt(e, f),
        }
    }
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
