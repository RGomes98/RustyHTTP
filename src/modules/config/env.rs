use crate::modules::utils::Logger;
use std::env;

pub struct Env;
impl Env {
    pub fn get_env_var_or_default(variable_name: &str, default_value: &str) -> String {
        match env::var(variable_name) {
            Ok(value) => value,
            Err(_) => {
                Logger::warn(&format!(
                    "Environment variable '{variable_name}' not found. Falling back to default value: '{default_value}'."
                ));
                default_value.to_string()
            }
        }
    }
}
