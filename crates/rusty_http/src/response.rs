use std::io::{Error, Write};
use std::net::TcpStream;

use super::{HttpError, HttpStatus};

use tracing::{debug, error};

pub struct Response<'a> {
    stream: &'a mut TcpStream,
}

impl<'a> Response<'a> {
    pub fn new(stream: &'a mut TcpStream) -> Self {
        Self { stream }
    }

    pub fn send(self, status: HttpStatus) -> Result<(), HttpError> {
        let status_code: u16 = status.into();

        debug!("Sending HTTP response: {status_code} {status}");
        let response: String = format!("HTTP/1.1 {} {}\r\n\r\n", status_code, status);

        self.stream.write_all(response.as_bytes()).map_err(|e: Error| {
            error!("Failed to write response bytes to stream: {e}");
            HttpError::from(e)
        })?;

        self.stream.flush().map_err(|e: Error| {
            error!("Failed to flush response stream: {e}");
            HttpError::from(e)
        })?;

        Ok(())
    }
}
