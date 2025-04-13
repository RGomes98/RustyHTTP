use crate::modules::http::Handler;
use crate::modules::utils::Logger;

use std::fmt;
use std::net::AddrParseError;
use std::num::ParseIntError;
use std::process;

pub enum ConfigError {
    Port(ParseIntError),
    Host(AddrParseError),
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let m: String = match self {
            ConfigError::Host(err) => {
                format!("Failed to parse the 'HOST' environment variable: {err}")
            }
            ConfigError::Port(err) => {
                format!("Failed to parse the 'PORT' environment variable: {err}")
            }
        };

        write!(f, "{m}")
    }
}

pub struct Config {
    pub port: String,
    pub host: String,
}

pub struct App;

impl App {
    pub fn new(config: Config) -> Self {
        match Handler::new(config) {
            Ok(handler) => {
                let address: String = handler.socket_address();
                Logger::info(&format!("Server is now listening on {address}."));
                handler.listen();
            }
            Err(err) => {
                Logger::error(&format!("Server startup failed. {err}"));
                process::exit(1)
            }
        }

        Self
    }
}
