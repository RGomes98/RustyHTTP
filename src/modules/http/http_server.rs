use crate::modules::{
    http::{HttpRequestError, Request},
    router::{Route, Router, RouterError},
    server::Config,
    utils::Logger,
};

use std::{
    fmt,
    io::{Error, Read},
    net::{Ipv4Addr, SocketAddr, TcpListener, TcpStream},
    str::FromStr,
};

pub enum StreamReadError {
    ParseError(Error),
}

pub enum HttpServerError {
    Router(RouterError),
    StreamRead(StreamReadError),
    Request(HttpRequestError),
}

impl From<RouterError> for HttpServerError {
    fn from(err: RouterError) -> Self {
        HttpServerError::Router(err)
    }
}

impl From<StreamReadError> for HttpServerError {
    fn from(err: StreamReadError) -> Self {
        HttpServerError::StreamRead(err)
    }
}

impl From<HttpRequestError> for HttpServerError {
    fn from(err: HttpRequestError) -> Self {
        HttpServerError::Request(err)
    }
}

impl fmt::Display for StreamReadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Stream Read Error: {}",
            match self {
                StreamReadError::ParseError(err) => format!("Failed to parse stream. {err}"),
            }
        )
    }
}

impl fmt::Display for HttpServerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                HttpServerError::StreamRead(err) => format!("{err}"),
                HttpServerError::Router(err) => format!("{err}"),
                HttpServerError::Request(err) => format!("{err}"),
            }
        )
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
                Err(err) => Logger::error(&format!("Error accepting incoming stream. {err}")),
            }
        }
    }

    fn read_stream(stream: &mut TcpStream) {
        if let Err(err) = Self::dispatch_request(stream) {
            Logger::error(&format!("Failed to dispatch request. {err}"))
        }
    }

    fn dispatch_request(stream: &mut TcpStream) -> Result<(), HttpServerError> {
        let incoming_stream: String = Self::parse_stream(stream)?;
        let http_request: Vec<&str> = Self::parse_request(&incoming_stream);
        let request: Request = Request::new(http_request)?;

        let identifier: String = Router::get_route_identifier(&request.path, &request.method);
        let route: &Route = Router::get_route_by_identifier(identifier)?;

        (route.handler)(request, None);
        Ok(())
    }

    fn parse_stream(stream: &mut TcpStream) -> Result<String, StreamReadError> {
        let mut buffer: [u8; 4096] = [0; 4096];

        match stream.read(&mut buffer) {
            Ok(size) => Ok(String::from_utf8_lossy(&buffer[..size]).to_string()),
            Err(err) => Err(StreamReadError::ParseError(err)),
        }
    }

    fn parse_request(request: &str) -> Vec<&str> {
        request
            .lines()
            .take_while(|line| !line.trim().is_empty())
            .collect::<Vec<&str>>()
    }
}
