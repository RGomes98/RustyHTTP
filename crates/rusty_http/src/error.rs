use std::io;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum HttpError {
    #[error("Malformed Request Line: '{0}'. Expected format: 'METHOD PATH HTTP/VERSION'")]
    MalformedRequestLine(String),

    #[error("Malformed Header: '{0}'. Expected format: 'Header-Key: Value'")]
    MalformedHeaders(String),

    #[error("Invalid Method: '{0}'. Supported methods: GET, POST, PUT, DELETE, PATCH...")]
    InvalidMethod(String),

    #[error("Unknown Status Code: {0}. Expected integer between 100 and 599")]
    UnknownStatusCode(u16),

    #[error("Early response triggered with status code: {0}")]
    ResponseStatus(u16),

    #[error("I/O Error: {0}")]
    Io(#[from] io::Error),
}
