use crate::modules::http::{HttpMethod, HttpMethodError, HttpStatus, HttpStatusError};

use std::borrow::Cow;
use std::collections::HashMap;
use std::fmt;
use std::io::{Error, Read};
use std::net::TcpStream;
use std::str::{FromStr, SplitWhitespace};

const HEADER_SPLIT_CHAR: char = ':';
const BUFFER_SIZE: usize = 4096;

pub enum RequestError {
    MalformedRequestLine(HttpStatusError),
    MalformedHeaders(HttpStatusError),
    InvalidMethod(HttpMethodError),
    Io(Error),
}

impl From<HttpMethodError> for RequestError {
    fn from(err: HttpMethodError) -> Self {
        RequestError::InvalidMethod(err)
    }
}

impl fmt::Display for RequestError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let m: String = match self {
            RequestError::MalformedRequestLine(err) => {
                format!("Malformed HTTP request line. Expected 'METHOD PATH HTTP/VERSION'. {err}")
            }
            RequestError::MalformedHeaders(err) => {
                format!("Malformed HTTP headers. Expected proper key-value pairs. {err}")
            }
            RequestError::InvalidMethod(err) => {
                format!("{err}")
            }
            RequestError::Io(err) => {
                format!("Stream I/O error: {err}.")
            }
        };

        write!(f, "{m}")
    }
}

pub struct Request<'a> {
    pub request_line: RequestLine<'a>,
    pub headers: HashMap<String, Cow<'a, str>>,
}

pub struct RequestLine<'a> {
    pub path: &'a str,
    pub version: String,
    pub method: HttpMethod,
}

impl<'a> Request<'a> {
    pub fn new(request: &[&'a str]) -> Result<Self, RequestError> {
        let (request_line_raw, header_lines_raw): (&&str, &[&str]) =
            request
                .split_first()
                .ok_or(RequestError::MalformedRequestLine(
                    HttpStatusError::from_status(HttpStatus::BadRequest),
                ))?;

        let request_line: RequestLine<'a> = Self::parse_request_line(request_line_raw)?;
        let headers: HashMap<String, Cow<'a, str>> = Self::parse_headers(header_lines_raw)?;

        Ok(Self {
            request_line,
            headers,
        })
    }

    pub fn read_http_request_raw(stream: &mut TcpStream) -> Result<String, RequestError> {
        let mut buffer: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];

        match stream.read(&mut buffer) {
            Ok(size) => Ok(String::from_utf8_lossy(&buffer[..size]).to_string()),
            Err(err) => Err(RequestError::Io(err)),
        }
    }

    pub fn parse_http_request(request: &str) -> Vec<&str> {
        request
            .lines()
            .take_while(|line: &&str| !line.trim().is_empty())
            .collect::<Vec<&str>>()
    }

    fn parse_request_line(line: &'a str) -> Result<RequestLine<'a>, RequestError> {
        let mut parts: SplitWhitespace<'a> = line.split_whitespace();

        match (parts.next(), parts.next(), parts.next()) {
            (Some(method), Some(path), Some(version)) => Ok(RequestLine {
                path,
                version: version.to_string(),
                method: HttpMethod::from_str(method)?,
            }),
            _ => Err(RequestError::MalformedRequestLine(
                HttpStatusError::from_status(HttpStatus::BadRequest),
            )),
        }
    }

    fn parse_headers(headers: &[&'a str]) -> Result<HashMap<String, Cow<'a, str>>, RequestError> {
        headers
            .iter()
            .map(|header: &&str| {
                let (key, value): (&str, &str) =
                    header
                        .split_once(HEADER_SPLIT_CHAR)
                        .ok_or(RequestError::MalformedHeaders(
                            HttpStatusError::FromErrorStatus(HttpStatus::BadRequest),
                        ))?;
                Ok((key.trim().to_ascii_lowercase(), Cow::Borrowed(value.trim())))
            })
            .collect::<Result<HashMap<String, Cow<'a, str>>, RequestError>>()
    }
}
