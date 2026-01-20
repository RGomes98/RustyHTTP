use std::env;
use std::fs;
use std::path::Path;
use std::str::FromStr;

use super::ConfigError;
use serde::de::DeserializeOwned;
use tracing::{debug, warn};

pub struct Config;
impl Config {
    pub fn from_env<T>(key: &'static str) -> Result<T, ConfigError>
    where
        T: FromStr,
        T::Err: std::error::Error + 'static,
    {
        debug!("Attempting to load environment variable");

        let value_str: String = env::var(key).map_err(|_| {
            warn!("Environment variable '{key}' not found");
            ConfigError::MissingOrInvalid(key.into())
        })?;

        let value: T = value_str.parse::<T>().map_err(|e: <T as FromStr>::Err| {
            warn!("Failed to parse environment variable '{key}'. Invalid format");
            ConfigError::StringParse(Box::new(e))
        })?;

        debug!("Environment variable '{key}' loaded successfully");
        Ok(value)
    }

    pub fn from_file<T, P>(path: P) -> Result<T, ConfigError>
    where
        T: DeserializeOwned,
        P: AsRef<Path>,
    {
        let p: &Path = path.as_ref();
        debug!("Attempting to read configuration file");

        let content: String = fs::read_to_string(p).inspect_err(|_| {
            warn!("Configuration file not found or unreadable: {p:?}");
        })?;

        let config: T = toml::from_str(&content).inspect_err(|_| {
            warn!("Invalid syntax in TOML configuration file: {p:?}");
        })?;

        debug!("Configuration file '{p:?}' loaded successfully");
        Ok(config)
    }
}
