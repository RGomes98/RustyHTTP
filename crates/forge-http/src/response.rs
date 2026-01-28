use std::borrow::Cow;

use super::{HttpError, HttpStatus};
use serde::Serialize;
use tokio::{io::AsyncWriteExt, net::TcpStream};
use tracing::debug;

pub struct Response<'a> {
    status: HttpStatus,
    body_content: Option<Cow<'a, str>>,
}

impl<'a> Response<'a> {
    pub fn new(status: HttpStatus) -> Self {
        Self {
            status,
            body_content: None,
        }
    }

    pub fn body<T>(mut self, body: T) -> Self
    where
        T: Into<Cow<'a, str>>,
    {
        self.body_content.replace(body.into());
        self
    }

    pub fn json<T>(mut self, body: T) -> Result<Self, HttpError>
    where
        T: Serialize,
    {
        let json: String = serde_json::to_string(&body)
            .map_err(|_| HttpError::new(HttpStatus::InternalServerError, "Failed to serialize response to JSON"))?;

        self.body_content.replace(json.into());
        Ok(self)
    }

    pub async fn send(self, stream: &mut TcpStream) -> Result<(), HttpError> {
        let body_content: Cow<str> = self.body_content.unwrap_or_default();
        let content_length: usize = body_content.len();
        let status_code: u16 = self.status.into();
        let status: HttpStatus = self.status;

        let response: String =
            format!("HTTP/1.1 {status_code} {status}\r\nContent-Length: {content_length}\r\n\r\n{body_content}");

        debug!("Sending HTTP response: {response:#?}",);

        stream
            .write_all(response.as_bytes())
            .await
            .map_err(|_| HttpError::new(HttpStatus::InternalServerError, "Failed to write response"))?;

        stream
            .flush()
            .await
            .map_err(|_| HttpError::new(HttpStatus::InternalServerError, "Failed to flush stream"))?;

        Ok(())
    }
}
