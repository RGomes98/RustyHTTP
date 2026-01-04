use std::io;

use rusty_http::HttpError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ServerError {
    #[error("HTTP Request error {0}")]
    Request(#[from] HttpError),

    #[error("I/O Error: {0}")]
    Io(#[from] io::Error),
}
