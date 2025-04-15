use crate::modules::utils::Logger;

use std::env;
use std::str::FromStr;

pub struct Env;

impl Env {
    pub fn get_parsed<T>(variable_name: &str) -> T
    where
        T: FromStr,
        T::Err: std::fmt::Display,
    {
        let raw_str: String = Self::get(variable_name).unwrap_or_else(|err: String| {
            Logger::error(&format!("Failed to retrieve environment variable: {err}"));
            std::process::exit(1)
        });

        raw_str
            .parse::<T>()
            .unwrap_or_else(|err: <T as FromStr>::Err| {
                Logger::error(&format!(
                    "Failed to parse environment variable '{variable_name}': {err}."
                ));
                std::process::exit(1)
            })
    }

    fn get(var: &str) -> Result<String, String> {
        match env::var(var) {
            Ok(value) if !value.trim().is_empty() => Ok(value),
            Ok(_) => Err(format!("Invalid format for environment variable '{var}'.")),
            Err(_) => Err(format!("Environment variable '{var}' not found.")),
        }
    }
}
