use crate::modules::utils::Logger;

use std::{
    io::Error,
    net::{Ipv4Addr, SocketAddr, TcpListener},
    str::FromStr,
};

pub struct HttpServer {
    pub address: String,
    listener: TcpListener,
}

impl HttpServer {
    pub fn new(port: String, host: String) -> Result<HttpServer, Error> {
        let host: Ipv4Addr = Ipv4Addr::from_str(&host).map_err(|_| {
            Error::new(
                std::io::ErrorKind::InvalidInput,
                "Failed to parse the 'HOST' environment variable.",
            )
        })?;

        let port: u16 = str::parse::<u16>(&port).map_err(|_| {
            Error::new(
                std::io::ErrorKind::InvalidInput,
                "Failed to parse the 'PORT' environment variable.",
            )
        })?;

        let address: String = SocketAddr::from((host, port)).to_string();
        let listener: TcpListener = TcpListener::bind(&address)?;
        Ok(HttpServer { address, listener })
    }

    pub fn handle_connection(&self) {
        for stream in self.listener.incoming() {
            match stream {
                Ok(_stream) => {
                    Logger::info("Connection established!");
                }
                Err(error) => {
                    Logger::error(&format!("Failed to accept connection: {error}"));
                }
            }
        }
    }
}
