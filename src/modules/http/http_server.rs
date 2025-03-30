use crate::modules::{
    http::{HttpMethodError, Request, RequestError},
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
    Io(Error),
    Router(RouterError),
    HttpMethod(HttpMethodError),
    StreamRead(StreamReadError),
    Request(RequestError),
}

impl fmt::Display for StreamReadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StreamReadError::ParseError(err) => write!(f, "{err}"),
        }
    }
}

impl fmt::Display for HttpServerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HttpServerError::Request(err) => write!(f, "{err}"),
            HttpServerError::Io(err) => write!(f, "{err}"),
            HttpServerError::StreamRead(err) => write!(f, "{err}"),
            HttpServerError::Router(err) => write!(f, "{err}"),
            HttpServerError::HttpMethod(err) => write!(f, "{err}"),
        }
    }
}

impl From<Error> for StreamReadError {
    fn from(err: Error) -> Self {
        StreamReadError::ParseError(err)
    }
}

impl From<Error> for HttpServerError {
    fn from(err: Error) -> Self {
        HttpServerError::Io(err)
    }
}

impl From<RouterError> for HttpServerError {
    fn from(err: RouterError) -> Self {
        HttpServerError::Router(err)
    }
}

impl From<HttpMethodError> for HttpServerError {
    fn from(err: HttpMethodError) -> Self {
        HttpServerError::HttpMethod(err)
    }
}

impl From<StreamReadError> for HttpServerError {
    fn from(err: StreamReadError) -> Self {
        HttpServerError::StreamRead(err)
    }
}

impl From<RequestError> for HttpServerError {
    fn from(err: RequestError) -> Self {
        HttpServerError::Request(err)
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
                Ok(mut stream) => Self::read_stream(&mut stream),
                Err(err) => Logger::error(&format!("Error accepting incoming stream: {err}")),
            }
        }
    }

    fn read_stream(stream: &mut TcpStream) {
        if let Err(err) = Self::dispatch_request(stream) {
            Logger::error(&format!("Failed to dispatch request: {err}"))
        }
    }

    fn dispatch_request(stream: &mut TcpStream) -> Result<(), HttpServerError> {
        let incoming_stream: String = Self::parse_stream(stream)?;
        let http_request: Vec<String> = Self::parse_request(incoming_stream);

        let request: Request = Request::new(&http_request)?;
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

    fn parse_request(request: String) -> Vec<String> {
        request
            .lines()
            .take_while(|line| !line.trim().is_empty())
            .map(|line| line.to_string())
            .collect::<Vec<String>>()
    }
}
