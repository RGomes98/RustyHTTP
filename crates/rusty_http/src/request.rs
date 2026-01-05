use std::borrow::Cow;
use std::collections::HashMap;
use std::str::{FromStr, Lines, SplitWhitespace};

use super::HttpError;
use super::HttpMethod;

use tracing::{debug, trace, warn};

type RequestLine<'a> = (&'a str, &'a str, HttpMethod);
pub type Headers<'a> = HashMap<Cow<'a, str>, Cow<'a, str>>;

const HEADERS_SEPARATOR: char = ':';

#[derive(Debug)]
pub struct Request<'a> {
    pub method: HttpMethod,
    pub path: &'a str,
    pub version: &'a str,
    pub headers: Headers<'a>,
}

impl<'a> Request<'a> {
    pub fn new(raw_request: &'a str) -> Result<Self, HttpError> {
        trace!("Starting request parsing");
        let mut lines: Lines = raw_request.lines();

        let request_lines: &str = lines.next().ok_or_else(|| {
            warn!("Received empty request");
            HttpError::MalformedRequestLine("Empty request".into())
        })?;

        let (path, version, method): RequestLine = Self::parse_request_line(request_lines)?;
        debug!("Parsed request line: {method} {path} {version}");

        let headers: Headers = Self::parse_headers(lines)?;
        trace!("Parsed {} headers", headers.len());

        Ok(Self {
            headers,
            path,
            version,
            method,
        })
    }

    fn parse_headers(raw_headers: Lines) -> Result<Headers, HttpError> {
        raw_headers
            .take_while(|line: &&str| !line.trim().is_empty())
            .map(|header: &str| {
                let values: (&str, &str) = header.split_once(HEADERS_SEPARATOR).ok_or_else(|| {
                    warn!("Malformed header found: '{header}'");
                    HttpError::MalformedHeaders(header.into())
                })?;

                let key: &str = values.0.trim();
                let value: &str = values.1.trim();

                let key_cow: Cow<str> = if key.as_bytes().iter().any(|byte: &u8| byte.is_ascii_uppercase()) {
                    Cow::Owned(key.to_ascii_lowercase())
                } else {
                    Cow::Borrowed(key)
                };

                Ok((key_cow, Cow::Borrowed(value)))
            })
            .collect::<Result<Headers, HttpError>>()
    }

    fn parse_request_line(raw_request_line: &str) -> Result<RequestLine<'_>, HttpError> {
        let mut parts: SplitWhitespace = raw_request_line.split_whitespace();

        let method_str: &str = parts.next().ok_or_else(|| {
            warn!("Missing HTTP Method in request line");
            HttpError::MalformedRequestLine("Missing 'METHOD'".into())
        })?;

        let path: &str = parts.next().ok_or_else(|| {
            warn!("Missing URI Path in request line");
            HttpError::MalformedRequestLine("Missing 'PATH'".into())
        })?;

        let version: &str = parts.next().ok_or_else(|| {
            warn!("Missing HTTP Version in request line");
            HttpError::MalformedRequestLine("Missing 'VERSION'".into())
        })?;

        let method: HttpMethod = HttpMethod::from_str(method_str).inspect_err(|_| {
            warn!("Invalid HTTP Method: '{method_str}'");
        })?;

        Ok((path, version, method))
    }
}
