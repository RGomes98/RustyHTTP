use crate::modules::{
    http::{HttpMethod, HttpMethodParseError},
    router::{Route, Router, RouterError},
    server::Config,
    utils::Logger,
};

use std::{
    fmt,
    io::{Error, Read},
    net::{Ipv4Addr, SocketAddr, TcpListener, TcpStream},
    str::{FromStr, SplitWhitespace},
};

pub enum HttpServerError {
    ParseStreamError,
}

impl fmt::Display for HttpServerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                HttpServerError::ParseStreamError => "Failed to parse stream.",
            }
        )
    }
}

pub enum DispatchRequestError {
    IoError(Error),
    RouterError(RouterError),
    HttpMethodParseError(HttpMethodParseError),
    HttpServerError(HttpServerError),
}

impl fmt::Display for DispatchRequestError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DispatchRequestError::IoError(e) => write!(f, "Io error: {}", e),
            DispatchRequestError::RouterError(e) => write!(f, "Router error: {}", e),
            DispatchRequestError::HttpMethodParseError(e) => {
                write!(f, "Http method parse error: {}", e)
            }
            DispatchRequestError::HttpServerError(e) => {
                write!(f, "Dispatch request error: {}", e)
            }
        }
    }
}

impl From<Error> for DispatchRequestError {
    fn from(err: Error) -> Self {
        DispatchRequestError::IoError(err)
    }
}

impl From<RouterError> for DispatchRequestError {
    fn from(err: RouterError) -> Self {
        DispatchRequestError::RouterError(err)
    }
}

impl From<HttpMethodParseError> for DispatchRequestError {
    fn from(err: HttpMethodParseError) -> Self {
        DispatchRequestError::HttpMethodParseError(err)
    }
}

impl From<HttpServerError> for DispatchRequestError {
    fn from(err: HttpServerError) -> Self {
        DispatchRequestError::HttpServerError(err)
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

        let address: String = SocketAddr::from((host, port)).to_string();
        let listener: TcpListener = TcpListener::bind(&address)?;

        Ok(HttpServer { config, listener })
    }

    pub fn address(&self) -> String {
        format!("{}:{}", self.config.host, self.config.port)
    }

    pub fn handle_connection(&self) {
        for stream in self.listener.incoming() {
            match stream {
                Ok(mut stream) => Self::read_request(&mut stream),
                Err(err) => Logger::error(&format!("Error accepting incoming connection {}", err)),
            }
        }
    }

    fn read_request(stream: &mut TcpStream) {
        if let Err(err) = Self::dispatch_request(stream) {
            Logger::error(&format!("Failed to dispatch request: {err}"))
        }
    }

    fn dispatch_request(stream: &mut TcpStream) -> Result<(), DispatchRequestError> {
        let request: String = Self::parse_stream(stream)?;
        let http_request: Vec<String> = Self::parse_request(request);

        //TODO
        let mut split: SplitWhitespace<'_> = http_request[0].split_whitespace();
        let method: &str = split.next().unwrap_or("");
        let path: &str = split.next().unwrap_or("");
        //

        let http_method: HttpMethod = HttpMethod::from_str(method)?;
        let identifier: String = Router::get_route_identifier(&path.to_string(), &http_method);
        let route: &Route = Router::get_route_by_identifier(identifier)?;

        Ok(())
    }

    fn parse_stream(stream: &mut TcpStream) -> Result<String, HttpServerError> {
        let mut buffer: [u8; 4096] = [0; 4096];

        match stream.read(&mut buffer) {
            Ok(size) => Ok(String::from_utf8_lossy(&buffer[..size]).to_string()),
            Err(_) => Err(HttpServerError::ParseStreamError),
        }
    }

    fn parse_request(request: String) -> Vec<String> {
        request
            .lines()
            .take_while(|line| !line.trim().is_empty())
            .map(|line| line.to_string())
            .collect::<Vec<String>>()
    }
}
