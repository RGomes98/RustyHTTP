use crate::modules::http::Handler;
use crate::modules::utils::Logger;

use std::process;

pub struct Config {
    pub port: String,
    pub host: String,
}

pub struct App;

impl App {
    pub fn new(config: Config) -> Self {
        match Handler::new(config) {
            Ok(server) => {
                Logger::info(&format!("Server is now listening on {}.", server.address()));
                server.handle_connection();
            }
            Err(err) => {
                Logger::error(&format!("Server startup failed. {err}"));
                process::exit(1)
            }
        }

        Self
    }
}
