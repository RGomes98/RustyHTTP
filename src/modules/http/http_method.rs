use std::{fmt, str};

pub enum HttpMethodError {
    InvalidHttpMethod,
}

impl fmt::Display for HttpMethodError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "HTTP Error: {}",
            match self {
                HttpMethodError::InvalidHttpMethod => "Invalid HTTP method.",
            }
        )
    }
}

#[derive(Debug, PartialEq)]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
    PATCH,
    HEAD,
    OPTIONS,
    TRACE,
}

impl fmt::Display for HttpMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                HttpMethod::GET => "GET",
                HttpMethod::POST => "POST",
                HttpMethod::PUT => "PUT",
                HttpMethod::DELETE => "DELETE",
                HttpMethod::PATCH => "PATCH",
                HttpMethod::HEAD => "HEAD",
                HttpMethod::OPTIONS => "OPTIONS",
                HttpMethod::TRACE => "TRACE",
            }
        )
    }
}

impl str::FromStr for HttpMethod {
    type Err = HttpMethodError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "GET" => Ok(HttpMethod::GET),
            "POST" => Ok(HttpMethod::POST),
            "PUT" => Ok(HttpMethod::PUT),
            "DELETE" => Ok(HttpMethod::DELETE),
            "PATCH" => Ok(HttpMethod::PATCH),
            "HEAD" => Ok(HttpMethod::HEAD),
            "OPTIONS" => Ok(HttpMethod::OPTIONS),
            "TRACE" => Ok(HttpMethod::TRACE),
            _ => Err(HttpMethodError::InvalidHttpMethod),
        }
    }
}
