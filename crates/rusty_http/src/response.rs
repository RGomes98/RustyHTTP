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
        let response: String = format!("HTTP/1.1 {status_code} {status}\r\n\r\n");

        self.stream.write_all(response.as_bytes()).inspect_err(|e: &Error| {
            error!("Failed to write response bytes to stream: {e}");
        })?;

        self.stream.flush().inspect_err(|e: &Error| {
            error!("Failed to flush response stream: {e}");
        })?;

        Ok(())
    }
}
