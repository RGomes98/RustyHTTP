use crate::modules::{http::HttpServer, utils::Logger};

use std::process;

pub struct Config {
    pub port: String,
    pub host: String,
}

pub struct Server;

impl Server {
    pub fn new(config: Config) -> Self {
        match HttpServer::new(config) {
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
