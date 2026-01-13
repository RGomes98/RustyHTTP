use std::io;

use crate::HttpStatus;
use thiserror::Error;

#[derive(Debug, Error)]
#[error("[{}] {}: {}", self.status as u16, status, message)]
pub struct RequestError {
    pub status: HttpStatus,
    pub message: String,
}

#[derive(Debug, Error)]
pub enum HttpError {
    #[error(transparent)]
    Status(#[from] RequestError),

    #[error("I/O Error: {0}")]
    Io(#[from] io::Error),
}

impl HttpError {
    pub fn new(status: HttpStatus, msg: impl Into<String>) -> Self {
        HttpError::Status(RequestError {
            status,
            message: msg.into(),
        })
    }

    pub fn status(&self) -> HttpStatus {
        match self {
            HttpError::Status(e) => e.status,
            HttpError::Io(..) => HttpStatus::InternalServerError,
        }
    }

    pub fn message(&self) -> String {
        match self {
            HttpError::Status(e) => e.message.clone(),
            HttpError::Io(..) => "Internal Server Error".into(),
        }
    }
}
