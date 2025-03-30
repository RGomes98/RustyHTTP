use crate::modules::http::{HttpMethod, HttpMethodError};

use std::{fmt, str::FromStr};

pub enum ParseRequestError {
    MalformedRequest,
}

pub enum HttpRequestError {
    MalformedRequest(ParseRequestError),
    InvalidHttpMethod(HttpMethodError),
}

impl From<HttpMethodError> for HttpRequestError {
    fn from(err: HttpMethodError) -> Self {
        HttpRequestError::InvalidHttpMethod(err)
    }
}

impl From<ParseRequestError> for HttpRequestError {
    fn from(err: ParseRequestError) -> Self {
        HttpRequestError::MalformedRequest(err)
    }
}

impl fmt::Display for ParseRequestError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "HTTP Request Error: {}",
            match self {
                ParseRequestError::MalformedRequest =>
                    "Malformed HTTP request line. Expected format: 'METHOD PATH HTTP/VERSION'.",
            }
        )
    }
}

impl fmt::Display for HttpRequestError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                HttpRequestError::MalformedRequest(err) => format!("{err}"),
                HttpRequestError::InvalidHttpMethod(err) => format!("{err}"),
            }
        )
    }
}

pub struct Request {
    pub method: HttpMethod,
    pub path: String,
    pub http_version: String,
}

impl Request {
    pub fn new(http_request: Vec<&str>) -> Result<Self, HttpRequestError> {
        let request_line: Vec<&str> = http_request
            .first()
            .ok_or(ParseRequestError::MalformedRequest)?
            .split_whitespace()
            .collect::<Vec<&str>>();

        let (method, path, http_version): (&&str, &&str, &&str) = (
            request_line
                .first()
                .ok_or(ParseRequestError::MalformedRequest)?,
            request_line
                .get(1)
                .ok_or(ParseRequestError::MalformedRequest)?,
            request_line
                .get(2)
                .ok_or(ParseRequestError::MalformedRequest)?,
        );

        Ok(Self {
            path: path.to_string(),
            method: HttpMethod::from_str(method)?,
            http_version: http_version.to_string(),
        })
    }
}
