use crate::modules::http::{HttpStatusCode, HttpStatusCodeError};

use std::{fmt, str};

pub enum HttpMethodError {
    Invalid(HttpStatusCodeError),
}

impl fmt::Display for HttpMethodError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let m: String = match self {
            HttpMethodError::Invalid(err) => {
                format!("Unexpected HTTP method. {err}")
            }
        };

        write!(f, "{m}")
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
        let m: &str = match self {
            HttpMethod::GET => "GET",
            HttpMethod::POST => "POST",
            HttpMethod::PUT => "PUT",
            HttpMethod::DELETE => "DELETE",
            HttpMethod::PATCH => "PATCH",
            HttpMethod::HEAD => "HEAD",
            HttpMethod::OPTIONS => "OPTIONS",
            HttpMethod::TRACE => "TRACE",
        };

        write!(f, "{m}")
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
            _ => Err(HttpMethodError::Invalid(HttpStatusCodeError::from_status(
                HttpStatusCode::NotImplemented,
            ))),
        }
    }
}
