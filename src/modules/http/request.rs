use crate::modules::http::{HttpMethod, HttpMethodError, HttpStatusCode, HttpStatusCodeError};

use std::fmt;
use std::str::{FromStr, SplitWhitespace};

pub enum RequestError {
    Malformed(HttpStatusCodeError),
    InvalidMethod(HttpMethodError),
}

impl From<HttpStatusCodeError> for RequestError {
    fn from(err: HttpStatusCodeError) -> Self {
        RequestError::Malformed(err)
    }
}

impl From<HttpMethodError> for RequestError {
    fn from(err: HttpMethodError) -> Self {
        RequestError::InvalidMethod(err)
    }
}

impl fmt::Display for RequestError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let m: String = match self {
            RequestError::Malformed(err) => {
                format!("Malformed HTTP request line. Expected 'METHOD PATH HTTP/VERSION': {err}")
            }
            RequestError::InvalidMethod(err) => {
                format!("{err}")
            }
        };

        write!(f, "{m}")
    }
}

pub struct Request<'a> {
    pub method: HttpMethod,
    pub path: &'a str,
    pub http_version: String,
}

impl<'a> Request<'a> {
    pub fn new(http_request: Vec<&'a str>) -> Result<Self, RequestError> {
        let mut request_line: SplitWhitespace<'a> = http_request
            .first()
            .ok_or(HttpStatusCodeError::from_status(HttpStatusCode::BadRequest))?
            .split_whitespace();

        let (method, path, http_version): (&str, &str, &str) = (
            request_line
                .next()
                .ok_or(HttpStatusCodeError::from_status(HttpStatusCode::BadRequest))?,
            request_line
                .next()
                .ok_or(HttpStatusCodeError::from_status(HttpStatusCode::BadRequest))?,
            request_line
                .next()
                .ok_or(HttpStatusCodeError::from_status(HttpStatusCode::BadRequest))?,
        );

        Ok(Self {
            path,
            method: HttpMethod::from_str(method)?,
            http_version: http_version.to_string(),
        })
    }
}
