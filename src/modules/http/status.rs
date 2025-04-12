use std::fmt;

pub enum HttpStatusError {
    UnknownStatusCode(HttpStatus),
    NonErrorStatusCode(HttpStatus),
    FromErrorStatus(HttpStatus),
}

impl fmt::Display for HttpStatusError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (label, http_status): (&str, &HttpStatus) = match self {
            HttpStatusError::UnknownStatusCode(http_status) => {
                ("Unrecognized HTTP status code", http_status)
            }
            HttpStatusError::NonErrorStatusCode(http_status) => {
                ("Expected an error status (4xx or 5xx)", http_status)
            }
            HttpStatusError::FromErrorStatus(http_status) => {
                ("Encountered unexpected HTTP error", http_status)
            }
        };

        write!(f, "{label}: {}.", Self::describe_status(http_status))
    }
}

impl HttpStatusError {
    fn describe_status(http_status: &HttpStatus) -> String {
        let status_code: u16 = (*http_status).into();
        format!("[{status_code}] - {http_status}")
    }

    fn from_error_status(http_status: HttpStatus) -> Result<Self, HttpStatus> {
        if http_status.is_error() {
            Ok(Self::FromErrorStatus(http_status))
        } else {
            Err(http_status)
        }
    }

    pub fn from_status(http_status: HttpStatus) -> Self {
        Self::from_error_status(http_status)
            .unwrap_or(Self::NonErrorStatusCode(HttpStatus::InternalServerError))
    }
}

#[derive(Debug, Copy, Clone)]
pub enum HttpStatus {
    Continue = 100,
    SwitchingProtocols = 101,
    Processing = 102,
    EarlyHints = 103,
    Ok = 200,
    Created = 201,
    Accepted = 202,
    NonAuthoritativeInformation = 203,
    NoContent = 204,
    ResetContent = 205,
    PartialContent = 206,
    MultiStatus = 207,
    AlreadyReported = 208,
    ImUsed = 226,
    MultipleChoices = 300,
    MovedPermanently = 301,
    Found = 302,
    SeeOther = 303,
    NotModified = 304,
    UseProxy = 305,
    TemporaryRedirect = 307,
    PermanentRedirect = 308,
    BadRequest = 400,
    Unauthorized = 401,
    PaymentRequired = 402,
    Forbidden = 403,
    NotFound = 404,
    MethodNotAllowed = 405,
    NotAcceptable = 406,
    ProxyAuthenticationRequired = 407,
    RequestTimeout = 408,
    Conflict = 409,
    Gone = 410,
    LengthRequired = 411,
    PreconditionFailed = 412,
    PayloadTooLarge = 413,
    UriTooLong = 414,
    UnsupportedMediaType = 415,
    RangeNotSatisfiable = 416,
    ExpectationFailed = 417,
    ImATeapot = 418,
    MisdirectedRequest = 421,
    UnprocessableEntity = 422,
    Locked = 423,
    FailedDependency = 424,
    TooEarly = 425,
    UpgradeRequired = 426,
    PreconditionRequired = 428,
    TooManyRequests = 429,
    RequestHeaderFieldsTooLarge = 431,
    UnavailableForLegalReasons = 451,
    InternalServerError = 500,
    NotImplemented = 501,
    BadGateway = 502,
    ServiceUnavailable = 503,
    GatewayTimeout = 504,
    HttpVersionNotSupported = 505,
    VariantAlsoNegotiates = 506,
    InsufficientStorage = 507,
    LoopDetected = 508,
    NotExtended = 510,
    NetworkAuthenticationRequired = 511,
}

impl TryFrom<u16> for HttpStatus {
    type Error = HttpStatusError;

