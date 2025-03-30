use crate::modules::http::{HttpMethod, HttpMethodError};

use std::{fmt, str::FromStr};

pub enum RequestError {
    MalformedRequest,
    InvalidHttpMethod(HttpMethodError),
}

impl From<HttpMethodError> for RequestError {
    fn from(err: HttpMethodError) -> Self {
        RequestError::InvalidHttpMethod(err)
    }
}

impl fmt::Display for RequestError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RequestError::MalformedRequest => {
                write!(
                    f,
                    "Malformed HTTP request line. Expected format: 'METHOD PATH HTTP/VERSION'"
                )
            }
            RequestError::InvalidHttpMethod(err) => write!(f, "{err}"),
        }
    }
}

pub struct Request {
    pub method: HttpMethod,
    pub path: String,
    pub http_version: String,
}

impl Request {
    pub fn new(http_request: &[String]) -> Result<Self, RequestError> {
        let request_line: Vec<&str> = http_request
            .first()
            .ok_or(RequestError::MalformedRequest)?
            .split_whitespace()
            .collect::<Vec<&str>>();

        let (method, path, http_version): (&&str, &&str, &&str) = (
            request_line.first().ok_or(RequestError::MalformedRequest)?,
            request_line.get(1).ok_or(RequestError::MalformedRequest)?,
            request_line.get(2).ok_or(RequestError::MalformedRequest)?,
        );

        Ok(Self {
            path: path.to_string(),
            method: HttpMethod::from_str(method)?,
            http_version: http_version.to_string(),
        })
    }
}
