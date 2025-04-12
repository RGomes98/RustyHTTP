use crate::modules::http::{HttpMethod, HttpMethodError, HttpStatusCode, HttpStatusCodeError};

use std::collections::HashMap;
use std::fmt;
use std::str::{FromStr, SplitWhitespace};

const HEADER_SPLIT_CHAR: char = ':';

pub enum RequestError {
    MalformedRequestLine(HttpStatusCodeError),
    MalformedHeaders(HttpStatusCodeError),
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

pub struct RequestLine<'a> {
    pub method: HttpMethod,
    pub path: &'a str,
    pub version: String,
}

pub struct Headers {
    pub host: String,
    pub connection: String,
    pub cache_control: String,
    pub user_agent: String,
}

pub struct Request<'a> {
    pub request_line: RequestLine<'a>,
    pub headers: Headers,
    // pub body: Option<String>, //TODO
    // pub query_params: HashMap<String, String>, //TODO
}

impl<'a> Request<'a> {
    pub fn new(request: Vec<&'a str>) -> Result<Self, RequestError> {
        let request_line: RequestLine<'a> = Self::parse_request_line(&request)?;
        let headers: Headers = Self::parse_headers(&request)?;

        Ok(Self {
            request_line,
            headers,
        })
    }

    fn parse_request_line(request: &Vec<&'a str>) -> Result<RequestLine<'a>, RequestError> {
        let mut request_line: SplitWhitespace<'a> = request
            .first()
            .ok_or(RequestError::MalformedRequestLine(
                HttpStatusCodeError::from_status(HttpStatusCode::BadRequest),
            ))?
            .split_whitespace();

        let (method, path, version): (&str, &str, &str) = (
            request_line
                .next()
                .ok_or(RequestError::MalformedRequestLine(
                    HttpStatusCodeError::from_status(HttpStatusCode::BadRequest),
                ))?,
            request_line
                .next()
                .ok_or(RequestError::MalformedRequestLine(
                    HttpStatusCodeError::from_status(HttpStatusCode::BadRequest),
                ))?,
            request_line
                .next()
                .ok_or(RequestError::MalformedRequestLine(
                    HttpStatusCodeError::from_status(HttpStatusCode::BadRequest),
                ))?,
        );

        Ok(RequestLine {
            path,
            version: version.to_string(),
            method: HttpMethod::from_str(method)?,
        })
    }

    fn parse_headers(request: &Vec<&'a str>) -> Result<Headers, RequestError> {
        let headers_map: HashMap<String, String> = request
            .iter()
            .skip(1)
            .map(|header: &&str| {
                let (key, value): (&str, &str) =
                    header
                        .split_once(HEADER_SPLIT_CHAR)
                        .ok_or(RequestError::MalformedHeaders(
                            HttpStatusCodeError::FromErrorStatus(HttpStatusCode::BadRequest),
                        ))?;
                Ok((key.trim().to_lowercase(), value.trim().to_lowercase()))
            })
            .collect::<Result<HashMap<String, String>, RequestError>>()?;

        // TODO:
        // Identify the cause of `[ERROR] - Malformed HTTP headers. Expected proper key-value pairs. Encountered unexpected HTTP error: [400] - Bad Request.`
        // in the first request.
        // Improve error handling "host is missing, connection...", make some headers an 'Option<T>'.
        // use 'match' instead??
        let (host, connection, cache_control, user_agent): (&String, &String, &String, &String) = (
            headers_map
                .get("host")
                .ok_or(RequestError::MalformedHeaders(
                    HttpStatusCodeError::FromErrorStatus(HttpStatusCode::BadRequest),
                ))?,
            headers_map
                .get("connection")
                .ok_or(RequestError::MalformedHeaders(
                    HttpStatusCodeError::FromErrorStatus(HttpStatusCode::BadRequest),
                ))?,
            headers_map
                .get("cache-control")
                .ok_or(RequestError::MalformedHeaders(
                    HttpStatusCodeError::FromErrorStatus(HttpStatusCode::BadRequest),
                ))?,
            headers_map
                .get("user-agent")
                .ok_or(RequestError::MalformedHeaders(
                    HttpStatusCodeError::FromErrorStatus(HttpStatusCode::BadRequest),
                ))?,
        );

        //Add custom headers???
        Ok(Headers {
            host: host.to_string(),
            connection: connection.to_string(),
            cache_control: cache_control.to_string(),
            user_agent: user_agent.to_string(),
        })
    }
}