    fn try_from(u: u16) -> Result<Self, Self::Error> {
        match u {
            100 => Ok(Self::Continue),
            101 => Ok(Self::SwitchingProtocols),
            102 => Ok(Self::Processing),
            103 => Ok(Self::EarlyHints),
            200 => Ok(Self::Ok),
            201 => Ok(Self::Created),
            202 => Ok(Self::Accepted),
            203 => Ok(Self::NonAuthoritativeInformation),
            204 => Ok(Self::NoContent),
            205 => Ok(Self::ResetContent),
            206 => Ok(Self::PartialContent),
            207 => Ok(Self::MultiStatus),
            208 => Ok(Self::AlreadyReported),
            226 => Ok(Self::ImUsed),
            300 => Ok(Self::MultipleChoices),
            301 => Ok(Self::MovedPermanently),
            302 => Ok(Self::Found),
            303 => Ok(Self::SeeOther),
            304 => Ok(Self::NotModified),
            305 => Ok(Self::UseProxy),
            307 => Ok(Self::TemporaryRedirect),
            308 => Ok(Self::PermanentRedirect),
            400 => Ok(Self::BadRequest),
            401 => Ok(Self::Unauthorized),
            402 => Ok(Self::PaymentRequired),
            403 => Ok(Self::Forbidden),
            404 => Ok(Self::NotFound),
            405 => Ok(Self::MethodNotAllowed),
            406 => Ok(Self::NotAcceptable),
            407 => Ok(Self::ProxyAuthenticationRequired),
            408 => Ok(Self::RequestTimeout),
            409 => Ok(Self::Conflict),
            410 => Ok(Self::Gone),
            411 => Ok(Self::LengthRequired),
            412 => Ok(Self::PreconditionFailed),
            413 => Ok(Self::PayloadTooLarge),
            414 => Ok(Self::UriTooLong),
            415 => Ok(Self::UnsupportedMediaType),
            416 => Ok(Self::RangeNotSatisfiable),
            417 => Ok(Self::ExpectationFailed),
            418 => Ok(Self::ImATeapot),
            421 => Ok(Self::MisdirectedRequest),
            422 => Ok(Self::UnprocessableEntity),
            423 => Ok(Self::Locked),
            424 => Ok(Self::FailedDependency),
            425 => Ok(Self::TooEarly),
            426 => Ok(Self::UpgradeRequired),
            428 => Ok(Self::PreconditionRequired),
            429 => Ok(Self::TooManyRequests),
            431 => Ok(Self::RequestHeaderFieldsTooLarge),
            451 => Ok(Self::UnavailableForLegalReasons),
            500 => Ok(Self::InternalServerError),
            501 => Ok(Self::NotImplemented),
            502 => Ok(Self::BadGateway),
            503 => Ok(Self::ServiceUnavailable),
            504 => Ok(Self::GatewayTimeout),
            505 => Ok(Self::HttpVersionNotSupported),
            506 => Ok(Self::VariantAlsoNegotiates),
            507 => Ok(Self::InsufficientStorage),
            508 => Ok(Self::LoopDetected),
            510 => Ok(Self::NotExtended),
            511 => Ok(Self::NetworkAuthenticationRequired),
            _ => Err(HttpStatusError::UnknownStatusCode(
                HttpStatus::InternalServerError,
            )),
        }
    }
}

impl From<HttpStatus> for u16 {
    fn from(http_status: HttpStatus) -> Self {
        match http_status {
            HttpStatus::Continue => 100,
            HttpStatus::SwitchingProtocols => 101,
            HttpStatus::Processing => 102,
            HttpStatus::EarlyHints => 103,
            HttpStatus::Ok => 200,
            HttpStatus::Created => 201,
            HttpStatus::Accepted => 202,
            HttpStatus::NonAuthoritativeInformation => 203,
            HttpStatus::NoContent => 204,
            HttpStatus::ResetContent => 205,
            HttpStatus::PartialContent => 206,
            HttpStatus::MultiStatus => 207,
            HttpStatus::AlreadyReported => 208,
            HttpStatus::ImUsed => 226,
            HttpStatus::MultipleChoices => 300,
            HttpStatus::MovedPermanently => 301,
            HttpStatus::Found => 302,
            HttpStatus::SeeOther => 303,
            HttpStatus::NotModified => 304,
            HttpStatus::UseProxy => 305,
            HttpStatus::TemporaryRedirect => 307,
            HttpStatus::PermanentRedirect => 308,
            HttpStatus::BadRequest => 400,
            HttpStatus::Unauthorized => 401,
            HttpStatus::PaymentRequired => 402,
            HttpStatus::Forbidden => 403,
            HttpStatus::NotFound => 404,
            HttpStatus::MethodNotAllowed => 405,
            HttpStatus::NotAcceptable => 406,
            HttpStatus::ProxyAuthenticationRequired => 407,
            HttpStatus::RequestTimeout => 408,
            HttpStatus::Conflict => 409,
            HttpStatus::Gone => 410,
            HttpStatus::LengthRequired => 411,
            HttpStatus::PreconditionFailed => 412,
            HttpStatus::PayloadTooLarge => 413,
            HttpStatus::UriTooLong => 414,
            HttpStatus::UnsupportedMediaType => 415,
            HttpStatus::RangeNotSatisfiable => 416,
            HttpStatus::ExpectationFailed => 417,
            HttpStatus::ImATeapot => 418,
            HttpStatus::MisdirectedRequest => 421,
            HttpStatus::UnprocessableEntity => 422,
            HttpStatus::Locked => 423,
            HttpStatus::FailedDependency => 424,
            HttpStatus::TooEarly => 425,
            HttpStatus::UpgradeRequired => 426,
            HttpStatus::PreconditionRequired => 428,
            HttpStatus::TooManyRequests => 429,
            HttpStatus::RequestHeaderFieldsTooLarge => 431,
            HttpStatus::UnavailableForLegalReasons => 451,
            HttpStatus::InternalServerError => 500,
            HttpStatus::NotImplemented => 501,
            HttpStatus::BadGateway => 502,
            HttpStatus::ServiceUnavailable => 503,
            HttpStatus::GatewayTimeout => 504,
            HttpStatus::HttpVersionNotSupported => 505,
            HttpStatus::VariantAlsoNegotiates => 506,
            HttpStatus::InsufficientStorage => 507,
            HttpStatus::LoopDetected => 508,
            HttpStatus::NotExtended => 510,
            HttpStatus::NetworkAuthenticationRequired => 511,
        }
    }
}

