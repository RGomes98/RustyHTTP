use crate::modules::http::{HttpMethod, HttpMethodError};

use std::{
    fmt,
    str::{FromStr, SplitWhitespace},
};

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

pub struct Request<'a> {
    pub method: HttpMethod,
    pub path: &'a str,
    pub http_version: String,
}

impl<'a> Request<'a> {
    pub fn new(http_request: Vec<&'a str>) -> Result<Self, HttpRequestError> {
        let mut request_line: SplitWhitespace<'a> = http_request
            .first()
            .ok_or(ParseRequestError::MalformedRequest)?
            .split_whitespace();

        let (method, path, http_version): (&str, &str, &str) = (
            request_line
                .next()
                .ok_or(ParseRequestError::MalformedRequest)?,
            request_line
                .next()
                .ok_or(ParseRequestError::MalformedRequest)?,
            request_line
                .next()
                .ok_or(ParseRequestError::MalformedRequest)?,
        );

        Ok(Self {
            path,
            method: HttpMethod::from_str(method)?,
            http_version: http_version.to_string(),
        })
    }
}
