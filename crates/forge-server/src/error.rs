use forge_http::HttpError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ListenerError {
    #[error(transparent)]
    Http(#[from] HttpError),

    #[error("Connection closed by peer")]
    ConnectionClosed,
}
