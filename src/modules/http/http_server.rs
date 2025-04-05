use crate::modules::http::{Request, RequestError};
use crate::modules::router::{Route, Router, RouterError};
use crate::modules::server::Config;
use crate::modules::utils::Logger;

use std::fmt;
use std::io::{Error, Read};
use std::net::{Ipv4Addr, SocketAddr, TcpListener, TcpStream};
use std::str::FromStr;

const BUFFER_SIZE: usize = 4096;

pub enum HttpServerError {
    Io(Error),
    Router(RouterError),
    Request(RequestError),
}

impl From<RouterError> for HttpServerError {
    fn from(err: RouterError) -> Self {
        HttpServerError::Router(err)
    }
}

impl From<RequestError> for HttpServerError {
    fn from(err: RequestError) -> Self {
        HttpServerError::Request(err)
    }
}

impl fmt::Display for HttpServerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let m: String = match self {
            HttpServerError::Io(err) => format!("Stream I/O error: {err}."),
            HttpServerError::Router(err) => format!("{err}"),
            HttpServerError::Request(err) => format!("{err}"),
        };

        write!(f, "{m}")
    }
}

pub struct HttpServer {
    config: Config,
    listener: TcpListener,
}

impl HttpServer {
    pub fn new(config: Config) -> Result<HttpServer, Error> {
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

        let address: SocketAddr = SocketAddr::from((host, port));
        let listener: TcpListener = TcpListener::bind(address)?;

        Ok(Self { config, listener })
    }

    pub fn address(&self) -> String {
        format!("{}:{}", self.config.host, self.config.port)
    }

    pub fn handle_connection(&self) {
        for stream in self.listener.incoming() {
            match stream {
                Ok(mut stream) => Self::read_stream(&mut stream),
                Err(err) => Logger::error(&err.to_string()),
            }
        }
    }

    fn read_stream(stream: &mut TcpStream) {
        if let Err(err) = Self::dispatch_request(stream) {
            Logger::error(&err.to_string())
        }
    }

    fn dispatch_request(stream: &mut TcpStream) -> Result<(), HttpServerError> {
        let incoming_stream: String = Self::parse_stream(stream)?;
        let http_request: Vec<&str> = Self::parse_request(&incoming_stream);
        let request: Request = Request::new(http_request)?;

        let identifier: String = Router::get_route_identifier(request.path, &request.method);
        let route: &Route = Router::get_route_by_identifier(identifier)?;

        (route.handler)(request, None);
        Ok(())
    }

    fn parse_stream(stream: &mut TcpStream) -> Result<String, HttpServerError> {
        let mut buffer: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];

        match stream.read(&mut buffer) {
            Ok(size) => Ok(String::from_utf8_lossy(&buffer[..size]).to_string()),
            Err(err) => Err(HttpServerError::Io(err)),
        }
    }

    fn parse_request(request: &str) -> Vec<&str> {
        request
            .lines()
            .take_while(|line| !line.trim().is_empty())
            .collect::<Vec<&str>>()
    }
}
