use crate::modules::http::{HttpMethod, HttpMethodError, HttpStatus, HttpStatusError};

use std::collections::HashMap;
use std::fmt;
use std::str::{FromStr, SplitWhitespace};

const HEADER_SPLIT_CHAR: char = ':';

pub enum RequestError {
    MalformedRequestLine(HttpStatusError),
    MalformedHeaders(HttpStatusError),
    InvalidMethod(HttpMethodError),
}

impl From<HttpMethodError> for RequestError {
    fn from(err: HttpMethodError) -> Self {
        RequestError::InvalidMethod(err)
    }
}

impl fmt::Display for RequestError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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
        };

        write!(f, "{m}")
    }
}

pub struct Request<'a> {
    pub request_line: RequestLine<'a>,
    pub headers: HashMap<String, String>,
}

pub struct RequestLine<'a> {
    pub path: &'a str,
    pub version: String,
    pub method: HttpMethod,
}

impl<'a> Request<'a> {
    pub fn new(request: Vec<&'a str>) -> Result<Self, RequestError> {
        let request_line: RequestLine<'a> = Self::parse_request_line(&request)?;
        let headers: HashMap<String, String> = Self::parse_headers(&request)?;

        Ok(Self {
            request_line,
            headers,
        })
    }

    pub fn parse_http_request(request: &str) -> Vec<&str> {
        request
            .lines()
            .take_while(|line: &&str| !line.trim().is_empty())
            .collect::<Vec<&str>>()
    }

    fn parse_request_line(request: &Vec<&'a str>) -> Result<RequestLine<'a>, RequestError> {
        let mut request_line: SplitWhitespace<'a> = request
            .first()
            .ok_or(RequestError::MalformedRequestLine(
                HttpStatusError::from_status(HttpStatus::BadRequest),
            ))?
            .split_whitespace();

        let (method, path, version): (&str, &str, &str) = (
            request_line
                .next()
                .ok_or(RequestError::MalformedRequestLine(
                    HttpStatusError::from_status(HttpStatus::BadRequest),
                ))?,
            request_line
                .next()
                .ok_or(RequestError::MalformedRequestLine(
                    HttpStatusError::from_status(HttpStatus::BadRequest),
                ))?,
            request_line
                .next()
                .ok_or(RequestError::MalformedRequestLine(
                    HttpStatusError::from_status(HttpStatus::BadRequest),
                ))?,
        );

        Ok(RequestLine {
            path,
            version: version.to_string(),
            method: HttpMethod::from_str(method)?,
        })
    }

    fn parse_headers(request: &Vec<&'a str>) -> Result<HashMap<String, String>, RequestError> {
        let headers: HashMap<String, String> = request
            .iter()
            .skip(1)
            .map(|header: &&str| {
                let (key, value): (&str, &str) =
                    header
                        .split_once(HEADER_SPLIT_CHAR)
                        .ok_or(RequestError::MalformedHeaders(
                            HttpStatusError::FromErrorStatus(HttpStatus::BadRequest),
                        ))?;
                Ok((key.trim().to_lowercase(), value.trim().to_lowercase()))
            })
            .collect::<Result<HashMap<String, String>, RequestError>>()?;

        Ok(headers)
    }
}
