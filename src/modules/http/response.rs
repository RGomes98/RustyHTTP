use crate::modules::http::HttpStatus;
use crate::modules::utils::Logger;

use std::fmt;
use std::io::{Error, Write};
use std::net::TcpStream;

pub enum ResponseError {
    Io(Error),
}

pub struct Response<'a> {
    stream: &'a mut TcpStream,
}

impl fmt::Display for ResponseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let m: String = match self {
            ResponseError::Io(err) => {
                format!("I/O error while writing HTTP response to client: {err}.")
            }
        };

        write!(f, "{m}")
    }
}

impl From<Error> for ResponseError {
    fn from(err: Error) -> Self {
        ResponseError::Io(err)
    }
}

impl<'a> Response<'a> {
    pub fn new(stream: &'a mut TcpStream) -> Self {
        Self { stream }
    }

    pub fn send(self, http_status: HttpStatus) {
        let status_code: u16 = http_status.into();
        let response: String = format!("HTTP/1.1 {status_code} {http_status}\r\n\r\n");

        if let Err(err) = self.stream.write_all(response.as_bytes()) {
            Logger::error(&format!("{}", ResponseError::Io(err)));
            let error_response: String = format!("{}", HttpStatus::InternalServerError);
            let _ = self.stream.write_all(error_response.as_bytes());
        }
    }
}
