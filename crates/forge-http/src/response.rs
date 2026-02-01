use std::{
    borrow::Cow,
    io::{Cursor, Write},
};

use super::{HttpError, HttpStatus};
use tokio::{io::AsyncWriteExt, net::TcpStream};

const BUFFER_SIZE: usize = 8192;

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
        let mut buffer: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];
        let bytes_written: usize = self.write_head_to_buffer(&mut buffer)?;

        stream
            .write_all(&buffer[..bytes_written])
            .await
            .map_err(|_| HttpError::new(HttpStatus::InternalServerError, "Failed to write response headers"))?;

        if let Some(body) = &self.body {
            stream
                .write_all(body.as_bytes())
                .await
                .map_err(|_| HttpError::new(HttpStatus::InternalServerError, "Failed to write response body"))?;
        }

        stream
            .flush()
            .await
            .map_err(|_| HttpError::new(HttpStatus::InternalServerError, "Failed to flush stream"))?;

        Ok(())
    }
}
