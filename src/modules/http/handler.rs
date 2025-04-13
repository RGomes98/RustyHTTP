use crate::modules::entry::{Config, ConfigError};
use crate::modules::http::{HttpStatus, Request, RequestError, Response};
use crate::modules::router::{Route, Router, RouterError};
use crate::modules::utils::Logger;

use std::fmt;
use std::io::{Error, Read};
use std::net::{AddrParseError, Ipv4Addr, SocketAddr, TcpListener, TcpStream};
use std::num::ParseIntError;
use std::str::FromStr;

const BUFFER_SIZE: usize = 4096;

pub enum HandlerError {
    Io(Error),
    Config(ConfigError),
    Router(RouterError),
    Request(RequestError),
}

impl From<Error> for HandlerError {
    fn from(err: Error) -> Self {
        HandlerError::Io(err)
    }
}

impl From<ConfigError> for HandlerError {
    fn from(err: ConfigError) -> Self {
        HandlerError::Config(err)
    }
}

impl From<RouterError> for HandlerError {
    fn from(err: RouterError) -> Self {
        HandlerError::Router(err)
    }
}

impl From<RequestError> for HandlerError {
    fn from(err: RequestError) -> Self {
        HandlerError::Request(err)
    }
}

impl fmt::Display for HandlerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let m: String = match self {
            HandlerError::Router(err) => format!("{err}"),
            HandlerError::Request(err) => format!("{err}"),
            HandlerError::Io(err) => format!("Stream I/O error: {err}."),
            HandlerError::Config(err) => format!("Failed to initialize handler. {err}."),
        };

        write!(f, "{m}")
    }
}

pub struct Handler {
    config: Config,
    listener: TcpListener,
}

impl Handler {
    pub fn new(config: Config) -> Result<Self, HandlerError> {
        let host: Ipv4Addr = Ipv4Addr::from_str(&config.host)
            .map_err(|err: AddrParseError| HandlerError::Config(ConfigError::Host(err)))?;

        let port: u16 = u16::from_str(&config.port)
            .map_err(|err: ParseIntError| HandlerError::Config(ConfigError::Port(err)))?;

        let address: SocketAddr = SocketAddr::from((host, port));
        let listener: TcpListener = TcpListener::bind(address)?;
        Ok(Self { config, listener })
    }

    pub fn socket_address(&self) -> String {
        let Handler { config, .. } = &self;
        format!("{}:{}", config.host, config.port)
    }

    pub fn listen(&self) {
        for stream in self.listener.incoming() {
            match stream {
                Ok(mut stream) => {
                    Self::process_stream(&mut stream);
                }
                Err(err) => {
                    Logger::error(&format!("Failed to accept incoming TCP connection: {err}"));
                }
            }
        }
    }

    fn process_stream(stream: &mut TcpStream) {
        if let Err(err) = Self::process_request(stream) {
            Logger::error(&err.to_string());
            Response::new(stream).send(HttpStatus::InternalServerError);
        }
    }

    fn process_request(stream: &mut TcpStream) -> Result<(), HandlerError> {
        let incoming_stream: String = Self::read_http_request_raw(stream)?;
        let http_request: Vec<&str> = Request::parse_http_request(&incoming_stream);
        let request: Request = Request::new(http_request)?;
        let response: Response<'_> = Response::new(stream);
        Self::dispatch_to_route(request, response)?;
        Ok(())
    }

    fn dispatch_to_route(request: Request<'_>, response: Response<'_>) -> Result<(), HandlerError> {
        let Request { request_line, .. } = &request;
        let identifier: String = Router::get_identifier(request_line.path, &request_line.method);
        let route: &Route = Router::get_route(identifier)?;
        (route.handler)(request, response);
        Ok(())
    }

    fn read_http_request_raw(stream: &mut TcpStream) -> Result<String, HandlerError> {
        let mut buffer: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];

        match stream.read(&mut buffer) {
            Ok(size) => Ok(String::from_utf8_lossy(&buffer[..size]).to_string()),
            Err(err) => Err(HandlerError::Io(err)),
        }
    }
}
