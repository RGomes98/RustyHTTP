use crate::modules::entry::Config;
use crate::modules::http::{HttpStatus, Request, RequestError, Response};
use crate::modules::router::{Route, Router, RouterError};
use crate::modules::utils::Logger;
use crate::modules::utils::ThreadPool;

use std::fmt;
use std::io::{Error, Read};
use std::net::{SocketAddr, TcpListener, TcpStream};

const BUFFER_SIZE: usize = 4096;

pub enum HandlerError {
    Io(Error),
    Router(RouterError),
    Request(RequestError),
}

impl From<Error> for HandlerError {
    fn from(err: Error) -> Self {
        HandlerError::Io(err)
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
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let m: String = match self {
            HandlerError::Router(err) => format!("{err}"),
            HandlerError::Request(err) => format!("{err}"),
            HandlerError::Io(err) => format!("Stream I/O error: {err}."),
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
        let address: SocketAddr = SocketAddr::from((config.host, config.port));
        let listener: TcpListener = TcpListener::bind(address)?;
        Ok(Self { config, listener })
    }

    pub fn socket_address(&self) -> String {
        let Handler { config, .. } = &self;
        format!("{}:{}", config.host, config.port)
    }

    pub fn listen(&self) {
        let pool: ThreadPool = ThreadPool::new(self.config.pool_size);

        for stream in self.listener.incoming() {
            match stream {
                Ok(mut stream) => {
                    pool.schedule(move || Self::process_stream(&mut stream));
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
        let raw_request: String = Self::read_http_request_raw(stream)?;
        let request_lines: Vec<&str> = Request::parse_http_request(&raw_request);
        let request: Request = Request::new(request_lines)?;
        let response: Response = Response::new(stream);
        Self::dispatch_to_route(request, response)?;
        Ok(())
    }

    fn dispatch_to_route(request: Request, response: Response) -> Result<(), HandlerError> {
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
