use std::{
    borrow::Cow,
    io::{Cursor, IoSlice, Write},
};

use super::{HttpError, HttpStatus};
use serde::Serialize;
use tokio::{io::AsyncWriteExt, net::TcpStream};

const BUFFER_SIZE: usize = 1024;

pub struct Response<'a> {
    status: HttpStatus,
    body: Option<Cow<'a, str>>,
    headers: Vec<(Cow<'a, str>, Cow<'a, str>)>,
}

impl<'a> Response<'a> {
    pub fn new(status: HttpStatus) -> Self {
        Self {
            status,
            body: None,
            headers: Vec::new(),
        }
    }

    pub fn body<T>(mut self, body: T) -> Self
    where
        T: Into<Cow<'a, str>>,
    {
        self.body.replace(body.into());
        self
    }

    pub fn header<T, K>(mut self, key: T, value: K) -> Self
    where
        T: Into<Cow<'a, str>>,
        K: Into<Cow<'a, str>>,
    {
        self.headers.push((key.into(), value.into()));
        self
    }

    pub fn text<T>(self, text: T) -> Self
    where
        T: Into<Cow<'a, str>>,
    {
        self.header("Content-Type", "text/plain").body(text)
    }

    pub fn json<T>(mut self, body: T) -> Self
    where
        T: Serialize,
    {
        match serde_json::to_string(&body) {
            Ok(v) => self.header("Content-Type", "application/json").body(v),
            Err(e) => {
                self.status = HttpStatus::InternalServerError;
                self.body.replace(format!("JSON Serialization Failed: {e}").into());
                self
            }
        }
    }

    fn write_head_to_buffer(&self, buffer: &mut [u8]) -> Result<usize, HttpError> {
        let mut cursor: Cursor<&mut [u8]> = Cursor::new(buffer);

        write!(cursor, "HTTP/1.1 {} {}\r\n", u16::from(self.status), self.status)
            .map_err(|_| HttpError::new(HttpStatus::InternalServerError, "Headers too long for buffer"))?;

        for (key, value) in &self.headers {
            write!(cursor, "{key}: {value}\r\n")
                .map_err(|_| HttpError::new(HttpStatus::InternalServerError, "Headers too long for buffer"))?;
        }

        let content_length: usize = self.body.as_ref().map(|b: &Cow<str>| b.len()).unwrap_or(0);
        write!(cursor, "Content-Length: {content_length}\r\n\r\n")
            .map_err(|_| HttpError::new(HttpStatus::InternalServerError, "Headers too long for buffer"))?;

        let bytes_written: usize = usize::try_from(cursor.position())
            .map_err(|_| HttpError::new(HttpStatus::InternalServerError, "Header size calculation overflow"))?;

        Ok(bytes_written)
    }

    pub async fn send(&self, stream: &mut TcpStream) -> Result<(), HttpError> {
        let mut head_buffer: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];
        let head_length: usize = self.write_head_to_buffer(&mut head_buffer)?;
        let head_slice: &[u8] = &head_buffer[..head_length];

        if let Some(body) = &self.body {
            stream
                .write_vectored(&[IoSlice::new(head_slice), IoSlice::new(body.as_bytes())])
                .await
                .map_err(|_| HttpError::new(HttpStatus::InternalServerError, "Failed to write vectored response"))?;
        } else {
            stream
                .write_all(head_slice)
                .await
                .map_err(|_| HttpError::new(HttpStatus::InternalServerError, "Failed to write response headers"))?;
        }

        stream
            .flush()
            .await
            .map_err(|_| HttpError::new(HttpStatus::InternalServerError, "Failed to flush stream"))?;

        Ok(())
    }
}

pub trait IntoResponse<'a> {
    fn into_response(self) -> Response<'a>;
}

impl<'a> IntoResponse<'a> for Response<'a> {
    fn into_response(self) -> Response<'a> {
        self
    }
}

impl<'a> From<HttpError> for Response<'a> {
    fn from(e: HttpError) -> Self {
        Response::new(e.status).body(e.message)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_response_into_response() {
        let response: Response = Response::new(HttpStatus::Ok).text("TEXT");
        let result: Response = response.into_response();

        assert_eq!(result.status, HttpStatus::Ok);
        assert_eq!(result.body.unwrap(), "TEXT");
    }

    #[test]
    fn test_http_error_conversion_via_into() {
        let error: HttpError = HttpError::new(HttpStatus::NotFound, "NOT_FOUND");
        let response: Response = error.into();

        assert_eq!(response.status, HttpStatus::NotFound);
        assert_eq!(response.body.unwrap(), "NOT_FOUND");
    }

    #[test]
    fn test_json_response_success() {
        let user: serde_json::Value = serde_json::json!({ "name": "John Doe", "age": 18 });
        let response: Response = Response::new(HttpStatus::Ok).json(&user);

        assert_eq!(response.status, HttpStatus::Ok);
        assert_eq!(response.body.unwrap(), r#"{"age":18,"name":"John Doe"}"#);
    }

    #[test]
    fn test_handler_returning_only_response() {
        fn mock_success_handler() -> Response<'static> {
            Response::new(HttpStatus::Ok).text("SUCCESS")
        }

        fn mock_error_handler_converted() -> Response<'static> {
            HttpError::new(HttpStatus::Unauthorized, "UNAUTHORIZED").into()
        }

        let success: Response = mock_success_handler();
        assert_eq!(success.status, HttpStatus::Ok);
        assert_eq!(success.body.unwrap(), "SUCCESS");

        let error_response: Response = mock_error_handler_converted();
        assert_eq!(error_response.status, HttpStatus::Unauthorized);
        assert_eq!(error_response.body.unwrap(), "UNAUTHORIZED");
    }
}
