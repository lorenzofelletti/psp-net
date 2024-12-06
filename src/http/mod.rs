#![allow(clippy::module_name_repetitions)]

use core::fmt;

use alloc::string::String;

#[cfg(feature = "macros")]
pub mod macros;
mod request;
mod response;

/// Enum for different supported HTTP versions
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum HttpVersion {
    /// HTTP/1
    V1,
    /// HTTP/1.1
    #[default]
    V1_1,
}

impl fmt::Display for HttpVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HttpVersion::V1 => write!(f, "HTTP/1"),
            HttpVersion::V1_1 => write!(f, "HTTP/1.1"),
        }
    }
}

/// Content Type of the HTTP packet's body.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum ContentType {
    #[default]
    /// text/plain
    TextPlain,
    /// application/json
    ApplicationJson,
    /// application/octet-stream
    OctetStream,
    /// Any other content type, as a string
    Other(String),
}

impl fmt::Display for ContentType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ContentType::TextPlain => write!(f, "text/plain"),
            ContentType::ApplicationJson => write!(f, "application/json"),
            ContentType::OctetStream => write!(f, "application/octet-stream"),
            ContentType::Other(s) => write!(f, "{s}"),
        }
    }
}

// re-exports
pub type Method = request::Method;
pub type Request = request::Request;

pub type Response<'a, 'b> = httparse::Response<'a, 'b>;
