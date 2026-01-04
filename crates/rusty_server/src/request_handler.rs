use std::io::Read;
use std::net::TcpStream;
use std::sync::Arc;

use super::error::ServerError;
use rusty_http::{HttpError, Request, Response};
use rusty_router::{Route, Router};
use tracing::{debug, error, trace, warn};

const BUFFER_SIZE: usize = 4096;

pub struct RequestHandler {
    pub router: Arc<Router>,
    pub stream: TcpStream,
}

impl RequestHandler {
    pub fn handle(&mut self) -> Result<(), ServerError> {
        let peer_addr: Option<std::net::SocketAddr> = self.stream.peer_addr().ok();
        debug!("Processing connection from: {peer_addr:?}");

        let raw_request: String = self.read_stream()?;
        let response: Response = Response::new(&mut self.stream);
        trace!("Raw request read ({} bytes)", raw_request.len());

        let request: Request = Request::new(&raw_request).inspect_err(|e: &HttpError| {
            warn!("Failed to parse request from {peer_addr:?}: {e}");
        })?;

        let route: &Route = self.router.get_route(request.path, &request.method).ok_or_else(|| {
            warn!("404 Not Found: {} {}", request.method, request.path);
            ServerError::Request(HttpError::ResponseStatus(404))
        })?;

        debug!("Route matched: '{}'", route.path);
        (route.handler)(request, response);

        debug!("Request finished successfully");
        Ok(())
    }

    fn read_stream(&mut self) -> Result<String, ServerError> {
        let mut buffer: [u8; 4096] = [0; BUFFER_SIZE];

        match self.stream.read(&mut buffer) {
            Ok(size) => {
                if size == 0 {
                    debug!("Stream closed by client (0 bytes read)");
                } else {
                    trace!("Read {size} bytes from stream");
                }

                Ok(String::from_utf8_lossy(&buffer[..size]).into())
            }
            Err(e) => {
                error!("Failed to read from stream: {e}");
                Err(ServerError::Io(e))
            }
        }
    }
}
