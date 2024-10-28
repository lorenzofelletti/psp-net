use core::fmt;

use alloc::string::String;

#[cfg(feature = "macros")]
pub mod macros;
mod request;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum ContentType {
    #[default]
    TextPlain,
    ApplicationJson,
    OctetStream,
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