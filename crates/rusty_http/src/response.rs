use std::io::Write;
use std::net::TcpStream;

use super::{HttpError, HttpStatus};

use tracing::debug;

pub struct Response {
    status: HttpStatus,
}

impl Response {
    pub fn new(status: HttpStatus) -> Self {
        Self { status }
    }

    pub fn write_to_stream(self, stream: &mut TcpStream) -> Result<(), HttpError> {
        let status_code: u16 = self.status.into();
        debug!("Sending HTTP response: {} {}", status_code, self.status);

        stream
            .write_all(format!("HTTP/1.1 {} {}\r\n\r\n", status_code, self.status).as_bytes())
            .map_err(|_| HttpError::new(HttpStatus::InternalServerError, "Failed to write response headers"))?;

        stream
            .flush()
            .map_err(|_| HttpError::new(HttpStatus::InternalServerError, "Failed to flush stream"))?;

        Ok(())
    }
}
