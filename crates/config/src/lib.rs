use std::env::{self, VarError};
use std::fmt::Debug;
use std::fs;
use std::io;
use std::path::Path;
use std::str::FromStr;

use serde::de::DeserializeOwned;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Failed to retrieve env variable: {0}")]
    ReadVariable(#[from] VarError),

    #[error("Failed to parse string: {0}")]
    StringParse(#[source] Box<dyn std::error::Error>),

    #[error("Failed to parse TOML: {0}")]
    TomlParse(#[from] toml::de::Error),

    #[error("IO Error: {0}")]
    Io(#[from] io::Error),
}

pub struct Config;
impl Config {
    pub fn from_env<T>(key: &str) -> Result<T, ConfigError>
    where
        T: FromStr,
        T::Err: std::error::Error + 'static,
    {
        let value_str: String = env::var(key)?;

        let value: T = value_str
            .parse::<T>()
            .map_err(|e: <T as FromStr>::Err| ConfigError::StringParse(Box::new(e)))?;

        Ok(value)
    }

    pub fn from_file<T, P>(path: P) -> Result<T, ConfigError>
    where
        T: DeserializeOwned,
        P: AsRef<Path>,
    {
        let content: String = fs::read_to_string(path)?;
        let config: T = toml::from_str(&content)?;
        Ok(config)
    }
}
