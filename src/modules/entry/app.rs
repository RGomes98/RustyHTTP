use crate::modules::http::Handler;
use crate::modules::utils::Logger;

use std::net::Ipv4Addr;
use std::process;

pub struct Config {
    pub port: u16,
    pub host: Ipv4Addr,
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