impl fmt::Display for HttpStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let m: &str = match self {
            HttpStatus::Continue => "Continue",
            HttpStatus::SwitchingProtocols => "Switching Protocols",
            HttpStatus::Processing => "Processing",
            HttpStatus::EarlyHints => "Early Hints",
            HttpStatus::Ok => "OK",
            HttpStatus::Created => "Created",
            HttpStatus::Accepted => "Accepted",
            HttpStatus::NonAuthoritativeInformation => "Non-Authoritative Information",
            HttpStatus::NoContent => "No Content",
            HttpStatus::ResetContent => "Reset Content",
            HttpStatus::PartialContent => "Partial Content",
            HttpStatus::MultiStatus => "Multi-Status",
            HttpStatus::AlreadyReported => "Already Reported",
            HttpStatus::ImUsed => "IM Used",
            HttpStatus::MultipleChoices => "Multiple Choices",
            HttpStatus::MovedPermanently => "Moved Permanently",
            HttpStatus::Found => "Found",
            HttpStatus::SeeOther => "See Other",
            HttpStatus::NotModified => "Not Modified",
            HttpStatus::UseProxy => "Use Proxy",
            HttpStatus::TemporaryRedirect => "Temporary Redirect",
            HttpStatus::PermanentRedirect => "Permanent Redirect",
            HttpStatus::BadRequest => "Bad Request",
            HttpStatus::Unauthorized => "Unauthorized",
            HttpStatus::PaymentRequired => "Payment Required",
            HttpStatus::Forbidden => "Forbidden",
            HttpStatus::NotFound => "Not Found",
            HttpStatus::MethodNotAllowed => "Method Not Allowed",
            HttpStatus::NotAcceptable => "Not Acceptable",
            HttpStatus::ProxyAuthenticationRequired => "Proxy Authentication Required",
            HttpStatus::RequestTimeout => "Request Timeout",
            HttpStatus::Conflict => "Conflict",
            HttpStatus::Gone => "Gone",
            HttpStatus::LengthRequired => "Length Required",
            HttpStatus::PreconditionFailed => "Precondition Failed",
            HttpStatus::PayloadTooLarge => "Payload Too Large",
            HttpStatus::UriTooLong => "URI Too Long",
            HttpStatus::UnsupportedMediaType => "Unsupported Media Type",
            HttpStatus::RangeNotSatisfiable => "Range Not Satisfiable",
            HttpStatus::ExpectationFailed => "Expectation Failed",
            HttpStatus::ImATeapot => "I'm a teapot",
            HttpStatus::MisdirectedRequest => "Misdirected Request",
            HttpStatus::UnprocessableEntity => "Unprocessable Entity",
            HttpStatus::Locked => "Locked",
            HttpStatus::FailedDependency => "Failed Dependency",
            HttpStatus::TooEarly => "Too Early",
            HttpStatus::UpgradeRequired => "Upgrade Required",
            HttpStatus::PreconditionRequired => "Precondition Required",
            HttpStatus::TooManyRequests => "Too Many Requests",
            HttpStatus::RequestHeaderFieldsTooLarge => "Request Header Fields Too Large",
            HttpStatus::UnavailableForLegalReasons => "Unavailable For Legal Reasons",
            HttpStatus::InternalServerError => "Internal Server Error",
            HttpStatus::NotImplemented => "Not Implemented",
            HttpStatus::BadGateway => "Bad Gateway",
            HttpStatus::ServiceUnavailable => "Service Unavailable",
            HttpStatus::GatewayTimeout => "Gateway Timeout",
            HttpStatus::HttpVersionNotSupported => "HTTP Version Not Supported",
            HttpStatus::VariantAlsoNegotiates => "Variant Also Negotiates",
            HttpStatus::InsufficientStorage => "Insufficient Storage",
            HttpStatus::LoopDetected => "Loop Detected",
            HttpStatus::NotExtended => "Not Extended",
            HttpStatus::NetworkAuthenticationRequired => "Network Authentication Required",
        };

        write!(f, "{m}")
    }
}

impl HttpStatus {
    pub fn is_error(self) -> bool {
        let status_code: u16 = self.into();
        (400..600).contains(&status_code)
    }
}
