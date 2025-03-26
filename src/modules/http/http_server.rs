use crate::modules::{http::HttpMethod, router::Router, utils::Logger};

use std::{
    io::{BufRead, BufReader, Error},
    net::{Ipv4Addr, SocketAddr, TcpListener, TcpStream},
    str::FromStr,
};

pub struct Config {
    pub port: String,
    pub host: String,
}

pub struct HttpServer {
    config: Config,
    router: Router,
    listener: TcpListener,
}

impl HttpServer {
    pub fn new(router: Router, config: Config) -> Result<HttpServer, Error> {
        let host: Ipv4Addr = Ipv4Addr::from_str(&config.host).map_err(|_| {
            Error::new(
                std::io::ErrorKind::InvalidInput,
                "Failed to parse the 'HOST' environment variable.",
            )
        })?;

        let port: u16 = str::parse::<u16>(&config.port).map_err(|_| {
            Error::new(
                std::io::ErrorKind::InvalidInput,
                "Failed to parse the 'PORT' environment variable.",
            )
        })?;

        let address: String = SocketAddr::from((host, port)).to_string();
        let listener: TcpListener = TcpListener::bind(&address)?;

        Ok(HttpServer {
            config,
            router,
            listener,
        })
    }

    pub fn get_address(&self) -> String {
        format!("{}:{}", self.config.host, self.config.port)
    }

    pub fn handle_connection(&self) {
        for stream in self.listener.incoming() {
            match stream {
                Ok(stream) => self.read_stream(stream),
                Err(error) => Logger::error(&format!("Failed to accept connection: {error}")),
            }
        }
    }

    //TODO refactor
    fn parse_stream(buf_reader: BufReader<&TcpStream>) -> Result<Vec<String>, Error> {
        let raw_http_request: Vec<String> = buf_reader
            .lines()
            .take_while(|result| match result {
                Ok(line) => !line.trim().is_empty(),
                Err(_) => true,
            })
            .collect::<Result<Vec<String>, Error>>()?;
        Ok(raw_http_request)
    }

    //TODO refactor
    fn read_stream(&self, stream: TcpStream) {
        let buf_reader: BufReader<&TcpStream> = BufReader::new(&stream);
        let result: Result<Vec<String>, Error> = Self::parse_stream(buf_reader);

        if let Ok(http_request) = result {
            println!("Request: {http_request:#?}");
        } else if let Err(error) = result {
            Logger::error(&format!(
                "Failed to parse HTTP request from TCP stream: {error}."
            ));
        }

        //test
        match self.router.invoke_route(String::from("/"), HttpMethod::GET) {
            Ok(_) => {
                println!("Success!");
            }
            Err(_) => {
                println!("Something went wrong!");
            }
        }
    }
}
