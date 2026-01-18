use rusty_http::HttpError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ServerError {
    #[error(transparent)]
    Http(#[from] HttpError),

    #[error("Connection closed by peer")]
    ConnectionClosed,
}
