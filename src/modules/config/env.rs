use crate::modules::utils::Logger;
use std::env;

pub struct Env;
impl Env {
    fn get_env_var(var: &str) -> Result<String, String> {
        match env::var(var) {
            Ok(value) if !value.trim().is_empty() => Ok(value),
            Ok(_) => Err(format!("Invalid format for environment variable '{var}'.")),
            Err(_) => Err(format!("Environment variable '{var}' not found.")),
        }
    }

    pub fn get_env_var_or_exit(variable_name: &str) -> String {
        Self::get_env_var(variable_name).unwrap_or_else(|error| {
            Logger::error(&format!("Failed to retrieve environment variable: {error}"));
            std::process::exit(1)
        })
    }
}
