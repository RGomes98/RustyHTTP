use std::net::TcpStream;
use std::str::Utf8Error;
use std::sync::Arc;
use std::{io::Read, net::SocketAddr};

use super::error::ServerError;
use rusty_http::{HttpError, Request};
use rusty_router::{Handler, Router};
use rusty_utils::PathMatch;
use tracing::{debug, error, trace, warn};

const BUFFER_SIZE: usize = 4096;

pub struct RequestHandler {
    pub router: Arc<Router>,
    pub stream: TcpStream,
}

impl RequestHandler {
    pub fn handle(&mut self) -> Result<(), ServerError> {
        let peer_addr: Option<SocketAddr> = self.stream.peer_addr().ok();
        debug!("Processing connection from: {peer_addr:?}");

        let mut buffer: [u8; 4096] = [0; BUFFER_SIZE]; // TODO: Dynamic Buffer & Keep-Alive
        let bytes_read: usize = self.read_stream(&mut buffer)?;
        let raw_bytes: &[u8] = &buffer[..bytes_read];

        let raw_request: &str = str::from_utf8(raw_bytes).map_err(|e: Utf8Error| {
            warn!("Invalid UTF-8 sequence from {peer_addr:?}: {e}");
            ServerError::Request(HttpError::BadRequest(format!("Invalid UTF-8 sequence: {e}")))
        })?;

        let mut request: Request = Request::new(raw_request).inspect_err(|e: &HttpError| {
            warn!("Failed to parse request from {peer_addr:?}: {e}");
        })?;

        let route: PathMatch<Handler> = self.router.get_route(request.path, &request.method).ok_or_else(|| {
            warn!("404 Not Found: {} {}", request.method, request.path);
            ServerError::Request(HttpError::ResponseStatus(404))
        })?;

        request.set_params(route.params);
        (route.value)(request).write_to_stream(&mut self.stream)?;

        debug!("Request finished successfully");
        Ok(())
    }

    fn read_stream(&mut self, buffer: &mut [u8]) -> Result<usize, ServerError> {
        match self.stream.read(buffer) {
            Ok(size) => {
                if size == 0 {
                    debug!("Stream closed by client (0 bytes read)");
                } else {
                    trace!("Read {size} bytes from stream");
                }

                Ok(size)
            }
            Err(e) => {
                error!("Failed to read from stream: {e}");
                Err(ServerError::Io(e))
            }
        }
    }
}
